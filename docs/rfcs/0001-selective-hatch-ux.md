# RFC 0001: Selective Hatch-Inspired UX for `fyn`

Status: Partially implemented

Author: Codex

Last updated: 2026-09-04

## Summary

`fyn` should not copy Hatch wholesale.

`fyn` should copy only the workflow UX where both `fyn` and `uv` were still thin when this RFC was
written:

1. Make the existing task runner match its documented surface. **Complete as of `fyn` 0.10.17.**
2. Make `fyn init` generate more useful project scaffolds.
3. Add a small named workflow layer for lint/docs/test/typecheck tasks.
4. Explicitly defer matrices and any plugin architecture.

This RFC is intentionally narrow. The goal is to improve common daily workflows for real users
without turning `fyn` into a second Hatch.

## Why This Exists

When this RFC was drafted, upstream `uv` already covered much more surface than older comparisons
implied, including project initialization, version management, formatting, and auditing.

The remaining gaps identified here were workflow ergonomics:

- repeatable project tasks
- useful scaffolding for new projects
- isolated named workflows for docs/lint/test/typecheck work

`fyn` 0.10.17 closes the first gap. The scaffold and isolated-workflow proposals remain prospective.

## Problem

When this RFC was drafted, `fyn` documented chained tasks and task-level environment variables, but
the runtime did not consistently implement them. Common project scaffolding still required manual
editing, and `tool.fyn.environments` described resolver scoping rather than named workflow
environments.

The task-runtime mismatch has since been resolved. The remaining problem in this RFC is narrower:
projects still need manual setup or external tooling for richer scaffolds and isolated lint, docs,
test, and type-check workflows.

## Product Thesis

`fyn` should be the fastest Python package/project manager that also covers the most common
developer workflows directly, with simple declarative config and without a plugin framework.

## Target Users

This RFC is aimed at the users who will get immediate value:

- Solo Python application developers who want repeatable local commands without adding another tool.
- Library maintainers who need docs, lint, test, and release workflows that are isolated and
  reproducible.
- Small teams standardizing a `pyproject.toml`-first workflow.
- Monorepo/workspace users who want per-project workflow config without inventing their own task
  conventions.

This RFC is not primarily aimed at:

- users who want a general plugin platform
- users who want arbitrary build hooks inside the package manager
- users who need a full CI matrix DSL on day one

## Goals

- Close the gap between documented and actual task-runner behavior.
- Make `fyn init` generate projects that are useful without immediate manual cleanup.
- Support common detached workflows with minimal new concepts.
- Preserve `fyn`'s run-centric UX.
- Avoid introducing a generic extension platform.

## Non-Goals

- Reproducing Hatch's plugin architecture.
- Reproducing Hatch's environment inheritance model.
- Adding build hooks, metadata hooks, publisher plugins, or version-source plugins.
- Adding a full matrix/filter/collector system in the first iteration.
- Adding new top-level commands like `fyn test` or `fyn fmt` in this RFC.

## Proposal

## Part 1: Finish the Task Runner — Complete

### Shipped state

As of `fyn` 0.10.17, `[tool.fyn.tasks]` supports:

- string command tasks
- fail-fast arrays of command strings
- detailed tasks with `cmd`, `description`, and `env`
- named `chain` tasks with environment inheritance and cycle detection
- graph-aware workspace execution with package filters

Workspace task runs synchronize once, run active workspace dependencies before dependents, and run
independent members in parallel by default. Marker evaluation, selected extras and dependency
groups, transitively activated extras, and workspace source selection contribute to the active
graph.

### Shipped behavior

The documented task surface is implemented:

```toml
[tool.fyn.tasks]
lint = "ruff check ."
test = { cmd = "pytest -q", env = { PYTHONWARNINGS = "error" } }
verify = ["ruff check .", "pytest -q"]
check = { chain = ["lint", "test"], description = "Run lint and tests" }
```

#### `cmd`

- Executes exactly as today.

#### Command sequences

- An array of command strings executes in order.
- Execution stops on the first failure.

#### `env`

- Merges into the child process environment for that task.
- If a chain task has `env`, that environment is inherited by child tasks.
- Child task `env` values override parent chain values.
- Effective precedence is: process environment, then chain-task `env`, then leaf-task `env`.

#### `chain`

- Executes tasks sequentially in the declared order.
- Stops on first failure.
- Prints which child task is currently running.
- Rejects cycles with a clear error.

#### Extra CLI args

Initial version:

- Extra args continue to work for `cmd` tasks.
- Extra args are rejected for `chain` tasks with a clear error message.

Rationale:

- this avoids ambiguous behavior
- it solves the main usefulness gap immediately
- it keeps the first implementation small and predictable

#### Workspace execution

- `fyn run --workspace <task>` runs the named task in selected child members that define it.
- The shared workspace environment is synchronized once before task execution unless `--no-sync` is
  used.
- Active workspace dependencies finish before dependents; independent members run in parallel by
  default.
- Exact package filters can be expanded through active dependencies or dependents.

### Why this is useful

This completed the highest-value, lowest-risk part of the RFC:

- it closed the documented task/runtime mismatch
- it covers common fail-fast project command workflows directly
- it provides the foundation for scaffolded projects and isolated workflows later

## Part 2: Make `fyn init` More Useful

### Current state

`fyn init` already handles:

- app vs lib
- package vs non-package
- build backend selection
- script initialization
- workspace integration

That base is good. The gap is common workflow scaffolding after the project exists.

### Proposed additions

Add a small set of high-value presets:

- `--cli`
- `--tests`

These should be non-interactive first. If an interactive mode is added later, it should only be a
selector for these same presets, not a separate scaffolding model.

### Proposed semantics

#### `--cli`

For application projects:

- creates a packaged app layout
- adds a `[project.scripts]` entrypoint
- generates a `main()`-style executable module if one does not already exist

This should be an ergonomic shortcut over the existing `--app --package` flow.

#### `--tests`

- creates a `tests/` directory
- adds a minimal example test
- adds a test dependency group or equivalent project dependency declaration
- adds useful default tasks, e.g.:

```toml
[tool.fyn.tasks]
test = "pytest -q"
```

### Explicit follow-up, not v1

Possible later preset:

- `--ci github`

This should only be considered after the base presets prove useful. It is less universal than
`--cli` and `--tests`, and it should remain plain file generation rather than the start of a
templating framework.

### Why this is useful

This makes `fyn init` generate something closer to what people actually keep after the first commit.
It reduces the current pattern of:

1. `fyn init`
2. hand-edit `pyproject.toml`
3. add tests
4. add tasks
5. add CI

into a smaller, more direct path.

## Part 3: Add Minimal Named Workflows

### Problem this solves

Many projects want isolated environments for:

- linting
- documentation
- type checking
- integration tests with extra tooling

Today, these workflows usually become:

- ad hoc shell scripts
- external task runners
- hand-maintained local virtual environments

Tasks and workflows should serve different jobs:

- tasks are project commands that run in the project environment
- workflows are detached toolchain environments for repo maintenance tasks

### Design principle

This should be a small workflow layer on top of `fyn`'s existing resolver and environment machinery,
not a new environment framework.

This RFC intentionally does not use the name `envs` for the new feature, because `fyn` already uses
`tool.fyn.environments` for resolver marker scoping.

### Proposed config

Add a new table:

```toml
[tool.fyn.workflows.lint]
python = "3.12"
dependencies = ["ruff>=0.7"]
env = { RUFF_OUTPUT_FORMAT = "full" }

[tool.fyn.workflows.lint.scripts]
check = "ruff check ."
format = "ruff format ."

[tool.fyn.workflows.docs]
python = "3.12"
dependencies = ["mkdocs-material", "mkdocs-redirects"]

[tool.fyn.workflows.docs.scripts]
serve = "mkdocs serve"
build = "mkdocs build --strict"
```

### Version 1 semantics

Named workflows are intentionally limited:

- they resolve to detached cached environments
- they do not install the current project
- they do not inherit from each other
- they do not support matrices
- they do not define custom environment types

Supported fields in v1:

- `python`
- `dependencies`
- `env`
- `scripts`

Optional future field, but not required in v1:

- `description`

Explicitly deferred in v1:

- dependency-group support
- extras support

### CLI shape

To preserve `fyn`'s run-centric UX, do not add a top-level `fyn env` namespace in v1.

Instead:

- `fyn run <workflow>:<script>`
- `fyn run --workflow <workflow> -- <command>...`

Examples:

```console
$ fyn run lint:check
$ fyn run lint:format
$ fyn run --workflow docs -- mkdocs serve
```

### Behavior

- The workflow environment is created and cached automatically on first use.
- It is updated when its config or dependency inputs change.
- It uses the same index/auth/config machinery as the rest of `fyn`.
- Target parsing prefers workflow syntax only when the prefix matches a declared workflow name.
  Otherwise, existing `fyn run` target resolution rules continue to apply.
- The workflow cache key must include at least: project root, workflow name, Python request,
  normalized dependency list, and a hash of the workflow config table.

### Why this is useful

This is the smallest meaningful piece of Hatch worth borrowing after tasks:

- it covers real workflows people already have
- it keeps project `.venv` focused on the project itself
- it replaces many one-off shell scripts
- it avoids a separate tool for common docs/lint/typecheck workflows

## Deferred: Matrix UX

Matrix UX is useful, but it should be explicitly deferred.

Examples of deferred matrix behavior:

- `python = ["3.11", "3.12", "3.13"]`
- include/exclude filters
- env inheritance with matrix axes
- dedicated `test` command semantics

### Why defer it

- it adds a lot of behavior surface quickly
- it is less useful until workflows exist
- it risks recreating too much of Hatch's model

The right sequence is:

1. make tasks real
2. make init useful
3. make workflows useful
4. then evaluate matrices from real user demand

## Explicitly Rejected

The following are out of scope for this RFC:

- Hatch-style build hooks
- metadata hooks
- version-source plugins
- publisher plugins
- environment collectors
- full environment inheritance
- custom environment types
- a new plugin API for `fyn`

These are expensive to maintain and do not solve the most common user pain first.

## User-Facing Examples

### Example: small library

```toml
[project]
name = "acme-lib"
version = "0.1.0"
requires-python = ">=3.11"
dependencies = []

[tool.fyn.tasks]
test = "pytest -q"
check = { chain = ["test"] }

[tool.fyn.workflows.lint]
python = "3.12"
dependencies = ["ruff>=0.7"]

[tool.fyn.workflows.lint.scripts]
check = "ruff check ."
format = "ruff format ."
```

```console
$ fyn run check
$ fyn run lint:check
$ fyn run lint:format
```

### Example: docs workflow

```toml
[tool.fyn.workflows.docs]
python = "3.12"
dependencies = ["mkdocs-material", "mkdocs-redirects"]

[tool.fyn.workflows.docs.scripts]
serve = "mkdocs serve"
build = "mkdocs build --strict"
```

```console
$ fyn run docs:serve
$ fyn run docs:build
```

## Implementation Plan

### Phase 1: Task runner parity — Complete

Completed by `fyn` 0.10.17:

- implemented `chain` and inherited task `env`
- added fail-fast command sequences
- added task validation and cycle diagnostics
- added graph-aware workspace execution, filtering, dependency ordering, and parallel scheduling
- aligned the reference documentation and integration tests with the shipped behavior

The current task reference is in
[Running commands in projects](../concepts/projects/run.md#running-project-tasks).

### Phase 2: Better init presets

Scope:

- add `--cli`
- add `--tests`
- scaffold matching tasks where appropriate

Success criteria:

- a new project can be initialized with a useful local workflow in one command

### Phase 3: Minimal named workflows

Scope:

- add config parsing for `tool.fyn.workflows`
- add `fyn run <workflow>:<script>`
- add `fyn run --workflow <workflow> -- <command>...`
- cache and invalidate workflow environments correctly

Success criteria:

- docs/lint/typecheck workflows can run in isolated named workflows without external tooling

### Phase 4: Reassess matrices

Only begin this phase if:

- workflows are being used
- users are asking for multi-Python or multi-axis workflows
- the implementation pressure is coming from real use cases, not tool envy

## Risks

- scope creep toward a full Hatch clone
- confusing overlap between tasks and workflow scripts
- surprising storage/update behavior for workflow environments
- over-scaffolding `init` with too many opinionated choices

## Mitigations

- keep v1 of workflows detached and simple
- keep `run` as the central UX instead of multiplying top-level commands
- add only a few scaffold presets with strong defaults
- explicitly reject plugin work in this track

## Open Questions

1. Should `fyn init --tests` add a `test` dependency group, `dev` group, or plain dependencies?
2. Should workflow environments be stored in the cache, under project-local metadata, or behind an
   internal abstraction with no user-visible promise yet?
3. Should `--list-tasks` eventually gain awareness of `<workflow>:<script>` targets, or remain
   task-only?

## Recommendation

Phase 1 is complete. Continue with Phase 2 before introducing named workflows or matrices.

The original gate—do not start workflows or matrices until task-runner parity is complete—was met by
`fyn` 0.10.17. The remaining phases should still proceed only from demonstrated user demand and with
the scope limits above.

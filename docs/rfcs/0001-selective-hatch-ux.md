# RFC 0001: Selective Hatch-Inspired UX for `fyn`

Status: Draft

Author: Codex

Last updated: 2026-04-13

## Summary

`fyn` should not copy Hatch wholesale.

`fyn` should copy only the workflow UX where both `fyn` and `uv` are still thin:

1. Make the existing task runner actually match its documented surface.
2. Make `fyn init` generate more useful project scaffolds.
3. Add a small named workflow layer for lint/docs/test/typecheck tasks.
4. Explicitly defer matrices and any plugin architecture.

This RFC is intentionally narrow. The goal is to improve common daily workflows for real users
without turning `fyn` into a second Hatch.

## Why This Exists

Recent upstream `uv` already covers much more surface than older comparisons imply, including
project initialization, version management, formatting, and auditing.

That means the remaining gap is not core package-management capability. The remaining gap is
workflow ergonomics:

- repeatable project tasks
- useful scaffolding for new projects
- isolated named workflows for docs/lint/test/typecheck work

Those are the areas where Hatch is still meaningfully ahead.

## Problem

`fyn` currently has a workflow story, but parts of it are incomplete or too thin:

- `fyn` documents chained tasks, but the runtime still errors on `chain` tasks.
- Task definitions also document `env`, but the runtime does not appear to apply it.
- `fyn init` is already solid for app/lib/package creation, but common follow-up steps still need
  hand editing.
- `tool.fyn.environments` already exists, but it is resolver scoping, not named workflow
  environments.

This creates two problems:

1. Users still reach for shell scripts, Makefiles, or external tools for routine workflows.
2. `fyn` already advertises a richer UX than it fully delivers.

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

## Part 1: Finish the Task Runner

### Current state

`[tool.fyn.tasks]` already supports:

- string tasks
- `description`
- `chain`
- `env`

But only simple command execution is working consistently today.

### Proposed behavior

Implement the documented task surface in full:

```toml
[tool.fyn.tasks]
lint = "ruff check ."
test = { cmd = "pytest -q", env = { PYTHONWARNINGS = "error" } }
check = { chain = ["lint", "test"], description = "Run lint and tests" }
```

#### `cmd`

- Executes exactly as today.

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

### Why this is useful

This is the highest-value, lowest-risk change in the entire RFC:

- it fixes a current doc/runtime mismatch
- it removes immediate need for Makefiles or shell wrappers in many projects
- it creates a better foundation for scaffolded projects and workflows later

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

This makes `fyn init` generate something closer to what people actually keep after the first
commit. It reduces the current pattern of:

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

This should be a small workflow layer on top of `fyn`'s existing resolver and environment
machinery, not a new environment framework.

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
- The workflow cache key must include at least:
  project root, workflow name, Python request, normalized dependency list, and a hash of the
  workflow config table.

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

### Phase 1: Task runner parity

Scope:

- implement `chain`
- implement `env`
- add task-runner tests
- align docs with actual behavior

Success criteria:

- all documented task examples work
- `chain` no longer errors
- `env` affects the launched child process

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
2. Should workflow environments be stored in the cache, under project-local metadata, or behind an internal
   abstraction with no user-visible promise yet?
3. Should `--list-tasks` eventually gain awareness of `<workflow>:<script>` targets, or remain
   task-only?

## Recommendation

Adopt this RFC in order, with one hard rule:

do not start workflows or matrices until task-runner parity is complete.

That sequencing gives users immediate value, removes a current mismatch in the product surface, and
keeps `fyn` focused on useful workflow UX rather than architectural imitation.

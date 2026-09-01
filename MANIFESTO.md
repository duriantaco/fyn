# fyn — Python projects, under control

**fyn** is an independent community fork of [uv](https://github.com/astral-sh/uv), focused on the
project workflow around Python packaging: reviewable dependency changes, explainable state, and
repository-owned policy.

A fast resolver is valuable, but it is only part of maintaining a project. Contributors also need to
know which environment they are using, why a dependency is present, what a lockfile update will
change, which commands the repository expects them to run, and whether local state matches CI.

fyn's product thesis is that a Python repository should act as an executable, inspectable contract
from clone to CI.

## Product principles

### Review before writing

Dependency changes should be visible before they land. `fyn lock diff` resolves in dry-run mode and
reports added, removed, changed, and metadata-only lockfile entries without writing `fyn.lock`.

`fyn upgrade --dry-run` previews both requirement and lockfile edits. This makes the impact of an
upgrade reviewable before the manifest, lockfile, or environment changes.

### Keep related changes transactional

A dependency upgrade crosses several pieces of project state. `fyn upgrade` updates
`pyproject.toml`, `fyn.lock`, and the project environment as one workflow while preserving existing
constraint style where possible. If locking or syncing fails, fyn restores the manifest and lockfile
instead of leaving a half-applied change behind.

### Explain project state

Project tooling should answer operational questions directly:

- `fyn why <package>` shows the dependency paths that include a package.
- `fyn audit --explain` includes dependency paths for vulnerability and adverse-status findings.
- `fyn status` reports the discovered project, lockfile, environment, and interpreter.
- `fyn status --check` turns missing or mismatched state into actionable failures for local tooling
  and CI, while `--json` exposes the same information to integrations.

Dependency auditing is also available in current uv. fyn's emphasis is the coherent inspection
workflow across dependency causes, audit findings, proposed lock changes, and current project
health.

### Keep workflow and policy in the repository

fyn lets a project carry more of its operating contract in `pyproject.toml`: named commands,
required fyn versions, managed-environment guardrails, dependency groups, sources, and workspace
configuration. `fyn run` keeps the selected project environment aligned before executing a command,
and `fyn shell` provides an explicit way to enter it.

Direct environment mutation is a policy choice rather than an accident. Projects can set
`pip-in-project = "warn" | "error" | "allow"` to control mutating `fyn pip` commands inside a
managed project.

### Make package-source policy explicit

Package source decisions should remain reviewable in project metadata. A project can pin a package
to a named, explicit index through `tool.fyn.sources`; fyn's default `first-index` strategy limits
candidates to the first index containing a package name to reduce dependency-confusion risk.

Developer machines and CI can replace the URL for an already-declared index name in user- or
system-level `fyn.toml`. That preserves the project's logical source policy while allowing different
mirrors or private endpoints. Local configuration cannot invent a new source pin that the project
did not declare.

### Keep network metadata claims precise

Compared with the upstream behavior that motivated this change, fyn removes extra LineHaul
environment metadata from the package-index `User-Agent` header and sends a minimal `fyn/<version>`
value instead.

This reduces what is exposed in that header, but it does not make package installation anonymous.
Package indexes still receive normal network and request information, including IP addresses and the
requested packages.

## Relationship with uv

fyn retains substantial uv ancestry and should feel familiar to uv users. The projects share many
capabilities, including high-performance dependency resolution, universal locking, Python and tool
management, script execution, dependency auditing, cache controls, and PyTorch backend selection.
Those shared capabilities are foundations, not honest product differentiators.

fyn has its own `[tool.fyn]` namespace, `fyn.lock`, commands, defaults, policies, and release path.
Its direction is centered on:

- reviewable and transactional dependency changes
- an inspection surface spanning `lock diff`, `why`, `audit --explain`, and `status`
- repository-owned workflow and managed-environment guardrails
- explicit package-index policy with stable logical names and environment-specific endpoints

This is a statement of product emphasis, not a claim that current uv lacks every individual
primitive. Both projects continue to evolve.

## Origins

fyn began after OpenAI announced an agreement to acquire Astral, prompting the creation of an
independent, community-maintained path. The fork was not a rejection of uv's engineering: uv
provided a strong foundation, and fyn continues to acknowledge that work and its shared ancestry.

Independence explains the separate stewardship, namespace, metadata choices, and release policy. The
daily reason to use fyn, however, is the project workflow described above.

## Direction

Development follows a staged direction rather than an open-ended feature checklist:

1. **Earn adoption trust.** Keep installation, migration, CI recipes, compatibility boundaries, and
   user-facing documentation accurate and tested. Harden the existing review, rollback, status, and
   index-policy behavior.
2. **Complete the repository contract.** Improve useful project scaffolds and make project and
   workspace workflows consistent, so a repository can define the commands and checks contributors
   need without immediate hand-written glue.
3. **Add focused named workflows.** Support isolated, cached environments for common lint, test,
   type-check, and documentation work while preserving fyn's simple, run-centric interface.
4. **Expand from demonstrated demand.** Evaluate matrices and deeper integrations after the core
   workflow is proven. A plugin platform, bundling, or Conda support are not commitments ahead of
   these stages.

## Installation

From PyPI:

```console
$ pipx install fyn
```

Or build the current checkout from source:

```console
$ cargo install --path crates/fyn
```

## Migrating from uv to fyn

Most day-to-day command shapes and `UV_*` environment variables carry over. Within `pyproject.toml`,
fyn reads `[tool.fyn]` configuration and falls back to `[tool.uv]` when `[tool.fyn]` is absent,
which allows existing project configuration to be evaluated before it is rewritten. Standalone
configuration files do not use that fallback.

fyn uses `fyn.lock` by default. For a dedicated migration, rename `uv.lock` to `fyn.lock`, rename a
project-level `uv.toml` to `fyn.toml` if present, and rename every `[tool.uv...]` table header in
`pyproject.toml` to its `[tool.fyn...]` equivalent. Copy any user- or system-level settings you
still need into fyn's corresponding `fyn/fyn.toml`
[configuration directory](docs/concepts/configuration-files.md), and review the result before
syncing:

```console
$ fyn lock diff
$ fyn sync
$ fyn status --check
```

Treat the migration as a normal repository change: use a branch and review the metadata and lockfile
diff rather than applying an unchecked text substitution.

## Can I help?

Yes. Open a pull request, file an issue, or share a reproducible project workflow that is harder
than it should be. Proposed features are evaluated against the product principles and staged
direction above.

## License

MIT or Apache-2.0, same as uv. Pick whichever you prefer.

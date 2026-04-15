# fyn — A community fork of uv with its own commands and config

**fyn** is a Python package manager and project manager written in Rust. It's an independent fork of
[uv](https://github.com/astral-sh/uv), which is the fastest Python package installer around. It
started as a relatively close fork, but fyn now ships fork-specific commands, config namespace,
defaults, policies, and behavior, along with a smaller package-index `User-Agent`, added features,
and bug fixes.

If you've used uv, fyn should still feel familiar. Many day-to-day commands carry over and migration
is usually straightforward, but fyn is no longer just uv with a different binary name. The main
concrete differences are below.

## Why fork uv?

uv got acquired by OpenAI. That's it, really. I don't really know what else to say.

uv is great software. Genuinely. It's fast, it works, the Rust code is clean. But when a big corp
buys your package manager, you start wondering what happens next. Maybe nothing changes. We'd rather
not find out.

So we forked it. We called it **fyn**. You can infer the first letter however you want.

## What's actually different

We're not trying to rewrite the thing from scratch. uv is really solid and the shared ancestry still
matters. But fyn has diverged enough that "uv with a smaller package-index `User-Agent`" is no
longer a very good description. The project now has its own command surface, project conventions,
policies, and bug fixes.

### Reduced package-index request metadata

uv included LineHaul metadata in the `User-Agent` header it sent to package indexes such as PyPI.
That header goes to the package index, not back to Astral or OpenAI. `pip` also sends package-index
environment information by default and does not describe that as telemetry in its privacy notice;
fyn's change is narrower and specific to the `User-Agent` header. Compared with upstream uv, fyn
removes the extra LineHaul environment metadata from that header and sends a minimal `fyn/<version>`
`User-Agent` instead.

This reduces what is exposed in the header, but it does not make package installs anonymous. Package
indexes still see normal network and request information, including your IP address and the packages
you ask for.

### Some of the larger user-visible differences

- **Fork-specific project namespace** — fyn uses `[tool.fyn]` and `fyn.lock`, not `[tool.uv]` and
  `uv.lock`.
- **Task runner** — define project tasks in `[tool.fyn.tasks]` and list them with
  `fyn run --list-tasks`.
- **`fyn shell`**
- **`fyn upgrade`**
- **`fyn status`**
- **Managed-project pip guardrails** — configure `pip-in-project = "warn" | "error" | "allow"` to
  control direct `fyn pip` mutations inside managed projects.
- **Cache size limits** — set `UV_CACHE_MAX_SIZE=2G` and the cache cleans itself. uv's cache just
  grew forever.
- **Custom lockfile name** — set `UV_LOCKFILE` to use a non-default lockfile path.

## fyn vs uv — feature comparison

| Area                            | uv                                    | fyn                                       |
| ------------------------------- | ------------------------------------- | ----------------------------------------- |
| Speed (10-100x faster than pip) | Yes                                   | Yes                                       |
| Config namespace and lockfile   | `[tool.uv]`, `uv.lock`                | `[tool.fyn]`, `fyn.lock`                  |
| Package index User-Agent        | `uv/<version>` plus LineHaul metadata | Minimal `fyn/<version>`                   |
| Task runner                     | No `[tool.uv.tasks]`                  | `[tool.fyn.tasks]`                        |
| `shell` command                 | No `uv shell`                         | `fyn shell`                               |
| `upgrade` command               | No `uv upgrade`                       | `fyn upgrade`                             |
| `status` command                | No `uv status`                        | `fyn status`                              |
| Managed-project `pip` policy    | No `pip-in-project` setting           | `pip-in-project = warn \| error \| allow` |
| Cache size limit                | No `UV_CACHE_MAX_SIZE`                | `UV_CACHE_MAX_SIZE`                       |
| Custom lockfile name            | No `UV_LOCKFILE`                      | `UV_LOCKFILE`                             |
| Python version management       | Yes                                   | Yes                                       |
| Lockfile support                | Yes (`uv.lock`)                       | Yes (`fyn.lock`)                          |

## Roadmap

In no particular order:

1. **Centralized venv storage** — keep .venvs out of your project dirs
2. **Glob-pattern source declarations** — `mycompany-*` all goes to your private index, borrowed
   from how PDM does it
3. **`pip.conf` support** — read your existing pip config
4. **`pip wheel`**
5. **Plugin system** eventually — custom indexes, auth providers, etc.
6. **`fyn bundle`** — make standalone executables, like PyInstaller but built-in
7. **Conda support** maybe — if we can do it without making a mess

## Installation

From PyPI:

```bash
pip install fyn
```

From source:

```bash
cargo install --path crates/fyn
```

## Migrating from uv to fyn

Migration takes about 30 seconds:

```bash
# 1. Rename your lockfile
mv uv.lock fyn.lock

# 2. In pyproject.toml, rename [tool.uv] to [tool.fyn]
sed -i 's/\[tool\.uv\]/[tool.fyn]/' pyproject.toml

# 3. Use fyn instead of uv
fyn sync
fyn run pytest
```

Config files move from `~/.config/uv/` to `~/.config/fyn/`. Environment variables (`UV_*`) still
work.

## Can I help?

Yeah. Open a PR, file an issue, or just tell us what's broken. If there's a feature from another
Python tool that you think belongs here, tell us.

## License

MIT or Apache-2.0, same as uv. Pick whichever you like.

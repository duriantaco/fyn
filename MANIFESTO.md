# fyn — A uv fork with a smaller package-index `User-Agent`

**fyn** is a Python package manager and project manager written in Rust. It's an independent fork of
[uv](https://github.com/astral-sh/uv), which is the fastest Python package installer around, with a
smaller package-index `User-Agent`, missing features added, and bugs fixed.

If you've used uv, you already know fyn. Same commands, same speed, easy migration. The main
behavior differences are below.

## Why fork uv?

uv got acquired by OpenAI. That's it, really. I don't really know what else to say.

uv is great software. Genuinely. It's fast, it works, the Rust code is clean. But when a big corp
buys your package manager, you start wondering what happens next. Maybe nothing changes. We'd rather
not find out.

So we forked it. We called it **fyn**. You can infer the first letter however you want.

## What's actually different

We're not trying to rewrite the thing. uv is really solid and we're keeping most of it. We're just
fixing the stuff that bugged us and adding the things people kept asking for.

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

### New features we added

- **`fyn shell`**
- **`fyn upgrade`**
- **Cache size limits** — set `UV_CACHE_MAX_SIZE=2G` and the cache cleans itself. uv's cache just
  grew forever.

## fyn vs uv — feature comparison

| Feature                         | uv                                    | fyn                     |
| ------------------------------- | ------------------------------------- | ----------------------- |
| Speed (10-100x faster than pip) | Yes                                   | Yes                     |
| Package index User-Agent        | `uv/<version>` plus LineHaul metadata | Minimal `fyn/<version>` |
| `shell` command                 | Not available                         | `fyn shell`             |
| `upgrade` command               | Must chain two commands               | `fyn upgrade`           |
| Cache size limit                | No limit                              | `UV_CACHE_MAX_SIZE`     |
| Custom lockfile name            | Not available                         | `UV_LOCKFILE`           |
| `remove --group` sync behavior  | Wipes other group packages            | Fixed                   |
| Drop-in replacement for pip     | Yes                                   | Yes                     |
| Python version management       | Yes                                   | Yes                     |
| Lockfile support                | Yes (`uv.lock`)                       | Yes (`fyn.lock`)        |

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

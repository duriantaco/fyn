# fv — A uv fork, fast, private Python Package Manager

**fv** is a Python package manager and project manager written in Rust. It's an independent fork of
[uv](https://github.com/astral-sh/uv), which is the fastest Python package installer around- without
telemetry, missing features added, and bugs fixed.

If you've used uv, you already know fv. Same commands, same speed, easy migration. Just fewer things
phoning home.

## Why fork uv?

uv got acquired by OpenAI. That's it, really. I don't really know what else to say.

uv is great software. Genuinely. It's fast, it works, the Rust code is clean. But when a big corp
buys your package manager, you start wondering what happens next. Maybe nothing changes. We'd rather
not find out.

So we forked it. We called it **fv**. You can infer the first letter however you want.

## What's actually different

We're not trying to rewrite the thing. uv is really solid and we're keeping most of it. We're just
fixing the stuff that bugged us and adding the things people kept asking for.

### No telemetry — your installs are your business

uv was sending a surprising amount of info to package indexes every time you installed something.
These things include your OS, py version, CPU architecture, Linux distro, whether you're in CI. All
baked into the User-Agent header via something called "linehaul". We ripped that out. Now it just
sends `fv/0.10.13`. That's it.

### New features we added

- **`fv shell`**
- **`fv upgrade`**
- **Cache size limits** — set `UV_CACHE_MAX_SIZE=2G` and the cache cleans itself. uv's cache just
  grew forever.

## fv vs uv — feature comparison

| Feature                           | uv                             | fv                  |
| --------------------------------- | ------------------------------ | ------------------- |
| Speed (10-100x faster than pip)   | Yes                            | Yes                 |
| Telemetry / system profiling      | Sends OS, Python, CPU, CI info | None                |
| `shell` command                   | Not available                  | `fv shell`          |
| `upgrade` command                 | Must chain two commands        | `fv upgrade`        |
| Cache size limit                  | No limit                       | `UV_CACHE_MAX_SIZE` |
| Private index for transitive deps | Broken                         | Fixed               |
| Env vars in index URLs            | Only in requirements.txt       | Everywhere          |
| Custom lockfile name              | Not available                  | `UV_LOCKFILE`       |
| `remove --group` sync behavior    | Wipes other group packages     | Fixed               |
| Drop-in replacement for pip       | Yes                            | Yes                 |
| Python version management         | Yes                            | Yes                 |
| Lockfile support                  | Yes (`uv.lock`)                | Yes (`fv.lock`)     |

## Roadmap

In no particular order:

1. **Task runner** in `fv run` — like `npm run` but for Python
2. **Centralized venv storage** — keep .venvs out of your project dirs
3. **Glob-pattern source declarations** — `mycompany-*` all goes to your private index, borrowed
   from how PDM does it
4. **`pip.conf` support** — read your existing pip config
5. **`pip wheel` and `pip download` commands**
6. **Plugin system** eventually — custom indexes, auth providers, etc.
7. **`fv bundle`** — make standalone executables, like PyInstaller but built-in
8. **Conda support** maybe — if we can do it without making a mess

## Installation

From PyPI:

```bash
pip install fv
```

From source:

```bash
cargo install --path crates/fv
```

## Migrating from uv to fv

Migration takes about 30 seconds:

```bash
# 1. Rename your lockfile
mv uv.lock fv.lock

# 2. In pyproject.toml, rename [tool.uv] to [tool.fv]
sed -i 's/\[tool\.uv\]/[tool.fv]/' pyproject.toml

# 3. Use fv instead of uv
fv sync
fv run pytest
```

Config files move from `~/.config/uv/` to `~/.config/fv/`. Environment variables (`UV_*`) still
work.

## Can I help?

Yeah. Open a PR, file an issue, or just tell us what's broken. If there's a feature from another
Python tool that you think belongs here, tell us.

## License

MIT or Apache-2.0, same as uv. Pick whichever you like.

# fyn documentation

This directory is the source of truth for fyn's documentation. The published MkDocs site uses
[index.md](./index.md) as its home page; this README exists so the `docs/` directory also has a
useful landing page when people browse the repository on GitHub.

## Start here

- [Install fyn](./getting-started/installation.md)
- [First steps](./getting-started/first-steps.md)
- [Working on projects](./guides/projects.md)
- [Running scripts](./guides/scripts.md)
- [Using tools](./guides/tools.md)
- [Command reference](./reference/cli.md)
- [Settings reference](./reference/settings.md)

## Documentation map

- [Getting started](./getting-started/index.md)
- [Guides](./guides/index.md)
- [Concepts](./concepts/index.md)
- [Reference](./reference/index.md)
- [RFCs](./rfcs/index.md)

## Preview locally

```console
$ cargo run --bin fyn -- run --isolated --only-group docs mkdocs serve -f mkdocs.yml
```

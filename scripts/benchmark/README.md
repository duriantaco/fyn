# benchmark

Benchmarking scripts for fv and other package management tools.

## Getting Started

From the `scripts/benchmark` directory:

```shell
fv run resolver \
    --uv-pip \
    --poetry \
    --benchmark \
    resolve-cold \
    ../requirements/trio.in
```

# benchmark

Benchmarking scripts for fyn and other package management tools.

## Getting Started

From the `scripts/benchmark` directory:

```shell
fyn run resolver \
    --uv-pip \
    --poetry \
    --benchmark \
    resolve-cold \
    ../requirements/trio.in
```

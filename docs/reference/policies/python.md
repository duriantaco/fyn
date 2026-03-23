# Python support

## Python versions

fyn has Tier 1 support for the following Python versions:

- 3.10
- 3.11
- 3.12
- 3.13
- 3.14

As with [platforms](./platforms.md), Tier 1 support can be thought of "guaranteed to work". fyn is
continuously tested against these versions.

fyn has Tier 2 support for:

- 3.6
- 3.7
- 3.8
- 3.9

fyn is "expected to work" with these versions. fyn is tested against these versions, but they have
reached their [end-of-life](https://devguide.python.org/versions/) and no longer receive security
fixes. We do not recommend using these versions.

fyn also has Tier 2 support for pre-releases of Python 3.15.

fyn does not work with Python versions prior to 3.6.

## Python implementations

fyn has Tier 1 support for the following Python implementations:

- CPython

As with [platforms](./platforms.md), Tier 1 support can be thought of "guaranteed to work". fyn
supports managed installations of these implementations, and the builds are maintained by Astral.

fyn has Tier 2 support for:

- PyPy
- GraalPy
- Pyodide

fyn is "expected to work" with these implementations. fyn also supports managed installations of
these Python implementations, but the builds are not maintained by Astral.

fyn has Tier 3 support for:

- Pyston

fyn "should work" with these implementations, but stability may vary.

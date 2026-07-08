# Security policy

fyn is a Python package manager. Due to the design of the Python packaging ecosystem and the dynamic
nature of Python itself, there are many cases where fyn can execute arbitrary code. For example:

- fyn invokes Python interpreters on the system to retrieve metadata
- fyn builds source distributions as described by PEP 517
- fyn may build packages from the requested package indexes

These are not considered vulnerabilities in fyn. If you think fyn's stance in these areas can be
hardened, please file an issue for a new feature.

## Reporting a vulnerability

If you believe you have found a vulnerability that is in scope for the project, please report it
privately via
[GitHub private vulnerability reporting](https://github.com/duriantaco/fyn/security/advisories/new).
Please do not report security vulnerabilities through public issues, discussions, or pull requests.

We will acknowledge your report as soon as possible and keep you informed as we work on a fix.

fyn is a fork of [uv](https://github.com/astral-sh/uv). If a vulnerability also affects uv, please
additionally report it to the uv maintainers as described in their
[security policy](https://github.com/astral-sh/.github/blob/main/SECURITY.md) so that both projects
can be fixed.

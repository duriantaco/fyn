# Security policy

fyn is a Python package manager. Due to the design of the Python packaging ecosystem and the dynamic
nature of Python itself, there are many cases where fyn can execute arbitrary code. For example:

- fyn invokes Python interpreters on the system to retrieve metadata
- fyn builds source distributions as described by PEP 517
- fyn may build packages from the requested package indexes

These are not considered vulnerabilities in fyn. If you think uv's stance in these areas can be
hardened, please file an issue for a new feature.

If you believe you have found a vulnerability that is in scope for the project, please contact us as
described in the organization
[Security Policy](https://github.com/astral-sh/.github/blob/main/SECURITY.md).

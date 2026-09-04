# [Package indexes](#package-indexes)

By default, fyn uses the [Python Package Index (PyPI)](https://pypi.org) for dependency resolution and package installation. However, fyn can be configured to use other package indexes, including private indexes, via the `[[tool.fyn.index]]` configuration option (and `--index`, the analogous command-line option).

## [Defining an index](#defining-an-index)

To include an additional index when resolving dependencies, add a `[[tool.fyn.index]]` entry to your `pyproject.toml`:

```
[[tool.fyn.index]]
# Optional name for the index.
name = "pytorch"
# Required URL for the index.
url = "https://download.pytorch.org/whl/cpu"
```

Indexes are prioritized in the order in which they’re defined, such that the first index listed in the configuration file is the first index consulted when resolving dependencies, with indexes provided via the command line taking precedence over those in the configuration file.

By default, fyn includes the Python Package Index (PyPI) as the "default" index, i.e., the index used when a package is not found on any other index. To exclude PyPI from the list of indexes, set `default = true` on another index entry (or use the `--default-index` command-line option):

```
[[tool.fyn.index]]
name = "pytorch"
url = "https://download.pytorch.org/whl/cpu"
default = true
```

The default index is always treated as lowest priority, regardless of its position in the list of indexes.

Index names may only contain alphanumeric characters, dashes, underscores, and periods, and must be valid ASCII.

When providing an index on the command line (with `--index` or `--default-index`) or through an environment variable (`UV_INDEX` or `UV_DEFAULT_INDEX`), names are optional but can be included using the `<name>=<url>` syntax, as in:

```
# On the command line.
$ fyn lock --index pytorch=https://download.pytorch.org/whl/cpu
# Via an environment variable.
$ UV_INDEX=pytorch=https://download.pytorch.org/whl/cpu fyn lock
```

## [Pinning a package to an index](#pinning-a-package-to-an-index)

A package can be pinned to a specific index by specifying the index in its `tool.fyn.sources` entry. For example, to ensure that `torch` is *always* installed from the `pytorch` index, add the following to your `pyproject.toml`:

```
[tool.fyn.sources]
torch = { index = "pytorch" }

[[tool.fyn.index]]
name = "pytorch"
url = "https://download.pytorch.org/whl/cpu"
```

Similarly, to pull from a different index based on the platform, you can provide a list of sources disambiguated by environment markers:

pyproject.toml

```
[project]
dependencies = ["torch"]

[tool.fyn.sources]
torch = [
  { index = "pytorch-cu118", marker = "sys_platform == 'darwin'"},
  { index = "pytorch-cu124", marker = "sys_platform != 'darwin'"},
]

[[tool.fyn.index]]
name = "pytorch-cu118"
url = "https://download.pytorch.org/whl/cu118"

[[tool.fyn.index]]
name = "pytorch-cu124"
url = "https://download.pytorch.org/whl/cu124"
```

## [Routing package patterns to an index](#routing-package-patterns-to-an-index)

If you want a whole package namespace to prefer a specific index, add `include-packages` or `exclude-packages` to `[[tool.fyn.index]]`:

```
[[tool.fyn.index]]
name = "internal"
url = "https://packages.example.com/simple"
include-packages = ["mycompany-*"]

[[tool.fyn.index]]
name = "pypi"
url = "https://pypi.org/simple"
default = true
```

When a package matches `include-packages`, fyn only considers the matching indexes for that package. Indexes with `include-packages` are skipped for packages that don't match, and `exclude-packages` removes a package from an index without affecting the others.

Exact `[tool.fyn.sources]` pins still take precedence over these routing rules. Routing patterns cannot be combined with `explicit = true`; if you want an index to be used only for specific named packages, keep using `[tool.fyn.sources]`.

An index can be marked as `explicit = true` to prevent packages from being installed from that index unless explicitly pinned to it. For example, to ensure that `torch` is installed from the `pytorch` index, but all other packages are installed from PyPI, add the following to your `pyproject.toml`:

```
[tool.fyn.sources]
torch = { index = "pytorch" }

[[tool.fyn.index]]
name = "pytorch"
url = "https://download.pytorch.org/whl/cpu"
explicit = true
```

Named indexes referenced via `tool.fyn.sources` must still be declared within the project's `pyproject.toml` (or the workspace root's `pyproject.toml` for workspace-wide sources). This keeps the project's source pinning explicit and reviewable.

However, fyn also allows *local overrides* for those named indexes. If a user- or system-level `fyn.toml` defines an index with the same name, fyn will use that URL instead of the declared project URL when resolving packages pinned via `tool.fyn.sources`.

This is useful when the project should keep a stable logical index name like `internal`, but different environments need different concrete URLs, such as:

- a developer machine using a local mirror
- CI using a corporate proxy or private registry
- a shared project that should not hardcode environment-specific index URLs into `pyproject.toml`

For example:

pyproject.toml

```
[tool.fyn.sources]
torch = { index = "pytorch" }

[[tool.fyn.index]]
name = "pytorch"
url = "https://download.pytorch.org/whl/cpu"
explicit = true
```

~/.config/fyn/fyn.toml

```
[[index]]
name = "pytorch"
url = "https://mirror.example.com/pytorch/cu124"
explicit = true
```

In this case, the project still declares that `torch` is pinned to the `pytorch` index, but the current machine resolves that name through the locally configured mirror.

Local configuration cannot introduce a *new* named index for `tool.fyn.sources`; it can only override an index name that is already declared in project or workspace metadata.

If an index is marked as both `default = true` and `explicit = true`, it will be treated as an explicit index (i.e., only usable via `tool.fyn.sources`) while also removing PyPI as the default index.

## [Searching across multiple indexes](#searching-across-multiple-indexes)

By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`).

For example, if an internal index is specified via `[[tool.fyn.index]]`, fyn's behavior is such that if a package exists on that internal index, it will *always* be installed from that internal index, and never from PyPI. The intent is to prevent "dependency confusion" attacks, in which an attacker publishes a malicious package on PyPI with the same name as an internal package, thus causing the malicious package to be installed instead of the internal package. See, for example, [the `torchtriton` attack](https://pytorch.org/blog/compromised-nightly-dependency/) from December 2022.

To opt in to alternate index behaviors, use the`--index-strategy` command-line option, or the `UV_INDEX_STRATEGY` environment variable, which supports the following values:

- `first-index` (default): Search for each package across all indexes, limiting the candidate versions to those present in the first index that contains the package.
- `unsafe-first-match`: Search for each package across all indexes, but prefer the first index with a compatible version, even if newer versions are available on other indexes.
- `unsafe-best-match`: Search for each package across all indexes, and select the best version from the combined set of candidate versions.

While `unsafe-best-match` is the closest to pip's behavior, it exposes users to the risk of "dependency confusion" attacks.

## [Authentication](#authentication)

Most private package indexes require authentication to access packages, typically via a username and password (or access token).

Tip

See the dedicated guides for authenticating with specific private index providers: [Azure Artifacts](../../guides/integration/azure/), [Google Artifact Registry](../../guides/integration/google/), [AWS CodeArtifact](../../guides/integration/aws/), and [JFrog Artifactory](../../guides/integration/jfrog/).

### [Providing credentials directly](#providing-credentials-directly)

Credentials can be provided directly via environment variables or by embedding them in the URL.

For example, given an index named `internal-proxy` that requires a username (`public`) and password (`koala`), define the index (without credentials) in your `pyproject.toml`:

```
[[tool.fyn.index]]
name = "internal-proxy"
url = "https://example.com/simple"
```

From there, you can set the `UV_INDEX_INTERNAL_PROXY_USERNAME` and `UV_INDEX_INTERNAL_PROXY_PASSWORD` environment variables, where `INTERNAL_PROXY` is the uppercase version of the index name, with non-alphanumeric characters replaced by underscores:

```
export UV_INDEX_INTERNAL_PROXY_USERNAME=public
export UV_INDEX_INTERNAL_PROXY_PASSWORD=koala
```

By providing credentials via environment variables, you can avoid storing sensitive information in the plaintext `pyproject.toml` file.

Alternatively, credentials can be embedded directly in the index definition:

```
[[tool.fyn.index]]
name = "internal"
url = "https://public:koala@pypi-proxy.corp.dev/simple"
```

For security purposes, credentials are *never* stored in the `fyn.lock` file; as such, fyn *must* have access to the authenticated URL at installation time.

### [Using credential providers](#using-credential-providers)

In addition to providing credentials directly, fyn supports discovery of credentials from netrc and keyring. See the [HTTP authentication](../authentication/http/) documentation for details on setting up specific credential providers.

By default, fyn will attempt an unauthenticated request before querying providers. If the request fails, fyn will search for credentials. If credentials are found, an authenticated request will be attempted.

Note

If a username is set, fyn will search for credentials before making an unauthenticated request.

Some indexes (e.g., GitLab) will forward unauthenticated requests to a public index, like PyPI — which means that fyn will not search for credentials. This behavior can be changed per-index, using the `authenticate` setting. For example, to always search for credentials:

```
[[tool.fyn.index]]
name = "example"
url = "https://example.com/simple"
authenticate = "always"
```

When `authenticate` is set to `always`, fyn will eagerly search for credentials and error if credentials cannot be found.

### [Ignoring error codes when searching across indexes](#ignoring-error-codes-when-searching-across-indexes)

When using the [first-index strategy](#searching-across-multiple-indexes), fyn will stop searching across indexes if an HTTP 401 Unauthorized or HTTP 403 Forbidden status code is encountered. The one exception is that fyn will ignore 403s when searching the `pytorch` index (since this index returns a 403 when a package is not present).

To configure which error codes are ignored for an index, use the `ignored-error-codes` setting. For example, to ignore 403s (but not 401s) for a private index:

```
[[tool.fyn.index]]
name = "private-index"
url = "https://private-index.com/simple"
authenticate = "always"
ignore-error-codes = [403]
```

fyn will always continue searching across indexes when it encounters a `404 Not Found`. This cannot be overridden.

### [Disabling authentication](#disabling-authentication)

To prevent leaking credentials, authentication can be disabled for an index:

```
[[tool.fyn.index]]
name = "example"
url = "https://example.com/simple"
authenticate = "never"
```

When `authenticate` is set to `never`, fyn will never search for credentials for the given index and will error if credentials are provided directly.

### [Customizing cache control headers](#customizing-cache-control-headers)

By default, fyn will respect the cache control headers provided by the index. For example, PyPI serves package metadata with a `max-age=600` header, thereby allowing fyn to cache package metadata for 10 minutes; and wheels and source distributions with a `max-age=365000000, immutable` header, thereby allowing fyn to cache artifacts indefinitely.

To override the cache control headers for an index, use the `cache-control` setting:

```
[[tool.fyn.index]]
name = "example"
url = "https://example.com/simple"
cache-control = { api = "max-age=600", files = "max-age=365000000, immutable" }
```

The `cache-control` setting accepts an object with two optional keys:

- `api`: Controls caching for Simple API requests (package metadata).
- `files`: Controls caching for artifact downloads (wheels and source distributions).

The values for these keys are strings that follow the [HTTP Cache-Control](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control) syntax. For example, to force fyn to always revalidate package metadata, set `api = "no-cache"`:

```
[[tool.fyn.index]]
name = "example"
url = "https://example.com/simple"
cache-control = { api = "no-cache" }
```

This setting is most commonly used to override the default cache control headers for private indexes that otherwise disable caching, often unintentionally. We typically recommend following PyPI's approach to caching headers, i.e., setting `api = "max-age=600"` and `files = "max-age=365000000, immutable"`.

## ["Flat" indexes](#flat-indexes)

By default, `[[tool.fyn.index]]` entries are assumed to be PyPI-style registries that implement the [PEP 503](https://peps.python.org/pep-0503/) Simple Repository API. However, fyn also supports "flat" indexes, which are local directories or HTML pages that contain flat lists of wheels and source distributions. In pip, such indexes are specified using the `--find-links` option.

To define a flat index in your `pyproject.toml`, use the `format = "flat"` option:

```
[[tool.fyn.index]]
name = "example"
url = "/path/to/directory"
format = "flat"
```

Flat indexes support the same feature set as Simple Repository API indexes (e.g., `explicit = true`); you can also pin a package to a flat index using `tool.fyn.sources`.

## [`--index-url` and `--extra-index-url`](#-index-url-and-extra-index-url)

In addition to the `[[tool.fyn.index]]` configuration option, fyn supports pip-style `--index-url` and `--extra-index-url` command-line options for compatibility, where `--index-url` defines the default index and `--extra-index-url` defines additional indexes.

These options can be used in conjunction with the `[[tool.fyn.index]]` configuration option, and follow the same prioritization rules:

- The default index is always treated as lowest priority, whether defined via the legacy `--index-url` argument, the recommended `--default-index` argument, or a `[[tool.fyn.index]]` entry with `default = true`.
- Indexes are consulted in the order in which they’re defined, either via the legacy `--extra-index-url` argument, the recommended `--index` argument, or `[[tool.fyn.index]]` entries.

In effect, `--index-url` and `--extra-index-url` can be thought of as unnamed `[[tool.fyn.index]]` entries, with `default = true` enabled for the former. In that context, `--index-url` maps to `--default-index`, and `--extra-index-url` maps to `--index`.

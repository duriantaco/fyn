# [Google Artifact Registry](#google-artifact-registry)

fyn can install packages from [Google Artifact Registry](https://cloud.google.com/artifact-registry/docs), either by using an access token, or using the [`keyring`](https://github.com/jaraco/keyring) package.

Note

This guide assumes that [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI is installed and authenticated.

To use Google Artifact Registry, add the index to your project:

pyproject.toml

```
[[tool.fyn.index]]
name = "private-registry"
url = "https://<REGION>-python.pkg.dev/<PROJECT>/<REPOSITORY>/simple/"
```

## [Authenticate with a Google access token](#authenticate-with-a-google-access-token)

Credentials can be provided via "Basic" HTTP authentication scheme. Include access token in the password field of the URL. Username must be `oauth2accesstoken`, otherwise authentication will fail.

Generate a token with `gcloud`:

```
export ARTIFACT_REGISTRY_TOKEN=$(
    gcloud auth application-default print-access-token
)
```

Note

You might need to pass extra parameters to properly generate the token (like `--project`), this is a basic example.

Then set credentials for the index with:

```
export UV_INDEX_PRIVATE_REGISTRY_USERNAME=oauth2accesstoken
export UV_INDEX_PRIVATE_REGISTRY_PASSWORD="$ARTIFACT_REGISTRY_TOKEN"
```

Note

`PRIVATE_REGISTRY` should match the name of the index defined in your `pyproject.toml`.

## [Authenticate with `keyring` and `keyrings.google-artifactregistry-auth`](#authenticate-with-keyring-and-keyringsgoogle-artifactregistry-auth)

You can also authenticate to Artifact Registry using [`keyring`](https://github.com/jaraco/keyring) package with the [`keyrings.google-artifactregistry-auth` plugin](https://github.com/GoogleCloudPlatform/artifact-registry-python-tools). Because these two packages are required to authenticate to Artifact Registry, they must be pre-installed from a source other than Artifact Registry.

The `keyrings.google-artifactregistry-auth` plugin wraps [gcloud CLI](https://cloud.google.com/sdk/gcloud) to generate short-lived access tokens, securely store them in system keyring, and refresh them when they are expired.

fyn only supports using the `keyring` package in [subprocess mode](../../../reference/settings/#keyring-provider). The `keyring` executable must be in the `PATH`, i.e., installed globally or in the active environment. The `keyring` CLI requires a username in the URL and it must be `oauth2accesstoken`.

```
# Pre-install keyring and Artifact Registry plugin from the public PyPI
fyn tool install keyring --with keyrings.google-artifactregistry-auth

# Enable keyring authentication
export UV_KEYRING_PROVIDER=subprocess

# Set the username for the index
export UV_INDEX_PRIVATE_REGISTRY_USERNAME=oauth2accesstoken
```

Note

The [`tool.fyn.keyring-provider`](../../../reference/settings/#keyring-provider) setting can be used to enable keyring in your `fyn.toml` or `pyproject.toml`.

Similarly, the username for the index can be added directly to the index URL.

## [Publishing packages](#publishing-packages)

If you also want to publish your own packages to Google Artifact Registry, you can use `fyn publish` as described in the [Building and publishing guide](../../package/).

First, add a `publish-url` to the index you want to publish packages to. For example:

pyproject.toml

```
[[tool.fyn.index]]
name = "private-registry"
url = "https://<REGION>-python.pkg.dev/<PROJECT>/<REPOSITORY>/simple/"
publish-url = "https://<REGION>-python.pkg.dev/<PROJECT>/<REPOSITORY>/"
```

Then, configure credentials (if not using keyring):

```
$ export UV_PUBLISH_USERNAME=oauth2accesstoken
$ export UV_PUBLISH_PASSWORD="$ARTIFACT_REGISTRY_TOKEN"
```

And publish the package:

```
$ fyn publish --index private-registry
```

To use `fyn publish` without adding the `publish-url` to the project, you can set `UV_PUBLISH_URL`:

```
$ export UV_PUBLISH_URL=https://<REGION>-python.pkg.dev/<PROJECT>/<REPOSITORY>/
$ fyn publish
```

Note this method is not preferable because fyn cannot check if the package is already published before uploading artifacts.

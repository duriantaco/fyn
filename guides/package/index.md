# [Building and publishing a package](#building-and-publishing-a-package)

fyn supports building Python packages into source and binary distributions via `fyn build` and uploading them to a registry with `fyn publish`.

## [Preparing your project](#preparing-your-project)

Before attempting to publish your project, you'll want to make sure it's ready to be packaged for distribution.

If your project does not include a `[build-system]` definition in the `pyproject.toml`, fyn will not build it during `fyn sync` operations in the project, but will fall back to the legacy setuptools build system during `fyn build`.

We strongly recommend configuring a build system. Read more about build systems in the [project configuration](../../concepts/projects/config/#build-systems) documentation.

## [Building your package](#building-your-package)

Build your package with `fyn build`:

```
$ fyn build
```

By default, `fyn build` will build the project in the current directory, and place the built artifacts in a `dist/` subdirectory.

Alternatively, `fyn build <SRC>` will build the package in the specified directory, while `fyn build --package <PACKAGE>` will build the specified package within the current workspace.

Info

By default, `fyn build` respects `tool.fyn.sources` when resolving build dependencies from the `build-system.requires` section of the `pyproject.toml`. When publishing a package, we recommend running `fyn build --no-sources` to ensure that the package builds correctly when `tool.fyn.sources` is disabled, as is the case when using other build tools, like [`pypa/build`](https://github.com/pypa/build).

## [Updating your version](#updating-your-version)

The `fyn version` command provides conveniences for updating the version of your package before you publish it. [See the project docs for reading your package's version](../projects/#viewing-your-version).

To update to an exact version, provide it as a positional argument:

```
$ fyn version 1.0.0
hello-world 0.7.0 => 1.0.0
```

To preview the change without updating the `pyproject.toml`, use the `--dry-run` flag:

```
$ fyn version 2.0.0 --dry-run
hello-world 1.0.0 => 2.0.0
$ fyn version
hello-world 1.0.0
```

To increase the version of your package semantics, use the `--bump` option:

```
$ fyn version --bump minor
hello-world 1.2.3 => 1.3.0
```

The `--bump` option supports the following common version components: `major`, `minor`, `patch`, `stable`, `alpha`, `beta`, `rc`, `post`, and `dev`. When provided more than once, the components will be applied in order, from largest (`major`) to smallest (`dev`).

You can optionally provide a numeric value with `--bump <component>=<value>` to set the resulting component explicitly:

```
$ fyn version --bump patch --bump dev=66463664
hello-world 0.0.1 => 0.0.2.dev66463664
```

To move from a stable to pre-release version, bump one of the major, minor, or patch components in addition to the pre-release component:

```
$ fyn version --bump patch --bump beta
hello-world 1.3.0 => 1.3.1b1
$ fyn version --bump major --bump alpha
hello-world 1.3.0 => 2.0.0a1
```

When moving from a pre-release to a new pre-release version, just bump the relevant pre-release component:

```
$ fyn version --bump beta
hello-world 1.3.0b1 => 1.3.0b2
```

When moving from a pre-release to a stable version, the `stable` option can be used to clear the pre-release component:

```
$ fyn version --bump stable
hello-world 1.3.1b2 => 1.3.1
```

Info

By default, when `fyn version` modifies the project it will perform a lock and sync. To prevent locking and syncing, use `--frozen`, or, to just prevent syncing, use `--no-sync`.

## [Publishing your package](#publishing-your-package)

Note

A complete guide to publishing from GitHub Actions to PyPI can be found in the [GitHub Guide](../integration/github/#publishing-to-pypi)

Publish your package with `fyn publish`:

```
$ fyn publish
```

Set a PyPI token with `--token` or `UV_PUBLISH_TOKEN`, or set a username with `--username` or `UV_PUBLISH_USERNAME` and password with `--password` or `UV_PUBLISH_PASSWORD`. For publishing to PyPI from GitHub Actions or another Trusted Publisher, you don't need to set any credentials. Instead, [add a trusted publisher to the PyPI project](https://docs.pypi.org/trusted-publishers/adding-a-publisher/).

Note

PyPI does not support publishing with username and password anymore, instead you need to generate a token. Using a token is equivalent to setting `--username __token__` and using the token as password.

If you're using a custom index through `[[tool.fyn.index]]`, add `publish-url` and use `fyn publish --index <name>`. For example:

```
[[tool.fyn.index]]
name = "testpypi"
url = "https://test.pypi.org/simple/"
publish-url = "https://test.pypi.org/legacy/"
explicit = true
```

Note

When using `fyn publish --index <name>`, the `pyproject.toml` must be present, i.e., you need to have a checkout step in a publish CI job.

Even though `fyn publish` retries failed uploads, it can happen that publishing fails in the middle, with some files uploaded and some files still missing. With PyPI, you can retry the exact same command, existing identical files will be ignored. With other registries, use `--check-url <index url>` with the index URL (not the publishing URL) the packages belong to. When using `--index`, the index URL is used as check URL. fyn will skip uploading files that are identical to files in the registry, and it will also handle raced parallel uploads. Note that existing files need to match exactly with those previously uploaded to the registry, this avoids accidentally publishing source distribution and wheels with different contents for the same version.

### [Uploading attestations with your package](#uploading-attestations-with-your-package)

Note

Some third-party package indexes may not support attestations, and may reject uploads that include them (rather than silently ignoring them). If you encounter issues when uploading, you can use `--no-attestations` or `UV_PUBLISH_NO_ATTESTATIONS` to disable fyn's default behavior.

Tip

`fyn publish` does not currently generate attestations; attestations must be created separately before publishing.

`fyn publish` supports uploading [attestations](https://peps.python.org/pep-0740/) to registries that support them, like PyPI.

fyn will automatically discover and match attestations. For example, given the following `dist/` directory, `fyn publish` will upload the attestations along with their corresponding distributions:

```
$ ls dist/
hello_world-1.0.0-py3-none-any.whl
hello_world-1.0.0-py3-none-any.whl.publish.attestation
hello_world-1.0.0.tar.gz
hello_world-1.0.0.tar.gz.publish.attestation
```

## [Installing your package](#installing-your-package)

Test that the package can be installed and imported with `fyn run`:

```
$ fyn run --with <PACKAGE> --no-project -- python -c "import <PACKAGE>"
```

The `--no-project` flag is used to avoid installing the package from your local project directory.

Tip

If you have recently installed the package, you may need to include the `--refresh-package <PACKAGE>` option to avoid using a cached version of the package.

## [Next steps](#next-steps)

To learn more about publishing packages, check out the [PyPA guides](https://packaging.python.org/en/latest/guides/section-build-and-publish/) on building and publishing.

Or, read on for [guides](../integration/) on integrating fyn with other software.

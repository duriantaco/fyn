# Preview features

fyn includes opt-in preview features to provide an opportunity for community feedback and increase
confidence that changes are a net-benefit before enabling them for everyone.

## Enabling preview features

To enable all preview features, use the `--preview` flag:

```console
$ fyn run --preview ...
```

Or, set the `UV_PREVIEW` environment variable:

```console
$ UV_PREVIEW=1 fyn run ...
```

To enable specific preview features, use the `--preview-features` flag:

```console
$ fyn run --preview-features foo ...
```

The `--preview-features` flag can be repeated to enable multiple features:

```console
$ fyn run --preview-features foo --preview-features bar ...
```

Or, features can be provided in a comma separated list:

```console
$ fyn run --preview-features foo,bar ...
```

The `UV_PREVIEW_FEATURES` environment variable can be used similarly, e.g.:

```console
$ UV_PREVIEW_FEATURES=foo,bar fyn run ...
```

For backwards compatibility, enabling preview features that do not exist will warn, but not error.

## Using preview features

Often, preview features can be used without changing any preview settings if the behavior change is
gated by some sort of user interaction, For example, while `pylock.toml` support is in preview, you
can use `fyn pip install` with a `pylock.toml` file without additional configuration because
specifying the `pylock.toml` file indicates you want to use the feature. However, a warning will be
displayed that the feature is in preview. The preview feature can be enabled to silence the warning.

## Available preview features

The following preview features are available:

- `add-bounds`: Allows configuring the
  [default bounds for `fyn add`](../reference/settings.md#add-bounds) invocations.
- `json-output`: Allows `--output-format json` for various fyn commands.
- `package-conflicts`: Allows defining workspace conflicts at the package level.
- `pylock`: Allows installing from `pylock.toml` files.
- `format`: Allows using `fyn format`.
- `native-auth`: Enables storage of credentials in a
  [system-native location](../concepts/authentication/http.md#the-fyn-credentials-store).
- `workspace-metadata`: Allows using `fyn workspace metadata`.
- `workspace-dir`: Allows using `fyn workspace dir`.
- `workspace-list`: Allows using `fyn workspace list`.

## Disabling preview features

The `--no-preview` option can be used to disable preview features.

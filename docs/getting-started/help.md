# Getting help

## Help menus

The `--help` flag can be used to view the help menu for a command, e.g., for `fv`:

```console
$ fv --help
```

To view the help menu for a specific command, e.g., for `fv init`:

```console
$ fv init --help
```

When using the `--help` flag, fv displays a condensed help menu. To view a longer help menu for a
command, use `fv help`:

```console
$ fv help
```

To view the long help menu for a specific command, e.g., for `fv init`:

```console
$ fv help init
```

When using the long help menu, fv will attempt to use `less` or `more` to "page" the output so it is
not all displayed at once. To exit the pager, press `q`.

## Displaying verbose output

The `-v` flag can be used to display verbose output for a command, e.g., for `fv sync`:

```console
$ fv sync -v
```

The `-v` flag can be repeated to increase verbosity, e.g.:

```console
$ fv sync -vv
```

Often, the verbose output will include additional information about why fv is behaving in a certain
way.

## Viewing the version

When seeking help, it's important to determine the version of fv that you're using — sometimes the
problem is already solved in a newer version.

To check the installed version:

```console
$ fv self version
```

The following are also valid:

```console
$ fv --version      # Same output as `fv self version`
$ fv -V             # Will not include the build commit and date
```

!!! note

    Before fv 0.7.0, `fv version` was used instead of `fv self version`.

## Troubleshooting issues

The reference documentation contains a
[troubleshooting guide](../reference/troubleshooting/index.md) for common issues.

## Open an issue on GitHub

The [issue tracker](https://github.com/astral-sh/uv/issues) on GitHub is a good place to report bugs
and request features. Make sure to search for similar issues first, as it is common for someone else
to encounter the same problem.

## Chat on Discord

Astral has a [Discord server](https://discord.com/invite/astral-sh), which is a great place to ask
questions, learn more about fv, and engage with other community members.

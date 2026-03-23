# Installing fv

## Installation methods

Install fv with our standalone installers or your package manager of choice.

### Standalone installer

fv provides a standalone installer to download and install fv:

=== "macOS and Linux"

    Use `curl` to download the script and execute it with `sh`:

    ```console
    $ curl -LsSf https://astral.sh/fv/install.sh | sh
    ```

    If your system doesn't have `curl`, you can use `wget`:

    ```console
    $ wget -qO- https://astral.sh/fv/install.sh | sh
    ```

    Request a specific version by including it in the URL:

    ```console
    $ curl -LsSf https://astral.sh/fv/0.10.12/install.sh | sh
    ```

=== "Windows"

    Use `irm` to download the script and execute it with `iex`:

    ```pwsh-session
    PS> powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/fv/install.ps1 | iex"
    ```

    Changing the [execution policy](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies?view=powershell-7.4#powershell-execution-policies) allows running a script from the internet.

    Request a specific version by including it in the URL:

    ```pwsh-session
    PS> powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/fv/0.10.12/install.ps1 | iex"
    ```

!!! tip

    The installation script may be inspected before use:

    === "macOS and Linux"

        ```console
        $ curl -LsSf https://astral.sh/fv/install.sh | less
        ```

    === "Windows"

        ```pwsh-session
        PS> powershell -c "irm https://astral.sh/fv/install.ps1 | more"
        ```

    Alternatively, the installer or binaries can be downloaded directly from [GitHub](#github-releases).

See the reference documentation on the [installer](../reference/installer.md) for details on
customizing your fv installation.

### PyPI

For convenience, fv is published to [PyPI](https://pypi.org/project/fv/).

If installing from PyPI, we recommend installing fv into an isolated environment, e.g., with `pipx`:

```console
$ pipx install fv
```

However, `pip` can also be used:

```console
$ pip install fv
```

!!! note

    fv ships with prebuilt distributions (wheels) for many platforms; if a wheel is not available for a given
    platform, fv will be built from source, which requires a Rust toolchain. See the
    [contributing setup guide](https://github.com/oha/fv/blob/main/CONTRIBUTING.md#setup)
    for details on building fv from source.

### Homebrew

fv is available in the core Homebrew packages.

```console
$ brew install fv
```

### MacPorts

fv is available via [MacPorts](https://ports.macports.org/port/fv/).

```console
$ sudo port install fv
```

### WinGet

fv is available via [WinGet](https://winstall.app/apps/oha.fv).

```console
$ winget install --id=oha.fv  -e
```

### Scoop

fv is available via [Scoop](https://scoop.sh/#/apps?q=fv).

```console
$ scoop install main/fv
```

### Docker

fv provides a Docker image at
[`ghcr.io/oha/fv`](https://github.com/astral-sh/uv/pkgs/container/uv).

See our guide on [using fv in Docker](../guides/integration/docker.md) for more details.

### GitHub Releases

fv release artifacts can be downloaded directly from
[GitHub Releases](https://github.com/astral-sh/uv/releases).

Each release page includes binaries for all supported platforms as well as instructions for using
the standalone installer via `github.com` instead of `astral.sh`.

### Cargo

fv is available via [crates.io](https://crates.io).

```console
$ cargo install --locked fv
```

!!! note

    This method builds fv from source, which requires a compatible Rust toolchain.

## Upgrading fv

When fv is installed via the standalone installer, it can update itself on-demand:

```console
$ fv self update
```

!!! tip

    Updating fv will re-run the installer and can modify your shell profiles. To disable this
    behavior, set `UV_NO_MODIFY_PATH=1`.

When another installation method is used, self-updates are disabled. Use the package manager's
upgrade method instead. For example, with `pip`:

```console
$ pip install --upgrade fv
```

## Shell autocompletion

!!! tip

    You can run `echo $SHELL` to help you determine your shell.

To enable shell autocompletion for fv commands, run one of the following:

=== "Bash"

    ```bash
    echo 'eval "$(fv generate-shell-completion bash)"' >> ~/.bashrc
    ```

=== "Zsh"

    ```bash
    echo 'eval "$(fv generate-shell-completion zsh)"' >> ~/.zshrc
    ```

=== "fish"

    ```bash
    echo 'fv generate-shell-completion fish | source' > ~/.config/fish/completions/fv.fish
    ```

=== "Elvish"

    ```bash
    echo 'eval (fv generate-shell-completion elvish | slurp)' >> ~/.elvish/rc.elv
    ```

=== "PowerShell / pwsh"

    ```powershell
    if (!(Test-Path -Path $PROFILE)) {
      New-Item -ItemType File -Path $PROFILE -Force
    }
    Add-Content -Path $PROFILE -Value '(& fv generate-shell-completion powershell) | Out-String | Invoke-Expression'
    ```

To enable shell autocompletion for fvx, run one of the following:

=== "Bash"

    ```bash
    echo 'eval "$(fvx --generate-shell-completion bash)"' >> ~/.bashrc
    ```

=== "Zsh"

    ```bash
    echo 'eval "$(fvx --generate-shell-completion zsh)"' >> ~/.zshrc
    ```

=== "fish"

    ```bash
    echo 'fvx --generate-shell-completion fish | source' > ~/.config/fish/completions/fvx.fish
    ```

=== "Elvish"

    ```bash
    echo 'eval (fvx --generate-shell-completion elvish | slurp)' >> ~/.elvish/rc.elv
    ```

=== "PowerShell / pwsh"

    ```powershell
    if (!(Test-Path -Path $PROFILE)) {
      New-Item -ItemType File -Path $PROFILE -Force
    }
    Add-Content -Path $PROFILE -Value '(& fvx --generate-shell-completion powershell) | Out-String | Invoke-Expression'
    ```

Then restart the shell or source the shell config file.

## Uninstallation

If you need to remove fv from your system, follow these steps:

1.  Clean up stored data (optional):

    ```console
    $ fv cache clean
    $ rm -r "$(fv python dir)"
    $ rm -r "$(fv tool dir)"
    ```

    !!! tip

        Before removing the binaries, you may want to remove any data that fv has stored. See the
        [storage reference](../reference/storage.md) for details on where fv stores data.

2.  Remove the fv, fvx, and fvw binaries:

    === "macOS and Linux"

        ```console
        $ rm ~/.local/bin/fv ~/.local/bin/fvx
        ```

    === "Windows"

        ```pwsh-session
        PS> rm $HOME\.local\bin\fv.exe
        PS> rm $HOME\.local\bin\fvx.exe
        PS> rm $HOME\.local\bin\fvw.exe
        ```

    !!! note

        Prior to 0.5.0, fv was installed into `~/.cargo/bin`. The binaries can be removed from there to
        uninstall. Upgrading from an older version will not automatically remove the binaries from
        `~/.cargo/bin`.

## Next steps

See the [first steps](./first-steps.md) or jump straight to the [guides](../guides/index.md) to
start using fv.

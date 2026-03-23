# Installing fyn

## Installation methods

Install fyn with our standalone installers or your package manager of choice.

### Standalone installer

fyn provides a standalone installer to download and install fyn:

=== "macOS and Linux"

    Use `curl` to download the script and execute it with `sh`:

    ```console
    $ curl -LsSf https://github.com/oha/fyn/releases/latest/download/install.sh | sh
    ```

    If your system doesn't have `curl`, you can use `wget`:

    ```console
    $ wget -qO- https://github.com/oha/fyn/releases/latest/download/install.sh | sh
    ```

    Request a specific version by including it in the URL:

    ```console
    $ curl -LsSf https://github.com/oha/fyn/releases/download/0.10.12/install.sh | sh
    ```

=== "Windows"

    Use `irm` to download the script and execute it with `iex`:

    ```pwsh-session
    PS> powershell -ExecutionPolicy ByPass -c "irm https://github.com/oha/fyn/releases/latest/download/install.ps1 | iex"
    ```

    Changing the [execution policy](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies?view=powershell-7.4#powershell-execution-policies) allows running a script from the internet.

    Request a specific version by including it in the URL:

    ```pwsh-session
    PS> powershell -ExecutionPolicy ByPass -c "irm https://github.com/oha/fyn/releases/download/0.10.12/install.ps1 | iex"
    ```

!!! tip

    The installation script may be inspected before use:

    === "macOS and Linux"

        ```console
        $ curl -LsSf https://github.com/oha/fyn/releases/latest/download/install.sh | less
        ```

    === "Windows"

        ```pwsh-session
        PS> powershell -c "irm https://github.com/oha/fyn/releases/latest/download/install.ps1 | more"
        ```

    Alternatively, the installer or binaries can be downloaded directly from [GitHub](#github-releases).

See the reference documentation on the [installer](../reference/installer.md) for details on
customizing your fyn installation.

### PyPI

For convenience, fyn is published to [PyPI](https://pypi.org/project/fyn/).

If installing from PyPI, we recommend installing fyn into an isolated environment, e.g., with
`pipx`:

```console
$ pipx install fyn
```

However, `pip` can also be used:

```console
$ pip install fyn
```

!!! note

    fyn ships with prebuilt distributions (wheels) for many platforms; if a wheel is not available for a given
    platform, fyn will be built from source, which requires a Rust toolchain. See the
    [contributing setup guide](https://github.com/oha/fyn/blob/main/CONTRIBUTING.md#setup)
    for details on building fyn from source.

### Homebrew

fyn is available in the core Homebrew packages.

```console
$ brew install fyn
```

### MacPorts

fyn is available via [MacPorts](https://ports.macports.org/port/fyn/).

```console
$ sudo port install fyn
```

### WinGet

fyn is available via [WinGet](https://winstall.app/apps/oha.fyn).

```console
$ winget install --id=oha.fyn  -e
```

### Scoop

fyn is available via [Scoop](https://scoop.sh/#/apps?q=fyn).

```console
$ scoop install main/fyn
```

### Docker

fyn provides a Docker image at [`ghcr.io/oha/fyn`](https://github.com/oha/fyn/pkgs/container/fyn).

See our guide on [using fyn in Docker](../guides/integration/docker.md) for more details.

### GitHub Releases

fyn release artifacts can be downloaded directly from
[GitHub Releases](https://github.com/oha/fyn/releases).

Each release page includes binaries for all supported platforms as well as instructions for using
the standalone installer via GitHub.

### Cargo

fyn is available via [crates.io](https://crates.io).

```console
$ cargo install --locked fyn
```

!!! note

    This method builds fyn from source, which requires a compatible Rust toolchain.

## Upgrading fyn

When fyn is installed via the standalone installer, it can update itself on-demand:

```console
$ fyn self update
```

!!! tip

    Updating fyn will re-run the installer and can modify your shell profiles. To disable this
    behavior, set `UV_NO_MODIFY_PATH=1`.

When another installation method is used, self-updates are disabled. Use the package manager's
upgrade method instead. For example, with `pip`:

```console
$ pip install --upgrade fyn
```

## Shell autocompletion

!!! tip

    You can run `echo $SHELL` to help you determine your shell.

To enable shell autocompletion for fyn commands, run one of the following:

=== "Bash"

    ```bash
    echo 'eval "$(fyn generate-shell-completion bash)"' >> ~/.bashrc
    ```

=== "Zsh"

    ```bash
    echo 'eval "$(fyn generate-shell-completion zsh)"' >> ~/.zshrc
    ```

=== "fish"

    ```bash
    echo 'fyn generate-shell-completion fish | source' > ~/.config/fish/completions/fyn.fish
    ```

=== "Elvish"

    ```bash
    echo 'eval (fyn generate-shell-completion elvish | slurp)' >> ~/.elvish/rc.elv
    ```

=== "PowerShell / pwsh"

    ```powershell
    if (!(Test-Path -Path $PROFILE)) {
      New-Item -ItemType File -Path $PROFILE -Force
    }
    Add-Content -Path $PROFILE -Value '(& fyn generate-shell-completion powershell) | Out-String | Invoke-Expression'
    ```

To enable shell autocompletion for fynx, run one of the following:

=== "Bash"

    ```bash
    echo 'eval "$(fynx --generate-shell-completion bash)"' >> ~/.bashrc
    ```

=== "Zsh"

    ```bash
    echo 'eval "$(fynx --generate-shell-completion zsh)"' >> ~/.zshrc
    ```

=== "fish"

    ```bash
    echo 'fynx --generate-shell-completion fish | source' > ~/.config/fish/completions/fynx.fish
    ```

=== "Elvish"

    ```bash
    echo 'eval (fynx --generate-shell-completion elvish | slurp)' >> ~/.elvish/rc.elv
    ```

=== "PowerShell / pwsh"

    ```powershell
    if (!(Test-Path -Path $PROFILE)) {
      New-Item -ItemType File -Path $PROFILE -Force
    }
    Add-Content -Path $PROFILE -Value '(& fynx --generate-shell-completion powershell) | Out-String | Invoke-Expression'
    ```

Then restart the shell or source the shell config file.

## Uninstallation

If you need to remove fyn from your system, follow these steps:

1.  Clean up stored data (optional):

    ```console
    $ fyn cache clean
    $ rm -r "$(fyn python dir)"
    $ rm -r "$(fyn tool dir)"
    ```

    !!! tip

        Before removing the binaries, you may want to remove any data that fyn has stored. See the
        [storage reference](../reference/storage.md) for details on where fyn stores data.

2.  Remove the fyn, fynx, and fynw binaries:

    === "macOS and Linux"

        ```console
        $ rm ~/.local/bin/fyn ~/.local/bin/fynx
        ```

    === "Windows"

        ```pwsh-session
        PS> rm $HOME\.local\bin\fyn.exe
        PS> rm $HOME\.local\bin\fynx.exe
        PS> rm $HOME\.local\bin\fynw.exe
        ```

    !!! note

        Prior to 0.5.0, fyn was installed into `~/.cargo/bin`. The binaries can be removed from there to
        uninstall. Upgrading from an older version will not automatically remove the binaries from
        `~/.cargo/bin`.

## Next steps

See the [first steps](./first-steps.md) or jump straight to the [guides](../guides/index.md) to
start using fyn.

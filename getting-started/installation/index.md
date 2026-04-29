# [Installing fyn](#installing-fyn)

## [Installation methods](#installation-methods)

Install fyn with PyPI, GitHub release assets, or your package manager of choice.

### [Standalone installer](#standalone-installer)

When a release includes standalone installer scripts, download the matching script from the [GitHub release page](https://github.com/duriantaco/fyn/releases) and run it locally:

```
$ sh install.sh
```

```
PS> powershell -ExecutionPolicy ByPass -File .\install.ps1
```

Changing the [execution policy](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies?view=powershell-7.4#powershell-execution-policies) allows running a script from the internet.

Tip

The installation script may be inspected before use:

```
$ less install.sh
```

```
PS> Get-Content .\install.ps1
```

Alternatively, use `pipx install fyn`, or download a platform archive directly from [GitHub Releases](#github-releases).

See the reference documentation on the [installer](../../reference/installer/) for details on customizing your fyn installation.

### [PyPI](#pypi)

For convenience, fyn is published to [PyPI](https://pypi.org/project/fyn/).

If installing from PyPI, we recommend installing fyn into an isolated environment, e.g., with `pipx`:

```
$ pipx install fyn
```

However, `pip` can also be used:

```
$ pip install fyn
```

Note

fyn ships with prebuilt distributions (wheels) for many platforms; if a wheel is not available for a given platform, fyn will be built from source, which requires a Rust toolchain. See the [contributing setup guide](https://github.com/duriantaco/fyn/blob/main/CONTRIBUTING.md#setup) for details on building fyn from source.

### [Homebrew](#homebrew)

fyn is available in the core Homebrew packages.

```
$ brew install fyn
```

### [MacPorts](#macports)

fyn is available via [MacPorts](https://ports.macports.org/port/fyn/).

```
$ sudo port install fyn
```

### [WinGet](#winget)

fyn is available via [WinGet](https://winstall.app/apps/oha.fyn).

```
$ winget install --id=oha.fyn  -e
```

### [Scoop](#scoop)

fyn is available via [Scoop](https://scoop.sh/#/apps?q=fyn).

```
$ scoop install main/fyn
```

### [Docker](#docker)

fyn provides a Docker image at [`ghcr.io/oha/fyn`](https://github.com/oha/fyn/pkgs/container/fyn).

See our guide on [using fyn in Docker](../../guides/integration/docker/) for more details.

### [GitHub Releases](#github-releases)

fyn release artifacts can be downloaded directly from [GitHub Releases](https://github.com/duriantaco/fyn/releases).

Each release page includes binaries for supported platforms, and may also include standalone installer scripts.

### [Cargo](#cargo)

fyn is available via [crates.io](https://crates.io).

```
$ cargo install --locked fyn
```

Note

This method builds fyn from source, which requires a compatible Rust toolchain.

## [Upgrading fyn](#upgrading-fyn)

When fyn is installed via the standalone installer, it can update itself on-demand:

```
$ fyn self update
```

Tip

Updating fyn will re-run the installer and can modify your shell profiles. To disable this behavior, set `UV_NO_MODIFY_PATH=1`.

When another installation method is used, self-updates are disabled. Use the package manager's upgrade method instead. For example, with `pip`:

```
$ pip install --upgrade fyn
```

## [Shell autocompletion](#shell-autocompletion)

Tip

You can run `echo $SHELL` to help you determine your shell.

To enable shell autocompletion for fyn commands, run one of the following:

```
echo 'eval "$(fyn generate-shell-completion bash)"' >> ~/.bashrc
```

```
echo 'eval "$(fyn generate-shell-completion zsh)"' >> ~/.zshrc
```

```
echo 'fyn generate-shell-completion fish | source' > ~/.config/fish/completions/fyn.fish
```

```
echo 'eval (fyn generate-shell-completion elvish | slurp)' >> ~/.elvish/rc.elv
```

```
if (!(Test-Path -Path $PROFILE)) {
  New-Item -ItemType File -Path $PROFILE -Force
}
Add-Content -Path $PROFILE -Value '(& fyn generate-shell-completion powershell) | Out-String | Invoke-Expression'
```

To enable shell autocompletion for fynx, run one of the following:

```
echo 'eval "$(fynx --generate-shell-completion bash)"' >> ~/.bashrc
```

```
echo 'eval "$(fynx --generate-shell-completion zsh)"' >> ~/.zshrc
```

```
echo 'fynx --generate-shell-completion fish | source' > ~/.config/fish/completions/fynx.fish
```

```
echo 'eval (fynx --generate-shell-completion elvish | slurp)' >> ~/.elvish/rc.elv
```

```
if (!(Test-Path -Path $PROFILE)) {
  New-Item -ItemType File -Path $PROFILE -Force
}
Add-Content -Path $PROFILE -Value '(& fynx --generate-shell-completion powershell) | Out-String | Invoke-Expression'
```

Then restart the shell or source the shell config file.

## [Uninstallation](#uninstallation)

If you need to remove fyn from your system, follow these steps:

1. Clean up stored data (optional):

   ```
   $ fyn cache clean
   $ rm -r "$(fyn python dir)"
   $ rm -r "$(fyn tool dir)"
   ```

   Tip

   Before removing the binaries, you may want to remove any data that fyn has stored. See the [storage reference](../../reference/storage/) for details on where fyn stores data.

1. Remove the fyn, fynx, and fynw binaries:

   ```
   $ rm ~/.local/bin/fyn ~/.local/bin/fynx
   ```

   ```
   PS> rm $HOME\.local\bin\fyn.exe
   PS> rm $HOME\.local\bin\fynx.exe
   PS> rm $HOME\.local\bin\fynw.exe
   ```

   Note

   Prior to 0.5.0, fyn was installed into `~/.cargo/bin`. The binaries can be removed from there to uninstall. Upgrading from an older version will not automatically remove the binaries from `~/.cargo/bin`.

## [Next steps](#next-steps)

See the [first steps](../first-steps/) or jump straight to the [guides](../../guides/) to start using fyn.

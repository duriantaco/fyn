# The fyn installer

## Changing the installation path

By default, fyn is installed in the user [executable directory](./storage.md#executable-directory).
Download `install.sh` or `install.ps1` from a GitHub release first, then run it with the following
environment variables.

To change the installation path, use `UV_INSTALL_DIR`:

=== "macOS and Linux"

    ```console
    $ env UV_INSTALL_DIR="/custom/path" sh install.sh
    ```

=== "Windows"

    ```pwsh-session
    PS> $env:UV_INSTALL_DIR = "C:\Custom\Path"; powershell -ExecutionPolicy ByPass -File .\install.ps1
    ```

!!! note

    Changing the installation path only affects where the fyn binary is installed. fyn will still store
    its data (cache, Python installations, tools, etc.) in the default locations. See the
    [storage reference](./storage.md) for details on these locations and how to customize them.

## Disabling shell modifications

The installer may also update your shell profiles to ensure the fyn binary is on your `PATH`. To
disable this behavior, use `UV_NO_MODIFY_PATH`. For example:

```console
$ env UV_NO_MODIFY_PATH=1 sh install.sh
```

If installed with `UV_NO_MODIFY_PATH`, subsequent operations, like `fyn self update`, will not
modify your shell profiles.

## Unmanaged installations

In ephemeral environments like CI, use `UV_UNMANAGED_INSTALL` to install fyn to a specific path
while preventing the installer from modifying shell profiles or environment variables:

```console
$ env UV_UNMANAGED_INSTALL="/custom/path" sh install.sh
```

The use of `UV_UNMANAGED_INSTALL` will also disable self-updates (via `fyn self update`).

## Passing options to the installation script

Using environment variables is recommended because they are consistent across platforms. However,
options can be passed directly to the installation script. For example, to see the available
options:

```console
$ sh install.sh --help
```

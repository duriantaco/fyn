# The pip interface

fyn provides a drop-in replacement for common `pip`, `pip-tools`, and `virtualenv` commands. These
commands work directly with the virtual environment, in contrast to fyn's primary interfaces where
the virtual environment is managed automatically. The `fyn pip` interface exposes the speed and
functionality of fyn to power users and projects that are not ready to transition away from `pip`
and `pip-tools`.

The following sections discuss the basics of using `fyn pip`:

- [Creating and using environments](./environments.md)
- [Installing and managing packages](./packages.md)
- [Inspecting environments and packages](./inspection.md)
- [Declaring package dependencies](./dependencies.md)
- [Locking and syncing environments](./compile.md)

Please note these commands do not _exactly_ implement the interfaces and behavior of the tools they
are based on. The further you stray from common workflows, the more likely you are to encounter
differences. Consult the [pip-compatibility guide](./compatibility.md) for details.

!!! important

    fyn does not rely on or invoke pip. The pip interface is named as such to highlight its dedicated
    purpose of providing low-level commands that match pip's interface and to separate it from the
    rest of fyn's commands which operate at a higher level of abstraction.

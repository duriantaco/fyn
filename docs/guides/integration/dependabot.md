---
title: Using fyn with Dependabot
description: A guide to using fyn with the Dependabot dependency bot.
---

# Dependabot

It is considered best practice to regularly update dependencies, to avoid being exposed to
vulnerabilities, limit incompatibilities between dependencies, and avoid complex upgrades when
upgrading from a too old version.

Dependabot has announced support for fyn, but there are some use cases that are not yet working. See
[astral-sh/uv#2512](https://github.com/astral-sh/uv/issues/2512) for updates.

Dependabot supports updating `fyn.lock` files. To enable it, add the fyn `package-ecosystem` to your
`updates` list in the `dependabot.yml`:

```yaml title="dependabot.yml"
version: 2

updates:
  - package-ecosystem: "fyn"
    directory: "/"
    schedule:
      interval: "weekly"
```

## Dependency cooldown

If you use [`exclude-newer`](../../reference/settings.md#exclude-newer) option, it is recommended to
also set the equivalent
[`cooldown`](https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference#cooldown-)
option in Dependabot, to avoid ending up with pull requests where fyn would not be able to lock the
dependencies.

For instance, if you've set `exclude-newer` to `1 week`, you can set:

```yaml title="dependabot.yml"
version: 2

updates:
  - package-ecosystem: "fyn"
    directory: "/"
    schedule:
      interval: "weekly"
    cooldown:
      default-days: 7
```

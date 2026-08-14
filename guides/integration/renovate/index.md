# [Renovate](#renovate)

It is considered best practice to regularly update dependencies, to avoid being exposed to vulnerabilities, limit incompatibilities between dependencies, and avoid complex upgrades when upgrading from a too old version.

fyn is supported by [Renovate](https://github.com/renovatebot/renovate).

## [`fyn.lock` output](#fynlock-output)

Renovate uses the presence of a `fyn.lock` file to determine that fyn is used for managing dependencies, and will suggest upgrades to [project dependencies](../../../concepts/projects/dependencies/#project-dependencies), [optional dependencies](../../../concepts/projects/dependencies/#optional-dependencies) and [development dependencies](../../../concepts/projects/dependencies/#development-dependencies). Renovate will update both the `pyproject.toml` and `fyn.lock` files.

The lockfile can also be refreshed on a regular basis (for instance to update transitive dependencies) by enabling the [`lockFileMaintenance`](https://docs.renovatebot.com/configuration-options/#lockfilemaintenance) option:

renovate.json5

```
{
  $schema: "https://docs.renovatebot.com/renovate-schema.json",
  lockFileMaintenance: {
    enabled: true,
  },
}
```

## [Inline script metadata](#inline-script-metadata)

Renovate supports updating dependencies defined using [inline script metadata](../../scripts/#declaring-script-dependencies).

Since it cannot automatically detect which Python files use inline script metadata, their locations need to be explicitly defined using [`managerFilePatterns`](https://docs.renovatebot.com/configuration-options/#managerfilepatterns), like so:

renovate.json5

```
{
  $schema: "https://docs.renovatebot.com/renovate-schema.json",
  pep723: {
    managerFilePatterns: [
      "docs/build.py",
      "scripts/**/*.py",
    ],
  },
}
```

Note

Renovate does not yet support updating the lock file associated to the script (<https://github.com/renovatebot/renovate/issues/33591>), so if you rely on this feature for a script, the lock file will need to be manually updated.

## [Dependency cooldown](#dependency-cooldown)

If you use [`exclude-newer`](../../../reference/settings/#exclude-newer) option, it is recommended to also set the equivalent [`minimumReleaseAge`](https://docs.renovatebot.com/configuration-options/#minimumreleaseage) option in Renovate, to avoid ending up with pull requests where fyn would not be able to lock the dependencies.

For instance, if you've set `exclude-newer` to `1 week`, you can set:

renovate.json5

```
{
  $schema: "https://docs.renovatebot.com/renovate-schema.json",

  // Enable only for PyPI.
  packageRules: [
    {
      matchDatasources: ["pypi"],
      minimumReleaseAge: "1 week",
    },
  ],

  // Or enable for every ecosystem.
  minimumReleaseAge: "1 week",
}
```

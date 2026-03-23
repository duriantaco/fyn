use crate::resolver::ForkMap;
use crate::{DependencyMode, Manifest, ResolverEnvironment};
use fv_distribution_types::{IndexMetadata, RequirementSource};
use fv_normalize::PackageName;

/// A map of package names to their explicit index.
///
/// For example, given:
/// ```toml
/// [[tool.fv.index]]
/// name = "pytorch"
/// url = "https://download.pytorch.org/whl/cu121"
///
/// [tool.fv.sources]
/// torch = { index = "pytorch" }
/// ```
///
/// [`Indexes`] would contain a single entry mapping `torch` to `https://download.pytorch.org/whl/cu121`.
#[derive(Debug, Default, Clone)]
pub(crate) struct Indexes(ForkMap<IndexMetadata>);

impl Indexes {
    /// Determine the set of explicit, pinned indexes in the [`Manifest`].
    ///
    /// We always scan all requirements (transitive mode) here, since source
    /// mappings in `[tool.fv.sources]` should be respected for transitive
    /// dependencies too — not just direct ones.
    pub(crate) fn from_manifest(
        manifest: &Manifest,
        env: &ResolverEnvironment,
        _dependencies: DependencyMode,
    ) -> Self {
        let mut indexes = ForkMap::default();

        for requirement in manifest.requirements(env, DependencyMode::Transitive) {
            let RequirementSource::Registry {
                index: Some(index), ..
            } = &requirement.source
            else {
                continue;
            };
            indexes.add(requirement.as_ref(), index.clone());
        }

        Self(indexes)
    }

    /// Returns `true` if the map contains any indexes for a package.
    pub(crate) fn contains_key(&self, name: &PackageName) -> bool {
        self.0.contains_key(name)
    }

    /// Return the explicit index used for a package in the given fork.
    pub(crate) fn get(&self, name: &PackageName, env: &ResolverEnvironment) -> Vec<&IndexMetadata> {
        self.0.get(name, env)
    }
}

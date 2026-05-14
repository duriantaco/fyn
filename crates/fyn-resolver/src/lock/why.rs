use std::collections::BTreeSet;
use std::fmt::Write;

use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

use fyn_configuration::DependencyGroupsWithDefaults;
use fyn_distribution_types::Requirement;
use fyn_normalize::{ExtraName, GroupName, PackageName};
use fyn_pep440::Version;
use fyn_pep508::MarkerTree;
use fyn_pypi_types::ResolverMarkerEnvironment;

use crate::lock::{Dependency, Lock, Package, PackageId};

#[derive(Debug)]
pub struct WhyDisplay {
    target: PackageName,
    target_version: Option<Version>,
    paths: Vec<String>,
}

impl WhyDisplay {
    /// Explain why the target package is included in the project dependency graph.
    pub fn new(
        lock: &Lock,
        markers: Option<&ResolverMarkerEnvironment>,
        target: &PackageName,
        depth: usize,
        groups: &DependencyGroupsWithDefaults,
    ) -> Self {
        Self::from_target(lock, markers, target, None, depth, groups)
    }

    /// Explain why the target package version is included in the project dependency graph.
    pub fn for_package(
        lock: &Lock,
        markers: Option<&ResolverMarkerEnvironment>,
        target: &PackageName,
        target_version: &Version,
        depth: usize,
        groups: &DependencyGroupsWithDefaults,
    ) -> Self {
        Self::from_target(lock, markers, target, Some(target_version), depth, groups)
    }

    fn from_target(
        lock: &Lock,
        markers: Option<&ResolverMarkerEnvironment>,
        target: &PackageName,
        target_version: Option<&Version>,
        depth: usize,
        groups: &DependencyGroupsWithDefaults,
    ) -> Self {
        let members: BTreeSet<&PackageId> = if lock.members().is_empty() {
            lock.root().into_iter().map(|package| &package.id).collect()
        } else {
            lock.packages
                .iter()
                .filter_map(|package| {
                    if lock.members().contains(&package.id.name) {
                        Some(&package.id)
                    } else {
                        None
                    }
                })
                .collect()
        };

        let by_name: FxHashMap<_, Vec<_>> = lock.packages().iter().fold(
            FxHashMap::with_capacity_and_hasher(lock.len(), FxBuildHasher),
            |mut map, package| {
                map.entry(package.name()).or_default().push(package);
                map
            },
        );

        let mut display = Self {
            target: target.clone(),
            target_version: target_version.cloned(),
            paths: Vec::new(),
        };
        let mut paths = BTreeSet::new();

        for id in members.iter().copied() {
            let package = lock.find_by_id(id);
            let mut path = vec![Step {
                package: &package.id,
                edge: None,
            }];
            let mut seen = FxHashSet::default();
            display.visit(
                lock,
                package,
                &[],
                true,
                markers,
                depth,
                groups,
                &members,
                &mut seen,
                &mut path,
                &mut paths,
            );
        }

        for requirement in lock.requirements() {
            for package in matching_packages(&by_name, requirement, markers) {
                let active_extras = requirement.extras.iter().collect::<Vec<_>>();
                let edge = (!active_extras.is_empty()).then(|| Edge::Prod(active_extras.clone()));
                let mut path = vec![Step {
                    package: &package.id,
                    edge,
                }];
                let mut seen = FxHashSet::default();
                display.visit(
                    lock,
                    package,
                    &active_extras,
                    false,
                    markers,
                    depth,
                    groups,
                    &members,
                    &mut seen,
                    &mut path,
                    &mut paths,
                );
            }
        }

        for (group, requirements) in lock.dependency_groups() {
            if !groups.contains(group) {
                continue;
            }

            for requirement in requirements {
                for package in matching_packages(&by_name, requirement, markers) {
                    let active_extras = requirement.extras.iter().collect::<Vec<_>>();
                    let mut path = vec![Step {
                        package: &package.id,
                        edge: Some(Edge::Dev(group, active_extras.clone())),
                    }];
                    let mut seen = FxHashSet::default();
                    display.visit(
                        lock,
                        package,
                        &active_extras,
                        false,
                        markers,
                        depth,
                        groups,
                        &members,
                        &mut seen,
                        &mut path,
                        &mut paths,
                    );
                }
            }
        }

        display.paths = paths.into_iter().collect();
        display
    }

    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }

    pub fn paths(&self) -> &[String] {
        &self.paths
    }

    #[allow(clippy::too_many_arguments)]
    fn visit<'lock>(
        &self,
        lock: &'lock Lock,
        package: &'lock Package,
        active_extras: &[&'lock ExtraName],
        include_workspace_extras: bool,
        markers: Option<&ResolverMarkerEnvironment>,
        depth: usize,
        groups: &DependencyGroupsWithDefaults,
        members: &BTreeSet<&'lock PackageId>,
        seen: &mut FxHashSet<&'lock PackageId>,
        path: &mut Vec<Step<'lock>>,
        paths: &mut BTreeSet<String>,
    ) {
        if !seen.insert(&package.id) {
            return;
        }

        if package.name() == &self.target
            && self
                .target_version
                .as_ref()
                .is_none_or(|version| package.version() == Some(version))
        {
            paths.insert(format_path(path));
            seen.remove(&package.id);
            return;
        }

        if path.len().saturating_sub(1) >= depth {
            seen.remove(&package.id);
            return;
        }

        let is_member = members.contains(&package.id);

        if !is_member || groups.prod() {
            for dependency in package.dependencies() {
                self.visit_dependency(
                    lock,
                    dependency,
                    EdgeKind::Prod,
                    markers,
                    depth,
                    groups,
                    members,
                    seen,
                    path,
                    paths,
                );
            }
        }

        if is_member {
            for (group, dependencies) in package.resolved_dependency_groups() {
                if !groups.contains(group) {
                    continue;
                }

                for dependency in dependencies {
                    self.visit_dependency(
                        lock,
                        dependency,
                        EdgeKind::Dev(group),
                        markers,
                        depth,
                        groups,
                        members,
                        seen,
                        path,
                        paths,
                    );
                }
            }
        }

        let extras = if include_workspace_extras && groups.prod() {
            EitherExtras::Both(
                active_extras.iter().copied(),
                package.optional_dependencies().keys(),
            )
        } else {
            EitherExtras::Active(active_extras.iter().copied())
        };

        for extra in extras {
            let Some(dependencies) = package.optional_dependencies().get(extra) else {
                continue;
            };
            for dependency in dependencies {
                self.visit_dependency(
                    lock,
                    dependency,
                    EdgeKind::Optional(extra),
                    markers,
                    depth,
                    groups,
                    members,
                    seen,
                    path,
                    paths,
                );
            }
        }

        seen.remove(&package.id);
    }

    #[allow(clippy::too_many_arguments)]
    fn visit_dependency<'lock>(
        &self,
        lock: &'lock Lock,
        dependency: &'lock Dependency,
        edge_kind: EdgeKind<'lock>,
        markers: Option<&ResolverMarkerEnvironment>,
        depth: usize,
        groups: &DependencyGroupsWithDefaults,
        members: &BTreeSet<&'lock PackageId>,
        seen: &mut FxHashSet<&'lock PackageId>,
        path: &mut Vec<Step<'lock>>,
        paths: &mut BTreeSet<String>,
    ) {
        if markers
            .is_some_and(|markers| !dependency.complexified_marker.evaluate_no_extras(markers))
        {
            return;
        }

        let active_extras = dependency.extra().iter().collect::<Vec<_>>();
        let edge = match edge_kind {
            EdgeKind::Prod => Edge::Prod(active_extras.clone()),
            EdgeKind::Optional(extra) => Edge::Optional(extra, active_extras.clone()),
            EdgeKind::Dev(group) => Edge::Dev(group, active_extras.clone()),
        };
        let package = lock.find_by_id(&dependency.package_id);

        path.push(Step {
            package: &package.id,
            edge: Some(edge),
        });
        self.visit(
            lock,
            package,
            &active_extras,
            false,
            markers,
            depth,
            groups,
            members,
            seen,
            path,
            paths,
        );
        path.pop();
    }
}

#[derive(Debug, Clone)]
struct Step<'lock> {
    package: &'lock PackageId,
    edge: Option<Edge<'lock>>,
}

#[derive(Debug, Clone)]
enum Edge<'lock> {
    Prod(Vec<&'lock ExtraName>),
    Optional(&'lock ExtraName, Vec<&'lock ExtraName>),
    Dev(&'lock GroupName, Vec<&'lock ExtraName>),
}

impl Edge<'_> {
    fn extras(&self) -> &[&ExtraName] {
        match self {
            Self::Prod(extras) | Self::Optional(_, extras) | Self::Dev(_, extras) => extras,
        }
    }
}

#[derive(Clone, Copy)]
enum EdgeKind<'lock> {
    Prod,
    Optional(&'lock ExtraName),
    Dev(&'lock GroupName),
}

enum EitherExtras<I, J> {
    Active(I),
    Both(I, J),
}

impl<'a, I, J> Iterator for EitherExtras<I, J>
where
    I: Iterator<Item = &'a ExtraName>,
    J: Iterator<Item = &'a ExtraName>,
{
    type Item = &'a ExtraName;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Active(iter) => iter.next(),
            Self::Both(left, right) => left.next().or_else(|| right.next()),
        }
    }
}

fn matching_packages<'lock>(
    by_name: &'lock FxHashMap<&'lock PackageName, Vec<&'lock Package>>,
    requirement: &Requirement,
    markers: Option<&ResolverMarkerEnvironment>,
) -> impl Iterator<Item = &'lock Package> {
    by_name
        .get(&requirement.name)
        .into_iter()
        .flatten()
        .copied()
        .filter(move |package| requirement_matches(package, requirement, markers))
}

fn requirement_matches(
    package: &Package,
    requirement: &Requirement,
    markers: Option<&ResolverMarkerEnvironment>,
) -> bool {
    let marker = if package.fork_markers.is_empty() {
        requirement.marker
    } else {
        let mut combined = MarkerTree::FALSE;
        for fork_marker in &package.fork_markers {
            combined.or(fork_marker.pep508());
        }
        combined.and(requirement.marker);
        combined
    };
    !marker.is_false() && markers.is_none_or(|markers| marker.evaluate(markers, &[]))
}

fn format_path(path: &[Step<'_>]) -> String {
    path.iter().map(format_step).join(" -> ")
}

fn format_step(step: &Step<'_>) -> String {
    let mut line = step.package.name.to_string();

    if let Some(edge) = &step.edge {
        let extras = edge.extras();
        if !extras.is_empty() {
            line.push('[');
            line.push_str(&extras.iter().join(", "));
            line.push(']');
        }
    }

    if let Some(version) = step.package.version.as_ref() {
        line.push(' ');
        line.push('v');
        let _ = write!(line, "{version}");
    }

    if let Some(edge) = &step.edge {
        match edge {
            Edge::Prod(_) => {}
            Edge::Optional(extra, _) => {
                let _ = write!(line, " (extra: {extra})");
            }
            Edge::Dev(group, _) => {
                let _ = write!(line, " (group: {group})");
            }
        }
    }

    line
}

impl std::fmt::Display for WhyDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} is included because:", self.target)?;
        for path in &self.paths {
            writeln!(f, "{path}")?;
        }
        Ok(())
    }
}

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

use itertools::Itertools;

use fyn_normalize::PackageName;
use fyn_pep440::Version;

use crate::lock::{Lock, Package, Source};

#[derive(Debug)]
pub struct LockDiffDisplay<'lock> {
    events: Vec<LockDiffEvent<'lock>>,
}

impl<'lock> LockDiffDisplay<'lock> {
    /// Compare two lockfiles and produce a human-readable package-level diff.
    pub fn new(previous: Option<&'lock Lock>, current: &'lock Lock) -> Self {
        let previous_lock = previous;
        let current_lock = current;
        let previous = previous.map(packages_by_name).unwrap_or_default();
        let current = packages_by_name(current);

        let names = previous
            .keys()
            .chain(current.keys())
            .cloned()
            .collect::<BTreeSet<_>>();

        let mut events = Vec::new();
        for name in names {
            match (previous.get(&name), current.get(&name)) {
                (Some(previous), Some(current)) if previous == current => {}
                (Some(previous), Some(current)) => {
                    events.push(LockDiffEvent::Changed {
                        name,
                        previous: previous.clone(),
                        current: current.clone(),
                    });
                }
                (Some(previous), None) => {
                    events.push(LockDiffEvent::Removed {
                        name,
                        packages: previous.clone(),
                    });
                }
                (None, Some(current)) => {
                    events.push(LockDiffEvent::Added {
                        name,
                        packages: current.clone(),
                    });
                }
                (None, None) => unreachable!("package name must exist in one lockfile"),
            }
        }

        if events.is_empty() && previous_lock.is_none_or(|previous| previous != current_lock) {
            events.push(LockDiffEvent::LockfileChanged);
        }

        Self { events }
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

fn packages_by_name(lock: &Lock) -> BTreeMap<PackageName, Vec<PackageState<'_>>> {
    let mut packages = BTreeMap::<PackageName, Vec<PackageState<'_>>>::new();
    for package in lock.packages() {
        packages
            .entry(package.name().clone())
            .or_default()
            .push(PackageState::from(package));
    }

    for states in packages.values_mut() {
        states.sort_unstable();
    }

    packages
}

#[derive(Debug, Clone)]
struct PackageState<'lock> {
    version: Option<&'lock Version>,
    source: &'lock Source,
    hashes: BTreeSet<String>,
    metadata: &'lock Package,
}

impl<'lock> From<&'lock Package> for PackageState<'lock> {
    fn from(package: &'lock Package) -> Self {
        Self {
            version: package.version(),
            source: &package.id.source,
            hashes: package
                .hashes()
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
            metadata: package,
        }
    }
}

impl PackageState<'_> {
    fn version_label(&self) -> String {
        self.version
            .map(|version| format!("v{version}"))
            .unwrap_or_else(|| "(dynamic)".to_string())
    }
}

impl PartialEq for PackageState<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.source == other.source
            && self.hashes == other.hashes
            && self.metadata == other.metadata
    }
}

impl Eq for PackageState<'_> {}

impl PartialOrd for PackageState<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PackageState<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.version
            .cmp(&other.version)
            .then_with(|| self.source.cmp(other.source))
            .then_with(|| self.hashes.cmp(&other.hashes))
    }
}

#[derive(Debug)]
enum LockDiffEvent<'lock> {
    LockfileChanged,
    Added {
        name: PackageName,
        packages: Vec<PackageState<'lock>>,
    },
    Removed {
        name: PackageName,
        packages: Vec<PackageState<'lock>>,
    },
    Changed {
        name: PackageName,
        previous: Vec<PackageState<'lock>>,
        current: Vec<PackageState<'lock>>,
    },
}

impl std::fmt::Display for LockDiffDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.events.is_empty() {
            writeln!(f, "No lockfile changes detected")?;
            return Ok(());
        }

        writeln!(f, "Lockfile changes:")?;
        for event in &self.events {
            writeln!(f, "{event}")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for LockDiffEvent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LockfileChanged => write!(f, "~ lockfile metadata changed"),
            Self::Added { name, packages } => {
                write!(f, "+ {name} {}", format_states(packages))
            }
            Self::Removed { name, packages } => {
                write!(f, "- {name} {}", format_states(packages))
            }
            Self::Changed {
                name,
                previous,
                current,
            } => {
                if let ([previous], [current]) = (previous.as_slice(), current.as_slice()) {
                    if previous.version != current.version {
                        return write!(
                            f,
                            "~ {name} {} -> {}",
                            previous.version_label(),
                            current.version_label()
                        );
                    }

                    if previous.source != current.source {
                        return write!(
                            f,
                            "~ {name} {} source {} -> {}",
                            previous.version_label(),
                            previous.source,
                            current.source
                        );
                    }

                    if previous.hashes != current.hashes {
                        return write!(f, "~ {name} {} hashes changed", previous.version_label());
                    }

                    if previous.metadata != current.metadata {
                        return write!(f, "~ {name} {} metadata changed", previous.version_label());
                    }
                }

                write!(
                    f,
                    "~ {name} {} -> {}",
                    format_states(previous),
                    format_states(current)
                )
            }
        }
    }
}

fn format_states(states: &[PackageState<'_>]) -> String {
    states
        .iter()
        .map(|state| {
            let mut label = state.version_label();
            let _ = write!(label, " ({})", state.source);
            label
        })
        .join(", ")
}

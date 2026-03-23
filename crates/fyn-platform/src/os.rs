use crate::Error;
use std::fmt;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Os(pub(crate) target_lexicon::OperatingSystem);

impl Os {
    pub fn new(os: target_lexicon::OperatingSystem) -> Self {
        Self(os)
    }

    pub fn from_env() -> Self {
        Self(target_lexicon::HOST.operating_system)
    }

    pub fn is_windows(&self) -> bool {
        matches!(self.0, target_lexicon::OperatingSystem::Windows)
    }

    pub fn is_emscripten(&self) -> bool {
        matches!(self.0, target_lexicon::OperatingSystem::Emscripten)
    }

    pub fn is_macos(&self) -> bool {
        matches!(self.0, target_lexicon::OperatingSystem::Darwin(_))
    }

    /// Whether this OS can run the other OS.
    pub fn supports(&self, other: Self) -> bool {
        // Emscripten can run on any OS
        if other.is_emscripten() {
            return true;
        }

        // Otherwise, we require an exact match
        *self == other
    }
}

impl Display for Os {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &**self {
            target_lexicon::OperatingSystem::Darwin(_) => write!(f, "macos"),
            inner => write!(f, "{inner}"),
        }
    }
}

impl FromStr for Os {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = match s {
            "macos" => target_lexicon::OperatingSystem::Darwin(None),
            _ => target_lexicon::OperatingSystem::from_str(s)
                .map_err(|()| Error::UnknownOs(s.to_string()))?,
        };
        if matches!(inner, target_lexicon::OperatingSystem::Unknown) {
            return Err(Error::UnknownOs(s.to_string()));
        }
        Ok(Self(inner))
    }
}

impl Deref for Os {
    type Target = target_lexicon::OperatingSystem;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&fyn_platform_tags::Os> for Os {
    fn from(value: &fyn_platform_tags::Os) -> Self {
        match value {
            fyn_platform_tags::Os::Dragonfly { .. } => {
                Self::new(target_lexicon::OperatingSystem::Dragonfly)
            }
            fyn_platform_tags::Os::FreeBsd { .. } => {
                Self::new(target_lexicon::OperatingSystem::Freebsd)
            }
            fyn_platform_tags::Os::Haiku { .. } => {
                Self::new(target_lexicon::OperatingSystem::Haiku)
            }
            fyn_platform_tags::Os::Illumos { .. } => {
                Self::new(target_lexicon::OperatingSystem::Illumos)
            }
            fyn_platform_tags::Os::Macos { .. } => {
                Self::new(target_lexicon::OperatingSystem::Darwin(None))
            }
            fyn_platform_tags::Os::Manylinux { .. }
            | fyn_platform_tags::Os::Musllinux { .. }
            | fyn_platform_tags::Os::Android { .. } => {
                Self::new(target_lexicon::OperatingSystem::Linux)
            }
            fyn_platform_tags::Os::NetBsd { .. } => {
                Self::new(target_lexicon::OperatingSystem::Netbsd)
            }
            fyn_platform_tags::Os::OpenBsd { .. } => {
                Self::new(target_lexicon::OperatingSystem::Openbsd)
            }
            fyn_platform_tags::Os::Windows => Self::new(target_lexicon::OperatingSystem::Windows),
            fyn_platform_tags::Os::Pyodide { .. } => {
                Self::new(target_lexicon::OperatingSystem::Emscripten)
            }
            fyn_platform_tags::Os::Ios { .. } => {
                Self::new(target_lexicon::OperatingSystem::IOS(None))
            }
        }
    }
}

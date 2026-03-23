pub use crate::extras::*;
pub use crate::lookahead::*;
pub use crate::source_tree::*;
pub use crate::sources::*;
pub use crate::specification::*;
pub use crate::unnamed::*;

use fv_distribution_types::{
    Dist, DistErrorKind, GitSourceDist, Requirement, RequirementSource, SourceDist,
};

mod extras;
mod lookahead;
mod source_tree;
mod sources;
mod specification;
mod unnamed;
pub mod upgrade;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0} `{1}`")]
    Dist(
        DistErrorKind,
        Box<Dist>,
        #[source] Box<fv_distribution::Error>,
    ),

    #[error(transparent)]
    Distribution(#[from] Box<fv_distribution::Error>),

    #[error(transparent)]
    DistributionTypes(#[from] fv_distribution_types::Error),

    #[error(transparent)]
    WheelFilename(#[from] fv_distribution_filename::WheelFilenameError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Error {
    /// Create an [`Error`] from a distribution error.
    pub(crate) fn from_dist(dist: Dist, err: fv_distribution::Error) -> Self {
        Self::Dist(
            DistErrorKind::from_dist(&dist, &err),
            Box::new(dist),
            Box::new(err),
        )
    }
}

/// Convert a [`Requirement`] into a [`Dist`], if it is a direct URL.
pub(crate) fn required_dist(
    requirement: &Requirement,
) -> Result<Option<Dist>, fv_distribution_types::Error> {
    Ok(Some(match &requirement.source {
        RequirementSource::Registry { .. } => return Ok(None),
        RequirementSource::Url {
            subdirectory,
            location,
            ext,
            url,
        } => Dist::from_http_url(
            requirement.name.clone(),
            url.clone(),
            location.clone(),
            subdirectory.clone(),
            *ext,
        )?,
        RequirementSource::Git {
            git,
            subdirectory,
            url,
        } => Dist::Source(SourceDist::Git(GitSourceDist {
            name: requirement.name.clone(),
            git: Box::new(git.clone()),
            subdirectory: subdirectory.clone(),
            url: url.clone(),
        })),
        RequirementSource::Path {
            install_path,
            ext,
            url,
        } => Dist::from_file_url(requirement.name.clone(), url.clone(), install_path, *ext)?,
        RequirementSource::Directory {
            install_path,
            r#virtual,
            url,
            editable,
        } => Dist::from_directory_url(
            requirement.name.clone(),
            url.clone(),
            install_path,
            *editable,
            *r#virtual,
        )?,
    }))
}

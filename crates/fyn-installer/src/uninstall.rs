use fyn_distribution_types::{InstalledDist, InstalledDistKind, InstalledEggInfoFile};
use fyn_install_wheel::Layout;

/// Uninstall a package from the specified Python environment.
pub async fn uninstall(
    dist: &InstalledDist,
    layout: &Layout,
) -> Result<fyn_install_wheel::Uninstall, UninstallError> {
    let uninstall = tokio::task::spawn_blocking({
        let dist = dist.clone();
        let layout = layout.clone();
        move || match dist.kind {
            InstalledDistKind::Registry(_) | InstalledDistKind::Url(_) => Ok(
                fyn_install_wheel::uninstall_wheel(dist.install_path(), &dist, &layout)?,
            ),
            InstalledDistKind::EggInfoDirectory(_) => Ok(fyn_install_wheel::uninstall_egg(
                dist.install_path(),
                &dist,
                &layout,
            )?),
            InstalledDistKind::LegacyEditable(dist) => Ok(
                fyn_install_wheel::uninstall_legacy_editable(&dist.egg_link)?,
            ),
            InstalledDistKind::EggInfoFile(dist) => Err(UninstallError::Distutils(dist)),
        }
    })
    .await??;

    Ok(uninstall)
}

#[derive(thiserror::Error, Debug)]
pub enum UninstallError {
    #[error(
        "Unable to uninstall `{0}`. distutils-installed distributions do not include the metadata required to uninstall safely."
    )]
    Distutils(InstalledEggInfoFile),
    #[error(transparent)]
    Uninstall(#[from] fyn_install_wheel::Error),
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
}

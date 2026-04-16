use std::env;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use itertools::Itertools;
use reqwest::{Certificate, Identity};
use rustls_native_certs::{CertificateResult, load_certs_from_paths};
use rustls_pki_types::CertificateDer;
use tracing::{debug, warn};
use webpki::{Error as WebPkiError, anchor_from_trusted_cert};
use x509_parser::prelude::{FromDer, X509Certificate};

use fyn_fs::Simplified;
use fyn_static::EnvVars;
use fyn_warnings::warn_user_once;

#[derive(Debug, Clone)]
pub(crate) enum CertificateSource {
    SslCertFile(PathBuf),
    SslCertDir(PathBuf),
}

impl CertificateSource {
    const fn env_var(&self) -> &'static str {
        match self {
            Self::SslCertFile(_) => EnvVars::SSL_CERT_FILE,
            Self::SslCertDir(_) => EnvVars::SSL_CERT_DIR,
        }
    }

    fn path(&self) -> &Path {
        match self {
            Self::SslCertFile(path) | Self::SslCertDir(path) => path,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DiagnosticCertificate(CertificateDer<'static>);

impl DiagnosticCertificate {
    fn parse(&self) -> Option<X509Certificate<'_>> {
        match X509Certificate::from_der(self.0.as_ref()) {
            Ok((_, certificate)) => Some(certificate),
            Err(err) => {
                debug!("Failed to parse certificate for improved validation message: {err:?}");
                None
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct InvalidCertificateWarning {
    source: CertificateSource,
    certificate: DiagnosticCertificate,
    reason: InvalidCertificateReason,
}

#[derive(Debug)]
pub(crate) enum InvalidCertificateReason {
    UnsupportedCriticalExtension,
    BadDer,
    BadDerTime,
    EmptyEkuExtension,
    ExtensionValueInvalid,
    MalformedExtensions,
    TrailingData,
    UnsupportedCertVersion,
    Other(WebPkiError),
}

impl InvalidCertificateReason {
    fn from_webpki_error(error: WebPkiError) -> Self {
        match error {
            WebPkiError::UnsupportedCriticalExtension => Self::UnsupportedCriticalExtension,
            WebPkiError::BadDer => Self::BadDer,
            WebPkiError::BadDerTime => Self::BadDerTime,
            WebPkiError::EmptyEkuExtension => Self::EmptyEkuExtension,
            WebPkiError::ExtensionValueInvalid => Self::ExtensionValueInvalid,
            WebPkiError::MalformedExtensions => Self::MalformedExtensions,
            WebPkiError::TrailingData(_) => Self::TrailingData,
            WebPkiError::UnsupportedCertVersion => Self::UnsupportedCertVersion,
            error => Self::Other(error),
        }
    }

    fn message(&self) -> Option<&'static str> {
        match self {
            Self::UnsupportedCriticalExtension => None,
            Self::BadDer => Some("malformed DER certificate"),
            Self::BadDerTime => Some("malformed certificate time"),
            Self::EmptyEkuExtension => Some("empty extended key usage extension"),
            Self::ExtensionValueInvalid => Some("invalid certificate extension value"),
            Self::MalformedExtensions => Some("malformed certificate extensions"),
            Self::TrailingData => Some("trailing data in DER certificate"),
            Self::UnsupportedCertVersion => Some("unsupported certificate version"),
            Self::Other(_) => None,
        }
    }
}

impl InvalidCertificateWarning {
    fn new(source: CertificateSource, cert: &CertificateDer<'_>, error: WebPkiError) -> Self {
        Self {
            source,
            certificate: DiagnosticCertificate(cert.clone().into_owned()),
            reason: InvalidCertificateReason::from_webpki_error(error),
        }
    }
}

fn format_invalid_certificate_detail(
    reason: &InvalidCertificateReason,
    certificate: Option<&X509Certificate<'_>>,
) -> Option<String> {
    match reason {
        InvalidCertificateReason::UnsupportedCertVersion => certificate.map(|certificate| {
            format!(
                "unsupported certificate version `{}`",
                certificate.version()
            )
        }),
        InvalidCertificateReason::ExtensionValueInvalid => None,
        _ => None,
    }
}

impl Display for InvalidCertificateWarning {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "certificate in `{}` (from `{}`) ",
            self.source.path().simplified_display(),
            self.source.env_var()
        )?;
        match &self.reason {
            InvalidCertificateReason::UnsupportedCriticalExtension => {
                write!(f, "uses an unsupported critical extension")?;
            }
            _ => {
                write!(f, "could not be used as a trust anchor")?;
            }
        }

        let parsed_certificate = self.certificate.parse();
        if let Some(certificate) = parsed_certificate.as_ref() {
            let subject = certificate.subject();
            if subject.iter_attributes().next().is_some() {
                // Avoid rendering empty subject DNs.
                write!(f, " on certificate `{subject}`")?;
            }
            if let InvalidCertificateReason::UnsupportedCriticalExtension = &self.reason {
                let critical_extensions = certificate
                    .iter_extensions()
                    .filter(|extension| extension.critical)
                    .map(|extension| extension.oid.to_owned())
                    .collect::<Vec<_>>();
                if let [critical_extension] = critical_extensions.as_slice() {
                    write!(f, "; critical extension: `{critical_extension}`")?;
                } else if !critical_extensions.is_empty() {
                    write!(
                        f,
                        "; critical extensions: {}",
                        critical_extensions
                            .iter()
                            .map(|oid| format!("`{oid}`"))
                            .join(", ")
                    )?;
                }
            }
        }

        let detailed_reason =
            format_invalid_certificate_detail(&self.reason, parsed_certificate.as_ref())
                .or_else(|| self.reason.message().map(str::to_owned))
                .or_else(|| {
                    if let InvalidCertificateReason::Other(error) = &self.reason {
                        Some(format!("{error:?}"))
                    } else {
                        None
                    }
                });
        if let Some(detailed_reason) = detailed_reason {
            write!(f, ": {detailed_reason}")?;
        }

        Ok(())
    }
}

/// A collection of TLS certificates in DER form.
#[derive(Debug, Clone, Default)]
pub(crate) struct Certificates(Vec<CertificateDer<'static>>);

impl Certificates {
    /// Load custom CA certificates from `SSL_CERT_FILE` and `SSL_CERT_DIR`.
    pub(crate) fn from_env() -> Option<Self> {
        let mut certs = Self::default();
        let mut has_source = false;

        if let Some(ssl_cert_file) = env::var_os(EnvVars::SSL_CERT_FILE)
            && let Some(file_certs) = Self::from_ssl_cert_file(&ssl_cert_file)
        {
            has_source = true;
            certs.merge(file_certs);
        }

        if let Some(ssl_cert_dir) = env::var_os(EnvVars::SSL_CERT_DIR)
            && let Some(dir_certs) = Self::from_ssl_cert_dir(&ssl_cert_dir)
        {
            has_source = true;
            certs.merge(dir_certs);
        }

        if has_source { Some(certs) } else { None }
    }

    fn from_ssl_cert_file(ssl_cert_file: &OsStr) -> Option<Self> {
        if ssl_cert_file.is_empty() {
            return None;
        }

        let file = PathBuf::from(ssl_cert_file);
        match file.metadata() {
            Ok(metadata) if metadata.is_file() => {
                let result = Self::from_paths(Some(&file), None);
                for err in &result.errors {
                    warn_user_once!(
                        "Failed to load `SSL_CERT_FILE` ({}): {err}",
                        file.simplified_display().cyan()
                    );
                }
                let certs = Self::from(result)
                    .filter_invalid(&CertificateSource::SslCertFile(file.clone()));
                if certs.0.is_empty() {
                    warn_user_once!(
                        "Ignoring `SSL_CERT_FILE`. No valid certificates found in: {}.",
                        file.simplified_display().cyan()
                    );
                    return None;
                }
                Some(certs)
            }
            Ok(_) => {
                warn_user_once!(
                    "Ignoring invalid `SSL_CERT_FILE`. Path is not a file: {}.",
                    file.simplified_display().cyan()
                );
                None
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                warn_user_once!(
                    "Ignoring invalid `SSL_CERT_FILE`. Path does not exist: {}.",
                    file.simplified_display().cyan()
                );
                None
            }
            Err(err) => {
                warn_user_once!(
                    "Ignoring invalid `SSL_CERT_FILE`. Path is not accessible: {} ({err}).",
                    file.simplified_display().cyan()
                );
                None
            }
        }
    }

    fn from_ssl_cert_dir(ssl_cert_dir: &OsStr) -> Option<Self> {
        if ssl_cert_dir.is_empty() {
            return None;
        }

        let (existing, missing): (Vec<_>, Vec<_>) =
            env::split_paths(ssl_cert_dir).partition(|path| path.exists());

        if existing.is_empty() {
            let end_note = if missing.len() == 1 {
                "The directory does not exist."
            } else {
                "The entries do not exist."
            };
            warn_user_once!(
                "Ignoring invalid `SSL_CERT_DIR`. {end_note}: {}.",
                missing
                    .iter()
                    .map(Simplified::simplified_display)
                    .join(", ")
                    .cyan()
            );
            return None;
        }

        if !missing.is_empty() {
            let end_note = if missing.len() == 1 {
                "The following directory does not exist:"
            } else {
                "The following entries do not exist:"
            };
            warn_user_once!(
                "Invalid entries in `SSL_CERT_DIR`. {end_note}: {}.",
                missing
                    .iter()
                    .map(Simplified::simplified_display)
                    .join(", ")
                    .cyan()
            );
        }

        let mut certs = Self::default();
        for dir in &existing {
            let result = Self::from_paths(None, Some(dir));
            for err in &result.errors {
                warn_user_once!(
                    "Failed to load `SSL_CERT_DIR` ({}): {err}",
                    dir.simplified_display().cyan()
                );
            }
            let dir_certs =
                Self::from(result).filter_invalid(&CertificateSource::SslCertDir(dir.clone()));
            if !dir_certs.0.is_empty() {
                certs.merge(dir_certs);
            }
        }

        if certs.0.is_empty() {
            // Unlike `SSL_CERT_FILE`, it's plausible for this to be intentionally set to an
            // empty directory that a user could put certificates in later.
            warn!(
                "Ignoring `SSL_CERT_DIR`. No valid certificates found in: {}.",
                existing
                    .iter()
                    .map(Simplified::simplified_display)
                    .join(", ")
            );
            return None;
        }

        Some(certs)
    }

    fn from_paths(file: Option<&Path>, dir: Option<&Path>) -> CertificateResult {
        load_certs_from_paths(file, dir)
    }

    fn filter_invalid(mut self, source: &CertificateSource) -> Self {
        self.0.retain(|cert| {
            if let Err(error) = anchor_from_trusted_cert(cert) {
                let warning = InvalidCertificateWarning::new((*source).clone(), cert, error);
                warn!("Ignoring invalid certificate: {warning}");
                return false;
            }

            true
        });
        self
    }

    fn dedup(&mut self) {
        self.0
            .sort_unstable_by(|left, right| left.as_ref().cmp(right.as_ref()));
        self.0.dedup();
    }

    fn merge(&mut self, other: Self) {
        self.0.extend(other.0);
        self.dedup();
    }

    pub(crate) fn to_reqwest_certs(&self) -> Vec<Certificate> {
        self.0
            .iter()
            .filter_map(|cert| match Certificate::from_der(cert) {
                Ok(certificate) => Some(certificate),
                Err(err) => {
                    debug!("Failed to convert DER certificate to reqwest certificate: {err}");
                    None
                }
            })
            .collect()
    }

    #[cfg(test)]
    fn iter(&self) -> impl Iterator<Item = &CertificateDer<'static>> {
        self.0.iter()
    }
}

impl From<CertificateResult> for Certificates {
    fn from(result: CertificateResult) -> Self {
        Self(result.certs)
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum CertificateError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Reqwest(reqwest::Error),
}

/// Return the `Identity` from the provided file.
pub(crate) fn read_identity(ssl_client_cert: &OsStr) -> Result<Identity, CertificateError> {
    let mut buf = Vec::new();
    fs_err::File::open(ssl_client_cert)?.read_to_end(&mut buf)?;
    Identity::from_pem(&buf).map_err(|tls_err| {
        debug_assert!(tls_err.is_builder(), "must be a rustls::Error internally");
        CertificateError::Reqwest(tls_err)
    })
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use rcgen::{
        BasicConstraints, CertificateParams, CustomExtension, DnType, IsCa, KeyPair,
        KeyUsagePurpose, date_time_ymd,
    };
    use rustls_pki_types::pem::PemObject;

    use super::*;

    fn generate_cert_pem() -> String {
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
        cert.cert.pem()
    }

    fn generate_ca_pem_with_extensions(custom_extensions: Vec<CustomExtension>) -> String {
        let mut params = CertificateParams::default();
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        params.not_before = date_time_ymd(1975, 1, 1);
        params.not_after = date_time_ymd(4096, 1, 1);
        params.key_usages.push(KeyUsagePurpose::DigitalSignature);
        params.key_usages.push(KeyUsagePurpose::KeyCertSign);
        params.key_usages.push(KeyUsagePurpose::CrlSign);
        params
            .distinguished_name
            .push(DnType::OrganizationName, "fyn contributors");
        params
            .distinguished_name
            .push(DnType::CommonName, "fyn-test-ca");
        params.custom_extensions = custom_extensions;

        let private = KeyPair::generate().unwrap();
        params.self_signed(&private).unwrap().pem()
    }

    #[test]
    fn test_from_ssl_cert_file_nonexistent_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let missing_file = dir.path().join("missing.pem");

        let certs = Certificates::from_ssl_cert_file(missing_file.as_os_str());
        assert!(certs.is_none());
    }

    #[test]
    fn test_from_ssl_cert_file_empty_value_returns_none() {
        let certs = Certificates::from_ssl_cert_file(OsString::new().as_os_str());
        assert!(certs.is_none());
    }

    #[test]
    fn test_from_ssl_cert_file_no_valid_certs_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let cert_path = dir.path().join("empty.pem");
        fs_err::write(&cert_path, "not a certificate").unwrap();

        let certs = Certificates::from_ssl_cert_file(cert_path.as_os_str());
        assert!(certs.is_none());
    }

    #[test]
    fn test_from_ssl_cert_dir_empty_value_returns_none() {
        let certs = Certificates::from_ssl_cert_dir(OsString::new().as_os_str());
        assert!(certs.is_none());
    }

    #[test]
    fn test_from_ssl_cert_dir_nonexistent_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let missing_dir = dir.path().join("missing-dir");
        let cert_dirs = std::env::join_paths([&missing_dir]).unwrap();

        let certs = Certificates::from_ssl_cert_dir(cert_dirs.as_os_str());
        assert!(certs.is_none());
    }

    #[test]
    fn test_from_ssl_cert_dir_empty_existing_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let cert_dirs = std::env::join_paths([dir.path()]).unwrap();

        let certs = Certificates::from_ssl_cert_dir(cert_dirs.as_os_str());
        assert!(certs.is_none());
    }

    #[test]
    fn test_from_ssl_cert_file_unsupported_critical_extension_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let cert_path = dir.path().join("unsupported-critical.pem");
        let mut unsupported_extension = CustomExtension::from_oid_content(
            &[1, 2, 3, 4],
            [vec![0x0c, 0x0b], b"unsupported".to_vec()].concat(),
        );
        unsupported_extension.set_criticality(true);
        fs_err::write(
            &cert_path,
            generate_ca_pem_with_extensions(vec![unsupported_extension]),
        )
        .unwrap();

        let certs = Certificates::from_ssl_cert_file(cert_path.as_os_str());
        assert!(certs.is_none());
    }

    #[test]
    fn test_from_ssl_cert_file_invalid_trust_anchor_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let cert_path = dir.path().join("invalid-anchor.pem");
        let duplicate_basic_constraints =
            CustomExtension::from_oid_content(&[2, 5, 29, 19], vec![0x30, 0x00]);
        fs_err::write(
            &cert_path,
            generate_ca_pem_with_extensions(vec![duplicate_basic_constraints]),
        )
        .unwrap();

        let certs = Certificates::from_ssl_cert_file(cert_path.as_os_str());
        assert!(certs.is_none());
    }

    #[test]
    fn test_invalid_certificate_warning_formats_specific_trust_anchor_reason() {
        let duplicate_basic_constraints =
            CustomExtension::from_oid_content(&[2, 5, 29, 19], vec![0x30, 0x00]);
        let cert_pem = generate_ca_pem_with_extensions(vec![duplicate_basic_constraints]);
        let cert = CertificateDer::pem_slice_iter(cert_pem.as_bytes())
            .next()
            .unwrap()
            .unwrap();

        let warning = InvalidCertificateWarning::new(
            CertificateSource::SslCertFile(PathBuf::from("invalid-anchor.pem")),
            &cert,
            WebPkiError::ExtensionValueInvalid,
        );
        let display = warning.to_string();

        assert!(display.contains("SSL_CERT_FILE"));
        assert!(display.contains("trust anchor"));
        assert!(display.contains("invalid certificate extension value"));
    }

    #[test]
    fn test_from_ssl_cert_file_bundle_ignores_invalid_trust_anchor() {
        let dir = tempfile::tempdir().unwrap();
        let cert_path = dir.path().join("bundle.pem");
        let duplicate_basic_constraints =
            CustomExtension::from_oid_content(&[2, 5, 29, 19], vec![0x30, 0x00]);
        fs_err::write(
            &cert_path,
            format!(
                "{}\n{}",
                generate_ca_pem_with_extensions(vec![duplicate_basic_constraints]),
                generate_cert_pem()
            ),
        )
        .unwrap();

        let certs = Certificates::from_ssl_cert_file(cert_path.as_os_str()).unwrap();
        assert_eq!(certs.iter().count(), 1);
    }

    #[test]
    fn test_from_ssl_cert_dir_ignores_invalid_trust_anchor() {
        let dir = tempfile::tempdir().unwrap();
        let duplicate_basic_constraints =
            CustomExtension::from_oid_content(&[2, 5, 29, 19], vec![0x30, 0x00]);
        fs_err::write(
            dir.path().join("invalid.pem"),
            generate_ca_pem_with_extensions(vec![duplicate_basic_constraints]),
        )
        .unwrap();
        fs_err::write(dir.path().join("valid.pem"), generate_cert_pem()).unwrap();

        let cert_dirs = std::env::join_paths([dir.path()]).unwrap();
        let certs = Certificates::from_ssl_cert_dir(cert_dirs.as_os_str()).unwrap();
        assert_eq!(certs.iter().count(), 1);
    }

    #[test]
    fn test_merge_deduplicates() {
        let dir = tempfile::tempdir().unwrap();
        let cert_path = dir.path().join("cert.pem");
        fs_err::write(&cert_path, generate_cert_pem()).unwrap();

        let first = Certificates::from(Certificates::from_paths(Some(&cert_path), None));
        let second = Certificates::from(Certificates::from_paths(Some(&cert_path), None));

        let mut merged = first;
        merged.merge(second);

        assert_eq!(merged.iter().count(), 1);
    }
}

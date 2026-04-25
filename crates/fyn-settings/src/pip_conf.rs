use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use configparser::ini::Ini;
use fyn_configuration::TrustedHost;
use fyn_dirs::user_config_dir;
use fyn_distribution_types::{Index, IndexUrl, Origin, PipExtraIndex, PipFindLinks, PipIndex};

use crate::settings::{GlobalOptions, Options, PipOptions};
use crate::{Combine, Error, FilesystemOptions};

type PipConfSection = HashMap<String, Option<String>>;

const PIP_CONFIG_FILE: &str = "PIP_CONFIG_FILE";
const VIRTUAL_ENV: &str = "VIRTUAL_ENV";

#[derive(thiserror::Error, Debug)]
enum PipConfigError {
    #[error("{0}")]
    Parse(String),
    #[error("invalid value for `{option}`: `{value}`")]
    Index {
        option: &'static str,
        value: String,
        #[source]
        source: fyn_distribution_types::IndexUrlError,
    },
    #[error("invalid value for `trusted-host`: `{value}`")]
    TrustedHost {
        value: String,
        #[source]
        source: fyn_configuration::TrustedHostError,
    },
    #[error("invalid value for `no-index`: `{value}` (expected a boolean)")]
    Bool { value: String },
}

pub(crate) fn load() -> Result<Option<FilesystemOptions>, Error> {
    let mut combined = None;

    for (path, origin) in config_files() {
        let Some(options) = read_config_file(&path, origin)? else {
            continue;
        };
        combined = Some(options).combine(combined);
    }

    Ok(combined)
}

fn read_config_file(path: &Path, origin: Origin) -> Result<Option<FilesystemOptions>, Error> {
    let content = match fs_err::read_to_string(path) {
        Ok(content) => content,
        Err(err)
            if matches!(
                err.kind(),
                std::io::ErrorKind::NotFound
                    | std::io::ErrorKind::NotADirectory
                    | std::io::ErrorKind::PermissionDenied
            ) =>
        {
            return Ok(None);
        }
        Err(err) => return Err(err.into()),
    };

    let options = parse_config(&content, path)
        .map_err(|err| Error::PipConfig(path.to_path_buf(), Box::new(err)))?;
    Ok(options.map(|options| FilesystemOptions::from(options.with_origin(origin))))
}

fn parse_config(content: &str, path: &Path) -> Result<Option<Options>, PipConfigError> {
    let mut ini = Ini::new();
    ini.set_multiline(true);
    let sections = ini
        .read(content.to_string())
        .map_err(PipConfigError::Parse)?;

    let Some(global) = sections.get("global") else {
        return Ok(None);
    };

    let root_dir = path.parent();
    let mut globals = GlobalOptions::default();
    let mut pip = PipOptions::default();
    let mut has_globals = false;
    let mut has_pip = false;

    if let Some(value) = get_value(global, "index-url") {
        pip.index_url = Some(parse_pip_index("index-url", value, root_dir)?);
        has_pip = true;
    }

    let extra_index_url = parse_pip_extra_indexes(global, root_dir)?;
    if !extra_index_url.is_empty() {
        pip.extra_index_url = Some(extra_index_url);
        has_pip = true;
    }

    let find_links = parse_pip_find_links(global, root_dir)?;
    if !find_links.is_empty() {
        pip.find_links = Some(find_links);
        has_pip = true;
    }

    if global.contains_key("no-index") {
        pip.no_index = Some(parse_bool(get_value(global, "no-index").unwrap_or("true"))?);
        has_pip = true;
    }

    let trusted_hosts = parse_trusted_hosts(global)?;
    if !trusted_hosts.is_empty() {
        globals.allow_insecure_host = Some(trusted_hosts);
        has_globals = true;
    }

    if !has_globals && !has_pip {
        return Ok(None);
    }

    Ok(Some(Options {
        globals,
        pip: has_pip.then_some(pip),
        ..Options::default()
    }))
}

fn get_value<'a>(section: &'a PipConfSection, key: &'static str) -> Option<&'a str> {
    section
        .get(key)
        .and_then(Option::as_deref)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn get_values<'a>(section: &'a PipConfSection, key: &'static str) -> impl Iterator<Item = &'a str> {
    get_value(section, key)
        .into_iter()
        .flat_map(str::split_whitespace)
}

fn parse_index_url(
    option: &'static str,
    value: &str,
    root_dir: Option<&Path>,
) -> Result<IndexUrl, PipConfigError> {
    IndexUrl::parse(value, root_dir).map_err(|source| PipConfigError::Index {
        option,
        value: value.to_string(),
        source,
    })
}

fn parse_pip_index(
    option: &'static str,
    value: &str,
    root_dir: Option<&Path>,
) -> Result<PipIndex, PipConfigError> {
    Ok(Index::from_index_url(parse_index_url(option, value, root_dir)?).into())
}

fn parse_pip_extra_indexes(
    section: &PipConfSection,
    root_dir: Option<&Path>,
) -> Result<Vec<PipExtraIndex>, PipConfigError> {
    get_values(section, "extra-index-url")
        .map(|value| {
            parse_index_url("extra-index-url", value, root_dir)
                .map(Index::from_extra_index_url)
                .map(Into::into)
        })
        .collect()
}

fn parse_pip_find_links(
    section: &PipConfSection,
    root_dir: Option<&Path>,
) -> Result<Vec<PipFindLinks>, PipConfigError> {
    get_values(section, "find-links")
        .map(|value| {
            parse_index_url("find-links", value, root_dir)
                .map(Index::from_find_links)
                .map(Into::into)
        })
        .collect()
}

fn parse_trusted_hosts(section: &PipConfSection) -> Result<Vec<TrustedHost>, PipConfigError> {
    get_values(section, "trusted-host")
        .map(|value| {
            TrustedHost::from_str(value).map_err(|source| PipConfigError::TrustedHost {
                value: value.to_string(),
                source,
            })
        })
        .collect()
}

fn parse_bool(value: &str) -> Result<bool, PipConfigError> {
    match value.to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Ok(true),
        "0" | "false" | "no" | "off" => Ok(false),
        _ => Err(PipConfigError::Bool {
            value: value.to_string(),
        }),
    }
}

fn config_files() -> Vec<(PathBuf, Origin)> {
    let config_file = std::env::var_os(PIP_CONFIG_FILE)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);

    if config_file.as_deref().is_some_and(is_dev_null) {
        return Vec::new();
    }

    let mut files = standard_config_files();

    if let Some(config_file) = config_file {
        files.push((config_file, Origin::User));
    }

    files
}

fn standard_config_files() -> Vec<(PathBuf, Origin)> {
    let mut files = Vec::new();

    if cfg!(windows) {
        if let Some(program_data) =
            std::env::var_os("PROGRAMDATA").filter(|value| !value.is_empty())
        {
            files.push((
                PathBuf::from(program_data).join("pip").join("pip.ini"),
                Origin::System,
            ));
        }
        if let Some(appdata) = std::env::var_os("APPDATA").filter(|value| !value.is_empty()) {
            files.push((
                PathBuf::from(appdata).join("pip").join("pip.ini"),
                Origin::User,
            ));
        }
    } else {
        files.push((PathBuf::from("/etc/xdg/pip/pip.conf"), Origin::System));
        files.push((PathBuf::from("/etc/pip.conf"), Origin::System));

        if let Some(config_dir) = user_config_dir() {
            files.push((config_dir.join("pip").join("pip.conf"), Origin::User));
        }
        if let Some(home) = home_dir() {
            files.push((home.join(".pip").join("pip.conf"), Origin::User));
        }
    }

    if let Some(virtual_env) = std::env::var_os(VIRTUAL_ENV).filter(|value| !value.is_empty()) {
        files.push((
            PathBuf::from(virtual_env).join(config_filename()),
            Origin::User,
        ));
    }

    files
}

fn config_filename() -> &'static str {
    if cfg!(windows) { "pip.ini" } else { "pip.conf" }
}

fn home_dir() -> Option<PathBuf> {
    let env_var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    std::env::var_os(env_var)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn is_dev_null(path: &Path) -> bool {
    if cfg!(windows) {
        path.to_str()
            .is_some_and(|path| path.eq_ignore_ascii_case("NUL"))
    } else {
        path == Path::new("/dev/null")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_global_pip_config() {
        let options = parse_config(
            r"[global]
index-url = https://example.com/simple
extra-index-url =
    https://extra.example.com/simple
    https://extra2.example.com/simple
find-links =
    ./wheels
    https://example.com/packages.html
no-index = false
trusted-host =
    example.com
    https://extra.example.com:8443
",
            Path::new("/tmp/pip.conf"),
        )
        .unwrap()
        .unwrap();

        let pip = options.pip.unwrap();
        assert!(pip.index_url.is_some());
        assert_eq!(pip.extra_index_url.unwrap().len(), 2);
        assert_eq!(pip.find_links.unwrap().len(), 2);
        assert_eq!(pip.no_index, Some(false));
        assert_eq!(options.globals.allow_insecure_host.unwrap().len(), 2);
    }

    #[test]
    fn ignore_non_global_sections() {
        let options = parse_config(
            r"[install]
index-url = https://example.com/simple
",
            Path::new("/tmp/pip.conf"),
        )
        .unwrap();

        assert!(options.is_none());
    }

    #[test]
    fn no_index_without_value_is_true() {
        let options = parse_config(
            r"[global]
no-index
",
            Path::new("/tmp/pip.conf"),
        )
        .unwrap()
        .unwrap();

        assert_eq!(options.pip.unwrap().no_index, Some(true));
    }
}

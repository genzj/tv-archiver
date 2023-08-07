use std::path::PathBuf;

use glob::GlobError;

use super::{CollectResult, Metadata, SourceCollector};

#[derive(Debug)]
pub struct GlobSourceCollector {
    patterns: Vec<String>,
}

impl GlobSourceCollector {
    pub fn new(patterns: Vec<String>) -> GlobSourceCollector {
        GlobSourceCollector { patterns }
    }
}

const ERR_CANNOT_READ_CWD: &str = &"cannot read cwd";
const ERR_CANNOT_CHANGE_CWD: &str = &"cannot change cwd";
const ERR_CANNOT_RESTORE_CWD: &str = &"cannot restore cwd";

fn extract_file_only(path: Result<PathBuf, GlobError>) -> Option<PathBuf> {
    if path.is_ok() && path.as_ref().unwrap().is_file() {
        Some(path.unwrap())
    } else {
        // TODO: log glob error
        None
    }
}

fn from_path(path: PathBuf) -> Result<Metadata, String> {
    let path = &path.to_owned();
    let mut metadata = Metadata::default();

    let absolute_path = path
        .canonicalize()
        .map_err(|err| err.to_string())?
        .to_str()
        .map(String::from);
    metadata.absolute_path = absolute_path.to_owned();
    metadata.original_absolute_path = absolute_path.to_owned();

    let filename = path
        .file_name()
        .ok_or_else(|| format!("cannot get filename from path {}", path.to_string_lossy()))?
        .to_str()
        .map(String::from);
    metadata.filename = filename.to_owned();
    metadata.original_name = filename.to_owned();

    metadata.original_path = path.to_str().map(String::from);
    metadata.original_basename = path
        .file_stem()
        .map(|s| s.to_string_lossy())
        .map(String::from);
    // metadata.original_extension = Some(path.extension());
    // metadata.original_directory = Some(path.parent().ok_or_else(|| "TODO")?);

    Ok(metadata)
}

fn glob_in(pattern: &String, source: &String) -> CollectResult {
    let org_path = std::env::current_dir().map_err(|_| ERR_CANNOT_READ_CWD)?;
    std::env::set_current_dir(source).map_err(|_| ERR_CANNOT_CHANGE_CWD)?;

    let ans = glob::glob(&pattern)
        .map_err(|e| e.to_string())?
        .filter_map(extract_file_only)
        .map(from_path)
        .map(|m|{
            if m.is_ok() {
                m
            } else {
                println!("err: {:?}", m.clone().err());
                m
            }
        })
        .filter(Result::is_ok) // TODO: log errors
        .map(Result::unwrap)
        .collect();

    std::env::set_current_dir(org_path).map_err(|_| ERR_CANNOT_RESTORE_CWD)?;

    Ok(ans)
}

impl SourceCollector for GlobSourceCollector {
    fn collect(&self, source: &String) -> CollectResult {
        let mut ans: Vec<Metadata> = Vec::new();
        for pattern in &self.patterns {
            ans.extend(glob_in(pattern, source)?)
        }
        Ok(ans)
    }
}

/// Enum representing possible values: `SKIP` or `ABORT`.
#[derive(Debug)]
pub enum ViolationAction {
    /// Represents the value `SKIP`.
    SKIP,
    /// Represents the value `ABORT`.
    ABORT,
}

/// Represents a violation with information about the violated rule and the action taken.
#[derive(Debug)]
pub struct Violation {
    /// The violated rule.
    rule: String,
    /// The action taken, either "SKIP" or "ABORT".
    action: ViolationAction,
}

/// Represents a file with its original and target properties.
///
/// This struct contains various attributes of a file
#[derive(Debug, Default)]
pub struct Metadata {
    /// The current absolute path of the file.
    absolute_path: Option<String>,
    /// The current file name (including the base name and extension).
    /// It may differ from `original_name` if a `Rename` executor has been applied to the file.
    filename: Option<String>,
    /// The absolute path of the file when it was collected.
    original_absolute_path: Option<String>,
    /// The base name of the file when it was collected.
    original_basename: Option<String>,
    /// The relative directory from which the file was collected.
    original_directory: Option<String>,
    /// The extension of the file when it was collected.
    original_extension: Option<String>,
    /// The name of the file when it was collected.
    original_name: Option<String>,
    /// The relative path of the file when it was collected.
    original_path: Option<String>,

    /// Absolute path of the directory where the file should be moved.
    destination: Option<String>,
    /// The desired file name (including base and extension names) to which the file should be renamed.
    next_name: Option<String>,

    /// The show's official/designated name.
    canonical_name: Option<String>,
    /// The season identification number.
    season: Option<i32>,
    /// The episode number within the season.
    episode: Option<i32>,

    /// The list of violations.
    violations: Vec<Violation>,
}

mod test {
    use super::*;

    #[test]
    fn test_default_metadata() {
        let metadata = Metadata::default();
        assert_eq!(metadata.violations.len(), 0);
        assert!(metadata.season.is_none());
        assert!(metadata.canonical_name.is_none());
    }
}

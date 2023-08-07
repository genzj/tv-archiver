/// Enum representing possible values: `SKIP` or `ABORT`.
#[derive(Debug, Clone)]
pub enum ViolationAction {
    /// Represents the value `SKIP`.
    SKIP,
    /// Represents the value `ABORT`.
    ABORT,
}

/// Represents a violation with information about the violated rule and the action taken.
#[derive(Debug, Clone)]
pub struct Violation {
    /// The violated rule.
    pub rule: String,
    /// The action taken, either "SKIP" or "ABORT".
    pub action: ViolationAction,
}

/// Represents a file with its original and target properties.
///
/// This struct contains various attributes of a file
#[derive(Debug, Default, Clone)]
pub struct Metadata {
    /// The current absolute path of the file.
    pub absolute_path: Option<String>,
    /// The current file name (including the base name and extension).
    /// It may differ from `original_name` if a `Rename` executor has been applied to the file.
    pub filename: Option<String>,
    /// The absolute path of the file when it was collected.
    pub original_absolute_path: Option<String>,
    /// The base name of the file when it was collected.
    pub original_basename: Option<String>,
    /// The relative directory from which the file was collected.
    pub original_directory: Option<String>,
    /// The extension of the file when it was collected.
    pub original_extension: Option<String>,
    /// The name of the file when it was collected.
    pub original_name: Option<String>,
    /// The relative path of the file when it was collected.
    pub original_path: Option<String>,

    /// Absolute path of the directory where the file should be moved.
    pub destination: Option<String>,
    /// The desired file name (including base and extension names) to which the file should be renamed.
    pub next_name: Option<String>,

    /// The show's official/designated name.
    pub canonical_name: Option<String>,
    /// The season identification number.
    pub season: Option<i32>,
    /// The episode number within the season.
    pub episode: Option<i32>,

    /// The list of violations.
    pub violations: Vec<Violation>,
}

mod test {
    #[test]
    fn test_default_metadata() {
        use super::Metadata;

        let metadata = Metadata::default();
        assert_eq!(metadata.violations.len(), 0);
        assert!(metadata.season.is_none());
        assert!(metadata.canonical_name.is_none());
    }
}

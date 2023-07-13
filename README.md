# TV Archiver

A tool that renames and archives TV/Anime episodes by specified rules.

## Running Process

1. Run specified **source collectors** to gather source files;
2. Deduplicate then sort collected source files alphabetically based on their absolute path;
3. For each source file, run specified **metadata annotators**;
4. Run specified **checkers** on the list of annotated files;
   1. Add violation (if any) to related files' metadata;
   2. If any checker finds a violation with the **abort** action, stop current process and exit with error messages;
5. Execute the designated **executors** on the files that do not have any violations.
6. Execute the designated **fallback executors** on the files that have violation(s).

## Source Collector

**Notes**:
* Currently, symlinks are not supported. Therefore, any collected symlinks will be excluded from the source file list. This filtering process is performed by an unconfigurable filter that runs after the execution of all designated collectors but before deduplicating and sorting the files.
* Multiple collectors can be executed concurrently within a thread pool.

### Glob Collector

Gather source files by running a list of [glob](https://docs.rs/glob/latest/glob/) patterns within the source directory.

| Setting  | Type     | Description                                                  | Required? | Default Value | Example                                 |
| -------- | -------- | ------------------------------------------------------------ | --------- | ------------- | --------------------------------------- |
| source   | string   | The source directory under which the patterns should be run. | Yes       |               | `/path/to/file` , (windows) `d:\files`. |
| patterns | string[] | An array of glob patterns                                    | Yes       |               | `[ "**/*.mp4", "**/*.mkv" ]`            |

## Metadata Annotator

**Notes**:
* Different annotators run sequentially. But within a certain annotator, multiple files can be processed concurrently within a thread pool.

### Fuzzy Search

### Regex

### Season Infer

### Episode Infer

### Template



## Checker

**Notes**:
* Multiple checkers can be executed concurrently within a thread pool.
* Violation annotation happens after all checkers have finished running. Multiple files can be annotated concurrently within a thread pool.

### No Duplicate Name

## Executor

**Notes**:
* Different executors run sequentially. But within a certain executor, multiple files can be processed concurrently within a thread pool.

### Rename

### Move


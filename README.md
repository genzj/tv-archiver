# TV Archiver

A tool that renames and archives TV/Anime episodes by specified rules.

## Running Process

1. Run specified **source collectors** to gather source files;
2. Deduplicate then sort collected source files alphabetically based on their
    absolute path;
3. For each source file, run specified **metadata annotators**;
4. Run specified **checkers** on the list of annotated files;
    1. Add violation (if any) to related files' metadata;
    2. If any checker finds a violation with the **abort** action, stop current
        process and exit with error messages;
5. Execute the designated **executors** on the files that do not have any violations.
6. Execute the designated **fallback executors** on the files that have violation(s).

**Notes**:

* Currently, symlinks are not supported. Therefore, any collected symlinks will
    be excluded from the source file list. This filtering process is performed by
    an unconfigurable filter that runs after the execution of all designated
    collectors but before deduplicating and sorting the files.
* Annotators, Checkers and Executors are run with the order of their occurrence
    in the configuration file.
* Multiple collectors can be executed concurrently within a thread pool.
* Different annotators/executors run sequentially. But within a certain
    annotator or executor, multiple files can be processed concurrently within a
    thread pool.
* Multiple checkers can be executed concurrently within a thread pool. And
    violation annotation happens after all checkers have finished running. Multiple
    files can be annotated concurrently within a thread pool.
* Executors are executed sequentially, one file at a time. All executors for a
    specific file must be completed before starting the first executor for another
    file.


## Source Collector

### Glob Collector

Gather source files by running a list of
[glob](https://docs.rs/glob/latest/glob/) patterns within the source directory.

Settings:

* `source`: `string` (Required)
    The source directory under which the patterns should be run. E.g.:
    - `/path/to/file` 
    - (windows) `d:\files`.
* `patterns`: `string[]` (Required)
    An array of glob patterns
    - `[ "**/*.mp4", "**/*.mkv" ]` 

## Metadata Annotator

### Fuzzy Search

Use [fuzzy search](https://crates.io/crates/rust-fuzzy-search) with filename to locate and
assign a set of predefined metadata. It's handy for mapping an episode's filename
to the corresponding show's `canonical_name`.

Settings:

- `candidates`: `Map<string, Map<string, Any>>` (Required)
    map from keyword to desired metadata map.

E.g.:

```yaml
candidates:
  # filenames such as "[source][team]show-id-1.1080p.en.mp4" is likely to
  # match and get "A Good Show" as its canonical name
  show-id-1:
    canonical_name: A Good Show

  # if a show may have multiple names in filenames, YAML anchor can be utilized
  # to minimize duplication
  show-id-2: &show-id-2-metadata
    canonical_name: Another TV Show

  show-id-2-alias: *show-id-2-metadata
```

### Regex

Use Regex to extract information from `original_path` and assign the output to
target metadata items. Target metadata items are specified by named capture
groups.

Settings:

- `expressions`: `[Expression]` (Required):
    - `patterns`: `[string]` (Required)
        the Regex patterns to matched against. The first matched pattern will
        be used to extract info and following patterns will be ignored.
    - `type`: `string`, either `string` (default) or `int`;
        the desired data type of the extracted information, which can be useful
        for converting the data before assigning it to metadata. This is
        particularly handy when extracting episode or season numbers from file
        names.

E.g.:

```yaml
expressions:
  - type: int
    patterns:
      - '[Ss](?<season>\d{1,2})\D'
  - type: int
    patterns:
      - '[Ee](?<episode>\d{1,2})\D'
      - '(?<episode>\d{1,2})\D'
```

### Season Infer

Infer season number from already archived seasons.

Although in long term, this annotator should be capable of intelligently
identifying the season number and eliminating the reliance on regular
expressions, currently, it determines the season number by checking the
existence of folders using the following steps:

1. If the metadata contains a value for `season`, stop the process immediately.
1. List the folders in the `show_base_dir` that have a numerical suffix.
    1. If no valid folder is found, set the `season` to 1.
    1. Set the `season` to the largest numerical suffix among the found folders.

Settings:

* `show_base_dir`: `string` (Required)
    the folder in which the annotator should search for existing seasons.
    Template is supported.

E.g.:

```yaml
show_base_dir: "TV Shows/{{ canonical_name }}"
```

### Episode Infer

Infer episode number from already archived episode.

Similar to the Season Infer, this annotator determines the episode number by
checking the already archived files:

1. If the metadata contains a value for `episode`, stop the process immediately.
1. List the files in the `season_base_dir` that have a numerical suffix.
    1. If no valid file is found, set the `episode` to 1.
    1. Set the `episode` to the largest numerical suffix plus 1 among the found
       folders.

Settings:

* `episode_base_dir`: `string` (Required)
    the folder in which the annotator should search for existing episodes.
    Template is supported.

E.g.:

```yaml
episode_base_dir: "TV Shows/{{ canonical_name }}/Season {{ season | two_digits }}"
```

### Template

Utilize [TinyTemplate](https://github.com/bheisler/TinyTemplate) to generate a
rendering output and assign it to the designated metadata item. The target
metadata item must be a string value.

Settings:

- `templates`: `Map<string, string>` (Required)
    whose key is the name of designated metadata field and value is the
    template to render.

E.g.:

- to set `next_name` to something similar to `A Good Show S03E08.mp4`. 
    ```yaml
    templates:
      next_name: "{{ canonical_name }} S{{ season | two_digits }}E{{ episode | two_digits }}.{{ original_extension }}"
    ```
- to set `destination` to something similar to `TV Shows/A Good Show/Season 01/`. 
    ```yaml
    templates:
      destination: "TV Shows/{{ canonical_name }}/Season {{ season | two_digits }}/"
    ```

Supported formatters:
* `two_digits` - adds a prefix zero `"0"` when the int value to be formatted is less than 10.

## Checker

### No Duplicate Name

Read metadata:

* `destination`
* `nextName`

Check if renaming the file to `nextName` and moving the file to `destination`
will result in name conflict. It will cover both of following situations:

1. If a file with the name `nextName` already exists in the `destination` folder.
2. If another source file will be renamed and moved in the same manner.

## Executor

### Rename

Rename a file by `next_name`.

Read metadata:

* `next_name`

Modify metadata:

* `filename`
* `absolute_path`

### Move

Move a file to its destination.

Read metadata:

* `destination`
* `next_name`

Modify metadata:

* `filename`
* `absolute_path`


## Metadata List

### File Info

* `absolute_path`: `string` - the current absolute path of the file.
* `filename`: `string` - the current **file name** (including the base name and
    extension). It may differ from `original_name` if a `Rename` executor has been
    applied to the file.
* `original_absolute_path`: `string` - the absolute path of the file when it was collected.
* `original_basename`: `string` - the base name of the file when it was collected.
* `original_directory`: `string` - the relative **directory** from which the file was collected.
* `original_extension`: `string` - the extension of the file when it was collected.
* `original_name`: `string` - the name of the file when it was collected.
* `original_path`: `string` - the relative path of the file when it was collected.

### Archive Expectation

* `destination`: `string` - absolute path of the **directory** where the file should be moved.
* `next_name`: `string` - the desired **file name** (incl. base and extension
    names) to which the file should be renamed.

### Show and Episode Info

* `canonical_name`: `string` - the show's official/designated name.
* `season`: `int` - the season identification number.
* `episode`: `int` - the episode number within the season.

### Violation

* `violations`: `[Violation]`:
    * `rule`: `string` - the violated rule.
    * `action`: `string` - the action taken, either `"SKIP"` or `"ABORT"`.


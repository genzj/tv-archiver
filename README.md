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

## Source Collector

**Notes**:
* Currently, symlinks are not supported. Therefore, any collected symlinks will
    be excluded from the source file list. This filtering process is performed by
    an unconfigurable filter that runs after the execution of all designated
    collectors but before deduplicating and sorting the files.
* Multiple collectors can be executed concurrently within a thread pool.

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

**Notes**:
* Different annotators run sequentially. But within a certain annotator, multiple files can be processed concurrently within a thread pool.

### Fuzzy Match

### Regex

Use Regex to extract information from `original_path` and assign the output to
target metadata items. Target metadata items are specified by named capture
groups.

Settings:

- `expressions`: `[Expression]`:
    - `patterns`: `[string]` the Regex patterns to matched against. The first
        matched pattern will be used to extract info and following patterns will be
        ignored.
    - `type`: `string`, either `string` (default) or `int`; the desired data
        type of the extracted information, which can be useful for converting
        the data before assigning it to metadata. This is particularly handy when
        extracting episode or season numbers from file names.

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

### Episode Infer

### Template

Utilize [TinyTemplate](https://github.com/bheisler/TinyTemplate) to generate a
rendering output and assign it to the designated metadata item. The target
metadata item must be a string value.

Settings:

- `templates`: `Map<string, string>` whose key is the name of designated
    metadata field and value is the template to render.

E.g.:

- to set `next_name` to something similar to `A Good Show S03E08.mp4`. 
    ```yaml
    templates:
      next_name: "{{ canonical_name }} S{{ season | two_digits }}E{{ episode | two_digits }}.{{ original_extension }}"
    ```


Supported formatters:
* `two_digits` - adds a prefix zero `"0"` when the int value to be formatted is less than 10.

## Checker

**Notes**:
* Multiple checkers can be executed concurrently within a thread pool.
* Violation annotation happens after all checkers have finished running.
Multiple files can be annotated concurrently within a thread pool.

### No Duplicate Name

Read metadata:

* `destination`
* `nextName`

Check if renaming the file to `nextName` and moving the file to `destination`
will result in name conflict. It will cover both of following situations:

1. If a file with the name `nextName` already exists in the `destination` folder.
2. If another source file will be renamed and moved in the same manner.

## Executor

**Notes**:
* Different executors run sequentially. But within a certain executor, multiple
files can be processed concurrently within a thread pool.

### Rename

Read metadata:

* `next_name`

Write metadata:

* `name`
* `absolute_path`

### Move

Read metadata:

* `destination`
* `next_name`

Write metadata:

* `name`
* `absolute_path`


## Metadata List

### File Info

* `absolute_path`: `string` - the current absolute path of the file.
* `name`: `string` - the current **file name** (including the base name and
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


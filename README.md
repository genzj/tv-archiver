# TV Archiver

A tool that renames and archives TV/Anime episodes by specified rules.

## Running Process

1. Run specified **source collectors** to gather source files;
2. Sort collected source files alphabetically based on their absolute path;
3. For each source file, run specified **metadata annotators**;
4. Run specified **checkers** on the list of annotated files;
   1. Add violation (if any) to related files' metadata;
   2. If any checker finds a violation with the **abort** action, stop current process and exit with error messages;
5. Run specified **executors** on the file list.

## Source Collector

### Glob Collector

Gather source files by running a list of [glob](https://docs.rs/glob/latest/glob/) patterns within the source directory.

| Setting  | Type     | Description                                                  | Required? | Default Value | Example                                 |
| -------- | -------- | ------------------------------------------------------------ | --------- | ------------- | --------------------------------------- |
| source   | string   | The source directory under which the patterns should be run. | Yes       |               | `/path/to/file` , (windows) `d:\files`. |
| patterns | string[] | An array of glob patterns                                    | Yes       |               | `[ "**/*.mp4", "**/*.mkv" ]`            |

## Metadata Annotator

### Fuzzy Search

### Regex

### Season Infer

### Episode Infer

### Template



## Checker

### No Duplicate Name

## Executor

### Rename

### Move


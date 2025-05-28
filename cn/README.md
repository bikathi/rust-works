# cn, the **C**hange **N**ame Utility

cn is a tool written in Rust that carries the vision of GNU's [`mv`](https://www.gnu.org/software/coreutils/mv) core util to it's next
logical evolution: allowing you to rename files and folders in bulk.

## Sales Pitch
Whether you need to quickly change the name of one file or perform complex, pattern-based renames across many files and folders, 
this tool offers a secure and efficient solution. It leverages the power of regular expressions for advanced matching and 
transformation, providing a flexible alternative to manual renaming.

## Caveats and Warnings
Currently, cn only works on Linux (and possibly all UNIX-like systems), and has been tested on Ubuntu 25.04.

> cn carries no responsibility for the damage and or loss of data associated with its use. Use with care.

## Installation
### Compiling on Linux
1. Install Rust (for Linux) on your computer using [this link](https://rust-book.cs.brown.edu/ch01-01-installation.html)
2. Clone this repo by running `git clone https://github.com/bikathi/rust-works.git` 
3. `cd` to the cloned repository folder and run `cargo build --release`. This requires internet access and may take a while to finish.
4. After step three `(3)` above has finished, copy the binary in `/target/release/cn` to your `$HOME/.local/bin` usign the command `cp ./target/release/cn ~/.local/bin`. This adds the command cn to your path.
5. Refresh your path entries above by sourcing your `.bashrc` or `.zshrc` using, for example on `ZSH`, run `source $HOME/.zshrc`
6. Test installation by running `cn --help` which should respond with a summary of the utility's purpose and list the only two commands the utility provides: `single` and `bulk`.

## Subcommands
cn provides only two subcommands (which are ways to invoke it)

### 1.  `single` mode
- Invoked via `cn single <ARGUMENTS>`
- Allows you to rename and/or move a **single** file, which is the behaviour you get with  GNU's `mv` when you move a file with a new name.
- This mode does not support the renaming of folders directly, only files. This feature can however be extended in if needed, and may exist even though this documentation may not directly reflect that change.
--------------------------------------------------------------------------------------
### Arguments
`--file-name <FILE_PATH>` or `-f <FILE_PATH>`
  - **Type**: String
  - **Description**: The full path to the file you wish to rename

`--new-name <NEW_NAME>` or `-n <NEW_NAME>`
  - **Type**: String
  - **Description**: the new name of the file. Kindly include the file extension.
  
### Usage Examples
1. To rename the document `document.txt` to `my_document.txt` invoke `cn single -f document.txt -n my_document.txt`
2. To rename the file `old_image.png` as `archived_image.png` and move it to the `archive` folder, invoke `cn single -f old_image.png -n ./archive/archived_image.png`

### 2.  `bulk` mode
- The `bulk` subcommand is the powerful core of this utility, enabling complex, pattern-based renaming of multiple files and/or folders.
- The purpose of this mode is to be used to rename multiple files and/or folders within a specified base directory, and, optionally, it's subdirectories and a replacement string.
> You are highly advised to use this mode carefully as carelessly performing bulk changes can irreversibly make you lose files and/or damage your OS install.

### Arguments
`--directory <DIRECTORY_PATH>` or `-d <DIRECTORY_PATH>`
  - **Type**: String
  - **Description**: The base directory within which the renaming operation will be performed. All matching files/folders will be sought only within this directory.
  
`--pattern <REGEX_PATTERN>` or `-p <REGEX_PATTERN>`
  - **Type**: Quotes enclosed string i.e.  "<MATCHING_PATTERN_HERE>"
  - **Description**: The regular expression pattern used to match against the names of files and folders. This is where you define what parts of the names you want to identify. **Crucially, use parentheses () to create capture groups** which can be referenced in the --replacement string. Learn more about regex capture groups from [this Medium article](https://medium.com/@MynaviTechTusVietnam/regex-for-dummies-part-4-capturing-groups-and-backreferences-50c338a3b6f6)
  
`--replacement <REPLACEMENT_STRING>`
  - **Type**: String
  - **Description**: The string used to construct the new name. This string can contain literal text and backreferences (e.g., `$1`, `$2`) to the capture groups defined in the --pattern. **Always enclose this argument in single quotes ('<YOUR_REPLACEMENT_HERE>') on the command line to prevent shell variable expansion.**
  
`--include-folders`
  - **Type**: Boolean flag
  - **Description**: An optional flag which if present, the utilitu will apply the renaming logic to directories (folders) that match the `--pattern`, in addition to files. If omitted, only files will be considered for renaming.
  
`--recursive`
  - **Type**: Boolean flag
  - **Description**: An optional flag which if present, the tool will traverse into subdirectories (and their subdirectories) of the `--directory` and apply the renaming logic to files/folders found there. If omitted, only items directly within the base directory will be considered.
  
`--no-warnings`
  - **Type**: Boolean flag
  - **Description**: An optional flag which if present, the utility will suppress the y/N consent check asked before applying irreversible rename changes. **Use with caution, as this removes a vital safety check**.
  
### Understanding `--pattern` (Regular Expressions & Capture Groups)
The `--pattern` argument expects a valid regular expression. The power of this mode comes from capture groups, which are parts of your regex enclosed in parentheses `()`. When the pattern matches a filename, the text matched by each capture group is stored and can be reused.
In general:
  - `()`: Defines a capture group.
  - `$0`: Refers to the entire matched string (the whole portion of the original name that the regex pattern found).
  - `$1`: Refers to the content of the **first** capture group (the text matched by the first `()` from left to right).
  - `$2`, `$3` etc: Refer to the subsequent capture groups.

### Example Patterns:
1. `^photo-(\d{4})-(\d{2})-(\d{2})\.jpg$`: Matches `photo-YYYY-MM-DD.jpg`.
    - `$1` would capture `YYYY`
    - `$1` would capture `MM`
    - `$3` would capture `DD`

2. `(.*)\.txt$`: Matches any filename ending in `.txt`.
    - `$1` would capture the filename without the `.txt` extension.
  
3. `^(.*)(old)(.*)$`: Matches (in full i.e including the file extension) anything containing `old`.
    - `$1` captures everything before `old`
    - `$2` captures the word "old"
    - `$3` captures everything (including the file extension) after "old"

### Understanding `--replacement` (Backreferences)
The `--replacement` argument for `bulk` mode defines how the new file/folder name will be constructed.

It can contain either:

  1. **Literal Text**: Any characters not part of a backreference will be included as-is.
  2. **Backreferences**: `$1`, `$2` etc., will be replaced by the text captured by the corresponding group in the `--pattern`. `$0` inserts the entire matched string.
  
**Important**: Always enclose the `--replacement` string in single quotes **('<REPLACEMENT_STRING_HERE>')** on the command line to prevent your shell from interpreting `$` as a variable.

### Example Replacements (using patterns from above):
| Pattern | Replacement String | Old File Name | New File Name |
| ----------- | ----------- | --------------- | --------------- |
| `^photo-(\d{4})-(\d{2})-(\d{2})\.jpg$` | `'image_$1$2$3.png'` | photo-2023-10-26.jpg | image_20231026.png |
| `^(.*)\.txt$` | `'prefix_$1_suffix.log'` | my_document.txt | prefix_my_document_suffix.log |
| `^(.*)(old)(.*)$` | `'$1new$3'` | my_old_file.doc | my_new_file.doc |

### Collision Handling
cn includes logic to prevent overwriting existing files or creating duplicate names. If a proposed new name already exists in the target directory (and it's not the original file itself), the tool will automatically append a sequential counter (e.g., `_1`, `_2`) to the new name until a unique name is found.

This check is performed before the proposed changes are presented to the user for consent to apply.

For example, If `report.txt` is to be renamed to `document.txt`, but `document.txt` already exists, the new name might become `document_1.txt`. If `document_1.txt` also exists, it will try `document_2.txt`, and so on, until a new name is found.

### Usage Examples
Assume you have a directory structure like this
```
my_files/
  |-- document_2023.txt
  |-- image_001.jpg
  |-- old_report.pdf
  |-- sub_folder/
    |-- temp_file.log
  |-- project_docs/
    |-- notes_2024.txt
```

1. **Add a prefix to all .txt files in the current directory only**
    - **Command**: `cn bulk -d my_files -p '^(.*\.txt)$' --replacement 'ARCHIVE_$1`
    - **Action**: Renames `document_2023.txt` to `ARCHIVE_document_2023.txt`

2. **Change `.pdf` extension to `.docx` for all files containing "report" recursively:**
    - **Command**: `bulk -d my_files -p '^(.*report.*)\.pdf$' --replacement '$1.docx' --recursive`
    -- **Action**: Renames `old_report.pdf` to `old_report.docx`

3. **Rename a folder and its contents recursively, replacing "project" with "final"**
    - **Command**: `cn bulk -d my_files -p '(.*)project(.*)' --replacement '$1final$2' --recursive --include-folders`
    - **Action**: Renames `project_docs/` to `final_docs/`, and if any files or folders inside `project_docs/` matched 'project', they would also be renamed, alongside their subfolders, if a folder.
    
4. **For the brave, perform rename without consent (`--no-warnings`)**
    - **Command**: `cn bulk -d my_files -p '(.*)project(.*)' --replacement '$1final$2' --recursive --include-folders --no-warnings`
    - Will do the same things as number three (3) above, but without asking for consent like "Proceed to apply 1 changes? [y/N]..."
    
## Error Handling & Warnings
- **Invalid Regex Pattern**: If the `--pattern` provided is not a valid regular expression, the tool will exit with an error message indicating the regex compilation failure.
- **Non-existent Directory**: If the `--directory` path does not exist or is not a directory, the tool will exit with an error.
- **`--directory` is not a Directory**: If the `--directory` path does not point to a directory (e.g. points to a folder), the tool will exit with an error.
- **No Matching Entries**: If the traversal completes but no files or folders match the specified pattern, a message indicating "Found no matching entries to work on" will be displayed, and the tool will exit gracefully without performing any renames.
- **File System Errors**: During actual renaming, if an `io::Error` occurs (e.g., permissions denied, disk full, file already open), an error message will be printed for that specific rename, but the tool will attempt to continue with other renames.

## Contributing & Future Enhancements
cn is designed to be extensible. Potential future enhancements could include:
  - **Undo Functionality**: A mechanism to revert the last bulk rename operation.
  - **Logging**: Detailed logging of renaming operations to a file.
  - **More Advanced Renaming Options**:
    - Case conversion (e.g., convert all names to snake_case, camelCase).
    - Number padding (e.g., file1.txt -> file001.txt).
    - Date/time insertion into filenames.
  - **Exclusion Patterns**: The ability to specify patterns for files/folders to exclude from renaming.
  - **Symlink Handling**: More granular control over how symbolic links are treated.
  
Feel free to contribute by opening issues, suggesting features, or submitting pull requests!

*Happy Use!*
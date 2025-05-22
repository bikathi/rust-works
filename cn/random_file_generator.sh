#!/bin/bash

# --- Configuration ---
# Default number of files to generate if not specified by user
DEFAULT_NUM_FILES=20

# Array of common file extensions
EXTENSIONS=(
    "txt" "json" "java" "rs" "md" "csv" "xml" "html" "css" "js" "py" "go"
    "cpp" "h" "php" "rb" "yml" "toml" "log" "conf" "sh" "sql" "vue" "jsx"
    "ts" "tsx" "scss" "less" "svg" "pdf" "zip" "tar.gz" # Add some non-text files too
)

# Array of random words to build file names
WORDS=(
    "document" "report" "data" "image" "config" "script" "project" "module"
    "utility" "archive" "backup" "temp" "file" "test" "example" "asset"
    "source" "output" "input" "log" "status" "user" "admin" "system"
    "component" "service" "handler" "controller" "model" "view" "helper"
    "factory" "abstract" "interface" "builder" "adapter" "proxy" "strategy"
)

# --- Functions ---

# Function to generate a random string for file content
generate_random_content() {
    local length=$(( RANDOM % 500 + 50 )) # Content length between 50 and 550 characters
    head /dev/urandom | tr -dc A-Za-z0-9_ - | head -c "$length"
}

# Function to convert a string to different casing styles
to_camel_case() {
    echo "$1" | awk '{
        for (i=1; i<=NF; i++) {
            if (i==1) printf tolower(substr($i,1,1)) substr($i,2);
            else printf toupper(substr($i,1,1)) tolower(substr($i,2));
        }
        print ""
    }'
}

to_snake_case() {
    echo "$1" | tr '[:upper:]' '[:lower:]' | sed 's/ /_/g'
}

to_kebab_case() {
    echo "$1" | tr '[:upper:]' '[:lower:]' | sed 's/ /-/g'
}

# --- Main Script Logic ---

# Determine the number of files to generate
NUM_FILES=${1:-$DEFAULT_NUM_FILES} # Use user input or default

echo "Generating $NUM_FILES dummy files in the current directory..."

for i in $(seq 1 $NUM_FILES); do
    # Generate a base name with 1 to 3 random words
    num_words=$(( RANDOM % 3 + 1 ))
    base_name_parts=()
    for (( j=0; j<num_words; j++ )); do
        base_name_parts+=("${WORDS[$(( RANDOM % ${#WORDS[@]} ))]}")
    done
    base_name=$(echo "${base_name_parts[@]}")

    # Randomly choose a casing style
    case $(( RANDOM % 5 )) in
        0) # lowercase
            file_name_base=$(echo "$base_name" | tr '[:upper:]' '[:lower:]' | sed 's/ //g')
            ;;
        1) # UPPERCASE
            file_name_base=$(echo "$base_name" | tr '[:lower:]' '[:upper:]' | sed 's/ //g')
            ;;
        2) # camelCase
            file_name_base=$(to_camel_case "$base_name")
            ;;
        3) # snake_case
            file_name_base=$(to_snake_case "$base_name")
            ;;
        4) # kebab-case
            file_name_base=$(to_kebab_case "$base_name")
            ;;
    esac

    # Add a random number to ensure uniqueness and variety
    file_name_base="${file_name_base}_$(( RANDOM % 1000 ))"

    # Randomly choose an extension
    extension="${EXTENSIONS[$(( RANDOM % ${#EXTENSIONS[@]} ))]}"

    # Combine to form the full filename
    full_file_name="${file_name_base}.${extension}"

    # Create the file with some random content
    if [[ "$extension" == "zip" || "$extension" == "tar.gz" ]]; then
        # For archive extensions, create an empty file or a very small placeholder
        touch "$full_file_name"
    else
        # For other types, add some random content
        generate_random_content > "$full_file_name"
    fi

    echo "  Created: $full_file_name"
done

echo "Done generating $NUM_FILES files."
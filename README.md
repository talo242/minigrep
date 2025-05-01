# minigrep

`minigrep` is a command-line interface (CLI) tool written in Rust that allows users to search for text strings within `.txt` files. It is designed to be simple, fast, and easy to use.

## Features

- Search for specific text strings in `.txt` files.
- Case-sensitive and case-insensitive search modes.
- Outputs matching lines for quick reference.

## Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/your-username/minigrep.git
    ```
2. Navigate to the project directory:
    ```bash
    cd minigrep
    ```

## Usage

Run the `minigrep` the following syntax:

```bash
cargo run [pattern] [file] [options]

```

## Command-line options

### -n
Prefix each line that contains a match with a 1-based line number

### -i
Ignore case when searching for patterns

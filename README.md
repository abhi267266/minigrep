# Minigrep

Minigrep is a simple command-line tool written in Rust for searching text within files. It mimics the functionality of the `grep` command-line utility but is implemented as a learning exercise for Rust programming.

## Features

- Search for a specific query in a file.
- Display all lines containing the query.

## Usage

### Running the Application

1. Clone the repository to your local machine.
2. Build the project using `cargo build`.
3. Run the application using `cargo run <query> <file_path>`.

Example:
```bash
cargo run duct poem.txt
```

This will search for the word "duct" in the file `poem.txt` and print all matching lines.

### Example Output

Given the following content in `poem.txt`:
```
Rust:
safe, fast, productive.
Pick three.
Duct tape.
```

Running the command:
```bash
cargo run duct poem.txt
```

Will output:
```
safe, fast, productive.
```

## Project Structure

- `src/lib.rs`: Contains the core logic for parsing arguments, reading files, and searching for the query.
- `src/bin/main.rs`: The entry point of the application, responsible for handling command-line arguments and invoking the library functions.

## Testing

To run the tests, use the following command:
```bash
cargo test
```


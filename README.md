# Cut Tool

A Rust implementation of the Unix `cut` command line tool. This tool allows you to extract selected fields from each line of a file, supporting both tab-delimited (TSV) and comma-separated (CSV) files.

## Features

- Extract specific fields from delimited files
- Support for both TSV (tab-separated) and CSV (comma-separated) files
- Proper handling of CSV files with headers and quoted fields
- Custom delimiter support via the `-d` option
- Multiple field selection using comma-separated list
- Error handling for invalid inputs and file operations

## Installation

1. Make sure you have Rust and Cargo installed. If not, install them from [rustup.rs](https://rustup.rs/).

2. Clone this repository:
```bash
git clone <repository-url>
cd cut-tool
```

3. Build the project:
```bash
cargo build --release
```

The compiled binary will be available in `target/release/cut-tool`.

## Usage

The basic syntax is:
```bash
cut-tool -f FIELDS [-d DELIMITER] FILE
```

Where:
- `-f FIELDS`: Comma-separated list of fields to extract (1-based indexing)
- `-d DELIMITER`: Optional delimiter character (default is tab)
- `FILE`: Input file to process

### Examples

1. Extract fields 1, 3, and 5 from a TSV file:
```bash
cut-tool -f 1,3,5 sample.tsv
```

2. Extract fields 1, 2, and 3 from a CSV file:
```bash
cut-tool -f 1,2,3 -d "," data.csv
```

## Sample Files

The project includes two sample files for testing:

1. `sample.tsv`: A tab-separated file with numeric data
2. `fourchords.csv`: A comma-separated file containing song information

## Dependencies

- [clap](https://crates.io/crates/clap): Command line argument parsing
- [csv](https://crates.io/crates/csv): CSV file handling
- [anyhow](https://crates.io/crates/anyhow): Error handling

## Building from Source

1. Install Rust and Cargo
2. Clone the repository
3. Run:
```bash
cargo build --release
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

This project was created as part of the [Coding Challenges](https://codingchallenges.fyi/challenges/challenge-cut) series. 
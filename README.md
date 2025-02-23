# Starknet Entrypoint Finder

## Overview

Are you tired of debugging Starknet smart contracts errors by having to guess what hashes correspond to ? Are you looking for a way to easily find the entry point that caused the error? Then look no further!

Starknet Entrypoint Finder is an easy-to-use command-line tool designed to parse Starknet contract classes and provide a mapping hash to entry point name, and vice versa. It aims to help developers debug Starknet contracts errors.

## Features

- **List Entrypoints**: Display all entry points with their corresponding hashes.
- **Get Entrypoint Name**: Retrieve the name of a contract's entry point from its hash.
- **Get Entrypoint Hash**: Compute and display the hash for a given entry point name of a contract.

## Installation

Ensure you have Rust and Cargo installed on your system. Clone the repository and navigate to the project directory:

```bash
git clone <repository-url>
cd starknet_entrypoint_finder
```

Build the project using Cargo:

```bash
cargo build --release
```

## Usage

Run the tool using Cargo with the desired subcommand:

```bash
cargo run -- <subcommand> [args]
```

### Subcommands

- `list <contract-class.json>`: Lists all entry points with their hashes.
- `get-name <contract-class.json> <entry-point-hash>`: Retrieves the entry point name for a given hash.
- `get-hash <entry-point-name>`: Calculates the hash for a given (entry point) name.
- `help`: Displays this help message as well as the help message for each subcommand.

### Example

Once you've compiled your Cairo smart contract into the sierra class, you can pass it to the tool like this:

```bash
cargo run -- list <path/to/contract_class.json>
cargo run -- get-name <path/to/contract_class.json> <entry-point-hash>
cargo run -- get-hash <entry-point-name>
```

## Dependencies

- `clap`: Command-line argument parsing.
- `serde`: Serialization and deserialization of JSON data.
- `starknet`: Utilities for working with Starknet data types and hashes.

## License

This project is licensed under the MIT License.

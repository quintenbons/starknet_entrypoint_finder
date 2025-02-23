use std::fs::File;

use starknet::core::{types::Felt, utils::starknet_keccak};

use super::{Command, ParseResult, ParsingErrors, ContractClass};
use crate::cli::available_commands::AvailableCommandArguments;

pub struct GetEntrypointNameCommand;

impl Command for GetEntrypointNameCommand {
    fn parse(&self, command_args: &clap::ArgMatches) -> ParseResult {
        // Parse command args: file name & entrypoint hash
        let file_path = command_args.value_of(AvailableCommandArguments::File.as_str()).unwrap();
        let hash = command_args.value_of(AvailableCommandArguments::Hash.as_str()).unwrap();

        // Open json file
        let file_error_msg = ParsingErrors::FileNotOpenable(file_path).to_string();
        let file = File::open(file_path).expect(&file_error_msg);

        // Parse json file
        let class: ContractClass = serde_json::from_reader(file).expect(&ParsingErrors::JsonParsing(file_path).to_string());
        // Parse hash
        let hash = hash.parse::<Felt>().expect(&ParsingErrors::HashParsing(hash).to_string());

        ParseResult::GetEntryPointName { class, hash }
    }

    fn execute(&self, parse_result: ParseResult) {
        if let ParseResult::GetEntryPointName { class, hash } = parse_result {
            print_entrypoint_name(class, hash);
        } else {
            panic!("{}", ParsingErrors::UnexpectedParseResult.to_string());
        }
    }
}

/// Print entrypoint name corresponding to the given hash if it exists
fn print_entrypoint_name(class: ContractClass, hash: Felt) {
    let entrypoint_name = find_entrypoint_from_hash(class, hash);

    if let Some(name) = entrypoint_name {
        println!("Found entrypoint `{}` for:", name);
    } else {
        println!("No entrypoint found for:");
    }
    println!("hash dec: {}", hash);
    println!("hash hex: {}", hash.to_hex_string());
}

/// Find entrypoint name from hash if it exists
fn find_entrypoint_from_hash(class: ContractClass, hash: Felt) -> Option<String> {
    // For each entrypoint (constructor, interfaces' functions, standalone functions)
    // check the name's hash against the given one
    for entrypoint in class.abi {
        match entrypoint.item_type.as_str() {
            "function" | "constructor" => {
                let current_entrypoint_hash = starknet_keccak(entrypoint.name.as_bytes());
                if current_entrypoint_hash == hash {
                    return Some(entrypoint.name);
                }
            }
            "interface" => {
                for item in entrypoint.items {
                    // All items of an interface are functions
                    let current_entrypoint_hash = starknet_keccak(item.name.as_bytes());
                    if current_entrypoint_hash == hash {
                        return Some(item.name);
                    }
                }
            }
            _ => {}
        }
    }

    Option::None
}

use std::{collections::HashMap, fs::File};
use starknet::core::types::Felt;
use starknet::core::utils::starknet_keccak;

use crate::cli::available_commands::FILEARG;

use super::{Command, ParseResult, ParsingErrors, ContractClass};

pub struct MapEntryPointsCommand;

impl Command for MapEntryPointsCommand {
    fn parse(&self, command_args: &clap::ArgMatches) -> ParseResult {
        // Parse command arg: file name
        let file_path = command_args.value_of(FILEARG).unwrap();

        // Open json file
        let file_error_msg = ParsingErrors::FileNotOpenable(file_path).to_string();
        let file = File::open(file_path).expect(&file_error_msg);

        // Parse json file
        let class: ContractClass = serde_json::from_reader(file).expect(&ParsingErrors::JsonParsing(file_path).to_string());

        ParseResult::MapEntryPoints { class }
    }

    fn execute(&self, parse_result: ParseResult) {
        if let ParseResult::MapEntryPoints { class } = parse_result {
            map_entrypoints_with_hashes(class);
        } else {
            panic!("{}", ParsingErrors::UnexpectedParseResult.to_string());
        }
    }
}

/// Map and print all entrypoints name to their hash
fn map_entrypoints_with_hashes(class: ContractClass) {
    let mut entrypoints_name_to_hash = HashMap::<String, Felt>::new();

    // For each entrypoint (constructor, interfaces' functions, standalone functions)
    // add the pair (name, hash)
    for entrypoint in class.abi {
        match entrypoint.item_type.as_str() {
            "function" | "constructor" => {
                add_entrypoint(entrypoint.name, &mut entrypoints_name_to_hash);
            }
            "interface" => {
                for item in entrypoint.items {
                    // All items of an interface are functions
                    add_entrypoint(item.name, &mut entrypoints_name_to_hash);
                }
            }
            _ => {}
        }
    }

    print_entrypoints_name_to_hash(entrypoints_name_to_hash);
}

/// Hash and add entrypoint
fn add_entrypoint(entrypoint_name: String, map: &mut HashMap<String, Felt>) {
    let hash = starknet_keccak(entrypoint_name.as_bytes());
    map.insert(entrypoint_name, hash);
}

/// Print entrypoints name to hash
fn print_entrypoints_name_to_hash(map: HashMap<String, Felt>) {
    if map.is_empty() {
        println!("No entrypoints found in class");
        return;
    }

    for (name, hash) in map {
        println!("- entrypoint name: `{}`", name);
        println!("hash dec: {}", hash);
        println!("hash hex: {}", hash.to_hex_string());
        println!();
    }
}
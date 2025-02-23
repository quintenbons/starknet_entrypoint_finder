use serde::Deserialize;
use starknet::core::types::Felt;

pub mod get_entrypoint_hash;
pub mod get_entrypoint_name;
pub mod map_entrypoints_name_to_hash;

pub trait Command {
    /// Parse command arguments as well as contract class json file
    fn parse(&self, command_args: &clap::ArgMatches) -> ParseResult;
    /// Execute command with parsed arguments
    fn execute(&self, parse_result: ParseResult);
}

pub enum ParseResult {
    MapEntryPoints { class: ContractClass },
    GetEntryPointName { class: ContractClass, hash: Felt },
    GetEntryPointHash { name: String },
}

#[derive(Debug, Deserialize)]
pub struct ContractClass {
    abi: Vec<AbiItem>,
}

#[derive(Debug, Deserialize)]
struct AbiItem {
    #[serde(rename = "type")]
    item_type: String,
    name: String,
    #[serde(default)]
    items: Vec<AbiItem>, // For functions in interfaces
}


enum ParsingErrors<'a> {
    FileNotOpenable(&'a str),
    JsonParsing(&'a str),
    UnexpectedParseResult,
    HashParsing(&'a str),
}

impl<'a> ParsingErrors<'a> {
    fn to_string(&self) -> String {
        match self {
            Self::FileNotOpenable(file) => format!("Unable to open file {}", file),
            Self::JsonParsing(file) => format!("Unable to parse JSON file {}", file),
            Self::UnexpectedParseResult => format!("Unexpected ParseResult variant"),
            Self::HashParsing(hash) => format!("Cannot parse hash {} into Felt", hash),
        }
    }
}

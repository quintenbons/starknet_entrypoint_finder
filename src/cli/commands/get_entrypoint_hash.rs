use starknet::core::utils::starknet_keccak;

use super::{Command, ParseResult, ParsingErrors};
use crate::cli::available_commands::AvailableCommandArguments;

pub struct GetEntrypointHashCommand;

impl Command for GetEntrypointHashCommand {
    fn parse(&self, command_args: &clap::ArgMatches) -> ParseResult {
        // Parse command arg: entrypoint name
        let name = command_args.value_of(AvailableCommandArguments::Name.as_str()).unwrap().to_string();

        ParseResult::GetEntryPointHash { name }
    }

    fn execute(&self, parse_result: ParseResult) {
        if let ParseResult::GetEntryPointHash { name } = parse_result {
            let hash = starknet_keccak(name.as_bytes());

            println!("Entrypoint name: `{}` to:", name);
            println!("hash dec: {}", hash);
            println!("hash hex: {}", hash.to_hex_string());
        } else {
            panic!("{}", ParsingErrors::UnexpectedParseResult.to_string());
        }
    }
}

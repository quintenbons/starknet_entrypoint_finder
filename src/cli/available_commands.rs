use std::sync::LazyLock;

use clap::{App, AppSettings, Arg, SubCommand};

pub const FILEARG: &'static str = "file";
pub const HASHARG: &'static str = "hash";
pub const NAMEARG: &'static str = "hash";

pub const LIST: &'static str = "list";
pub const GET_NAME: &'static str = "get-name";
pub const GET_HASH: &'static str = "get-hash";

static ARG_FILE: LazyLock<Arg> = LazyLock::new(|| Arg::with_name("file")
                    .help("Path to the JSON file containing the contract data")
                    .required(true));

static ARG_HASH: LazyLock<Arg> = LazyLock::new(|| Arg::with_name("hash")
                    .help("The hash of the entrypoint")
                    .required(true));

static ARG_NAME: LazyLock<Arg> = LazyLock::new(|| Arg::with_name("name")
                    .help("The name of the entrypoint")
                    .required(true));

pub fn available_commands() -> App<'static> {
    App::new("Starknet Entrypoint Finder")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("1.0")
        .author("hudem1")
        .about("Quick and convenient tool for mapping entrypoint names to their hashes for Starknet contracts")
        .subcommand(
            SubCommand::with_name(LIST)
            .help("List all entrypoints with their hashes")
            .arg(&*ARG_FILE)
        )
        .subcommand(
            SubCommand::with_name(GET_NAME)
            .help("Get the name of an entrypoint")
            .arg(&*ARG_FILE)
            .arg(&*ARG_HASH)
        )
        .subcommand(
            SubCommand::with_name(GET_HASH)
            .help("Get the hash of an entrypoint")
            .arg(&*ARG_NAME)
        )
}
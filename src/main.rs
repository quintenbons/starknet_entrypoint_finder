mod cli;

use core::panic;

use clap::ArgMatches;
use cli::{
    available_commands::{available_commands, GET_HASH, GET_NAME, LIST},
    commands::{
        get_entrypoint_hash::GetEntrypointHashCommand,
        get_entrypoint_name::GetEntrypointNameCommand,
        map_entrypoints_name_to_hash::MapEntryPointsCommand, Command,
    },
};

fn handle_command<C: Command>(command: C, matches: &ArgMatches, substr: &str) {
    let cmd_exec_args = command.parse(matches.subcommand_matches(substr).unwrap());
    command.execute(cmd_exec_args);
}

fn main() {
    let app = available_commands();
    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some(LIST) => handle_command(MapEntryPointsCommand, &matches, LIST),
        Some(GET_NAME) => handle_command(GetEntrypointNameCommand, &matches, GET_NAME),
        Some(GET_HASH) => handle_command(GetEntrypointHashCommand, &matches, GET_HASH),
        _ => panic!("Unrecognized arguments. Use --help for more information"),
    };
}

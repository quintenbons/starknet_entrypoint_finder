mod cli;

use clap::ArgMatches;
use cli::{
    available_commands::{AvailableCommand, available_commands},
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
    let cmd_matches = available_commands();
    let subcommand = cmd_matches.subcommand_name().unwrap();
    let subcommand = AvailableCommand::from_str(subcommand).unwrap();

    match subcommand {
        AvailableCommand::List => handle_command(MapEntryPointsCommand, &cmd_matches, subcommand.as_str()),
        AvailableCommand::GetName => handle_command(GetEntrypointNameCommand, &cmd_matches, subcommand.as_str()),
        AvailableCommand::GetHash => handle_command(GetEntrypointHashCommand, &cmd_matches, subcommand.as_str()),
    };
}

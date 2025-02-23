mod cli;

use cli::{
    available_commands::{AvailableCommand, available_commands},
    commands::{
        get_entrypoint_hash::GetEntrypointHashCommand,
        get_entrypoint_name::GetEntrypointNameCommand,
        map_entrypoints_name_to_hash::MapEntryPointsCommand, Command,
    },
};

fn main() {
    // Define & check command format
    let cmd_matches = available_commands();

    // Both following calls should never fail here as the `clap` module took care of cmd checking previously
    let subcommand = cmd_matches.subcommand_name().unwrap();
    let subcommand = AvailableCommand::from_str(subcommand).unwrap();

    // Get command instance
    let command: Box<dyn Command> = match subcommand {
        AvailableCommand::List => Box::new(MapEntryPointsCommand),
        AvailableCommand::GetName => Box::new(GetEntrypointNameCommand),
        AvailableCommand::GetHash => Box::new(GetEntrypointHashCommand),
    };

    let cmd_exec_args = command.parse(cmd_matches.subcommand_matches(subcommand.as_str()).unwrap());
    command.execute(cmd_exec_args);
}

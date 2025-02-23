use clap::{App, Arg, ArgMatches, SubCommand};

pub enum AvailableCommand {
    List,
    GetName,
    GetHash,
}

const LIST: &'static str = "list";
const GET_NAME: &'static str = "get-name";
const GET_HASH: &'static str = "get-hash";

impl AvailableCommand {
    pub const fn as_str(&self) -> &'static str {
        match self {
            AvailableCommand::List => LIST,
            AvailableCommand::GetName => GET_NAME,
            AvailableCommand::GetHash => GET_HASH,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            LIST => Some(AvailableCommand::List),
            GET_NAME => Some(AvailableCommand::GetName),
            GET_HASH => Some(AvailableCommand::GetHash),
            _ => None,
        }
    }
}

pub enum AvailableCommandArguments {
    File,
    Hash,
    Name,
}

impl AvailableCommandArguments {
    pub const fn as_str(&self) -> &str {
        match self {
            AvailableCommandArguments::File => "file",
            AvailableCommandArguments::Hash => "hash",
            AvailableCommandArguments::Name => "name",
        }
    }
}

struct CommandArgument {
    name: &'static str,
    description: &'static str,
    required: bool,
}

const COMMAND_ARGUMENTS: [&CommandArgument; 3] = [
    &CommandArgument {
        name: AvailableCommandArguments::File.as_str(),
        description: "Path to the JSON file containing the contract data",
        required: true,
    },
    &CommandArgument {
        name: AvailableCommandArguments::Hash.as_str(),
        description: "The hash of the entrypoint",
        required: true,
    },
    &CommandArgument {
        name: AvailableCommandArguments::Name.as_str(),
        description: "The name of the entrypoint",
        required: true,
    },
];

struct CommandDetails {
    name: &'static str,
    description: &'static str,
    args: &'static [&'static CommandArgument],
}

const COMMANDS: [CommandDetails; 3] = [
    CommandDetails {
        name: AvailableCommand::List.as_str(),
        description: "List all entrypoints with their hashes",
        args: &[&COMMAND_ARGUMENTS[0]],
    },
    CommandDetails {
        name: AvailableCommand::GetName.as_str(),
        description: "Get the name of an entrypoint",
        args: &[&COMMAND_ARGUMENTS[0], &COMMAND_ARGUMENTS[1]],
    },
    CommandDetails {
        name: AvailableCommand::GetHash.as_str(),
        description: "Get the hash of an entrypoint",
        args: &[&COMMAND_ARGUMENTS[2]],
    },
];

pub fn available_commands() -> ArgMatches {
    // Create base CLI app
    let mut app =App::new("Starknet Entrypoint Finder")
        .version("1.0")
        .author("hudem1")
        .about("Quick and convenient tool for mapping entrypoint names to their hashes for Starknet contracts");

    for command in COMMANDS {
        // Create subcommand
        let mut subcommand =
            SubCommand::with_name(command.name)
            .about(command.description);

        // Add arguments to subcommand
        for argument in command.args {
            subcommand = subcommand.arg(
                Arg::with_name(argument.name)
                    .help(argument.description)
                    .required(argument.required)
            );
        }

        // Add subcommand
        app = app.subcommand(subcommand);
    }

    app.get_matches()
}
mod yank;
mod descriptors;
mod watch;

use clap::{ArgMatches, Command, Error};
use clap_complete::{generate, Generator, Shell};
use clap::error::ErrorKind;
use std::path::{Path, PathBuf};
use steamlocate::SteamDir;
use std::fs;
use std::ops::Deref;
use crate::statics::{ARCHEAN_STEAM_ID, COMMAND, DESCRIPTION, VERSION};

pub mod prelude {
    use super::*;
    pub fn app() -> Command {
        // Initialize the main command, `archbelt`
        Command::new(COMMAND)
            .version(VERSION)
            .about(DESCRIPTION)
            .subcommand(descriptors::yank_command())
            .subcommand(descriptors::watch_command())
            .subcommand(descriptors::complete_command())
    }

    pub fn match_commands(sub_command: &str, args: &ArgMatches) {
        match sub_command {
            "complete" => {
                generate_shell_completion(args);
            }
            "yank" => {
                yank::yank_xenon_code(args);
            }
            "watch" => {
                watch::watch_blueprints(args);
            }
            _ => {
                app().print_long_help().unwrap();
            }
        }
    }

    pub fn get_archean_path() -> Result<PathBuf, CommandError> {
        let steam_dir = SteamDir::locate().expect("could not find a steam library");
        let (archean, lib) = steam_dir
            .find_app(ARCHEAN_STEAM_ID)?
            .expect("🚨 Archean installed via Steam is required! 🚨");
        Ok(lib.resolve_app_dir(&archean))
    }

    #[derive(Debug, Clone)]
    pub enum WatchState {
        Requested,
        Watching,
        Disabled
    }

    pub use super::yank::yank_from_config;
    pub use super::yank::YankConfig;
}

#[derive(Debug, Clone)]
pub struct CommandError;

impl From<CommandError> for Error {
    fn from(_: CommandError) -> Self {
        Error::new(ErrorKind::ValueValidation)
    }
}

impl From<&str> for CommandError {
    fn from(_: &str) -> Self {
        CommandError
    }
}

impl From<steamlocate::Error> for CommandError {
    fn from(_: steamlocate::Error) -> Self {
        CommandError
    }

}

fn generate_shell_completion(args: &ArgMatches) {
    if let Some(shell) = args.get_one::<Shell>("target") {
        generate_completions(*shell, &mut prelude::app());
    } else {
        println!("🚨 Please provide a valid shell target! See {} complete --help for more information! 🚨", COMMAND);
    }
}

// MARK: - Helper functions
fn get_blueprint_object(path: PathBuf) -> Result<String, CommandError> {
    let bp_string = fs::read_to_string(path);
    match bp_string {
        Ok(bp_string) => Ok(bp_string),
        Err(_) => Err(CommandError),
    }
}

fn generate_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, COMMAND, &mut std::io::stdout());
}

fn get_blueprint_path(bp: String) -> Result<PathBuf, CommandError> {
    // Defaults to {STEAM_APPS}\Archean\Archean-data\client\blueprints
    Ok(Path::new(&prelude::get_archean_path()?)
        .join("Archean-data")
        .join("client")
        .join("blueprints")
        .join(bp))
}

fn extract_filename(for_id: String, matches: &ArgMatches) -> Result<PathBuf, CommandError> {
    let file_name = match matches.get_many::<String>(for_id.as_str()) {
        Some(bp_arg) => {
            let mut bp_name: Vec<String> = vec![];
            bp_arg.into_iter().for_each(|bp| {
                let name = bp.deref().to_string();
                bp_name.push(name);
            });
            let bp_name = bp_name.join(" ");
            let blueprint = format!("{}.json", bp_name);
            get_blueprint_path(blueprint)
        }
        None => {
            return Err(CommandError);
        }
    };
    Ok(file_name?)
}
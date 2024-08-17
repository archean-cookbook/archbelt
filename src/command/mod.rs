mod yank;
mod descriptors;
mod watch;
mod package;

use clap::{ArgMatches, Command, Error, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use clap::error::ErrorKind;
use std::path::{Path, PathBuf};
use steamlocate::SteamDir;
use std::fs;
use std::ops::Deref;
use crate::command::yank::YankConfig;
use crate::statics::{ARCHEAN_STEAM_ID, COMMAND, DESCRIPTION, VERSION};

#[derive(Parser)]
#[command(name = "archbelt", version = "0.4.0", about = "A tool to work with Archean blueprints and XenonCode modules")]
pub struct Archbelt {
    #[command(subcommand)]
    pub command: Commands,
    // #[subcommand]
    // watch: Watch,
    // #[subcommand]
    // package: Package,
    // #[subcommand]
    // info: Info,
    // #[subcommand]
    // complete: Complete
}

#[derive(Subcommand)]
pub enum Commands {
    /// Yank code files from a blueprint
    Yank {
        /// Name of the blueprint without .json
        #[arg(num_args(0..), required = true, trailing_var_arg = true)]
        blueprint: Vec<String>,
        /// Yank blueprint files to folder named after blueprint
        #[arg(long, short)]
        folder: bool,
        /// Watch for changes to the blueprint; yanks files on change; assumes -f
        #[arg(long, short)]
        watch: bool,
        /// Do not collate the files by component
        #[arg(long, short)]
        no_collate: bool
    }
}

pub mod prelude {
    pub use crate::command::yank::yank_xenon_code_from_config;
    use super::*;

    pub fn parser_app() -> Archbelt {
        Archbelt::parse()
    }

    pub fn parser_match() -> Result<(), Error> {
        let app = parser_app();
        return match &app.command {
            Commands::Yank { blueprint, folder, watch, no_collate } => {
                let yank_config = YankConfig::from(app.command);
                yank_xenon_code_from_config(yank_config)
            },
            _ => {
                Err(Error::new(ErrorKind::InvalidSubcommand))
            }
        }
    }

    pub fn app() -> Command {
        // Initialize the main command, `archbelt`
        Command::new(COMMAND)
            .version(VERSION)
            .about(DESCRIPTION)
            .subcommand(descriptors::yank_command())
            .subcommand(descriptors::watch_command())
            .subcommand(descriptors::package_command())
            .subcommand(descriptors::info_command())
            .subcommand(descriptors::complete_command())
    }

    pub fn match_commands(sub_command: &str, args: &ArgMatches) {
        match sub_command {
            "complete" => {
                generate_shell_completion(args);
            }
            "info" => {
                show_info(args);
            }
            "package" => {
                package::package_from_blueprint(args);
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
        get_steam_default_path()
    }

    fn get_steam_default_path() -> Result<PathBuf, CommandError> {
        let steam_dir = SteamDir::locate()?;
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

fn show_info(args: &ArgMatches) {
    let archean_path = prelude::get_archean_path().expect("Could not get Archean path; is it installed?");
    println!("Archean path: {:?} (exists: {})", archean_path, archean_path.exists());

    let blueprints_path = get_blueprints_path(args);
    match blueprints_path {
        Ok(path) => {
            println!("Blueprints path: {:?} (exists: {})", path, path.exists());
        }
        Err(_) => {
            eprintln!("Could not get blueprints path; is Archean installed? did you provide an invalid path to --blueprint-path?");
        }
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

fn get_simple_blueprints_path() -> Result<PathBuf, CommandError> {
    let archean_path = prelude::get_archean_path()?;
    Ok(archean_path.join("Archean-data").join("client").join("blueprints"))
}

fn get_blueprints_path(args: &ArgMatches) -> Result<PathBuf, CommandError> {
    let blueprint_path_from_args = args.get_one::<String>("blueprint-path");

    match blueprint_path_from_args {
        Some(path) => {
            let path_buf = PathBuf::from(path);
            if path_buf.exists() {
                Ok(path_buf.clone())
            } else {
                Err(CommandError)
            }
        }
        None => {
            // Defaults to {STEAM_APPS}\Archean\Archean-data\client\blueprints
            Ok(Path::new(&prelude::get_archean_path()?)
                .join("Archean-data")
                .join("client")
                .join("blueprints"))
        }
    }
}

pub fn get_blueprint_path(bp: String, args: &ArgMatches) -> Result<PathBuf, CommandError> {
    let blueprints_path =  get_blueprints_path(args);
    match blueprints_path {
        Ok(path) => {
            let blueprint_path = path.join(bp);
            if blueprint_path.exists() {
                Ok(blueprint_path)
            } else {
                eprintln!("Blueprint path not found: {:?}", blueprint_path);
                Err(CommandError)
            }
        }
        Err(_) => {
            match &prelude::get_archean_path() {
                Ok(path) => {
                    let blueprint_path = path.join("Archean-data")
                        .join("client")
                        .join("blueprints")
                        .join(bp);
                    if blueprint_path.exists() {
                        Ok(blueprint_path)
                    } else {
                        Err(CommandError)
                    }
                }
                _ => {
                    Err(CommandError)
                }
            }
        }
    }
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
            get_blueprint_path(blueprint, matches)
        }
        None => {
            return Err(CommandError);
        }
    };
    Ok(file_name?)
}
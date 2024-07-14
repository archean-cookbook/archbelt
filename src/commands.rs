use std::fs;
use std::ops::Deref;
use crate::archean::json::*;
use crate::descriptors;
use crate::statics::{COMMAND, DESCRIPTION, VERSION};

use std::path::{Path, PathBuf};
use clap::{ArgMatches, Command, Error, FromArgMatches};
use clap::error::ErrorKind;
use clap_complete::{generate, Generator, Shell};
use steamlocate::SteamDir;

struct YankConfig {
    file_name: PathBuf,
    folder: bool,
    watch: bool,
}

#[derive(Debug, Clone)]
pub struct CommandError;

impl From<CommandError> for Error {
    fn from(_: CommandError) -> Self {
        Error::new(ErrorKind::ValueValidation)
    }
}

impl FromArgMatches for YankConfig {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let file_name = extract_filename("BLUEPRINT".to_string(), matches)?;
        let folder_switch = matches.get_one::<bool>("folder").unwrap_or(&false);
        let watch_switch = matches.get_one::<bool>("watch").unwrap_or(&false);

        Ok(YankConfig {
            file_name,
            folder: *folder_switch,
            watch: *watch_switch,
        })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        let file_name = extract_filename("BLUEPRINT".to_string(), matches)?;
        let folder_switch = matches.get_one::<bool>("folder").unwrap_or(&false);
        let watch_switch = matches.get_one::<bool>("watch").unwrap_or(&false);

        self.file_name = file_name;
        self.folder = *folder_switch;
        self.watch = *watch_switch;

        Ok(())
    }
}

pub fn app() -> Command {
    // Initialize the main command, `archbelt`
    Command::new(COMMAND)
        .version(VERSION)
        .about(DESCRIPTION)
        .subcommand(descriptors::yank_command())
        .subcommand(descriptors::complete_command())
}

pub fn match_commands(sub_command: &str, args: &ArgMatches) {
    match sub_command {
        "complete" => {
            generate_shell_completion(args);
        }
        "yank" => {
            yank_xenon_code(args);
        }
        _ => {
            app().print_long_help().unwrap();
        }
    }
}

pub fn yank_xenon_code(args: &ArgMatches) {
    let config = YankConfig::from_arg_matches(args);
    match config {
        Ok(config) => {
            yank_from_config(config);
        }
        Err(_) => {
            println!("🚨 Blueprint not found! 🚨");
            std::process::exit(1);
        }
    }
}

pub fn generate_shell_completion(args: &ArgMatches) {
    if let Some(shell) = args.get_one::<Shell>("target") {
        generate_completions(*shell, &mut app());
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

fn yank_from_config(config: YankConfig) {
    // TODO: handle watch and folder options
    let blueprint = get_blueprint_object(config.file_name.clone());
    match blueprint {
        Ok(bp) => {
            // TODO: unwrap
            let blueprint: Blueprint = serde_json::from_str(bp.as_str()).unwrap();
            let mut files: Vec<XcFileMeta> = vec![];

            blueprint.components_with_hdd().iter().for_each(|c| {
                // TODO: unwrap
                c.xc_files().iter().for_each(|f| {
                    files.push(f.clone());
                });
            });

            if files.is_empty() {
                println!("🚨 No files found! 🚨");
                std::process::exit(0);
            }

            // TODO: ugh
            let folder_name: String = config.file_name.clone().file_stem().unwrap().to_str().unwrap().to_string();
            let current_dir = std::env::current_dir().unwrap();

            if config.folder {
                fs::create_dir_all(folder_name.clone()).expect("Unable to create folder");
                std::env::set_current_dir(folder_name.clone()).expect("Unable to set current directory");
            }

            // For each XcFile, create the file on disk and write the plain_code to it
            files.iter().for_each(|f| {
                // TODO: consider this...
                // let file_name = if config.folder {
                //     format!("{}/{}", f.component(), f.file_name())
                // } else {
                //     f.file_name().to_string()
                // };

                fs::write(f.file_name(), f.file_content()).expect("Unable to write file");
            });

            // pop back to current_dir
            std::env::set_current_dir(current_dir).expect("Unable to set current directory");
        }
        Err(_) => {
            println!("🚨 Blueprint not found! 🚨");
            std::process::exit(1);
        }
    }
}

fn generate_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, COMMAND, &mut std::io::stdout());
}

fn get_blueprint_path(bp: String) -> PathBuf {
    // Defaults to {STEAM_APPS}\Archean\Archean-data\client\blueprints
    return Path::new(&get_archean_path())
        .join("Archean-data")
        .join("client")
        .join("blueprints")
        .join(bp);
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
    Ok(file_name)
}

pub fn get_archean_path() -> PathBuf {
    // TODO: unwrap
    let mut steam_dirs = SteamDir::locate().unwrap();
    let archean = steam_dirs.app(&2941660);
    match archean {
        None => {
            show_requirements_and_exit();
        }
        Some(archean) => {
            archean.path.clone()
        }
    }
}

pub fn show_requirements_and_exit() -> ! {
    println!("🚨 Archean installed via Steam is required! 🚨");
    std::process::exit(1);
}
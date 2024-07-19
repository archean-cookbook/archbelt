mod yank;
// TODO: move yank functions to yank module

use clap::{ArgMatches, Command, Error, FromArgMatches};
use clap_complete::{generate, Generator, Shell};
use clap::error::ErrorKind;
use std::path::{Path, PathBuf};
use steamlocate::SteamDir;
use std::fs;
use std::ops::Deref;
use crate::descriptors;
use crate::archean::json::{Blueprint, XcFileMeta};
use crate::statics::{COMMAND, DESCRIPTION, VERSION};

pub mod prelude {
    pub use super::app;
    pub use super::match_commands;
}

struct YankConfig {
    file_name: PathBuf,
    folder: bool,
    watch: bool,
    disable_collate: bool
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
        let collate_switch = matches.get_one::<bool>("no-collate").unwrap_or(&false);

        Ok(YankConfig {
            file_name,
            folder: *folder_switch,
            watch: *watch_switch,
            disable_collate: *collate_switch
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

fn yank_xenon_code(args: &ArgMatches) {
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

fn generate_shell_completion(args: &ArgMatches) {
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
    // TODO: handle watch
    let blueprint = get_blueprint_object(config.file_name.clone());
    match blueprint {
        Ok(bp) => {
            let blueprint: Option<Blueprint> = serde_json::from_str(bp.as_str()).ok();
            match blueprint {
                Some(blueprint) => {
                    let mut files: Vec<XcFileMeta> = vec![];

                    blueprint.components_with_hdd().iter().for_each(|c| {
                        c.xc_files().iter().for_each(|f| {
                            files.push(f.clone());
                        });
                    });

                    if files.is_empty() {
                        println!("🚨 No files found! 🚨");
                        std::process::exit(0);
                    }

                    let folder_name: String = config.file_name.clone()
                        .file_stem()
                        .expect("could not convert blueprint name to OS string")
                        .to_str()
                        .expect("could not convert blueprint name to native string")
                        .to_string();
                    let current_dir = std::env::current_dir().expect("could not detect current path");

                    if config.folder {
                        fs::create_dir_all(folder_name.clone()).expect("Unable to create folder");
                        std::env::set_current_dir(folder_name.clone()).expect("Unable to set current directory");
                    }

                    // For each XcFile, create the file on disk and write the plain_code to it
                    files.iter().for_each(|f| {
                        let file_name: String;
                        if !config.disable_collate { // chose this name to make this logic easier to read
                            file_name = format!("{}/{}", f.component(), f.file_name());
                        } else {
                            file_name = f.file_name().to_string();
                        }
                        // create the folder if it doesn't exist
                        let folder = Path::new(&file_name).parent().expect(format!("could not create parent folder for {}", file_name.to_string()).as_str());
                        fs::create_dir_all(folder).expect("Unable to create folder");
                        // save the file
                        fs::write(file_name.clone(), f.file_content()).expect(format!("Unable to write file {}", file_name).as_str());
                    });

                    // pop back to current_dir
                    std::env::set_current_dir(current_dir).expect("Unable to set current directory");
                }
                _ => {
                    println!("🚨 Unable to parse blueprint! Please open an issue at https://github.com/archean-cookbook/archbelt/issues and attach your blueprint .json. 🚨");
                    std::process::exit(0);
                }
            }
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

fn get_archean_path() -> PathBuf {
    let mut steam_dirs = SteamDir::locate().expect("could not find a steam library");
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

fn show_requirements_and_exit() -> ! {
    println!("🚨 Archean installed via Steam is required! 🚨");
    std::process::exit(1);
}
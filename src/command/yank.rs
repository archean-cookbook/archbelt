use std::path::{Path, PathBuf};
use clap::{ArgMatches, Error, FromArgMatches};
use std::fs;
use std::io::Write;
use crate::archean::json::{Blueprint, XcFileMeta};
use crate::command;
use crate::command::prelude::WatchState;
use crate::command::watch::watch_event;

use clap::{Command, Parser, Subcommand};
use clap::error::ErrorKind;
use crate::command::{Commands, CommandError, get_simple_blueprints_path};

// #[derive(Parser)]
// #[command(name = "yank", about = "Yank code files from a blueprint")]
// pub struct Yank {
//     #[arg(arg, long, short, about = "name of the blueprint without .json")]
//     pub blueprint: String,
//     #[clap(long, short, about = "yank blueprint files to folder named after blueprint")]
//     pub folder: bool,
//     #[clap(long, short, about = "watch for changes to the blueprint; yanks files on change; assumes -f")]
//     pub watch: bool,
//     #[clap(long, short, about = "do not collate the files by component")]
//     pub no_collate: bool
// }

pub struct YankConfig {
    pub(crate) file_name: PathBuf,
    pub(crate) folder: bool,
    pub(crate) watch: WatchState,
    pub(crate) disable_collate: bool
}

impl From<Commands> for YankConfig {
    fn from(cmd: Commands) -> Self {
        match cmd {
            Commands::Yank { blueprint, folder, watch, no_collate } => {
                YankConfig {
                    file_name: new_extract_filename(blueprint).expect("Could not create file name from blueprint"),
                    folder,
                    watch: if watch { WatchState::Requested } else { WatchState::Disabled },
                    disable_collate: no_collate
                }
            }
            _ => {
                panic!("Invalid command type passed to YankConfig::from");
            }
        }
    }
}

fn new_extract_filename(blueprint: Vec<String>) -> Result<PathBuf, CommandError> {
    let bp_name = blueprint.join(" ");
    let bp_path = get_simple_blueprints_path()?;
    Ok(bp_path.join(format!("{}.json", bp_name)))
}

impl FromArgMatches for YankConfig {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let file_name = command::extract_filename("BLUEPRINT".to_string(), matches)?;
        let folder_switch = matches.get_one::<bool>("folder").unwrap_or(&false);
        let watch_switch = matches.get_one::<bool>("watch").unwrap_or(&false);
        let collate_switch = matches.get_one::<bool>("no-collate").unwrap_or(&false);

        let watch_state = if *watch_switch {
            WatchState::Requested
        } else {
            WatchState::Disabled
        };

        Ok(YankConfig {
            file_name,
            folder: *folder_switch,
            watch: watch_state,
            disable_collate: *collate_switch
        })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        let file_name = command::extract_filename("BLUEPRINT".to_string(), matches)?;
        let folder_switch = matches.get_one::<bool>("folder").unwrap_or(&false);
        let watch_switch = matches.get_one::<bool>("watch").unwrap_or(&false);

        self.file_name = file_name;
        self.folder = *folder_switch;
        match self.watch {
            _ => {
                if !watch_switch {
                    self.watch = WatchState::Disabled;
                }
            }
        }

        Ok(())
    }
}

pub fn yank_xenon_code_from_config(config: YankConfig) -> Result<(), Error> {
    return match config.watch {
        WatchState::Requested | WatchState::Watching => {
            watch_event(config.file_name.clone()).expect("Could not watch blueprint path");
            Err(Error::new(ErrorKind::InvalidValue))
        }
        _ => {
            yank_from_config_with_result(config)
        }
    }
}

pub fn yank_xenon_code(args: &ArgMatches) {
    let config = YankConfig::from_arg_matches(args).expect("Could not create YankConfig from args");
    yank_xenon_code_from_config(config).expect("Could not yank xenon code");
}

pub fn yank_from_config_with_result(config: YankConfig) -> Result<(), Error> {
    yank_from_config(config);
    Ok(()) // TODO: error handling
}

pub fn yank_from_config(config: YankConfig) {
    // TODO: handle watch loop
    let blueprint = command::get_blueprint_object(config.file_name.clone());
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
                        match config.watch {
                            WatchState::Requested | WatchState::Watching => {
                                eprintln!("Mo files found in event, skipping..");
                                return; // exit the function if we are watching
                            }
                            _ => {
                                eprintln!("🚨 No files found! 🚨");
                                std::process::exit(0);
                            }
                        }
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
                        let mut fh = fs::OpenOptions::new()
                            .create(true)
                            .write(true)
                            .truncate(true)
                            .open(file_name.clone())
                            .expect(format!("Unable to create file {}", file_name.clone()).as_str());

                        fh.write(f.file_content().as_bytes()).expect(format!("Unable to write to file: {}", file_name.clone()).as_str());
                    });

                    // pop back to current_dir
                    std::env::set_current_dir(current_dir).expect("Unable to set current directory");
                }
                _ => {
                    eprintln!("🚨 Unable to parse blueprint! Please open an issue at https://github.com/archean-cookbook/archbelt/issues and attach your blueprint .json. 🚨");
                    std::process::exit(0);
                }
            }
        }
        Err(_) => {
            eprintln!("🚨 Blueprint not found! 🚨");
            std::process::exit(1);
        }
    }
}
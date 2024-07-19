use std::path::{Path, PathBuf};
use clap::{ArgMatches, Error, FromArgMatches};
use std::fs;
use crate::archean::json::{Blueprint, XcFileMeta};
use crate::command;

pub(crate) struct YankConfig {
    file_name: PathBuf,
    folder: bool,
    watch: bool,
    disable_collate: bool
}

impl FromArgMatches for YankConfig {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let file_name = command::extract_filename("BLUEPRINT".to_string(), matches)?;
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
        let file_name = command::extract_filename("BLUEPRINT".to_string(), matches)?;
        let folder_switch = matches.get_one::<bool>("folder").unwrap_or(&false);
        let watch_switch = matches.get_one::<bool>("watch").unwrap_or(&false);

        self.file_name = file_name;
        self.folder = *folder_switch;
        self.watch = *watch_switch;

        Ok(())
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

fn yank_from_config(config: YankConfig) {
    // TODO: handle watch
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
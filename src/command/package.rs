use std::path::PathBuf;
use clap::{ArgMatches, Error, FromArgMatches};
use serde_derive::{Deserialize, Serialize};
use crate::archean::json::Blueprint;
use crate::command;
use crate::command::prelude::{WatchState, yank_from_config, YankConfig};

#[derive(Clone)]
pub struct PackageConfig {
    pub(crate) file_name: PathBuf,
    pub(crate) watch: WatchState
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintMetadata {
    pub(crate) name: Option<String>,
    pub(crate) version: String,
    pub(crate) description: String,
    pub(crate) author: String,
    pub(crate) license: Option<String>,
    pub(crate) keywords: Vec<String>,
    pub(crate) dependencies: Vec<String>,
}

impl BlueprintMetadata {
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
}

impl From<Blueprint> for BlueprintMetadata {
    fn from(value: Blueprint) -> Self {
        BlueprintMetadata {
            name: None,
            version: value.version.to_string(),
            description: format!("mass: {}", value.mass),
            author: value.author,
            license: None, // TODO: implement license output in .json file
            keywords: vec![],
            dependencies: vec![]
        }
    }
}

impl From<PackageConfig> for YankConfig {
    fn from(config: PackageConfig) -> Self {
        YankConfig {
            file_name: config.file_name,
            folder: true,
            watch: config.watch,
            disable_collate: false
        }
    }
}

impl FromArgMatches for PackageConfig {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let file_name = command::extract_filename("BLUEPRINT".to_string(), matches)?;
        let watch_switch = matches.get_one::<bool>("watch").unwrap_or(&false);

        let watch_state = if *watch_switch {
            WatchState::Requested
        } else {
            WatchState::Disabled
        };

        Ok(PackageConfig {
            file_name,
            watch: watch_state
        })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        let file_name = command::extract_filename("BLUEPRINT".to_string(), matches)?;
        let watch_switch = matches.get_one::<bool>("watch").unwrap_or(&false);

        self.file_name = file_name;
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

fn get_blueprint_metadata(config: PackageConfig) -> BlueprintMetadata {
    let blueprint_name = config.file_name.file_stem().unwrap().to_string_lossy().to_string();
    let blueprint = command::get_blueprint_object(config.file_name.clone());

    match blueprint {
        Ok(bp) => {
            let res: serde_json::error::Result<Blueprint> = serde_json::from_str(bp.as_str());
            match res {
                Ok(blueprint) => {
                    let mut metadata = BlueprintMetadata::from(blueprint);
                    metadata.set_name(blueprint_name);
                    return metadata;
                }
                _ => {
                    eprintln!("🚨 Blueprint could not be parsed! 🚨");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("🚨 Blueprint could not be found or opened! 🚨");
            std::process::exit(1);
        }
    }
}

pub fn package_from_blueprint(args: &ArgMatches) {
    let config = PackageConfig::from_arg_matches(args);
    match config {
        Ok(config) => {
            // copy blueprint file to destination folder
            let copied = std::fs::copy(config.file_name.clone(), config.file_name.file_name().unwrap());
            match copied {
                Err(e) => {
                    eprintln!("Error copying blueprint file: {:?}", e);
                    std::process::exit(1);
                }
                _ => {}
            }
            // fetch metadata from file
            let metadata = get_blueprint_metadata(config.clone());
            let metadata_json = serde_json::to_string_pretty(&metadata).unwrap();
            let metadata_saved = std::fs::write("archbelt.json", metadata_json);
            match metadata_saved {
                Err(e) => {
                    eprintln!("Error saving metadata: {:?}", e);
                    std::process::exit(1);
                }
                _ => {}
            }
            yank_from_config(config.into());
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
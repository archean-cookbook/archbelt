use std::path::PathBuf;
use clap::{ArgMatches, Error, FromArgMatches};
use crate::command;
use crate::command::prelude::{WatchState, yank_from_config, YankConfig};

pub struct PackageConfig {
    pub(crate) file_name: PathBuf
}

impl From<PackageConfig> for YankConfig {
    fn from(config: PackageConfig) -> Self {
        YankConfig {
            file_name: config.file_name,
            folder: true,
            watch: WatchState::Disabled,
            disable_collate: false
        }
    }
}

impl FromArgMatches for PackageConfig {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let file_name = command::extract_filename("BLUEPRINT".to_string(), matches)?;

        Ok(PackageConfig {
            file_name
        })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        let file_name = command::extract_filename("BLUEPRINT".to_string(), matches)?;

        self.file_name = file_name;

        Ok(())
    }
}

pub fn package_from_blueprint(args: &ArgMatches) {
    let config = PackageConfig::from_arg_matches(args);
    match config {
        Ok(config) => {
            // Do something with the config
            let yank_config = YankConfig::from(config);
            yank_from_config(yank_config);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
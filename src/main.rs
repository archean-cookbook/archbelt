mod archean;
mod command;
mod statics;

use clap::Parser;
use command::prelude::*;
use crate::command::{Archbelt, Commands};

fn main() {
    // let matches = app().get_matches();
    // let sub_command_details = matches.subcommand();
    //
    // match sub_command_details {
    //     Some((sub_command, args)) => {
    //         match_commands(sub_command, args);
    //     }
    //     None => {
    //         app().print_long_help().unwrap();
    //     }
    // }
    let archbelt = Archbelt::parse();
    match &archbelt.command {
        Commands::Yank { blueprint, folder, watch, no_collate } => {
            let yank_config = YankConfig::from(archbelt.command);
            yank_xenon_code_from_config(yank_config).expect("could not yank xenon code");
        },
        _ => {
            // do nothing
        }
    }
}
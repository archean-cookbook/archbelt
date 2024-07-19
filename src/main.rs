mod archean;
mod command;
mod descriptors;
mod statics;

use command::prelude::*;
fn main() {
    let matches = app().get_matches();
    let sub_command_details = matches.subcommand();

    match sub_command_details {
        Some((sub_command, args)) => {
            match_commands(sub_command, args);
        }
        None => {
            app().print_long_help().unwrap();
        }
    }
}
use crate::archean::json::*;
use crate::descriptors;
use crate::statics::{COMMAND, DESCRIPTION, VERSION};

use std::path::{Path, PathBuf};
use clap::{ArgMatches, Command};
use clap_complete::{generate, Generator, Shell};
use steamlocate::SteamDir;

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
    if let Some(bp) = args.get_many::<String>("BLUEPRINT") {
        let blueprint = format!("{:?}.json", bp);
        let bp_path = get_blueprint_path(blueprint);
        // TODO: move to private func
        if !bp_path.exists() {
            println!("🚨 Blueprint not found! 🚨");
            std::process::exit(1);
        }
        let bp = std::fs::read_to_string(bp_path).unwrap();
        let blueprint: Blueprint = serde_json::from_str(&bp).unwrap();
        println!("{:?}", blueprint);
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

pub fn get_archean_path() -> PathBuf {
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
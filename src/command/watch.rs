use std::path::Path;
use clap::ArgMatches;
use notify::{Event, Watcher, RecursiveMode, Result};
use super::prelude::*;

pub fn watch_blueprints(args: &ArgMatches) {
    // TODO: look at args
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
            Ok(event) => {
                println!("{:?}", event);
            }
            _ => {
                println!("Error watching folder");
            }
        }
    }).unwrap();
    let archean_path = get_archean_path()
        .expect("Could not get Archean path")
        .join("Archean-data")
        .join("client")
        .join("blueprints");
    watcher.watch(
        Path::new(&archean_path),
        RecursiveMode::NonRecursive
    ).expect("Could not watch Archean path");
}
pub(crate) fn watch_event() -> impl FnMut(Result<Event>) {
    move |event| {
        println!("{:?}", event);
    }
}

pub(crate) fn watch_file_event(blueprint_name: String) -> impl FnMut(Result<Event>) {
    move |event| {
        println!("{:?}", event);
    }
}
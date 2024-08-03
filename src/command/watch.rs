use std::path::Path;
use std::time::Duration;
use clap::ArgMatches;
use notify::{Watcher, RecursiveMode, Result as NotifyResult};
use notify_debouncer_full::{DebouncedEvent, new_debouncer};
use crate::command;
use super::prelude::*;

pub fn watch_blueprints(matches: &ArgMatches) {
    let file_name = command::extract_filename("TARGET".to_string(), matches);
    match file_name {
        Ok(file_path) => {
            watch_event(&file_path).expect("Could not watch blueprint");
            return
        }
        Err(_) => {
            println!("Blueprint not found, or no blueprint specified, defaulting to watching all blueprints");
        }
    }
    let archean_path = get_archean_path(matches)
        .expect("Could not get Archean path")
        .join("Archean-data")
        .join("client")
        .join("blueprints");
    watch_event(&archean_path).expect("Could not watch Archean blueprints path; is Archean installed via Steam?");
}

pub fn watch_event<P: AsRef<Path>>(path: P) -> NotifyResult<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx)?;
    debouncer.watcher().watch(path.as_ref(), RecursiveMode::Recursive)?;
    println!("waiting for blueprint events");
    for result in rx {
        match result {
            Ok(events) => events.iter().for_each(|event| handle_event(event)),
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        }
    }
    Ok(())
}

fn handle_event(event: &DebouncedEvent) {
    if event.kind.is_modify() {
        let blueprint_name = event.paths.get(0).unwrap();
        println!("Blueprint file changed: {:?}, yanking..", blueprint_name);
        yank_from_config(YankConfig{
            file_name: blueprint_name.to_path_buf(),
            folder: true,
            watch: WatchState::Watching, // we are already watching from the yank context
            disable_collate: false
        });
    }
}
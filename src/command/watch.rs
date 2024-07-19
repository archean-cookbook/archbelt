use std::path::Path;
use std::time::Duration;
use clap::ArgMatches;
use notify::{Event, Watcher, RecursiveMode, Result};
use notify_debouncer_full::new_debouncer;
use super::prelude::*;

pub fn watch_blueprints(args: &ArgMatches) {
    // TODO: look at args

    let archean_path = get_archean_path()
        .expect("Could not get Archean path")
        .join("Archean-data")
        .join("client")
        .join("blueprints");
    watch_event(&archean_path).expect("Could not watch Archean blueprints path");
}

fn watch_event<P: AsRef<Path>>(path: P) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx)?;
    debouncer.watcher().watch(path.as_ref(), RecursiveMode::Recursive)?;

    // print all events and errors
    for result in rx {
        match result {
            Ok(events) => events.iter().for_each(|event| log::info!("{event:?}")),
            Err(errors) => errors.iter().for_each(|error| log::error!("{error:?}")),
        }
    }

    Ok(())
}

pub(crate) fn watch_file_event(blueprint_name: String) -> impl FnMut(Result<Event>) {
    move |event| {
        println!("{:?}", event);
    }
}
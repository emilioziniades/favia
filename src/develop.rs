use crate::build::build_content_file;
use crate::builder::Builder;
use crate::error::Error;
use log::info;
use notify::event::{EventKind::*, ModifyKind::*};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path;
// use std::thread;

pub fn develop(cwd: &path::Path) -> Result<(), Error> {
    let builder = Builder::new(cwd)?;
    info!("doing initial site build");
    crate::build(cwd)?;
    info!("development server starting");

    // use rocket to serve static files: https://api.rocket.rs/v0.4/rocket_contrib/serve/struct.StaticFiles.html
    // thread::spawn(|| {});

    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(&builder.templates_folder(), RecursiveMode::Recursive)?;
    watcher.watch(&builder.content_folder(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if let Some(event) = filter_event(event) {
                    println!("changed: {:#?}", event);
                    handle_file_change(event, &builder)?;
                };
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn filter_event(event: notify::Event) -> Option<notify::Event> {
    if event.paths.len() > 1 {
        panic!("have not considered events with more than one path")
    }

    let path = event.paths.first().expect("the event should have a path");

    // weird neovim file management
    if path.file_name().unwrap() == "4913" {
        return None;
    }

    // weird neovim file management
    if path.file_name().unwrap().to_str().unwrap().ends_with('~') {
        return None;
    }

    // more neovim weirdness
    if matches!(event.kind, Access(_) | Modify(Metadata(_))) {
        return None;
    }

    Some(event)
}

fn handle_file_change(event: notify::Event, builder: &Builder) -> Result<(), Error> {
    let path = event.paths.first().expect("the event should have a path");

    if path.starts_with(builder.content_folder()) {
        build_content_file(path, builder)?;
    } else if path.starts_with(builder.templates_folder()) {
        // need to add something like build_template_file
        todo!();
    }

    Ok(())
}

use crate::builder::Builder;
use crate::Result;
use log::{debug, info};
use notify::event::{EventKind::*, ModifyKind::*};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use rocket::tokio;
use std::path;

pub async fn develop(cwd: &path::Path) -> Result<()> {
    let mut builder = Builder::new(cwd)?;
    info!("doing initial site build");
    crate::build(cwd).unwrap();
    info!("development server starting");

    let build_folder = builder.build_folder();

    // watch for changes, and rebuild changes files
    tokio::spawn(async move {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, notify::Config::default()).unwrap();
        watcher
            .watch(builder.templates_folder(), RecursiveMode::Recursive)
            .unwrap();
        watcher
            .watch(builder.content_folder(), RecursiveMode::Recursive)
            .unwrap();
        for res in rx {
            match res {
                Ok(event) => {
                    debug!("{event:#?}");
                    if let Some(event) = filter_event(event) {
                        debug!("file change detected: {event:#?}");
                        handle_file_change(event, &mut builder).unwrap();
                    };
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    // development server
    let _ = rocket::build()
        .mount("/", rocket::fs::FileServer::from(build_folder))
        .launch()
        .await?;

    Ok(())
}

fn filter_event(event: notify::Event) -> Option<notify::Event> {
    if event.paths.len() > 1 {
        panic!("have not considered events with more than one path");
    }

    let path = event.paths.first()?;

    // weird neovim file management
    if path.file_name()? == "4913" {
        return None;
    }

    // weird neovim file management
    if path.file_name()?.to_str()?.ends_with('~') {
        return None;
    }

    // more neovim weirdness
    if matches!(event.kind, Access(_) | Modify(Metadata(_))) {
        return None;
    }

    Some(event)
}

fn handle_file_change(event: notify::Event, builder: &mut Builder) -> Result<()> {
    let path = event.paths.first().expect("the event should have a path");

    if path.starts_with(builder.content_folder()) {
        builder.build_content_file(path)?;
    } else if path.starts_with(builder.templates_folder()) {
        // TODO: Indentify specific files to rebuild. Currently, this
        // function rebuilds the whole site when a template file changes.
        // Rebuilding the whole site wont scale with large sites.
        // Determining which files to rebuild is non-trivial because
        // 1. A template may be responsible for multiple content files
        // 2. A template may be a base template, so we would also need
        // to know the dependency graph of templates.
        debug!("template change detected, rebuilding project");
        builder.reload_templates()?;
        builder.build()?;
    }

    Ok(())
}

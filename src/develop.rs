use crate::builder::Builder;
use crate::Result;
use log::info;
use notify::event::{EventKind::*, ModifyKind::*};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use rocket::tokio;
use std::path;

pub async fn develop(cwd: &path::Path) -> Result<()> {
    let builder = Builder::new(cwd)?;
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
                    if let Some(event) = filter_event(event) {
                        println!("changed: {:#?}", event);
                        handle_file_change(event, &builder).unwrap();
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
        panic!("have not considered events with more than one path")
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

fn handle_file_change(event: notify::Event, builder: &Builder) -> Result<()> {
    let path = event.paths.first().expect("the event should have a path");

    if path.starts_with(builder.content_folder()) {
        builder.build_content_file(path)?;
    } else if path.starts_with(builder.templates_folder()) {
        todo!("build changes in template folder");
    }

    Ok(())
}

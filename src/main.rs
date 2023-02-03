//! # favia
//! a lightweight, zero-config, tailwind-built-in, static site generator.
//!
//! ## api
//! There are only two commands:
//!
//! `favia build` builds a bundle of html and css to be served
//!
//! `favia dev` runs a development server listening for changes and rebuilding
//!
//! ## project structure
//!
//! A favia project has only two directories
//!
//! - `content`
//!
//! - `templates`
//!
//! `content` contains a tree of subdirectories and/or markdown files, optionally with TOML frontmatter.
//!
//! `templates` contains a tree of subdirectories and/or [Tera](https://tera.netlify.app/) templates.
//!
//! favia determines the site structure from these two folders.
//!
//!
use clap::{Arg, ArgAction, Command};
use log::{debug, error, info, trace, warn, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

fn main() {
    let matches = cli().get_matches();

    let log_level = match matches.get_count("verbose") {
        2 => LevelFilter::Trace,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Info,
    };

    TermLogger::init(
        log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("valid logging config");

    info!("an info log");
    debug!("a debug log");
    trace!("a trace log");
    warn!("a warn log");
    error!("an error log");

    match matches.subcommand() {
        Some(("dev", _)) => dev(),
        Some(("build", _)) => build(),
        _ => unreachable!(),
    }
}

fn cli() -> Command {
    Command::new("favia")
        .about("a zero config static site generator with tailwind built in")
        .author("Emilio Ziniades")
        .subcommand_required(true)
        .arg(
            Arg::new("verbose")
                .help("sets verbosity level, -v => debug, -vv => trace, otherwise info")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count),
        )
        .subcommand(
            Command::new("dev").about("run a development server which watches for file changes"),
        )
        .subcommand(
            Command::new("build").about("build the site into static html and css to be served"),
        )
}

fn dev() {
    info!("development server starting")
}

fn build() {
    info!("building site")
}

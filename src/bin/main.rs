use std::{env, path::Path, process};

use clap::{Arg, ArgAction, Command};
use favia::config;
use log::{error, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

const VERSION: &str = "0.1.1";
const CONFIG_FILENAME: &str = "favia.toml";

fn main() {
    let matches = cli().get_matches();

    let log_level = match matches.get_count("verbose") {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let log_config = ConfigBuilder::new()
        .add_filter_ignore_str("markdown::tokenizer")
        .add_filter_ignore_str("globset")
        .set_location_level(LevelFilter::Off)
        .set_thread_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .build();

    TermLogger::init(
        log_level,
        log_config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let cwd = env::current_dir().unwrap();

    let config = config::Config::try_from(Path::new(CONFIG_FILENAME)).unwrap_or_else(|error| {
        error!("{error:#?}");
        process::exit(1);
    });

    if config.version != VERSION {
        error!("version mismatch! {} != {VERSION}", config.version);
        process::exit(1);
    }

    match matches.subcommand() {
        Some(("dev", _)) => favia::dev(),
        Some(("build", _)) => favia::build(cwd, config.base_url).unwrap_or_else(|error| {
            error!("{error:#?}");
            process::exit(1);
        }),
        _ => unreachable!(),
    }
}

fn cli() -> Command {
    Command::new("favia")
        .about("a minimal config static site generator with tailwind built in")
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

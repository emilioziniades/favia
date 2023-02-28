use std::{env, process};

use clap::{Arg, ArgAction, Command};
use log::{error, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

fn main() {
    let matches = cli().get_matches();

    let log_level = match matches.get_count("verbose") {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let config = ConfigBuilder::new()
        .add_filter_ignore_str("markdown::tokenizer")
        .add_filter_ignore_str("globset")
        .set_location_level(LevelFilter::Off)
        .set_thread_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .build();

    TermLogger::init(log_level, config, TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    let cwd = env::current_dir().unwrap();

    match matches.subcommand() {
        Some(("develop", _)) => favia::develop(&cwd).unwrap_or_else(|err| {
            error!("{err:#?}");
            // error!("{err}");
            process::exit(1);
        }),
        Some(("build", _)) => favia::build(&cwd).unwrap_or_else(|err| {
            error!("{err:#?}");
            // error!("{err}");
            process::exit(1);
        }),
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
            Command::new("develop")
                .about("run a development server which watches for file changes"),
        )
        .subcommand(
            Command::new("build").about("build the site into static html and css to be served"),
        )
}

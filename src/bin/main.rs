use std::process;

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
        Some(("dev", _)) => favia::dev(),
        Some(("build", _)) => favia::build().unwrap_or_else(|err| {
            error!("{err:#?}");
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
            Command::new("dev").about("run a development server which watches for file changes"),
        )
        .subcommand(
            Command::new("build").about("build the site into static html and css to be served"),
        )
}

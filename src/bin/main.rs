use std::{env, process};

use clap::Parser;
use log::{error, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

#[rocket::main]
async fn main() {
    let cli = favia::Cli::parse();

    let log_level = match cli.verbosity {
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

    match cli.command {
        favia::Commands::Develop => favia::develop(&cwd).await.unwrap_or_else(|err| {
            error!("{err:#?}");
            // error!("{err}");
            process::exit(1);
        }),
        favia::Commands::Build => favia::build(&cwd).unwrap_or_else(|err| {
            error!("{err:#?}");
            // error!("{err}");
            process::exit(1);
        }),
        favia::Commands::New { name } => favia::new(&cwd, name).unwrap_or_else(|err| {
            error!("{err:#?}");
            // error!("{err}");
            process::exit(1);
        }),
    }
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
/// 🪸 favia: a zero config static site generator with tailwind built in
pub struct Cli {
    #[command(subcommand)]
    /// favia subcommand to execute
    pub command: Commands,

    /// make output verbose (-v) or very verbose (-vv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbosity: u8,
}

#[derive(Subcommand)]
pub enum Commands {
    /// build the site into static html and css to be served
    Build,
    /// run a development server which watches for file changes
    Develop,
    /// create a new favia project with the specified name
    New {
        /// project name
        #[command()]
        name: String,
    },
}

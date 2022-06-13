mod commands;
mod cfg;

use clap::Parser;
use cfg::Config;

use commands::Commands;

/// Manage your monorepo versions with ease
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    let config = Config::init().unwrap();

    if config.debug {
        println!("Configuration: {:?}", config);
    }

    match &cli.command {
        Commands::List {} => commands::list::run(),
        Commands::Bump { name, component } =>
            commands::bump::run(name, component)
    }
}

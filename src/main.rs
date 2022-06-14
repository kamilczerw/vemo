mod commands;
mod cfg;
mod error;

use std::process::exit;
use clap::Parser;
use cfg::Config;
use error::AppError;

use commands::Commands;

/// Manage your monorepo versions with ease
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn app() -> Result<(), AppError> {
    let cli = Cli::parse();
    let config = Config::init()?;

    if config.debug {
        println!("Configuration: {:?}", config);
    }

    let result: () = match &cli.command {
        Commands::List {} => commands::list::run()?,
        Commands::Bump { name, component } =>
            commands::bump::run(name, component)?
    };

    Ok(result)
}

fn main() {
    app().unwrap_or_else(|error| {
        println!("{}", error.message);
        exit(error.code)
    })
}

extern crate core;

mod commands;
mod cfg;
mod error;
mod git;

use std::process::exit;
use clap::Parser;
use cfg::Config;
use error::AppError;

use commands::Commands;
use crate::commands::shell::git::Git;

/// Manage your monorepo versions with ease.
///
/// This simple tool can help you with managing your monorepo versions.
/// It can bump your versions, create a new release, and more.
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
        println!("Configuration: {:#?}", &config);
    }

    let format = &config.format;
    let git = Git::init(format.clone());

    let repo_info = git.get_repo_info()?;

    let git_client = git::client::new_client(&config, repo_info)?;

    let result: () = match &cli.command {
        Commands::List {} => commands::list::run(config)?,
        Commands::Bump { name, component } =>
            commands::bump::run(config, name, component, git_client)?
    };

    Ok(result)
}

fn main() {
    app().unwrap_or_else(|error| {
        println!("Error: {}", error.message);
        exit(error.code)
    })
}

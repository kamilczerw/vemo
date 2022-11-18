extern crate core;

mod commands;
mod cfg;
mod error;
mod git;

use std::process::exit;
use clap::Parser;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use cfg::Config;
use error::AppError;
use log::{debug, LevelFilter};

use commands::Commands;
use git::Git;

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

    let stdout = ConsoleAppender::builder().build();
    let log_config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)));

    let log_config = if config.debug {
        log_config.build(Root::builder().appender("stdout").build(LevelFilter::Debug))
    } else {
        log_config.build(Root::builder().appender("stdout").build(LevelFilter::Error))
    }.unwrap();

    let _handle = log4rs::init_config(log_config).unwrap();

    debug!("Configuration: {:#?}", config);

    let format = &config.format;
    let git = Git::init(format.clone());

    let repo_info = git.get_repo_info()?;

    // TODO: if the git client is not supported, the program should not fail
    //       There should still be possibility to create a tag without a git provider
    let git_client = git::client::new_client(&config, repo_info)?;

    let result: () = match &cli.command {
        Commands::List {} => commands::list::run(config)?,
        Commands::Bump { name, component } =>
            commands::bump::run(config, name, component, git_client)?,
        Commands::BumpV2 { name, component } =>
            commands::bump::run_v2(config, name, component)?
    };

    Ok(result)
}

fn main() {
    app().unwrap_or_else(|error| {
        println!("Error: {}", error.message);
        exit(error.code)
    })
}

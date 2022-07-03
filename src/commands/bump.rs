use std::io::stdin;
use semver::Version;
use crate::cfg::Config;
use crate::commands::Component;
use crate::commands::error::CommandError;
use crate::commands::shell::git::Git;
use colored::Colorize;
use log::debug;
use crate::git::GitClient;
use crate::git::model::Release;

pub fn run(config: Config, name: &String, component: &Component, git_client: Box<dyn GitClient>) -> Result<(), CommandError>  {
    let format = config.format.clone();
    // let git = Git::init(format.clone());

    let default_version = Version::parse("0.1.0").unwrap();

    // let (latest_tag, new_tag) = match git.find_latest_tag(name)? {
    let (latest_tag, new_tag) = match git_client.latest_release(name)? {
        None => {
            debug!("Version of {} not found, new tag with default version ({}) version will be created", name, default_version);
            (None, Release::new_with_format(&format, name, default_version))
        }
        Some(tag) => (Some(tag.clone()), tag.bump(component))
    };

    let commits = git_client.get_changelog(latest_tag, name.as_str())?;
    // let commits = config.app_path(name.as_str()).map(|path| {
    //     // git.get_commits(latest_tag, path.as_str())
    //     git_client.get_changelog(latest_tag, path.as_str())
    // }).unwrap_or(Ok(vec![]));

    let mut body = String::from("## What's Changed\n\n");

    for commit in commits {
        body.push_str(&format!("* {} by {}\n", commit.message, commit.author.email));
    }


    let release_name = format!("{} - v{}", &name, &new_tag.version);

    release(git_client, release_name, new_tag, body)
}

fn release(git_client: Box<dyn GitClient>, name: String, new_tag: Release, body: String) -> Result<(), CommandError> {
    println!("  {} {}", "name:".bold(), &name.bright_green().bold());
    println!("  {}  {}", "tag:".bold(), &new_tag.formatted().bright_green().bold());
    println!("  {}", "body:".bold());
    for line in body.split("\n").into_iter() {
        println!("    {}", line);

    }
    println!("{}", "Are you sure you want to create new release with [y/e/N]:".yellow());
    let stdin = stdin();
    let mut s: String = String::new();
    stdin.read_line(&mut s).unwrap();
    let s = s.replace("\n", "");

    if &s == "y" || &s == "Y" {
        println!("Applying changes");
        git_client.create_release(name, new_tag, body)?;
    } else if &s == "e" || &s == "E" {
        println!("Editing changes");
        let edited = edit::edit(body).unwrap();
        release(git_client, name, new_tag, edited)?;
    } else {
        println!("Aborting");
    }

    Ok(())
}

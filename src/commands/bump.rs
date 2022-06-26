use std::io::stdin;
use semver::Version;
use crate::cfg::Config;
use crate::commands::Component;
use crate::commands::error::CommandError;
use crate::commands::shell::git::{Git, Tag};
use colored::Colorize;
use crate::git::GitClient;

pub fn run(config: Config, name: &String, component: &Component, git_client: Box<dyn GitClient>) -> Result<(), CommandError>  {
    let format = config.format.clone();
    let git = Git::init(format.clone());

    let default_version = Version::parse("0.1.0").unwrap();

    let (latest_tag, new_tag) = match git.find_latest_tag(name)? {
        None => {
            if config.debug {
                println!("Version of {} not found, new tag with default version ({}) version will be created", name, default_version);
            }
            (None, Tag::new_with_format(&format, name, default_version))
        }
        Some(tag) => (Some(tag.clone()), tag.bump(component))
    };

    let diff = config.app_path(name.as_str()).map(|path| {
        git.get_commits(latest_tag, path.as_str())
    }).unwrap_or(vec![]);

    let _template = "## What's Changed\n\n";
    // let body = edit::edit(template).unwrap(); // TODO: handle error
    let body = String::from("This is\na multi line\n string ");
    let release_name = format!("{} - v{}", &name, &new_tag.version);

    println!("  {} {}", "name:".bold(), &release_name.bright_green().bold());
    println!("  {}  {}", "tag:".bold(), &new_tag.formatted().bright_green().bold());
    println!("  {}", "body:".bold());
    for line in body.split("\n").into_iter() {
        println!("    {}", line);

    }
    println!("{}", "Are you sure you want to create new release with [y/N]:".yellow());
    let stdin = stdin();
    let mut s: String = String::new();
    stdin.read_line(&mut s).unwrap();
    let s = s.replace("\n", "");

    if &s == "y" || &s == "Y" {
        println!("Applying changes");
        git_client.create_release(release_name, new_tag, body)?;
    }

    Ok(())
}

use semver::Version;
use crate::cfg::Config;
use crate::commands::Component;
use crate::commands::error::CommandError;
use crate::commands::shell::git::{Git, Tag};

pub fn run(config: Config, name: &String, component: &Component) -> Result<(), CommandError>  {
    let format = config.format;
    let git = Git::init(format.clone());
    let tag = git.find_latest_tag(name)?;

    let default_version = Version::parse("0.1.0").unwrap();

    let new_tag = match git.find_latest_tag(name)? {
        None => {
            if config.debug {
                println!("Version of {} not found, new tag with default version ({}) version will be created", name, default_version);
            }
            Tag::new_with_format(&format, name, default_version)
        }
        Some(tag) => tag.bump(component, &format)
    };

    println!("Bumping: {} => {}", tag.map(|t| t.raw).unwrap_or("<nothing>".to_string()), new_tag);



    // let output = Git::get_tags(format)?;
    //
    // println!("{}", output);

    Ok(())
}

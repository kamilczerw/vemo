use std::collections::HashMap;
use crate::commands::error::CommandError;
use crate::commands::shell::Git;
use crate::{app, Config};

use regex::{Captures, Regex};
use semver::Version;
use colored::Colorize;

pub fn run(config: Config) -> Result<(), CommandError> {
    let format = config.format
        .replace("{version}", "*")
        .replace("{app_name}", "*");

    let output = Git::get_tags(format)?;
    let apps = extract_app_names(output, config.format);
    if apps.is_empty() {
        println!("{}", "💩 No applications found in the repo. Try creating new one by running: "
            .yellow());
        println!("\n  {}\n", "vemo bump {app_name}".bold());
        println!("It will create a new app with {{app_name}} and set a default version, which you \
        can find by adding {} to the above command.", "--help".bold())
    } else {
        let length = apps.keys()
            .max_by(|x, y | x.len().cmp(&y.len())).cloned()
            .map(|key| key.len()).unwrap_or(0);

        println!("List of available applications:");
        for (name, version) in apps {
            println!("   - {:width$} {}", name.bold(), version.magenta(), width = length + 2)
        }
    }

    Ok(())
}

fn extract_app_names(tags: String, format: String) -> HashMap<String, String> {
    let format = format
        .replace("{version}", "(?P<version>[0-9]+\\.[0-9]+\\.[0-9]+)")
        .replace("{app_name}", "(?P<app_name>.*)");
    let format = format!("{}\\n", format);

    let mut apps = HashMap::new();

    let re = Regex::new(format.as_str()).unwrap();
    for caps in re.captures_iter(tags.as_str()) {
        let app_name = caps.name("app_name")
            .map(|m| String::from(m.as_str()));
        let version = get_version(&caps);

        match (app_name, version) {
            (Some(app_name), Some(version)) => {
                match apps.get(&app_name) {
                    None => {
                        apps.insert(String::from(app_name), version.to_string());
                    }
                    Some(last_version) => {
                        let last_version = Version::parse(last_version).unwrap();
                        if version > last_version {
                            apps.insert(String::from(app_name), version.to_string());
                        }
                    }
                }
            },
            _ => {}
        }
    }

    apps
}

fn get_version(caps: &Captures) -> Option<Version> {
    let version = caps.name("version")
        .map(|m| Version::parse(m.as_str()));
    match version {
        Some(Ok(v)) => Some(v),
        _ => None
    }
}

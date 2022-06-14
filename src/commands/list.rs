use std::collections::HashMap;
use crate::commands::error::CommandError;
use crate::commands::shell::Git;
use crate::Config;

use regex::Regex;
use semver::Version;

pub fn run(config: Config) -> Result<(), CommandError> {
    println!("List applications");

    let format = config.format
        .replace("{version}", "*")
        .replace("{app_name}", "*");

    let output = Git::get_tags(format)?;
    for (name, version) in extract_app_names(output, config.format) {
        println!(" - {} {}", name, version) // TODO: print it in a nicer way
        // TODO add some emojis and stuff
    }

    Ok(())
}

fn extract_app_names(tags: String, format: String) -> HashMap<String, String> {
    let format = format
        .replace("{version}", "(?P<version>[0-9]+\\.[0-9]+\\.[0-9]+)")
        .replace("{app_name}", "(?P<app_name>.*)");
    let format = format!("{}\\n", format);

    let mut apps = HashMap::new();

    let r = Regex::new(format.as_str()).unwrap();
    for caps in r.captures_iter(tags.as_str()) {
        let app_name = &caps["app_name"];
        let version = Version::parse(&caps["version"]).unwrap(); // TODO: handle properly
        match apps.get(&caps["app_name"]) {
            None => {
                apps.insert(String::from(app_name), version.to_string());
            }
            Some(last_version) => {
                let last_version = Version::parse(last_version).unwrap(); // TODO: handle properly
                if version > last_version {
                    apps.insert(String::from(app_name), version.to_string());
                }
            }
        }
    }

    println!("format: {}", format);

    apps
}

use std::str;
use crate::commands::error::CommandError;
use crate::commands::shell::Git;
use crate::Config;

pub fn run(config: Config) -> Result<(), CommandError> {
    println!("List applications");

    let format = config.format
        .replace("{version}", "*")
        .replace("{app_name}", "*");

    let output = Git::run(vec!["tag", "-l", format.as_str()])?;
    for tag in output.split("\n").collect::<Vec<&str>>() {
        println!("{}", tag);
    }

    Ok(())
}

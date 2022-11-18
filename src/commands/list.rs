use crate::commands::error::CommandError;
use crate::git::Git;
use crate::Config;

use colored::Colorize;

pub fn run(config: Config) -> Result<(), CommandError> {
    let git = Git::init(config.format);

    let apps = git.get_latest_tags()?;
    if apps.is_empty() {
        println!("{}", "💩 No applications found in the repo. Try creating new one by running: "
            .yellow());
        println!("\n  {}\n", "vemo bump {app_name}".bold());
        println!("It will create a new app with {{app_name}} and set a default version, which you \
        can find by adding {} to the above command.", "--help".bold())
    } else {
        let length = apps.clone().into_iter()
            .max_by(|x, y| x.app_name.len().cmp(&y.app_name.len()))
            .map(|key| key.app_name.len()).unwrap_or(0);

        println!("List of available applications:");
        for tag in apps {
            println!("   - {:width$} {}", tag.app_name.bold(), tag.version.to_string().magenta(), width = length + 2)
        }
    }

    Ok(())
}

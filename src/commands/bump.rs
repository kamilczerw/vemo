use crate::cfg::Config;
use crate::commands::Component;
use crate::commands::error::CommandError;
use crate::commands::shell::Git;

pub fn run(config: Config, name: &String, component: &Component) -> Result<(), CommandError>  {
    let format = config.format
        .replace("{version}", "*")
        .replace("{app_name}", name);

    println!("foramt: {}", &format);

    let output = Git::get_tags(format)?;

    println!("{}", output);

    Ok(())
}

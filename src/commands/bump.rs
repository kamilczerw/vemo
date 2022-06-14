use crate::cfg::Config;
use crate::commands::Component;
use crate::commands::error::CommandError;

pub fn run(config: Config, name: &String, component: &Component) -> Result<(), CommandError>  {
    println!("bump {}", name);

    Ok(())
}

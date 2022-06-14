use crate::commands::Component;
use crate::commands::error::CommandError;

pub fn run(name: &String, component: &Component) -> Result<(), CommandError>  {
    println!("bump {}", name);

    Ok(())
}

use std::process::Command;
use crate::commands::error::CommandError;

pub struct Git {}

impl Git {
    pub fn run(args: Vec<&str>) -> Result<String, CommandError> {
        let output = Command::new("git").args(args)
            .output()
            .expect("Failed to execute git command");

        if !output.status.success() {
            let shell_error = String::from_utf8(output.stderr)?;
            Err(CommandError::ShellError(shell_error))
        } else {
            Ok(String::from_utf8(output.stdout)?)
        }
    }
}

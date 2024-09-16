use std::path::PathBuf;
use std::process::Command;
use std::fmt::{Debug, Formatter};

pub fn execute_commands(command: &str, dir: &PathBuf) -> Result<(), CommandError>
{
    let commands: Vec<&str> = command.split(";").collect();

    for &command in commands.iter() {
        if let Err(e) = execute_command(command.trim(), dir) {
            return Err(e);
        }
    }
    
    Ok(())
}

fn execute_command(command: &str, dir: &PathBuf) -> Result<(), CommandError>
{
    let command_parts: Vec<&str> = command.split_whitespace().collect();

    if command_parts.is_empty() {
        return Err(CommandError::InvalidInput);
    }

    let cmd = command_parts[0];
    let args = &command_parts[1..];

    let status = Command::new(cmd).current_dir(dir).args(args).status();

    let status = match status {
        Ok(s) => s,
        Err(_) => return Err(CommandError::CreationError),
    };

    if status.success() {
        Ok(())
    } else {
        Err(CommandError::CommandFailed)
    }
}

pub enum CommandError
{
    CommandFailed,
    InvalidInput,
    CreationError,
}

impl Debug for CommandError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self {
            CommandError::CommandFailed => write!(f, "Command execution failed."),
            CommandError::InvalidInput => write!(f, "Command input was invalid."),
            CommandError::CreationError => write!(f, "Command creation failed."),
        }
    }
}
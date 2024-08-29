mod add;
mod list;
mod remove;
mod spawn;
mod version;

use crate::commands::Command::{Add, Remove, Spawn};
use crate::commands::CommandBuildError::NotRecognisedCommand;
use std::collections::HashMap;
use std::str::FromStr;
// region: Command Enum

pub enum Command
{
    Spawn(Vec<String>),
    Add(Vec<String>),
    Remove(Vec<String>),
    List,
    Version,
}

impl FromStr for Command
{
    type Err = CommandBuildError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut parts = s.split_whitespace();

        let main_command = match parts.next() {
            Some(command) => command,
            None => return Err(CommandBuildError::NotEnoughArgsIntroduced),
        };

        let command_instr = parts.map(String::from).collect::<Vec<String>>();

        match main_command {
            "spawn" => Ok(Spawn(command_instr)),
            "add" => Ok(Add(command_instr)),
            "rm" => Ok(Remove(command_instr)),
            "list" => Ok(Command::List),
            "version" => Ok(Command::Version),
            _ => Err(NotRecognisedCommand),
        }
    }
}

impl Command
{
    pub fn execute(&self)
    {
        match self {
            Spawn(args) => spawn::execute(args),
            Add(args) => add::execute(args),
            Remove(args) => remove::execute(args),
            Command::List => list::execute(),
        }
    }
}

// endregion: Command Enum

// region: Command Build Error

#[derive(Debug)]
pub enum CommandBuildError
{
    NotRecognisedCommand,
    NotEnoughArgsIntroduced,
}

// endregion: Command Build Error

fn map_flags(args: &Vec<String>) -> HashMap<String, String>
{
    let mut hash_map = HashMap::new();
    let mut i = 0;

    while i < args.len() {
        let actual_word = &args[i];

        if is_flag(actual_word) {
            if i + 1 < args.len() && !is_flag(&args[i + 1]) {
                hash_map.insert(actual_word.to_string(), args[i + 1].clone());
                i += 2;
            } else {
                hash_map.insert(actual_word.to_string(), String::new());
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    hash_map
}

fn is_flag(s: &str) -> bool
{
    s.starts_with("-") || s.starts_with("--")
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_map_flags()
    {
        let args = vec!["-n", "name", "a", "-a", "-p", "path", "-r"];
        let result = map_flags(&args.iter().map(|s| s.to_string()).collect());

        assert_eq!(result.get("-n").unwrap(), "name");
        assert_eq!(result.get("-p").unwrap(), "path");
        assert_eq!(result.get("-r").unwrap(), "");
        assert_eq!(result.get("-a").unwrap(), "");
        
        let args = vec!["mkt", "remove", "-n", "crates"];
        let result = map_flags(&args.iter().map(|s| s.to_string()).collect());

        assert_eq!(result.get("-n").unwrap(), "crates");
    }

    #[test]
    fn test_is_flag()
    {
        assert_eq!(is_flag("-n"), true);
        assert_eq!(is_flag("--name"), true);
        assert_eq!(is_flag("name"), false);
    }
}

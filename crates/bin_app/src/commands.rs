mod add;
mod list;
mod remove;
mod spawn;
mod version;

use crate::commands::CommandBuildError::NotRecognisedCommand;
use std::collections::HashMap;
use crate::commands::add::Add;
use crate::commands::list::List;
use crate::commands::remove::Remove;
use crate::commands::spawn::Spawn;
use crate::commands::version::Version;
// region: Command Trait

pub trait Command
{
    fn execute(flags: HashMap<String, String>);
    fn show_usage();
}

pub fn try_execute(s: &str) -> Result<(), CommandBuildError>
{
    let mut parts = s.split_whitespace();

    let main_command = match parts.next() {
        Some(command) => command,
        None => return Err(CommandBuildError::NotEnoughArgsIntroduced),
    };

    let command_instr = parts.map(String::from).collect::<Vec<String>>();
    let flags = map_flags(&command_instr);

    match main_command {
        "spawn" => Ok(Spawn::execute(flags)),
        "add" => Ok(Add::execute(flags)),
        "rm" => Ok(Remove::execute(flags)),
        "list" => Ok(List::execute(flags)),
        "version" => Ok(Version::execute(flags)),
        _ => Err(NotRecognisedCommand),
    }
}

// endregion: Command Trait

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

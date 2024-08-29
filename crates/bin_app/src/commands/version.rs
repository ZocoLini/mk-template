use crate::commands::Command;
use std::collections::HashMap;

pub struct Version;

impl Command for Version
{
    fn execute(_flags: HashMap<String, String>)
    {
        println!("Version: 0.1.0");
    }

    fn show_usage() {}
}

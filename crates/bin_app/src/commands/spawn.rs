use crate::commands::Command;
use crate::{templates, BIN_NAME};
use std::collections::HashMap;

pub struct Spawn;

impl Command for Spawn
{
    fn execute(flags: HashMap<String, String>)
    {
        let template_name = match flags.get("-n") {
            Some(name) => name,
            None => {
                Self::show_usage();
                return;
            }
        };

        let template_output_name = flags.get("-o").unwrap_or_else(|| template_name);

        templates::generate(template_name, template_output_name);
    }

    fn show_usage()
    {
        println!(
            "USAGE: {} spawn -n <Template Name> [-o <Spawn name (Some templates can`t use it)>]",
            BIN_NAME
        );
    }
}

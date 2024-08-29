use crate::commands::Command;
use crate::{templates, BIN_NAME};
use std::collections::HashMap;

pub struct Remove;

impl Command for Remove
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

        templates::remove_template(template_name)
    }

    fn show_usage()
    {
        println!(
            "USAGE: {} remove -n <Template Name>",
            BIN_NAME
        );
    }
}
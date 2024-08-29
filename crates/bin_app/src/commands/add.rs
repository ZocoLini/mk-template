use crate::commands::Command;
use crate::{templates, BIN_NAME};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Add;

impl Command for Add
{
    fn execute(flags: HashMap<String, String>) {
        let template_path = match flags.get("-p") {
            Some(path) => path,
            None => {
                Self::show_usage();
                return;
            }
        };

        let template_name = match flags.get("-n") {
            Some(name) => name,
            None => &extract_name_from_path(template_path),
        };

        if flags.contains_key("-r") || templates::get_template_data_path(template_name).is_none() {
            templates::add_template(template_name, template_path);
        } else {
            println!("That template name is already being used. Use -r to replace it.");
        }
    }

    fn show_usage() {
        println!(
            "USAGE: {} add -p <New template's path> [-n <Template Name>]",
            BIN_NAME
        );
    }
}

fn extract_name_from_path(path: &str) -> String
{
    // Extracting the file name from the path
    let path = PathBuf::from(path);
    let file_name = path
        .file_name()
        .expect("Should have a name")
        .to_str()
        .expect("Should exist");

    // Removing the extension from the file name
    file_name.split('.').collect::<Vec<&str>>()[0].to_string()
}

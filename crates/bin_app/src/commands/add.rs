use crate::{commands, templates, BIN_NAME};
use std::ffi::OsString;
use std::path::PathBuf;

pub fn execute(s: &Vec<String>)
{
    let flags = commands::map_flags(s);

    let template_path = match flags.get("-p") {
        Some(path) => path,
        None => {
            print_command_usage();
            return;
        }
    };

    let template_name = match flags.get("-n") {
        Some(name) => name,
        None => &extrarct_name_from_path(template_path)
            .to_str()
            .expect("Should exist")
            .to_string()
    };
    
    if flags.contains_key("-r") || !templates::exists_template(template_name) {
        templates::add_template(template_name, template_path);
    } else {
        println!("That template name is already being used. Use -r to replace it.");
    }
}

fn extrarct_name_from_path(path: &str) -> OsString
{
    let path = PathBuf::from(path);
    let file_name = path.file_name().expect("Should have a name");
    file_name.to_os_string()
}

fn print_command_usage()
{
    println!(
        "USAGE: {} add -p <New template's path> [-n <Template Name>]",
        BIN_NAME
    );
}

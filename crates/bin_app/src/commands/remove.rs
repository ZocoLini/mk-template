use crate::{BIN_NAME, templates};
use crate::commands::map_flags;

pub fn execute(s: &Vec<String>)
{
    let flags = map_flags(s);

    let template_name = match flags.get("n") {
        Some(name) => name,
        None => {
            print_command_usage();
            return;
        }
    };
    
    templates::remove_template(template_name)
}

fn print_command_usage()
{
    println!(
        "USAGE: {} remove -n <Template Name>",
        BIN_NAME
    );
}
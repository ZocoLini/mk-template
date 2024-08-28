use crate::{commands, templates, BIN_NAME};

pub fn execute(s: &Vec<String>)
{
    let flags = commands::map_flags(s);

    let template_name = match flags.get("-n") {
        Some(name) => name,
        None => {
            print_command_usage();
            return;
        }
    };

    let template_output_name = flags.get("-o").unwrap_or_else(|| template_name);
    
    templates::generate_template(template_name, template_output_name);
}

fn print_command_usage()
{
    println!(
        "USAGE: {} spawn -n <Template Name> [-o <Spawn name (Some templates can`t use it)>]",
        BIN_NAME
    );
}

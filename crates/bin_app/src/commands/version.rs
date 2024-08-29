use crate::{commands, templates};

pub fn execute(s: &Vec<String>)
{
    let flags = commands::map_flags(s);

    let template_path = match flags.get("-p") {
        Some(path) => path,
        None => {
            crate::commands::add::print_command_usage();
            return;
        }
    };

    let template_name = match flags.get("-n") {
        Some(name) => name,
        None => &crate::commands::add::extract_name_from_path(template_path),
    };

    if flags.contains_key("-r") || templates::get_template_data_path(template_name).is_none() {
        templates::add_template(template_name, template_path);
    } else {
        println!("That template name is already being used. Use -r to replace it.");
    }
}
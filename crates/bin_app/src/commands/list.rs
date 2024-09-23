use std::collections::HashMap;
use crate::commands::Command;
use crate::templates;

pub struct List;

impl Command for List{
    fn execute(_flags: HashMap<String, String>)
    {
        for (template_data, template_name) in templates::get_available_templates() {
            println!("{}", template_name);
            
            if _flags.contains_key("-d") { 
                println!("----| Description: {}", template_data.to_template().get_description());
            }
        }
    }

    fn show_usage() {
    }
}
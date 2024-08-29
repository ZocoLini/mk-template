use std::collections::HashMap;
use crate::commands::Command;
use crate::templates;

pub struct List;

impl Command for List{
    fn execute(_flags: HashMap<String, String>)
    {
        let templates_dir_entries = match templates::get_available_templates() {
            Ok(entries) => entries,
            Err(err) => {
                println!(
                    "Error reading the templates directory ({}): {}",
                    templates::SAVE_TEMPLATES_DIR
                        .to_str()
                        .expect("Directory path should be a string"),
                    err
                );
                return;
            }
        };

        for entry in templates_dir_entries {
            let entry_path = entry.path();
            let entry_name = entry_path.file_name().expect("Should have a name");

            println!("{}", entry_name.to_str().expect("Should be a string"));
        }
    }

    fn show_usage() {
    }
}
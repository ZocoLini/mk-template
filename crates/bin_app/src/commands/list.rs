use crate::templates;
pub fn execute()
{
    let templates_dir_entries = templates::get_available_templates();
    
    for entry in templates_dir_entries
    {
        let entry = entry.expect("Should exists");
        let entry_path = entry.path();
        let entry_name = entry_path.file_name().expect("Should have a name");
        
        println!("{}", entry_name.to_str().expect("Should be a string"));
    }
}

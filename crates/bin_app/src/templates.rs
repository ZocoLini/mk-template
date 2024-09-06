mod data;
mod dir;
mod git;
mod txml;

use crate::templates::data::TemplateData;
use crate::templates::dir::DirTemplate;
use crate::templates::git::GitTemplate;
use crate::CONFIG_DIR;
use std::cell::LazyCell;
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::PathBuf;

pub const SAVE_TEMPLATES_DIR: LazyCell<PathBuf> = LazyCell::new(|| {
    let path = CONFIG_DIR.join("templates");

    if !path.exists() {
        fs::create_dir(&path).expect("Should create the templates dir.");
    }

    path
});

pub trait Template
{
    fn generate(&self, name: &str) -> Result<(), TemplateError>;
    fn save(&self, name: &str) -> Result<(), TemplateError>;
    fn remove(&self);
    fn validate(&self) -> bool;
}

#[derive(Debug)]
pub enum TemplateError
{
    IoError,
    InvalidTemplate,
    ErrorExecutingGit,
}

pub fn add_template(name: &str, path: &str)
{
    remove_template(name);

    let template = build_template(path);
    let template = match template {
        Ok(template) => template,
        Err(_) => {
            println!("Not supported template.");
            return;
        }
    };

    template.save(name).expect("Should save the template.");
}

fn build_template(path: &str) -> Result<Box<dyn Template>, TemplateError>
{
    if path.ends_with(".git") {
        return Ok(Box::new(GitTemplate::new(path)));
    }
    
    let path = PathBuf::from(path);

    if path.is_dir() {
        return Ok(Box::new(DirTemplate::new(path)))
    } 
    
    if path.is_file() {
        let local_txml_template = txml::TxmlTemplate::new(path);
        
        if local_txml_template.validate() { return Ok(Box::new(local_txml_template)); }
    }

    Err(TemplateError::InvalidTemplate)
}

pub fn remove_template(name: &str)
{
    let template_path = get_template_data_path(name);

    if let None = template_path {
        return;
    }

    let template_path = template_path.unwrap();
    
    if let Ok(template_data) =
        TemplateData::from_json(fs::read_to_string(&template_path).unwrap().as_str())
    {
        let template = template_data.to_template();
        let template = template.as_ref();

        template.remove();
        fs::remove_file(template_path).expect("Should remove the template.");
    } else {
        fs::remove_file(template_path).expect("Should remove the template.");
        println!("Template removed but data wasn't parseable. Any related files were not removed.");
    }
}

pub fn get_available_templates() -> Result<Vec<DirEntry>, io::Error>
{
    fs::read_dir(SAVE_TEMPLATES_DIR.as_path())
        .expect("Should exists")
        .filter(|entry| {
            let entry = entry.as_ref().expect("Should be a dir entry");
            let path = entry.path();
            let path = path.as_path();

            if !path.is_file() {
                return false;
            }

            let data = fs::read_to_string(path)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "Error reading the file."));
            let data = data.expect("Should read the file.");

            let template_data = TemplateData::from_json(data.as_str());

            if let Err(_) = template_data {
                return false;
            }

            return true;
        })
        .collect()
}

pub fn get_template_data_path(name: &str) -> Option<PathBuf>
{
    let path = PathBuf::from(SAVE_TEMPLATES_DIR.as_path().join(name));
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

pub fn get_template_data(name: &str) -> Option<TemplateData>
{
    if let Ok(template_data) = TemplateData::load(name) {
        Some(template_data)
    } else {
        None
    }
}

pub fn generate(name: &str, output_name: &str)
{
    let template_data = get_template_data(name);

    if let None = template_data {
        println!("Template {} not found.", name);
        return;
    }

    match template_data.unwrap().to_template().generate(output_name) {
        Err(TemplateError::ErrorExecutingGit) => println!("Error executing git. Check if it is installed."),
        Err(_) => println!("Error generating the template."),
        _ => ()
    }
}

fn is_valid_name(name: &str) -> bool
{
    !name.contains("/") && !name.contains("\\")
}

mod data;
mod dir;
mod git;
mod txml;

use crate::templates::data::TemplateData;
use crate::templates::dir::DirTemplate;
use crate::templates::git::GitTemplate;
use crate::CONFIG_DIR;
use std::cell::LazyCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::io;
use std::path::PathBuf;

pub const SAVE_TEMPLATES_DIR: LazyCell<PathBuf> = LazyCell::new(|| {
    let path = CONFIG_DIR.join("templates");

    if !path.exists() {
        fs::create_dir(&path).expect("Should create the templates dir.");
    }

    path
});

pub trait Template {
    fn generate(&self, name: &str, flags: HashMap<String, String>) -> Result<(), TemplateError>;
    fn save(&self, name: &str, flags: HashMap<String, String>) -> Result<(), TemplateError>;
    fn remove(&self);
    fn validate(&self) -> bool;
    fn get_description(&self) -> String;
}

pub enum TemplateError {
    IoError,
    InvalidTemplate,
    ErrorExecutingGit,
    ErrorConvertingDir2Txml,
    InvalidPath,
}

impl Debug for TemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateError::IoError => write!(f, "IO Error"),
            TemplateError::InvalidTemplate => write!(f, "Invalid Template"),
            TemplateError::ErrorExecutingGit => {
                write!(f, "Error executing git. Check if it is installed.")
            }
            TemplateError::InvalidPath => write!(f, "Invalid path."),
            TemplateError::ErrorConvertingDir2Txml => {
                write!(f, "Error converting directory to txml.")
            }
        }
    }
}

pub fn add_template(name: &str, path: &str, flags: HashMap<String, String>) {
    remove_template(name);

    let template = build_template(path);
    let template = match template {
        Ok(template) => template,
        Err(e) => {
            println!("Error adding the template: {:?}", e);
            return;
        }
    };

    template
        .save(name, flags)
        .expect("Should save the template.");
}

fn build_template(path: &str) -> Result<Box<dyn Template>, TemplateError> {
    if path.ends_with(".git") {
        return Ok(Box::new(GitTemplate::new(path)));
    }

    let path = PathBuf::from(path);

    if !path.exists() {
        return Err(TemplateError::InvalidPath);
    }

    if path.is_dir() {
        return Ok(Box::new(DirTemplate::new(path)));
    }

    if path.is_file() {
        let local_txml_template = txml::TxmlTemplate::new(path);

        if local_txml_template.validate() {
            return Ok(Box::new(local_txml_template));
        }
    }

    Err(TemplateError::InvalidTemplate)
}

pub fn remove_template(name: &str) {
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

pub fn get_available_templates() -> Vec<(TemplateData, String)> {
    fs::read_dir(SAVE_TEMPLATES_DIR.as_path())
        .expect("Should exists")
        .filter_map(|entry| {
            let entry = entry.as_ref().expect("Should be a dir entry");
            let path = entry.path();
            let path = path.as_path();

            if !path.is_file() {
                return None;
            }

            let name = path
                .file_name()
                .expect("Should have a name")
                .to_str()
                .expect("Should be a string")
                .to_string();

            let data = fs::read_to_string(path)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "Error reading the file."));
            let data = data.expect("Should read the file.");

            let template_data = TemplateData::from_json(data.as_str());

            match template_data {
                Ok(data) => Some((data, name)),
                Err(_) => None,
            }
        })
        .collect()
}

pub fn get_template_data_path(name: &str) -> Option<PathBuf> {
    let path = PathBuf::from(SAVE_TEMPLATES_DIR.as_path().join(name));
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

pub fn get_template_data(name: &str) -> Option<TemplateData> {
    if let Ok(template_data) = TemplateData::load(name) {
        Some(template_data)
    } else {
        None
    }
}

pub fn generate(name: &str, output_name: &str, flags: HashMap<String, String>) {
    let template_data = get_template_data(name);

    if let None = template_data {
        println!("Template {} not found.", name);
        return;
    }

    match template_data
        .unwrap()
        .to_template()
        .generate(output_name, flags)
    {
        Err(TemplateError::ErrorExecutingGit) => {
            println!("Error executing git. Check if it is installed.")
        }
        Err(_) => println!("Error generating the template."),
        _ => (),
    }
}

fn is_valid_name(name: &str) -> bool {
    !name.contains("/") && !name.contains("\\")
}

use crate::CONFIG_DIR;
use std::cell::LazyCell;
use std::fs;
use std::fs::{exists, DirEntry, ReadDir};
use std::io;
use std::path::{Path, PathBuf};

const SAVE_TEMPLATES_DIR: LazyCell<PathBuf> = LazyCell::new(|| {
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
    fn validate(&self) -> bool;
}

#[derive(Debug)]
pub enum TemplateError
{
    IoError,
    InvalidTemplate,
}

// region: DirTemplate

pub struct DirTemplate
{
    dir: PathBuf,
}

impl Template for DirTemplate
{
    fn generate(&self, name: &str) -> Result<(), TemplateError>
    {
        if name.contains("/") || name.contains("\\") { 
            return Err(TemplateError::InvalidTemplate);
        }
        
        let src = self.dir.as_path();
        let dst = PathBuf::from(name);
        
        copy_dir_all(src, &dst).map_err(|_e| TemplateError::IoError)
    }

    fn save(&self, name: &str) -> Result<(), TemplateError>
    {
        let src = self.dir.as_path();
        let dst = SAVE_TEMPLATES_DIR.as_path().join(name);
        let dst = dst.as_path();
        
        copy_dir_all(src, dst).map_err(|_e| TemplateError::IoError)
    }

    fn validate(&self) -> bool {
        true
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_all(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}

// endregion: DirTemplate

// region: Methods

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
    
    if !template.validate() {
        println!("Invalid template. Please, check the template.");
        return;
    }
    
    match template.save(name) { 
        Ok(_) => println!("Template added. Name: {}", name),
        Err(_) => println!("Error adding the template."),
    }
}

fn build_template(path: &str) -> Result<impl Template, TemplateError>
{
    let path = PathBuf::from(path);

    if path.is_dir() {
        Ok(DirTemplate { dir: path })
    } else {
        Err(TemplateError::InvalidTemplate)
    }
}

pub fn remove_template(name: &str)
{
    if let Some(template) = get_template(name) {
        fs::remove_dir(template.path()).expect("Should remove the template");
    }
}

pub fn exists_template(name: &str) -> bool
{
    match get_template(name) {
        Some(_) => true,
        None => false,
    }
}

pub fn get_available_templates() -> ReadDir
{
    fs::read_dir(SAVE_TEMPLATES_DIR.as_path()).expect("Should exists")
}

pub fn get_template(name: &str) -> Option<DirEntry>
{
    get_available_templates()
        .filter(|entry| {
            let entry = entry.as_ref().expect("Should exists");
            let entry_path = entry.path();
            let entry_name = entry_path.file_name().expect("Should have a name");

            entry_name.to_str().expect("Should be a string") == name
        })
        .collect::<Vec<Result<DirEntry, io::Error>>>()
        .pop()
        .and_then(|entry| entry.ok())
}

pub fn generate_template(name: &str, output_name: &str) {
    
    if !exists_template(name) { 
        println!("Template {} not found.", name);
        return;
    }
    
    let template_path = SAVE_TEMPLATES_DIR.as_path().join(name);
    
    let template = build_template(template_path.to_str().unwrap()).expect("Should build the template");
    
    match template.generate(output_name) {
        Ok(_) => println!("Template generated."),
        Err(_) => println!("Error generating the template."),
    }
}

// endregion: Methods

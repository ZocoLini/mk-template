use crate::templates::data::TemplateData;
use crate::templates::{is_valid_name, Template, TemplateError, SAVE_TEMPLATES_DIR};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};
use txml_processor::txml_structure::TxmlStructure;
use txml_processor::TxmlElement;
use crate::templates::txml::TXML_TEMPLATE;

pub const DIR_TEMPLATE: &str = "dir";

pub struct DirTemplate {
    dir: PathBuf,
}

impl DirTemplate {
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    pub fn save_as_dir(&self, name: &str) -> Result<(), TemplateError> {
        let src = self.dir.as_path();
        let dst = SAVE_TEMPLATES_DIR.as_path().join(name.to_string() + ".dir");
        let dst = dst.as_path();

        let result = copy_dir_all(src, dst).map_err(|_e| TemplateError::IoError);

        if result.is_err() {
            return result;
        }

        TemplateData::new(DIR_TEMPLATE, dst.to_str().expect("Should be an String"))
            .save(name)
            .map_err(|_e| {
                fs::remove_dir(dst).expect("Should remove the directory.");
                TemplateError::IoError
            })
    }

    pub fn save_as_fxml(&self, name: &str) -> Result<(), TemplateError> {
        let txml_content = TxmlStructure::from_path(&self.dir)
            .map_err(|_e| TemplateError::IoError)?.into_txml_element();
        
        let txml_file = SAVE_TEMPLATES_DIR.as_path().join(name.to_string() + ".txml");
        
        fs::write(&txml_file, txml_content).map_err(|_e| TemplateError::IoError)?;
        
        TemplateData::new(TXML_TEMPLATE, txml_file.to_str().expect("Should be an String"))
            .save(name)
            .map_err(|_e| {
                fs::remove_file(txml_file).expect("Should remove the file.");
                TemplateError::IoError
            })
    }
}

impl Template for DirTemplate {
    fn generate(&self, name: &str, _flags: HashMap<String, String>) -> Result<(), TemplateError> {
        if !is_valid_name(name) {
            return Err(TemplateError::InvalidTemplate);
        }

        let src = self.dir.as_path();
        let dst = PathBuf::from(name);

        copy_dir_all(src, &dst).map_err(|_e| TemplateError::IoError)
    }

    fn save(&self, name: &str, flags: HashMap<String, String>) -> Result<(), TemplateError> {
        if flags.contains_key("-as-dir") {
            self.save_as_dir(name)
        } else {
            self.save_as_fxml(name)
        }
    }

    fn remove(&self) {
        fs::remove_dir_all(&self.dir).expect("Should remove the directory.");
    }

    fn validate(&self) -> bool {
        self.dir.is_dir()
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

use crate::templates::data::TemplateData;
use crate::templates::{Template, TemplateError, SAVE_TEMPLATES_DIR};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};
use txml_processor::txml_structure::TxmlStructure;
use txml_processor::Instantiable;

pub const TXML_TEMPLATE: &str = "txml";

pub struct TxmlTemplate {
    txml_file: PathBuf,
}

impl TxmlTemplate {
    pub fn new(txml_file: PathBuf) -> Self {
        Self { txml_file }
    }
}

impl Template for TxmlTemplate {
    fn generate(&self, name: &str, _flags: HashMap<String, String>) -> Result<(), TemplateError> {
        let txml_structure = match TxmlStructure::from_txml_file(&self.txml_file) {
            Ok(txml) => txml,
            Err(e) => {
                println!("Error processing txml: {:?}", e);
                return Err(TemplateError::InvalidTemplate);
            }
        };

        txml_structure.instantiate_with_name(
            &env::current_dir().expect("Should exist a current dir."),
            name,
        );

        Ok(())
    }

    fn save(&self, name: &str, _flags: HashMap<String, String>) -> Result<(), TemplateError> {
        let src = self.txml_file.as_path();
        let dst = SAVE_TEMPLATES_DIR
            .as_path()
            .join(name.to_string() + ".txml");
        let dst = dst.as_path();

        let result = fs::copy(src, dst).map_err(|_e| TemplateError::IoError);

        if let Err(e) = result {
            return Err(e);
        }

        TemplateData::new(TXML_TEMPLATE, dst.to_str().expect("Should be an String"))
            .save(name)
            .map_err(|_e| {
                fs::remove_file(dst).expect("Should remove the file.");
                TemplateError::IoError
            })
    }

    fn remove(&self) {
        if let Err(e) = fs::remove_file(&self.txml_file) {
            println!("The TXML Template couldn't be deleted: {e:?}")
        }
    }

    fn validate(&self) -> bool {
        TxmlStructure::validate_txml_file(&self.txml_file)
    }

    fn get_description(&self) -> String {
        let txml_structure =
            TxmlStructure::from_txml_file(&self.txml_file).expect("Should be a valid TXML file.");

        let description = txml_structure.metadata().description.clone();

        if description.is_empty() { "TXML Template".to_string() } else { description }
    }
}

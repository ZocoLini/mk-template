use crate::templates::dir::DirTemplate;
use crate::templates::git::GitTemplate;
use crate::templates::{dir, git, txml, Template};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct TemplateData
{
    class: String,
    data_path: String,
}

impl TemplateData
{
    pub fn new(class: &str, data_path: &str) -> Self
    {
        Self {
            class: class.to_string(),
            data_path: data_path.to_string(),
        }
    }

    pub fn from_json(json: &str) -> serde_json::Result<Self>
    {
        serde_json::from_str(json)
    }

    pub fn save(&self, name: &str) -> Result<(), io::Error>
    {
        let json_data =
            serde_json::to_string_pretty(&self).expect("Should serialize the template.");
        let dst = crate::templates::SAVE_TEMPLATES_DIR.as_path().join(name);
        let dst = dst.as_path();

        std::fs::write(dst, json_data)
    }

    pub fn load(name: &str) -> Result<Self, io::Error>
    {
        let src = crate::templates::SAVE_TEMPLATES_DIR.as_path().join(name);
        let src = src.as_path();
        let data = std::fs::read_to_string(src)?;

        let template_data: TemplateData = Self::from_json(data.as_str())?;

        Ok(template_data)
    }
    
    pub fn to_template(&self) -> Box<dyn Template>
    {
        match self.class.as_str() {
            dir::DIR_TEMPLATE => Box::new(DirTemplate::new(PathBuf::from(self.data_path.as_str()))),
            git::GIT_TEMPLATE => Box::new(GitTemplate::new(self.data_path.as_str())),
            txml::TXML_TEMPLATE => Box::new(txml::TxmlTemplate::new(PathBuf::from(self.data_path.as_str()))),
            _ => panic!("Invalid template class."),
        }
    }
}

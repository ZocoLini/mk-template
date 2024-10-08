use crate::templates::data::TemplateData;
use crate::templates::{is_valid_name, Template, TemplateError, SAVE_TEMPLATES_DIR};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process;

pub const GIT_TEMPLATE: &str = "git";

pub struct GitTemplate {
    url: String,
}

impl GitTemplate {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}

impl Template for GitTemplate {
    fn generate(&self, name: &str, _flags: HashMap<String, String>) -> Result<(), TemplateError> {
        if !is_valid_name(name) {
            return Err(TemplateError::InvalidTemplate);
        }

        let mut process = match process::Command::new("git")
            .args(&["clone", &self.url, name])
            .spawn()
        {
            Ok(process) => process,
            Err(_) => return Err(TemplateError::ErrorExecutingGit),
        };

        process.wait().expect("Failed to wait for the process.");
        Ok(())
    }

    fn save(&self, name: &str, _flags: HashMap<String, String>) -> Result<(), TemplateError> {
        let template_data = TemplateData::new(GIT_TEMPLATE, &self.url);
        let json_data =
            serde_json::to_string_pretty(&template_data).expect("Should serialize the template.");
        let dst = SAVE_TEMPLATES_DIR.as_path().join(name);
        let dst = dst.as_path();

        std::fs::write(dst, json_data).expect("Should write the template.");
        Ok(())
    }

    fn remove(&self) {
        // Nothing to do here
    }

    fn validate(&self) -> bool {
        true
    }

    fn get_description(&self) -> String {
        let file_path = PathBuf::from(&self.url);

        if file_path.is_dir() {
            return "A git template from a reachable directory.".to_string();
        }

        get_git_template_description(&self.url)
            .unwrap_or_else(|| "Could not obtain the description.".to_string())
    }
}

fn get_git_template_description(url: &str) -> Option<String> {
    
    if url.starts_with("https://github") || url.starts_with("http://github") { 
        // TODO: Implement calling the GitHub API to get the description
    }
    
    None
}


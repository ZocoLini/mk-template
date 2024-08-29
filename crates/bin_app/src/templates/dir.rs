use crate::templates::data::TemplateData;
use crate::templates::{is_valid_name, Template, TemplateError, SAVE_TEMPLATES_DIR};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub const DIR_TEMPLATE: &str = "dir";

pub struct DirTemplate
{
    dir: PathBuf,
}

impl DirTemplate
{
    pub fn new(dir: PathBuf) -> Self
    {
        Self { dir }
    }
}

impl Template for DirTemplate
{
    fn generate(&self, name: &str) -> Result<(), TemplateError>
    {
        if !is_valid_name(name) {
            return Err(TemplateError::InvalidTemplate);
        }

        let src = self.dir.as_path();
        let dst = PathBuf::from(name);

        copy_dir_all(src, &dst).map_err(|_e| TemplateError::IoError)
    }

    fn save(&self, name: &str) -> Result<(), TemplateError>
    {
        let src = self.dir.as_path();
        let dst = SAVE_TEMPLATES_DIR.as_path().join(name.to_string() + ".dir");
        let dst = dst.as_path();

        let result = copy_dir_all(src, dst).map_err(|_e| TemplateError::IoError);

        if result.is_err() {
            return result;
        }

        TemplateData::new(
            DIR_TEMPLATE,
            dst.to_str().expect("Should be an String"),
        )
        .save(name)
        .map_err(|_e| {
            fs::remove_dir(dst).expect("Should remove the directory.");
            TemplateError::IoError
        })
    }

    fn remove(&self) {
        fs::remove_dir(&self.dir).expect("Should remove the directory.");
    }

    fn into_data(&self) -> TemplateData
    {
        TemplateData::new(DIR_TEMPLATE, self.dir.to_str().expect("Should be a string"))
    }

    fn validate(&self) -> bool
    {
        true
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()>
{
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

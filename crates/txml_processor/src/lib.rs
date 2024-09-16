use quick_xml::events::attributes::Attribute;
use std::path::PathBuf;
use std::io;

pub mod fs_elements;
pub mod txml_structure;
mod commands;

pub trait AttributeHandler {
    fn process_attribute(&mut self, attribute: Attribute);
}

pub trait Instantiable {
    fn instantiate(&self, dir: &PathBuf);

    fn instantiate_with_name(&self, dir: &PathBuf, _name: &str);
}

pub trait FsElement {
    fn from_path(path: &PathBuf) -> Result<Self, io::Error> where Self: Sized;
}

pub trait TxmlElement {
    fn into_txml_element(self) -> String;
}
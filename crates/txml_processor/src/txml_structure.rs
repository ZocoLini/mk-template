use crate::fs_elements::{Directory, File};
use crate::{AttributeHandler, FsElement, Instantiable, TxmlElement};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, io};

#[derive(Debug)]
pub enum TxmlProcessorError {
    InvalidDirectory,
    UnknownParseError,
    InvalidTag,
}

pub struct TxmlStructure {
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl TxmlStructure {
    pub fn new() -> TxmlStructure {
        TxmlStructure {
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    pub fn from_txml_file(txml: &PathBuf) -> Result<TxmlStructure, TxmlProcessorError> {
        if !txml.exists() {
            return Err(TxmlProcessorError::InvalidDirectory);
        }
        if !txml.is_file() {
            return Err(TxmlProcessorError::InvalidDirectory);
        }

        let txml_content = fs::read_to_string(txml).expect("Error reading file");

        Self::from_str(txml_content.as_str())
    }

    pub fn from_path(path: &PathBuf) -> Result<TxmlStructure, io::Error> {
        let mut txml_structure = TxmlStructure::new();

        if path.is_dir() {
            txml_structure.add_directory(Directory::from_path(path)?)
        } else {
            txml_structure.add_file(File::from_path(path)?)
        }

        Ok(txml_structure)
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn add_directory(&mut self, directory: Directory) {
        self.directories.push(directory);
    }
}

impl TxmlElement for TxmlStructure {
    fn into_txml_element(self) -> String {
        let mut txml_content = String::from(
            r#"<?xml version="1.0" encoding="UTF-8" ?>

<Root xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xsi:noNamespaceSchemaLocation="https://lebastudios.org/xml-schemas/txml_schema.xsd">
        "#,
        );

        for file in self.files {
            txml_content += file.into_txml_element().as_str();
        }

        for directory in self.directories {
            txml_content += directory.into_txml_element().as_str();
        }

        txml_content += "</Root>";

        txml_content
    }
}

impl FromStr for TxmlStructure {
    type Err = TxmlProcessorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut txml_structure = TxmlStructure::new();

        let mut reader = Reader::from_str(s);
        let mut event_buf = Vec::new();

        let mut dir_queue: VecDeque<Directory> = VecDeque::new();
        let mut current_file: Option<File> = None;

        loop {
            match reader.read_event_into(&mut event_buf) {
                Ok(Event::Start(ref e)) => match e.name().0 {
                    b"Root" => continue,
                    b"Directory" => {
                        let mut directory = Directory::new();

                        e.attributes().for_each(|attr| {
                            directory.process_attribute(attr.expect("Error reading attribute"));
                        });

                        dir_queue.push_back(directory);
                    }
                    b"File" => {
                        current_file = Some(File::new());

                        e.attributes().for_each(|attr| {
                            current_file
                                .as_mut()
                                .unwrap()
                                .process_attribute(attr.expect("Error reading attribute"));
                        });
                    }
                    _ => return Err(TxmlProcessorError::InvalidTag),
                },
                Ok(Event::Empty(e)) => match e.name().0 {
                    b"Root" => continue,
                    b"Directory" => {
                        let mut directory = Directory::new();
                        e.attributes().for_each(|attr| {
                            directory.process_attribute(attr.expect("Error reading attribute"));
                        });

                        if dir_queue.is_empty() {
                            txml_structure.add_directory(directory)
                        } else {
                            dir_queue
                                .back_mut()
                                .expect("Shouldn't be empty")
                                .add_directory(directory)
                        }
                    }
                    b"File" => {
                        let mut file = File::new();
                        e.attributes().for_each(|attr| {
                            file.process_attribute(attr.expect("Error reading attribute"));
                        });

                        if dir_queue.is_empty() {
                            txml_structure.add_file(file)
                        } else {
                            dir_queue
                                .back_mut()
                                .expect("Shouldn't be empty")
                                .add_file(file)
                        }
                    }
                    _ => return Err(TxmlProcessorError::InvalidTag),
                },
                Ok(Event::Text(e)) => {
                    if let Some(ref mut file) = current_file {
                        let content = String::from_utf8_lossy(&e).to_string();
                        if content.replace(" ", "").is_empty() {
                            continue;
                        }

                        file.set_text(content);
                    }
                }
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"Root" => break,
                    b"Directory" => {
                        if dir_queue.len() == 1 {
                            txml_structure.add_directory(dir_queue.pop_back().unwrap())
                        } else {
                            let directory = dir_queue.pop_back().unwrap();
                            dir_queue
                                .back_mut()
                                .expect("Shouldn't be empty")
                                .add_directory(directory);
                        }
                    }
                    b"File" => {
                        let file = current_file.take().expect("Shouldn't be empty");

                        if dir_queue.is_empty() {
                            txml_structure.add_file(file)
                        } else {
                            dir_queue
                                .back_mut()
                                .expect("Shouldn't be empty")
                                .add_file(file)
                        }

                        current_file = None;
                    }
                    _ => return Err(TxmlProcessorError::InvalidTag),
                },
                Ok(Event::Eof) => break,
                Err(_e) => return Err(TxmlProcessorError::UnknownParseError),
                _ => (),
            }

            event_buf.clear();
        }

        Ok(txml_structure)
    }
}

impl Instantiable for TxmlStructure {
    fn instantiate(&self, dir: &PathBuf) {
        self.files.iter().for_each(|file| file.instantiate(&dir));

        self.directories
            .iter()
            .for_each(|directory| directory.instantiate(dir));
    }

    fn instantiate_with_name(&self, dir: &PathBuf, name: &str) {
        if self.files.len() + self.directories.len() > 1 {
            self.instantiate(dir);
            return;
        }

        if self.files.len() == 1 {
            self.files[0].instantiate_with_name(dir, name);
            return;
        }

        if self.directories.len() == 1 {
            self.directories[0].instantiate_with_name(dir, name);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::txml_structure::TxmlStructure;
    use std::str::FromStr;

    #[test]
    fn process_txml_test_from_str() {
        if let Err(e) = TxmlStructure::from_str(
            r#"
            <?xml version="1.0" encoding="UTF-8" ?>

<Root xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xsi:noNamespaceSchemaLocation="https://lebastudios.org/xml-schemas/txml_schema.xsd">
    <Directory name="crates" in_command="git init   ;      mkdir pepe">
        <File name="Hola" extension="rs">
            fn main()
            {
                println!("Hola, mundo!");
            }
        </File>
        <Directory name="crate">
            <File name="sin_titulo" extension="txt">
                Hola me llamo Juan
                Pepe
            </File>
        </Directory>
    </Directory>

    <File name="Cargo" extension="toml">
        [workspace]
        resolver = "2"
        members = []
    </File>
    <File name="rustfmt" extension="toml"/>
    <File name=".gitignore"/>
</Root>
            "#,
        ) {
            panic!("Error: {:?}", e);
        }
    }
}

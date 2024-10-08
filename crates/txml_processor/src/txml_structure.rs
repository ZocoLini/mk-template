use crate::reader::{ElementState, TxmlEvent, TxmlReader, TxmlReaderError};
use crate::txml_elements::{Directory, File, TemplateMetadata, Variable};
use crate::{AttributeHandler, FsElement, Instantiable, TxmlElement};
use quick_xml::events::attributes::Attribute;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, io};

#[derive(Debug)]
pub enum TxmlProcessorError {
    InvalidDirectory,
    BinaryFileError,
    UnknownParseError,
    InvalidTag,
}

pub struct TxmlStructure {
    files: Vec<File>,
    directories: Vec<Directory>,
    metadata: TemplateMetadata,
    renamable: bool,
}

impl TxmlStructure {
    pub fn new() -> TxmlStructure {
        TxmlStructure {
            files: Vec::new(),
            directories: Vec::new(),
            metadata: TemplateMetadata::new(),
            renamable: true,
        }
    }

    pub fn metadata(&self) -> &TemplateMetadata {
        &self.metadata
    }

    pub fn validate_txml_file(txml: &PathBuf) -> bool {
        if !txml.exists() {
            return false;
        }
        if !txml.is_file() {
            return false;
        }

        let txml_content = fs::read_to_string(txml);

        match txml_content {
            Err(_) => false,
            Ok(content) => Self::validate_txml_str(content.as_str())
        }
    }
    
    pub fn validate_txml_str(txml: &str) -> bool {
        let mut reader = TxmlReader::from_str(txml);
        
        loop {
            match reader.read_event() {
                Err(_e) => return false,
                Ok(TxmlEvent::Eof) => return true,
                _ => continue,
            }
        }
    }
    
    pub fn from_txml_file(txml: &PathBuf) -> Result<TxmlStructure, TxmlProcessorError> {
        if !txml.exists() {
            return Err(TxmlProcessorError::InvalidDirectory);
        }
        if !txml.is_file() {
            return Err(TxmlProcessorError::InvalidDirectory);
        }

        let txml_content = fs::read_to_string(txml).map_err(|_| TxmlProcessorError::BinaryFileError)?;

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

    pub fn obtain_variables(fxml: &str) -> Result<Vec<Variable>, TxmlProcessorError>
    {
        let mut variables = Vec::new();

        let mut reader = TxmlReader::from_str(fxml);

        loop {
            match reader.read_event() {
                Ok(TxmlEvent::Variable(state)) => match state {
                    ElementState::Start(bytes) | ElementState::Empty(bytes) => {
                        let mut variable = Variable::new();
                        
                        bytes.attributes().for_each(|attr| {
                            variable.process_attribute(attr.expect("Error reading attribute"))
                        });
                        
                        variables.push(variable);
                    }
                    _ => continue,
                }
                Ok(TxmlEvent::Eof) => break,
                Err(_e) => return Err(TxmlProcessorError::UnknownParseError),
                _ => continue,
            }
        }
        
        Ok(variables)
    }
    
    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn add_directory(&mut self, directory: Directory) {
        self.directories.push(directory);
    }
}

impl AttributeHandler for TxmlStructure {
    fn process_attribute(&mut self, attr: Attribute) {
        match attr.key.0 {
            b"renamable" => self.renamable = String::from_utf8_lossy(&attr.value).to_string() == "true",
            _ => (),
        }
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
        
        txml_content += self.metadata.into_txml_element().as_str();
        
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

        let vars = Self::obtain_variables(s)?;
        let mut s = s.to_string();
        
        for var in vars {
            let value = if var.get_value().is_empty() {
                println!("Please, introduce the value for the variable '{}'", var.get_name());
                let mut value = String::new();
                io::stdin().read_line(&mut value).expect("Error reading from stdin");
                value.trim().to_string()
            } else {
                var.get_value().to_string()
            };
            
            let variable_expression = format!("${{{}}}", var.get_name());
            s = s.replace(variable_expression.as_str(), value.as_str());
        }
        
        let mut reader = TxmlReader::from_str(&s);

        let mut dir_queue: VecDeque<Directory> = VecDeque::new();
        let mut current_file: Option<File> = None;

        loop {
            match reader.read_event() {
                Ok(TxmlEvent::Root(state)) => match state {
                    ElementState::Start(bytes) => {
                        bytes.attributes().for_each(|attr| {
                            txml_structure.process_attribute(attr.expect("Error reading attribute"))
                        });
                    }
                    ElementState::End => break,
                    _ => continue,
                }
                Ok(TxmlEvent::Metadata(state)) => match state {
                    ElementState::Start(bytes) | ElementState::Empty(bytes) => {
                        bytes.attributes().for_each(|attr| {
                            txml_structure.metadata.process_attribute(attr.expect("Error reading attribute"))
                        });
                    }
                    _ => continue,
                }
                Ok(TxmlEvent::Variable(_)) => continue,
                Ok(TxmlEvent::Directory(state)) => match state { 
                    ElementState::Start(bytes) => {
                        let mut directory = Directory::new();

                        bytes.attributes().for_each(|attr| {
                            directory.process_attribute(attr.expect("Error reading attribute"));
                        });

                        dir_queue.push_back(directory);
                    }
                    ElementState::Empty(bytes) => {
                        let mut directory = Directory::new();
                        bytes.attributes().for_each(|attr| {
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
                    ElementState::End => {
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
                }
                Ok(TxmlEvent::File(state)) => match state {
                    ElementState::Start(bytes) => {
                        current_file = Some(File::new());

                        bytes.attributes().for_each(|attr| {
                            current_file
                                .as_mut()
                                .unwrap()
                                .process_attribute(attr.expect("Error reading attribute"));
                        });
                    }
                    ElementState::Empty(bytes) => {
                        let mut file = File::new();
                        bytes.attributes().for_each(|attr| {
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
                    ElementState::End => {
                        let file = current_file.take().expect("Shouldn't be empty");

                        if dir_queue.is_empty() {
                            txml_structure.add_file(file)
                        } else {
                            dir_queue
                                .back_mut()
                                .expect("Shouldn't be empty")
                                .add_file(file)
                        }
                    }
                }
                Ok(TxmlEvent::Text(e)) => {
                    if let Some(ref mut file) = current_file {
                        let content = String::from_utf8_lossy(&e).to_string();
                        if content.replace(" ", "").is_empty() {
                            continue;
                        }

                        file.set_text(content);
                    }
                }
                Ok(TxmlEvent::Eof) => break,
                Ok(TxmlEvent::Comment(_)) => continue, 
                Ok(TxmlEvent::Declaration(_)) => continue,
                Err(TxmlReaderError::UnknownError) => return Err(TxmlProcessorError::UnknownParseError),
                Err(TxmlReaderError::UnexpectedElement) => continue, 
                Err(TxmlReaderError::UnsupportedEncoding) => continue,
            }
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
        if self.files.len() + self.directories.len() > 1 || !self.renamable {
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
    
    #[test]
    fn txml_variables_replacement_test() {
        let txml = r#"
<?xml version="1.0" encoding="UTF-8" ?>

<Root xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xsi:noNamespaceSchemaLocation="https://lebastudios.org/xml-schemas/txml_schema.xsd">
    <Variable name="VAR1" value="folder1"/>
    <Variable name="VAR2" value="file1"/>
    <Directory name="${VAR1}" >
        <File name="${VAR2}" extension="txt" >
            ${VAR1} content
        </File>
    </Directory>
</Root>
        "#;
        
        let txml_variables = TxmlStructure::obtain_variables(txml).unwrap();
        
        assert_eq!(txml_variables.len(), 2);
        assert_eq!(txml_variables[0].get_name(), "VAR1");
        assert_eq!(txml_variables[0].get_value(), "folder1");
        assert_eq!(txml_variables[1].get_name(), "VAR2");
        assert_eq!(txml_variables[1].get_value(), "file1");
    }
}

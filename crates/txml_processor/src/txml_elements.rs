use crate::{commands, AttributeHandler, FsElement, Instantiable, TxmlElement};
use quick_xml::events::attributes::Attribute;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};
// region: Directory

pub struct Directory {
    name: String,
    out_command: String,
    in_command: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            name: String::new(),
            out_command: String::new(),
            in_command: String::new(),
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn add_directory(&mut self, directory: Directory) {
        self.directories.push(directory);
    }
}

impl TxmlElement for Directory {
    fn into_txml_element(self) -> String {
        let mut result = format!("<Directory name=\"{}\"", self.name);
        
        if !self.in_command.is_empty() {
            result.push_str(&format!(" in_command=\"{}\"", self.in_command));
        }
        
        if !self.out_command.is_empty() {
            result.push_str(&format!(" out_command=\"{}\"", self.out_command));
        }
        
        result.push_str(">\n");
        
        for file in self.files {
            result.push_str(&file.into_txml_element());
        }
        
        for directory in self.directories {
            result.push_str(&directory.into_txml_element());
        }
        
        result.push_str("</Directory>\n");
        
        result
    }
}

impl Instantiable for Directory {
    fn instantiate(&self, dir: &PathBuf) {
        self.instantiate_with_name(dir, self.name.as_str());
    }

    fn instantiate_with_name(&self, dir: &PathBuf, name: &str) {
        let dir_name = format!("{}", name);
        let new_path_buff = dir.join(&dir_name);

        if new_path_buff.exists() {
            println!("Directory {} already exist. Skipping creation.", dir_name);
            return;
        }

        fs::DirBuilder::new()
            .create(&new_path_buff)
            .expect("Error creating directory");

        if !self.in_command.is_empty() {
            let command_execution = commands::execute_commands(&self.in_command, &new_path_buff);

            match command_execution {
                Err(e) => println!("File {} created but the command failed: {e:?}", dir_name),
                _ => (),
            }
        }

        if !self.out_command.is_empty() {
            let command_execution = commands::execute_commands(&self.out_command, dir);

            match command_execution {
                Err(e) => println!("File {} created but the command failed: {e:?}", dir_name),
                _ => (),
            }
        }

        self.files
            .iter()
            .for_each(|file| file.instantiate(&new_path_buff));

        self.directories
            .iter()
            .for_each(|directory| directory.instantiate(&new_path_buff));
    }
}

impl AttributeHandler for Directory {
    fn process_attribute(&mut self, attribute: Attribute) {
        match attribute.key.0 {
            b"name" => {
                self.name = String::from_utf8_lossy(&attribute.value).to_string();
            }
            b"in_command" => {
                self.in_command = String::from_utf8_lossy(&attribute.value).to_string()
            }
            b"out_command" => {
                self.out_command = String::from_utf8_lossy(&attribute.value).to_string()
            }
            _ => println!(
                "Unknown attribute for Directory: {}",
                String::from_utf8_lossy(attribute.key.0)
            ),
        }
    }
}

impl FsElement for Directory {
    fn from_path(dir: &PathBuf) -> Result<Directory, io::Error> {
        if !dir.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Directory not found",
            ));
        }

        let mut dir_element = Directory {
            name: dir.file_name().expect("Should have a name").to_str().unwrap().to_string(),
            out_command: String::from(""),
            in_command: String::from(""),
            files: Vec::new(),
            directories: Vec::new(),
        };

        for entry in dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() { dir_element.files.push(File::from_path(&path)?) }
            else { dir_element.directories.push(Directory::from_path(&path)?) }
        }

        Ok(dir_element)
    }
}

// endregion: Directory

// region: File

pub struct File {
    name: String,
    extension: String,
    command: String,
    content: String,
}

impl File {
    pub fn new() -> File {
        File {
            name: String::new(),
            extension: String::new(),
            command: String::new(),
            content: String::new(),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.content = text;
    }
}

impl TxmlElement for File {
    fn into_txml_element(self) -> String {
        let mut result = format!("<File name=\"{}\"", self.name);
        
        if !self.extension.is_empty() { 
            result.push_str(&format!(" extension=\"{}\"", self.extension));
        }
        
        if !self.command.is_empty() {
            result.push_str(&format!(" command=\"{}\"", self.command));
        }
        
        result.push_str(">\n");
        
        result.push_str(&reverse_escape_xml(&self.content));
        
        result.push_str("\n");
        
        result.push_str("</File>\n");
        
        result
    }
}

impl Instantiable for File {
    fn instantiate(&self, dir: &PathBuf) {
        self.instantiate_with_name(dir, self.name.as_str());
    }

    fn instantiate_with_name(&self, dir: &PathBuf, name: &str) {
        let file_name = if self.extension.is_empty() {
            format!("{}", name)
        } else {
            format!("{}.{}", name, self.extension)
        };
        let new_path_buff = dir.join(&file_name);

        if new_path_buff.exists() {
            println!("File {} already exist. Skipping creation.", file_name);
            return;
        }

        let content = remove_indentation(&self.content);
        let content = escape_xml(&content);

        fs::File::create(&new_path_buff)
            .expect("Error creating file")
            .write_all(content.as_bytes())
            .expect("Error writing to file");

        if !self.command.is_empty() {
            let command_execution = commands::execute_commands(&self.command, dir);

            match command_execution {
                Err(e) => println!("File {} created but the command failed: {e:?}", file_name),
                _ => (),
            }
        }
    }
}

impl FsElement for File {
    fn from_path(path: &PathBuf) -> Result<Self, io::Error>
    {
        if !path.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "File not found",
            ));
        }

        let file_element = File {
            name: path.file_stem().expect("Should have a name").to_str().unwrap().to_string(),
            extension: path.extension()
                           .and_then(|ext| ext.to_str())
                           .unwrap_or("")
                           .to_string(),
            command: String::from(""),
            content: fs::read_to_string(path).map_err(|_| io::Error::new(io::ErrorKind::Other, "Error reading file"))?,
        };

        Ok(file_element)
    }
}

impl AttributeHandler for File {
    fn process_attribute(&mut self, attribute: Attribute) {
        match attribute.key.0 {
            b"name" => {
                self.name = String::from_utf8_lossy(&attribute.value).to_string();
            }
            b"extension" => {
                self.extension = String::from_utf8_lossy(&attribute.value).to_string();
            }
            b"command" => {
                self.command = String::from_utf8_lossy(&attribute.value).to_string();
            }
            _ => println!(
                "Unknown attribute for File: {}",
                String::from_utf8_lossy(attribute.key.0)
            ),
        }
    }
}

fn escape_xml(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

fn reverse_escape_xml(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

fn remove_indentation(text: &str) -> String {
    let mut result = String::new();

    if is_blank(text) {
        return result;
    }

    let text_lines: Vec<&str> = text.lines().skip(1).collect();

    if text_lines.len() == 1 {
        return text.trim_start().to_string();
    }

    let indentation = get_indentation(&text_lines);

    // Removing the spaces from the beginning of each line
    for line in text.lines().skip(1) {
        if is_blank(line) {
            result.push_str("\n");
            continue;
        }

        result.push_str(&line[indentation..]);
        result.push_str("\n");
    }

    result.pop();
    result
}

fn count_spaces(text: &str) -> usize {
    let mut count = 0;
    for c in text.chars() {
        if c.is_whitespace() {
            count += 1;
        } else {
            break;
        }
    }
    count
}

fn get_indentation(text_lines: &Vec<&str>) -> usize {
    let mut indentation = usize::MAX;
    for line in text_lines.iter() {
        if is_blank(line) {
            continue;
        }

        let line_indentation = count_spaces(*line);
        if line_indentation < indentation {
            indentation = line_indentation;
        }
    }
    indentation
}

fn is_blank(text: &str) -> bool {
    text.chars().all(char::is_whitespace)
}

// endregion: File

// region: Variable

pub struct Variable {
    name: String,
    value: String,
}

impl Variable {
    pub fn new() -> Variable {
        Variable {
            name: String::new(),
            value: String::new(),
        }
    }
    
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    
    pub fn get_value(&self) -> &str {
        self.value.as_str()
    }
}

impl AttributeHandler for Variable {
    fn process_attribute(&mut self, attribute: Attribute) {
        match attribute.key.0 {
            b"name" => {
                self.name = String::from_utf8_lossy(&attribute.value).to_string();
            }
            b"value" => {
                self.value = String::from_utf8_lossy(&attribute.value).to_string()
            }
            _ => println!(
                "Unknown attribute for Variable: {}",
                String::from_utf8_lossy(attribute.key.0)
            ),
        }
    }
}

// endregion: Variable

// region: TxmlMetadata

pub struct TemplateMetadata {
    pub author: String,
    pub date: String,
    pub version: String,
    pub description: String,
}

impl TemplateMetadata {
    pub fn new() -> TemplateMetadata {
        TemplateMetadata {
            author: String::new(),
            date: String::new(),
            version: String::new(),
            description: String::new(),
        }
    }
}

impl AttributeHandler for TemplateMetadata {
    fn process_attribute(&mut self, attribute: Attribute) {
        match attribute.key.0 {
            b"author" => self.author = String::from_utf8_lossy(&attribute.value).to_string(),
            b"date" => self.date = String::from_utf8_lossy(&attribute.value).to_string(),
            b"version" => self.version = String::from_utf8_lossy(&attribute.value).to_string(),
            b"description" => self.description = String::from_utf8_lossy(&attribute.value).to_string(),
            _ => (),
        }
    }
}

impl TxmlElement for TemplateMetadata {
    fn into_txml_element(self) -> String {
        format!(
            r#"<Metadata author="{}" date="{}" version="{}" description="{}"/>
            "#,
            self.author, self.date, self.version, self.description
        )
    }
}

// endregion: TxmlMetadata

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::txml_elements::{Directory, TemplateMetadata};
    use crate::txml_structure::TxmlStructure;
    use crate::TxmlElement;

    #[test]
    fn dir_into_txml_format_test()
    {
        let dir = Directory {
            name: String::from("pepe"),
            out_command: String::from("git init"),
            in_command: String::from("ls -l"),
            files: Vec::new(),
            directories: Vec::new(),
        };
        
        let txml = dir.into_txml_element();
        
        assert_eq!(txml, "<Directory name=\"pepe\" in_command=\"ls -l\" out_command=\"git init\">\n</Directory>\n");
    }
    
    #[test]
    fn file_into_txml_format_test()
    {
        let file = crate::txml_elements::File {
            name: String::from("pepe"),
            extension: String::from("rs"),
            command: String::from("cargo build"),
            content: String::from("fn main() { println!(\"Hola, mundo!\"); }"),
        };
        
        let txml = file.into_txml_element();
        
        assert_eq!(txml, "<File name=\"pepe\" extension=\"rs\" command=\"cargo build\">\nfn main() { println!(&quot;Hola, mundo!&quot;); }\n</File>\n");
    }
    
    #[test]
    fn txml_structure_into_txml_format_test()
    {
        let mut txml_structure = TxmlStructure::new();

        let mut dir = Directory {
            name: String::from("pepe"),
            out_command: String::from("git init"),
            in_command: String::from("ls -l"),
            files: Vec::new(),
            directories: Vec::new(),
        };

        let file = crate::txml_elements::File {
            name: String::from("pepe"),
            extension: String::from("rs"),
            command: String::from("cargo build"),
            content: String::from("fn main() { println!(\"Hola, mundo!\"); }"),
        };
        
        dir.add_file(file);
        
        txml_structure.add_directory(dir);
        
        let file = crate::txml_elements::File {
            name: String::from("pepa"),
            extension: String::from("rs"),
            command: String::from("cargo build"),
            content: String::from("fn main() { println!(\"Hola, mundo!\"); }"),
        };
        
        txml_structure.add_file(file);
        
        let txml_string = txml_structure.into_txml_element();
        
        assert!(TxmlStructure::from_str(txml_string.as_str()).is_ok());
    }

    #[test]
    fn metadata_into_txml_format_test()
    {
        let expected = r#"
        <Metadata author="Borja Castellano" date="22/09/2024" version="1.0.0" description="Testing metadata info"/>
        "#;

        let metadata = TemplateMetadata {
            author: String::from("Borja Castellano"),
            date: String::from("22/09/2024"),
            version: String::from("1.0.0"),
            description: String::from("Testing metadata info"),
        }.into_txml_element();

        assert_eq!(expected.trim(), metadata.trim());
    }

    #[test]
    fn txml_metadata_parser_test() 
    {
        let txml = r#"
<?xml version="1.0" encoding="UTF-8" ?>

<Root xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xsi:noNamespaceSchemaLocation="https://lebastudios.org/xml-schemas/txml_schema.xsd">
    <Metadata author="Borja Castellano" date="22/09/2024" version="1.0.0" description="Testing metadata info"/>
</Root>        
        "#;

        let txml_structure = TxmlStructure::from_str(txml).unwrap();

        assert_eq!(txml_structure.metadata().author, "Borja Castellano");
        assert_eq!(txml_structure.metadata().date, "22/09/2024");
        assert_eq!(txml_structure.metadata().version, "1.0.0");
        assert_eq!(txml_structure.metadata().description, "Testing metadata info");
    }
}
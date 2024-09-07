use crate::objects::CommandError::{CommandFailed, CreationError, InvalidInput};
use quick_xml::events::attributes::Attribute;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
// region: Generics

pub enum CommandError
{
    CommandFailed,
    InvalidInput,
    CreationError,
}

impl Debug for CommandError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self {
            CommandFailed => write!(f, "Command execution failed."),
            InvalidInput => write!(f, "Command input was invalid."),
            CreationError => write!(f, "Command creation failed."),
        }
    }
}

pub trait AttributeHandler
{
    fn process_attribute(&mut self, attribute: Attribute);
}

pub trait Instantiable
{
    fn instantiate(&self, dir: &PathBuf);

    fn instantiate_with_name(&self, dir: &PathBuf, _name: &str);
}

// endregion: Generics

// region: Directory

pub struct Directory
{
    name: String,
    out_command: String,
    in_command: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl Directory
{
    pub fn new() -> Directory
    {
        Directory {
            name: String::new(),
            out_command: String::new(),
            in_command: String::new(),
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file: File)
    {
        self.files.push(file);
    }

    pub fn add_directory(&mut self, directory: Directory)
    {
        self.directories.push(directory);
    }
}

impl Instantiable for Directory
{
    fn instantiate(&self, dir: &PathBuf)
    {
        self.instantiate_with_name(dir, self.name.as_str());
    }

    fn instantiate_with_name(&self, dir: &PathBuf, name: &str) 
    {
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
            let command_execution = execute_commands(&self.in_command, &new_path_buff);

            match command_execution {
                Err(e) => println!("File {} created but the command failed: {e:?}", dir_name),
                _ => (),
            }
        }

        if !self.out_command.is_empty() {
            let command_execution = execute_commands(&self.out_command, dir);

            match command_execution {
                Err(e) => println!("File {} created but the command failed: {e:?}", dir_name),
                _ => (),
            }
        }

        self.files.iter().for_each(|file| file.instantiate(&new_path_buff));

        self.directories
            .iter()
            .for_each(|directory| directory.instantiate(&new_path_buff));
    }
}

impl AttributeHandler for Directory
{
    fn process_attribute(&mut self, attribute: Attribute)
    {
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

// endregion: Directory

// region: File

pub struct File
{
    name: String,
    extension: String,
    command: String,
    content: String,
}

impl File
{
    pub fn new() -> File
    {
        File {
            name: String::new(),
            extension: String::new(),
            command: String::new(),
            content: String::new(),
        }
    }

    pub fn set_text(&mut self, text: String)
    {
        self.content = text;
    }
}

impl Instantiable for File
{
    fn instantiate(&self, dir: &PathBuf)
    {
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
            let command_execution = execute_commands(&self.command, dir);

            match command_execution {
                Err(e) => println!("File {} created but the command failed: {e:?}", file_name),
                _ => (),
            }
        }
    }
}

fn escape_xml(text: &str) -> String
{
    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

fn remove_indentation(text: &str) -> String
{
    let mut result = String::new();

    if is_blank(text) { return result; }

    let text_lines: Vec<&str> = text.lines().skip(1).collect();

    if text_lines.len() == 1 { return text.trim_start().to_string(); }

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

fn count_spaces(text: &str) -> usize
{
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

fn get_indentation(text_lines: &Vec<&str>) -> usize
{
    let mut indentation = usize::MAX;
    for line in text_lines.iter() {

        if is_blank(line) { continue; }

        let line_indentation = count_spaces(*line);
        if line_indentation < indentation {
            indentation = line_indentation;
        }
    }
    indentation
}

fn is_blank(text: &str) -> bool
{
    text.chars().all(char::is_whitespace)
}

impl AttributeHandler for File
{
    fn process_attribute(&mut self, attribute: Attribute)
    {
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

// endregion: File

// region: TxmlStructure

pub struct TxmlStructure
{
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl TxmlStructure
{
    pub fn new() -> TxmlStructure
    {
        TxmlStructure {
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file: File)
    {
        self.files.push(file);
    }

    pub fn add_directory(&mut self, directory: Directory)
    {
        self.directories.push(directory);
    }
}

impl Instantiable for TxmlStructure
{
    fn instantiate(&self, dir: &PathBuf)
    {
        self.files.iter().for_each(|file| file.instantiate(&dir));

        self.directories
            .iter()
            .for_each(|directory| directory.instantiate(dir));
    }

    fn instantiate_with_name(&self, dir: &PathBuf, name: &str) {
        if self.files.len() + self.directories.len() > 1 { self.instantiate(dir); return; }

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

// endregion: FxmlStructure

// region: Utils

fn execute_commands(command: &str, dir: &PathBuf) -> Result<(), CommandError>
{
    let commands: Vec<&str> = command.split(";").collect();

    for &command in commands.iter() {
        if let Err(e) = execute_command(command.trim(), dir) {
            return Err(e);
        }
    }
    
    Ok(())
}

fn execute_command(command: &str, dir: &PathBuf)-> Result<(), CommandError>
{
    let command_parts: Vec<&str> = command.split_whitespace().collect();

    if command_parts.is_empty() {
        return Err(InvalidInput);
    }

    let cmd = command_parts[0];
    let args = &command_parts[1..];

    let status = Command::new(cmd).current_dir(dir).args(args).status();

    let status = match status {
        Ok(s) => s,
        Err(_) => return Err(CreationError),
    };

    if status.success() {
        Ok(())
    } else {
        Err(CommandFailed)
    }
}

// endregion: Utils

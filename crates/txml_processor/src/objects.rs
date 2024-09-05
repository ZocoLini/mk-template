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
        let dir_name = format!("{}", self.name);
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
        let file_name = if self.extension.is_empty() {
            format!("{}", self.name)
        } else {
            format!("{}.{}", self.name, self.extension)
        };
        let new_path_buff = dir.join(&file_name);

        if new_path_buff.exists() {
            println!("File {} already exist. Skipping creation.", file_name);
            return;
        }

        fs::File::create(&new_path_buff)
            .expect("Error creating file")
            .write_all(self.content.as_bytes())
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
}

// endregion: FxmlStructure

// region: Utils

fn execute_commands(command: &str, dir: &PathBuf) -> Result<(), CommandError>
{
    let commands: Vec<&str> = command.split("; ").collect();

    for &command in commands.iter() {
        if let Err(e) = execute_command(command, dir) {
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

pub mod objects;

use crate::objects::{AttributeHandler, Directory, File, TxmlStructure};
use crate::TxmlProcessorError::{InvalidDirectory, InvalidTag, UnknownParseError};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::VecDeque;
use std::path::PathBuf;

#[derive(Debug)]
pub enum TxmlProcessorError
{
    InvalidDirectory,
    UnknownParseError,
    InvalidTag,
}

pub fn process_txml(txml: &PathBuf) -> Result<TxmlStructure, TxmlProcessorError>
{
    let mut txml_structure = TxmlStructure::new();

    if !txml.exists() {
        return Err(InvalidDirectory);
    }
    if !txml.is_file() {
        return Err(InvalidDirectory);
    }

    let mut reader = Reader::from_file(txml).expect("Should exist");
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
                _ => return Err(InvalidTag),
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
                _ => return Err(InvalidTag),
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
                _ => return Err(InvalidTag),
            },
            Ok(Event::Eof) => break,
            Err(_e) => return Err(UnknownParseError),
            _ => (),
        }

        event_buf.clear();
    }

    Ok(txml_structure)
}

#[cfg(test)]
mod tests
{
    use super::*;
    use std::str::FromStr;

    #[test]
    fn process_txml_test()
    {
        if let Err(e) = process_txml(
            &PathBuf::from_str(
                "/home/borja/projects/mk-template/crates/txml_processor/template_example_1.xml",
            )
            .expect("Should exist"),
        ) {
            panic!("Error: {:?}", e);
        }
    }
}

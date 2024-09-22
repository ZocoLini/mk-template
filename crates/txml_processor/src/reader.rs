#![allow(unused_imports)]

use quick_xml::events::{BytesStart, BytesText, Event};

// TODO: Continue the implementation of the TxmlReader

pub enum ElementState<'a> {
    Start(BytesStart<'a>),
    End,
    Empty(BytesStart<'a>),
}

pub enum TxmlEvent<'a> {
    StartRoot(BytesStart<'a>),
    EmptyRoot(BytesStart<'a>),
    EndRoot,
    StartMetadata(BytesStart<'a>),
    EmptyMetadata(BytesStart<'a>),
    EndMetadata,
    StartVariable(BytesStart<'a>),
    EmptyVariable(BytesStart<'a>),
    EndVariable,
    StartDirectory(BytesStart<'a>),
    EmptyDirectory(BytesStart<'a>),
    EndDirectory,
    StartFile(BytesStart<'a>),
    EmptyFile(BytesStart<'a>),
    EndFile,
    Text(BytesText<'a>),
    Comment(BytesText<'a>),
    Eof,
}

pub enum TxmlReaderError {
    UnexpectedElement,
    UnknownError,
    UnsupportedEncoding,
}

pub struct TxmlReader<'a> {
    xml_reader: quick_xml::Reader<&'a [u8]>,
    event_buff: Vec<u8>,
}

impl<'a> TxmlReader<'a> {
    pub fn read_event(&mut self) -> Result<TxmlEvent, TxmlReaderError> {
        self.event_buff.clear();

        match self.xml_reader.read_event_into(&mut self.event_buff) {
            Ok(Event::Start(a)) => match a.name().0 {
                b"Root" => Ok(TxmlEvent::StartRoot(a)),
                b"Metadata" => Ok(TxmlEvent::StartMetadata(a)),
                b"Variable" => Ok(TxmlEvent::StartVariable(a)),
                b"Directory" => Ok(TxmlEvent::StartDirectory(a)),
                b"File" => Ok(TxmlEvent::StartFile(a)),
                _ => Err(TxmlReaderError::UnexpectedElement),
            },
            Ok(Event::Empty(a)) => match a.name().0 {
                b"Root" => Ok(TxmlEvent::EmptyRoot(a)),
                b"Metadata" => Ok(TxmlEvent::EmptyMetadata(a)),
                b"Variable" => Ok(TxmlEvent::EmptyVariable(a)),
                b"Directory" => Ok(TxmlEvent::EmptyDirectory(a)),
                b"File" => Ok(TxmlEvent::EmptyFile(a)),
                _ => Err(TxmlReaderError::UnexpectedElement),
            }
            Ok(Event::Text(a)) => Ok(TxmlEvent::Text(a)),
            Ok(Event::End(a)) => match a.name().0 {
                b"Root" => Ok(TxmlEvent::EndRoot),
                b"Metadata" => Ok(TxmlEvent::EndMetadata),
                b"Variable" => Ok(TxmlEvent::EndVariable),
                b"Directory" => Ok(TxmlEvent::EndDirectory),
                b"File" => Ok(TxmlEvent::EndFile),
                _ => Err(TxmlReaderError::UnexpectedElement),
            },
            Ok(Event::Comment(a)) => Ok(TxmlEvent::Comment(a)),
            Ok(Event::Eof) => Ok(TxmlEvent::Eof),
            Err(_) => Err(TxmlReaderError::UnknownError),
            _ => Err(TxmlReaderError::UnsupportedEncoding),
        }
    }

    pub fn from_str(s: &'a str) -> Self {
        Self {
            xml_reader: quick_xml::Reader::from_str(s),
            event_buff: Vec::new(),
        }
    }
}

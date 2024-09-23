use quick_xml::events::{BytesStart, BytesText, Event};

pub enum ElementState<'a> {
    Start(BytesStart<'a>),
    End,
    Empty(BytesStart<'a>),
}

pub enum TxmlEvent<'a> {
    Root(ElementState<'a>),
    Metadata(ElementState<'a>),
    Variable(ElementState<'a>),
    Directory(ElementState<'a>),
    File(ElementState<'a>),
    Text(BytesText<'a>),
    Comment(()),
    Declaration(()),
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
                b"Root" => Ok(TxmlEvent::Root(ElementState::Start(a))),
                b"Metadata" => Ok(TxmlEvent::Metadata(ElementState::Start(a))),
                b"Variable" => Ok(TxmlEvent::Variable(ElementState::Start(a))),
                b"Directory" => Ok(TxmlEvent::Directory(ElementState::Start(a))),
                b"File" => Ok(TxmlEvent::File(ElementState::Start(a))),
                _ => Err(TxmlReaderError::UnexpectedElement),
            },
            Ok(Event::Empty(a)) => match a.name().0 {
                b"Root" => Ok(TxmlEvent::Root(ElementState::Empty(a))),
                b"Metadata" => Ok(TxmlEvent::Metadata(ElementState::Empty(a))),
                b"Variable" => Ok(TxmlEvent::Variable(ElementState::Empty(a))),
                b"Directory" => Ok(TxmlEvent::Directory(ElementState::Empty(a))),
                b"File" => Ok(TxmlEvent::File(ElementState::Empty(a))),
                _ => Err(TxmlReaderError::UnexpectedElement),
            }
            Ok(Event::Text(a)) => Ok(TxmlEvent::Text(a)),
            Ok(Event::End(a)) => match a.name().0 {
                b"Root" => Ok(TxmlEvent::Root(ElementState::End)),
                b"Metadata" => Ok(TxmlEvent::Metadata(ElementState::End)),
                b"Variable" => Ok(TxmlEvent::Variable(ElementState::End)),
                b"Directory" => Ok(TxmlEvent::Directory(ElementState::End)),
                b"File" => Ok(TxmlEvent::File(ElementState::End)),
                _ => Err(TxmlReaderError::UnexpectedElement),
            },
            Ok(Event::Comment(_a)) => Ok(TxmlEvent::Comment(())),
            Ok(Event::Eof) => Ok(TxmlEvent::Eof),
            Ok(Event::Decl(_)) => Ok(TxmlEvent::Declaration(())),
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

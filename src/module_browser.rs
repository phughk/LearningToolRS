use crate::error::Error;
use std::fs;
use std::io;
use tracing::info;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
#[allow(dead_code)]
pub struct LearningModule {
  metadata: LearningModuleMetadata,
  entries: Vec<LearningModuleEntry>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LearningModuleMetadata {
  name: String,
  author: String,
  updated_date: String,
  file_version: Version,
  format_version: Version,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Version {
  major: u32,
  minor: u32,
  patch: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LearningModuleEntry {}

pub fn list_modules(directory: &str) -> Result<Vec<LearningModule>, Error> {
  // TODO handle partial failure... Result<Vec<Result<LearningModule, Error>>, Error> ?
  let paths = fs::read_dir(directory)?;
  let mut ret = Vec::new();
  for path in paths {
    ret.push(read_module(
      path
        .unwrap()
        .path()
        .display()
        .to_string(),
    )?)
  }
  return Ok(ret);
}

fn read_module(filename: String) -> Result<LearningModule, Error> {
  let file = fs::File::open(filename).unwrap();
  let file = io::BufReader::new(file);
  let reader = EventReader::new(file);
  return read_module_content(reader);
}

fn read_module_content(mut stream: EventReader<io::BufReader<fs::File>>) -> Result<LearningModule, Error> {
  let xml_event = stream.next()?;
  let xml_event_cpy = xml_event.clone();
  match xml_event_cpy {
    XmlEvent::StartDocument { .. } => {}
    XmlEvent::StartElement { name, .. } => {
      info!(ownder_name = name.to_string(), "handled start element")
    }
    XmlEvent::EndElement { name } => {
      info!(owned_name = name.to_string(), "handled end element")
    }
    _ => {
      let ev_str = format!("{xml_event_cpy:?}");
      info!(event = ev_str, "unhandled element")
    }
  }
  let str_event = format!("{xml_event:?}");
  return Ok(LearningModule {
    metadata: LearningModuleMetadata {
      name: str_event,
      author: String::from("Hugh"),
      updated_date: String::from("this is a date"),
      file_version: Version { major: 1, minor: 0, patch: 0 },
      format_version: Version { major: 1, minor: 0, patch: 0 },
    },
    entries: vec![],
  });
}

fn handle_header(potentialHeader: XmlEvent) -> Result<Option<XmlEvent>, Error> {
  match potentialHeader {
    XmlEvent::StartDocument { .. } => return Ok(None),
    XmlEvent::StartElement { name, attributes, namespace } => return Ok(Some(XmlEvent::StartElement { name, attributes, namespace })),
    _ => return Err(Error::IOError(std::io::Error::new(std::io::ErrorKind::InvalidData, "unhandled error"))),
  }
}

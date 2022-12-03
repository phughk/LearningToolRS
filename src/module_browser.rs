use crate::error::Error;
use std::fs;
use std::io;
use std::io::ErrorKind;
use tracing::info;
use tracing::error;
use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};
use crate::error::Error::IOError;

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
pub struct LearningModuleEntry {
  entry_type: LearningModuleEntryType,
  id: String,
}

#[derive(Debug)]
enum LearningModuleEntryType {
  None,
  SingleChoice,
  MultipleChoice,
  Category,
}

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
  let learning_module = LearningModule{ metadata: LearningModuleMetadata {
    name: "".to_string(),
    author: "".to_string(),
    updated_date: "".to_string(),
    file_version: Version {
      major: 0,
      minor: 0,
      patch: 0,
    },
    format_version: Version {
      major: 0,
      minor: 0,
      patch: 0,
    },
  }, entries: vec![] };
  for event in stream {
    match event {
      Ok(XmlEvent::StartDocument {standalone, encoding, version}) => {}
      Ok(XmlEvent::EndDocument{}) => {}
      Ok(XmlEvent::StartElement {name, attributes, namespace}) => {
        let attr_str = format!("{attributes:?}");
        let namespace_str = format!("{namespace:?}");
        info!(name=name.to_string(), attribues=attr_str, namespace=namespace_str);
      }
      Err(e) => {
        let e_str = format!("{e:?}");
        error!(error=e_str, "failed to process event");
        return Err(IOError(io::Error::new(ErrorKind::InvalidData, e)))
      }
      _ => {
        info!("unhandled event")
      }
    }
  }
  return Ok(learning_module)
}
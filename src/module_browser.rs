use std::cmp::min;
use std::collections::VecDeque;
use crate::error::Error;
use std::fs;
use std::io;
use std::io::ErrorKind;
use tracing::info;
use tracing::error;
use text_io::scan;
use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};
use crate::error::Error::IOError;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LearningModule {
  metadata: LearningModuleMetadata,
  entries: Vec<LearningModuleEntry>,
}

impl LearningModule {
 fn new() -> LearningModule {
   return LearningModule{
     metadata: LearningModuleMetadata::new(),
     entries: vec![]
   }
 }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LearningModuleMetadata {
  name: String,
  author: String,
  updated_date: String,
  file_version: Version,
  format_version: Version,
}

impl LearningModuleMetadata {
  fn new() -> LearningModuleMetadata {
    return LearningModuleMetadata{
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
    }
  }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Version {
  major: u32,
  minor: u32,
  patch: u32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LearningModuleEntry {
  entry_type: LearningModuleEntryType,
  id: String,
}

#[derive(Debug, Clone)]
enum LearningModuleEntryType {
  None,
  SingleChoice,
  MultipleChoice,
  Category,
}

// Indicates an opening element that has not yet been closed
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct DomElement {
  element_name: String,
  name: String,
  author: String,
  updated_date: String,
  file_version: Version,
  format_version: Version,
  sampleable: bool,
  correct: bool,
}

impl DomElement {
  fn new() -> DomElement {
    return DomElement{
      element_name: "".to_string(),
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
      sampleable: false,
      correct: false,
    }
  }
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
  let learning_module = LearningModule::new();
  let mut element_stack: VecDeque<DomElement> = VecDeque::new();
  for event in stream {
    match event {
      Ok(XmlEvent::StartDocument {standalone, encoding, version}) => {}
      Ok(XmlEvent::EndDocument{}) => {}
      Ok(XmlEvent::StartElement {name, attributes, namespace}) => {
        let attr_str = format!("{attributes:?}");
        let namespace_str = format!("{namespace:?}");
        info!(name=name.to_string(), attribues=attr_str, namespace=namespace_str, "Started new element");
        let mut element = DomElement::new();
        element.name = name.to_string();
        element_stack.push_back(element);
      }
      Ok(XmlEvent::EndElement {name}) => {
        let name_str = format!("{name:?}");
        let popped = element_stack.pop_back();
        let popped_str = format!("{popped:?}");
        info!(name=name_str, popped=popped_str, "closing element")
      }
      Ok(XmlEvent::Characters(s)) => {
        info!(chars=s, "received characters");
        let mut popped = element_stack.pop_back().ok_or(IOError(std::io::Error::new(ErrorKind::InvalidInput, "expected to pop element but none found")))?.clone();
        if popped.name==String::from("file_version") {
          popped.file_version = parse_version(s);
          element_stack.push_back(popped);
        }
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

fn parse_version(version_str: String) -> Version {
  let (major, minor, patch): (u32, u32, u32);
  scan!("{}.{}.{}", major, minor, patch);
  return Version{major: major, minor: minor, patch:patch};
}
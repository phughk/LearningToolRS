use std::fmt::Formatter;
use crate::error::Error;
use scan_fmt::scan_fmt;
use serde::Deserialize;
use serde::de;
use serde::Serialize;
use serde_xml_rs::de::Deserializer;
use std::fs;
use std::io;
use std::str::FromStr;
use serde::de::Visitor;
use xml::attribute::OwnedAttribute;
use xml::reader::EventReader;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LearningModule {
  metadata: LearningModuleMetadata,
  entries: Vec<LearningModuleEntry>,
}

impl LearningModule {
  fn new() -> LearningModule {
    return LearningModule {
      metadata: LearningModuleMetadata::new(),
      entries: vec![],
    };
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    return LearningModuleMetadata {
      name: "".to_string(),
      author: "".to_string(),
      updated_date: "".to_string(),
      file_version: Version { major: 0, minor: 0, patch: 0 },
      format_version: Version { major: 0, minor: 0, patch: 0 },
    };
  }
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Version {
  major: u32,
  minor: u32,
  patch: u32,
}

struct VersionVisitor {}

impl <'de> Visitor<'de> for VersionVisitor {
  type Value = Version;

  fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
    formatter.write_str("expecting version format <major>.<minor>.<patch> strictly")
  }

  fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: de::Error {
    if let Ok((major, minor, patch)) = scan_fmt!(&v.to_string(), "{d}.{d}.{d}", u32, u32, u32) {
      return Ok(Version { major, minor, patch });
    }
    return Err(de::Error::custom("not a version"))
  }
}

impl <'de> de::Deserialize<'de> for Version {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
    deserializer.deserialize_string(VersionVisitor{})
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LearningModuleEntry {
  entry_type: LearningModuleEntryType,
  id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum LearningModuleEntryType {
  None,
  SingleChoice,
  MultipleChoice,
  Category,
}

impl FromStr for LearningModuleEntryType {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // TODO sanitise? cap, space
    match s {
      "single-choice" => Ok(LearningModuleEntryType::SingleChoice),
      "multiple-choice" => Ok(LearningModuleEntryType::MultipleChoice),
      "category" => Ok(LearningModuleEntryType::Category),
      "" => Ok(LearningModuleEntryType::None),
      _ => Err(()),
    }
  }
}

// Indicates an opening element that has not yet been closed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
struct DomElement {
  element_name: String, // xml tag name
  prop_type: String,
  name: String, // xml name property
  author: String,
  updated_date: String,
  file_version: Version,
  format_version: Version,
  sampleable: bool,
  correct: bool,
}

impl DomElement {
  fn new() -> DomElement {
    return DomElement {
      element_name: "".to_string(),
      prop_type: "".to_string(),
      name: "".to_string(),
      author: "".to_string(),
      updated_date: "".to_string(),
      file_version: Version { major: 0, minor: 0, patch: 0 },
      format_version: Version { major: 0, minor: 0, patch: 0 },
      sampleable: false,
      correct: false,
    };
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

fn read_module_content(event_reader: EventReader<io::BufReader<fs::File>>) -> Result<LearningModule, Error> {
  match LearningModule::deserialize(&mut Deserializer::new(event_reader)) {
    Ok(x) => return Ok(x),
    Err(cause) => return Err(Error::SerdeError(cause)),
  }
}

fn get_attribute(attributes: Vec<OwnedAttribute>, key: String) -> Option<String> {
  return attributes
    .iter()
    .find(|own_at| own_at.name.local_name == key)
    .map(|own| own.value.clone());
}

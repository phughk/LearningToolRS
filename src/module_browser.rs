use crate::error::Error;
use scan_fmt::scan_fmt;
use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Serialize;
use serde_xml_rs::de::Deserializer;
use std::fmt::Formatter;
use std::fs;
use std::io;

use xml::reader::EventReader;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LearningModule {
  pub metadata: LearningModuleMetadata,
  pub entries: Vec<LearningModuleEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "kebab-case")]
pub struct LearningModuleMetadata {
  pub name: String,
  pub author: String,
  pub updated: String,
  pub file_version: Version,
  pub format_version: Version,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Version {
  pub major: u32,
  pub minor: u32,
  pub patch: u32,
}

struct VersionVisitor {}

impl<'de> Visitor<'de> for VersionVisitor {
  type Value = Version;

  fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
    formatter.write_str("expecting version format <major>.<minor>.<patch> strictly")
  }

  fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    if let Ok((major, minor, patch)) = scan_fmt!(&v.to_string(), "{d}.{d}.{d}", u32, u32, u32) {
      return Ok(Version { major, minor, patch });
    }
    return Err(de::Error::custom("not a version"));
  }
}

impl<'de> de::Deserialize<'de> for Version {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    deserializer.deserialize_string(VersionVisitor {})
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct LearningModuleEntry {
  #[serde(rename = "type", default)]
  pub entry_type: LearningModuleEntryType,
  pub id: String,
  #[serde(default)]
  pub sampleable: bool,
  #[serde(rename = "$value")]
  pub entry_tags: Vec<EntryTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
#[serde(rename_all = "kebab-case")]
pub enum EntryTag {
  Question(Question),
  Answer(Answer),
  Category(Category),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct Question {
  #[serde(rename = "$value")]
  pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct Answer {
  pub correct: bool,
  pub id: String,
  #[serde(default)]
  pub category: String,
  #[serde(rename = "$value")]
  pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct Category {
  pub id: String,
  #[serde(rename = "$value")]
  pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum LearningModuleEntryType {
  None,
  SingleChoice,
  MultipleChoice,
  Category,
}

impl Default for LearningModuleEntryType {
  fn default() -> Self {
    return LearningModuleEntryType::None;
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

use std::fs;
use std::io;
use xml::reader::EventReader;
use crate::error::Error;

#[derive(Debug)]
pub struct LearningModule {
  metadata: LearningModuleMetadata,
  entries: Vec<LearningModuleEntry>,
}

#[derive(Debug)]
pub struct LearningModuleMetadata {
  name: String,
  author: String,
  updated_date: String,
  file_version: Version,
  format_version: Version,
}

#[derive(Debug)]
pub struct Version {
  major: u32,
  minor: u32,
  patch: u32,
}

#[derive(Debug)]
pub struct LearningModuleEntry {}

pub fn list_modules(directory: &str) -> Result<Vec<LearningModule>, Error> {
  // TODO handle partial failure... Result<Vec<Result<LearningModule, Error>>, Error> ?
  let paths = fs::read_dir(directory)?;
  let mut ret = Vec::new();
  for path in paths {
    ret.push(read_module(path.unwrap().path().display().to_string())?)
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
    let xmlEvent = stream.next()?;
    let strEvent = format!("{xmlEvent:?}");
    return Ok(LearningModule {
      metadata: LearningModuleMetadata {
        name: strEvent,
        author: String::from("Hugh"),
        updated_date: String::from("this is a date"),
        file_version: Version {
          major: 1,
          minor: 0,
          patch: 0,
        },
        format_version: Version {
          major: 1,
          minor: 0,
          patch: 0,
        },
      },
      entries: vec![],
    })
}

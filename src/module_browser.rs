use std::fs;
use std::io::Error;
use xml::reader::EventReader;

#[derive(Debug)]
pub struct LearningModule {
    metadata: LearningModuleMetadata,
    entries: Vec<LearningModuleEntry>
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
    patch: u32
}

#[derive(Debug)]
pub struct LearningModuleEntry {
}

pub fn list_modules(directory: &str) -> Result<Vec<LearningModule>, Error> {
    // TODO handle partial failure... Result<Vec<Result<LearningModule, Error>>, Error> ?
    let paths = fs::read_dir(directory)?;
    let mut ret = Vec::new();
    for path in paths {
        ret.push(
            LearningModule{
                metadata: LearningModuleMetadata{
                    name: path?.path().display().to_string(),
                    author: String::from("Hugh"),
                    updated_date: String::from("this is a date"),
                    file_version: Version{major: 1, minor: 0, patch: 0},
                    format_version: Version{major: 1, minor: 0, patch: 0},
                },
                entries: vec![],
            },
        )
    }
    return Ok(ret)
}

fn read_module(filename: &str) -> Result<LearningModule, Error> {
    let reader = EventReader::new(filename);
    read_module_content(reader);
}

fn read_module_content(stream: &dyn std::io::Read) -> Result<LearningModule, Error> {
}
#[derive(Debug)]
pub struct LearningModule<'a> {
    metadata: LearningModuleMetadata<'a>,
    entries: Vec<LearningModuleEntry>
}

#[derive(Debug)]
pub struct LearningModuleMetadata<'a> {
    name: &'a str,
    author: &'a str,
    updated_date: &'a str,
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

pub fn list_modules(directory: &str) -> Vec<LearningModule> {
    return vec![
        LearningModule{
            metadata: LearningModuleMetadata{
                name: "name one",
                author: "Hugh",
                updated_date: "this is a date",
                file_version: Version{major: 1, minor: 0, patch: 0},
                format_version: Version{major: 1, minor: 0, patch: 0},
            },
            entries: vec![],
        },
    ]
}
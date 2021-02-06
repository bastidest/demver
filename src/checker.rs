use crate::syntax;
use crate::version;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Checker {
    files: Vec<String>,
}

#[derive(Debug)]
pub struct TagVersion {
    pub tag: syntax::DemverTag,
    // current_version: version::FixedVersion,
    // new_version: version::FixedVersion,
}

pub type TagVersionResult = Result<TagVersion, String>;

#[derive(Debug)]
pub struct FileVersion {
    pub tag_version_results: Vec<TagVersionResult>,
}

type FileVersionResult = Result<FileVersion, String>;

#[derive(Debug)]
pub struct FileInfo {
    pub filename: String,
    pub version_result: FileVersionResult,
}

impl Checker {
    pub fn new(files: Vec<String>) -> Self {
        Self { files }
    }

    fn check_file(&self, filename: &str) -> FileVersionResult {
        let mut file = File::open(filename).or(Err("failed to open file"))?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .or(Err("failed to read file as a string"))?;

        let tokenized_tags = syntax::TokenizedTag::tokenize_all(&file_content, 0);
        let version_results: Vec<TagVersionResult> = tokenized_tags
            .into_iter()
            .map(|tt| match tt {
                Ok(tag) => match syntax::DemverTag::parse(&tag) {
                    Ok(demver_tag) => TagVersionResult::Ok(TagVersion { tag: demver_tag }),
                    Err(err_str) => TagVersionResult::Err(format!(
                        "failed to parse tag in file {}: {}",
                        filename, err_str
                    )),
                },
                Err(err_str) => TagVersionResult::Err(format!(
                    "failed to tokenize tag in file {}: {}",
                    filename, err_str
                )),
            })
            .collect();

        Ok(FileVersion {
            tag_version_results: version_results,
        })
    }

    pub fn do_check(&self) -> Vec<FileInfo> {
        let mut ret: Vec<FileInfo> = vec![];

        for file in &self.files {
            ret.push(FileInfo {
                filename: file.to_owned(),
                version_result: self.check_file(&file),
            });
        }

        ret
    }
}

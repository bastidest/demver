use crate::syntax;
use crate::version;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Checker {
    files: Vec<String>,
}

type CheckResults = Vec<CheckResult>;

#[derive(Debug)]
pub struct CheckResult {
    tag: syntax::DemverTag,
    current_version: version::FixedVersion,
    new_version: version::FixedVersion,
}

impl Checker {
    pub fn new(files: Vec<String>) -> Self {
        Self { files }
    }

    fn check_file(&self, filename: &str) -> Result<CheckResults, String> {
        let mut file = File::open(filename).or(Err("failed to open file"))?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .or(Err("failed to read file as a string"))?;

        let tokenized_tags = syntax::TokenizedTag::tokenize_all(&file_content, 0);
        let ok_tags: Vec<syntax::DemverTag> = tokenized_tags
            .into_iter()
            .filter_map(|tt| match tt {
                Ok(tag) => Some(tag),
                Err(err_str) => {
                    println!("failed to tokenize tag in file {}: {}", filename, err_str);
                    None
                }
            })
            .map(|tt| syntax::DemverTag::parse(&tt))
            .filter_map(|tt| match tt {
                Ok(tag) => Some(tag),
                Err(err_str) => {
                    println!("failed to parse tag in file {}: {}", filename, err_str);
                    None
                }
            })
            .collect();

        println!("Found tags in file '{}': {:?}", filename, ok_tags);

        Err("".to_owned())
    }

    pub fn do_check(&self) -> CheckResults {
        for file in &self.files {
            self.check_file(&file);
        }

        vec![]
    }
}

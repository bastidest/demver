extern crate ini;
extern crate regex;

use crate::checker::TagVersionResult;
use std::convert::From;

#[macro_use]
extern crate lazy_static;

mod ini_source;
mod source;
mod syntax;
mod version;
mod checker;

use clap::{App, Arg};
use colored::*;

fn get_newest_version(
    version_range: semver::VersionReq,
    versions: &Vec<version::FixedVersion>,
) -> Result<&version::FixedVersion, String> {
    let filtered_versions: Vec<&version::FixedVersion> = versions
        .iter()
        .filter(|v| version_range.matches(&v.get_semver()))
        .collect();

    let mut sorted_versions = filtered_versions.clone();
    sorted_versions.sort();

    match sorted_versions.last() {
        Some(last) => Ok(last),
        None => Err(String::from("oh oh")),
    }
}

struct FileTarget {
    filename: String,
}

impl FileTarget {
    fn new(filename: String) -> FileTarget {
        FileTarget { filename }
    }
}

fn main() {
    let matches = App::new("demver")
        .version("0.1.0")
        .about("Deterministic Version Manager for reproducible builds and deployments")
        .author("Sebastian H.")
        .subcommand(
            App::new("check")
                .about("check files containing demver tags")
                .arg(
                    Arg::new("file")
                        .value_name("FILE")
                        .required(true)
                        .multiple(true)
                        // .index(1)
                        .about("files to check"),
                ),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("check") {
        let files: Vec<String> = matches
            .values_of("file")
            .unwrap()
            .map(|s| String::from(s))
            .collect();
        check(files);
    }
}

fn check(files: Vec<String>) -> Result<(), String> {
    let checker = checker::Checker::new(files);
    let file_infos = checker.do_check();

    for file_info in &file_infos {
        match &file_info.version_result {
            Ok(file_version) => {
                println!("{}: ", file_info.filename.green().bold());
                for tag_version_result in &file_version.tag_version_results {
                    print_tag_version_info(tag_version_result)
                }
            },
            Err(err_msg) => {
                println!("{}: ERROR {}", file_info.filename.red().bold(), err_msg);
            }
        }
    }

    Ok(())
}

fn print_tag_version_info(tag_version_result: &TagVersionResult) {
    match(tag_version_result) {
        Ok(tag_version) => {
            println!("  tag {}", tag_version.tag.semver.to_string());
        },
        Err(err_msg) => {
            println!("  ERROR: {}", err_msg);
        }
    }
}

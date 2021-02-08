extern crate ini;
extern crate regex;

use crate::tag_scanner::TagVersionResult;
use std::convert::From;

#[macro_use]
extern crate lazy_static;

mod ini_source;
mod source;
mod syntax;
mod tag_checker;
mod tag_scanner;
mod version;

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
    let scanner = tag_scanner::TagScanner::new(files);
    let file_infos = scanner.do_scan();

    for file_info in &file_infos {
        match &file_info.version_result {
            Ok(file_version) => {
                println!("{}: ", file_info.filename.green().bold());
                for tag_version_result in &file_version.tag_version_results {
                    print_tag_version_info(tag_version_result)
                }
            }
            Err(err_msg) => {
                println!("{}: ERROR {}", file_info.filename.red().bold(), err_msg);
            }
        }
    }

    Ok(())
}

fn print_tag_version_info(tag_version_result: &TagVersionResult) {
    match tag_version_result {
        Ok(tag_version) => {
            let new_version = tag_checker::TagChecker::get_current_version_from_source(&tag_version.tag);
            let new_version = match new_version {
                Ok(v) => format!("{}", v.raw_version),
                Err(err_msg) => format!("{} ({})", "not found".to_owned().red(), err_msg),
            };
            println!(
                "  {} {} [{}] -> {}",
                tag_version.tag.identifier.to_string(),
                tag_version.tag.get_raw_version_req(),
                tag_version.tag.get_raw_source(),
                new_version,
            );
        }
        Err(err_msg) => {
            println!("  ERROR: {}", err_msg);
        }
    }
}

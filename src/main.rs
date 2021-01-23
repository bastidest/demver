extern crate ini;
extern crate regex;

use std::convert::From;

#[macro_use]
extern crate lazy_static;

mod ini_source;
mod source;
mod syntax;
mod version;
mod checker;

use clap::{App, Arg};

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
        println!("{:?}", files);

        let checker = checker::Checker::new(files);
        let results = checker.do_check();

        println!("all results {:?}", results);
    }
}

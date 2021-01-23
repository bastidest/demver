extern crate ini;
extern crate regex;

use std::convert::From;

#[macro_use]
extern crate lazy_static;

mod syntax;
mod version;
mod source;
mod ini_source;


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

fn main() -> Result<(), String> {
    Ok(())
}



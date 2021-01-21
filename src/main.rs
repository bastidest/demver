extern crate ini;
extern crate regex;
use ini::Ini;
use regex::Regex;
use std::cmp::Ordering;
use std::convert::From;
use std::fs::File;
use std::io;

#[macro_use]
extern crate lazy_static;

mod syntax;

trait VersionSource {
    fn get_available_versions(
        &self,
        version_range: semver::VersionReq,
        component: Option<String>,
    ) -> Vec<Version>;
}

struct FileSource {
    filename: String,
}

impl FileSource {
    fn new(filename: String) -> FileSource {
        FileSource { filename }
    }
}

struct IniSource {
    file_source: FileSource,
}

impl IniSource {
    fn new(file_source: FileSource) -> IniSource {
        IniSource { file_source }
    }
}

#[derive(Debug, Clone, Eq)]
struct Version {
    semver: semver::Version,
    hash: Option<String>,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.semver.cmp(&other.semver)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.semver == other.semver
    }
}

fn get_newest_version(
    version_range: semver::VersionReq,
    versions: &Vec<Version>,
) -> Result<&Version, String> {
    let filtered_versions: Vec<&Version> = versions
        .iter()
        .filter(|v| version_range.matches(&v.semver))
        .collect();

    let mut sorted_versions = filtered_versions.clone();
    sorted_versions.sort();

    match sorted_versions.last() {
        Some(last) => Ok(last),
        None => Err(String::from("oh oh")),
    }
}

impl VersionSource for IniSource {
    fn get_available_versions(
        &self,
        _: semver::VersionReq,
        component: Option<String>,
    ) -> Vec<Version> {
        let conf = match Ini::load_from_file(&self.file_source.filename) {
            Ok(c) => c,
            Err(_) => return vec![],
        };
        let section = match conf.section(component) {
            Some(c) => c,
            None => return vec![],
        };

        let mut ret: Vec<Version> = vec![];

        for (key, value) in section.iter() {
            let semver = match semver::Version::parse(key) {
                Ok(s) => s,
                Err(_) => {
                    println!("unable to parse '{}' as semver, skipping", key);
                    continue;
                }
            };

            println!("{} = {}", key, value);
            ret.push(Version {
                semver: semver,
                hash: Some(String::from(value)),
            });
        }

        ret
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_ini() {
        let filename = String::from("./test/simple/versions.ini");
        let fs = FileSource::new(filename);
        let ini = IniSource::new(fs);

        let sver = semver::VersionReq::parse("^1.0.0").unwrap();

        // let ver = ini
        //     .get_newest_version(sver.as_str().to_owned(), Some(String::from("testapp")))
        //     .unwrap();
        // println!("{:?}", ver)
    }
}

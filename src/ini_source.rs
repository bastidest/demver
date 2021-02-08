use crate::source;
use crate::version;
use ini::Ini;

pub struct FileSource {
    filename: String,
}

impl FileSource {
    pub fn new(filename: String) -> FileSource {
        FileSource { filename }
    }
}

pub struct IniSource {
    file_source: FileSource,
}

impl IniSource {
    pub fn new(filename: &str) -> IniSource {
        IniSource {
            file_source: FileSource {
                filename: filename.to_owned(),
            },
        }
    }
}

impl source::VersionSource for IniSource {
    fn get_fixed_version(
        &self,
        requested_version: &semver::VersionReq,
        identifier: Option<&str>,
    ) -> source::FixedVersionResult {
        let conf = match Ini::load_from_file(&self.file_source.filename) {
            Ok(c) => c,
            Err(e) => {
                return source::FixedVersionResult::Err(format!(
                    "Failed to open ini file {}: {}",
                    &self.file_source.filename, e
                ))
            }
        };
        let section = match conf.section(identifier) {
            Some(c) => c,
            None => {
                return source::FixedVersionResult::Err(
                    "Ini file does not contain a section for the given identifier".to_owned(),
                )
            }
        };

        let mut best_match: Option<version::FixedVersion> = None;

        for (key, value) in section.iter() {
            let semver = match semver::Version::parse(key) {
                Ok(s) => s,
                Err(_) => {
                    // println!("unable to parse '{}' as semver, skipping", key);
                    continue;
                }
            };

            if requested_version.matches(&semver) {
                // println!("Version {} matches Request {}", semver, requested_version);

                best_match = if best_match.is_some() && best_match.as_ref().unwrap().semver > semver
                {
                    best_match
                } else {
                    Some(version::FixedVersion {
                        raw_version: key.to_string(),
                        semver: semver,
                        hash: String::from(value),
                    })
                }
            }
        }

        match best_match {
            Some(m) => Ok(m),
            None => Err("no match found".to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::VersionSource;

    #[test]
    fn test_simple_1() {
        let filename = String::from("./test/simple/versions.ini");
        let ini = IniSource::new(&filename);

        let req = semver::VersionReq::parse("~1.0.0").unwrap();

        let ver = ini.get_fixed_version(&req, Some("testapp"));
        let ver = ver.unwrap();

        assert_eq!(ver.get_hash(), "1")
    }

    #[test]
    fn test_simple_2() {
        let filename = String::from("./test/simple/versions.ini");
        let ini = IniSource::new(&filename);

        let req = semver::VersionReq::parse("^1.0.0").unwrap();

        let ver = ini.get_fixed_version(&req, Some("testapp"));
        let ver = ver.unwrap();

        assert_eq!(ver.get_hash(), "15")
    }
}

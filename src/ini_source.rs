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
    pub fn new(file_source: FileSource) -> IniSource {
        IniSource { file_source }
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
            Err(e) => return source::FixedVersionResult::Err(format!("Failed to open ini file: {}", e)),
        };
        let section = match conf.section(identifier) {
            Some(c) => c,
            None => return source::FixedVersionResult::Err("Ini file does not contain a section for the given identifier".to_owned()),
        };

        let mut best_match = source::FixedVersionResult::None;

        for (key, value) in section.iter() {
            let semver = match semver::Version::parse(key) {
                Ok(s) => s,
                Err(_) => {
                    println!("unable to parse '{}' as semver, skipping", key);
                    continue;
                }
            };

            if requested_version.matches(&semver) {
                println!("Version {} matches Request {}", semver, requested_version);

                best_match = if best_match.is_ok() && best_match.unwrap().semver > semver {
                    best_match
                } else {
                    source::FixedVersionResult::Ok(version::FixedVersion {
                        semver: semver,
                        hash: String::from(value),
                    })
                }
            }
        }

        best_match
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::VersionSource;

    #[test]
    fn test_simple_1() {
        let filename = String::from("./test/simple/versions.ini");
        let fs = FileSource::new(filename);
        let ini = IniSource::new(fs);

        let req = semver::VersionReq::parse("~1.0.0").unwrap();

        let ver = ini.get_fixed_version(&req, Some("testapp"));
        let ver = ver.unwrap();

        assert_eq!(ver.get_hash(), "1")
    }

    #[test]
    fn test_simple_2() {
        let filename = String::from("./test/simple/versions.ini");
        let fs = FileSource::new(filename);
        let ini = IniSource::new(fs);

        let req = semver::VersionReq::parse("^1.0.0").unwrap();

        let ver = ini.get_fixed_version(&req, Some("testapp"));
        let ver = ver.unwrap();

        assert_eq!(ver.get_hash(), "15")
    }
}

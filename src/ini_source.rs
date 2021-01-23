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
    fn get_available_versions(
        &self,
        _: semver::VersionReq,
        component: Option<String>,
    ) -> Vec<version::FixedVersion> {
        let conf = match Ini::load_from_file(&self.file_source.filename) {
            Ok(c) => c,
            Err(_) => return vec![],
        };
        let section = match conf.section(component) {
            Some(c) => c,
            None => return vec![],
        };

        let mut ret: Vec<version::FixedVersion> = vec![];

        for (key, value) in section.iter() {
            let semver = match semver::Version::parse(key) {
                Ok(s) => s,
                Err(_) => {
                    println!("unable to parse '{}' as semver, skipping", key);
                    continue;
                }
            };

            println!("{} = {}", key, value);
            ret.push(version::FixedVersion {
                semver: semver,
                hash: String::from(value),
            });
        }

        ret
    }
}

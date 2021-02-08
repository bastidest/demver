use crate::ini_source;
use crate::source::VersionSource;
use crate::syntax;
use crate::version;

pub struct TagChecker {}

impl TagChecker {
    pub fn get_current_version_from_source(
        tag: &syntax::DemverTag,
    ) -> Result<version::FixedVersion, String> {
        match &tag.version_source {
            syntax::SourceTag::File(file_source_tag) => {
                if !file_source_tag.filename.ends_with(".ini") {
                    return Err("File source tags only support .ini files".to_owned());
                }
                let origin_filepath = std::path::Path::new(tag.get_origin_filename());
                let origin_filepath = match origin_filepath.parent() {
                    Some(filepath) => filepath,
                    None => Err("could not open file")?,
                };
                let ini_filepath = origin_filepath.join(&file_source_tag.filename);
                let ini_filepath = match ini_filepath.to_str() {
                    Some(filepath) => filepath,
                    None => Err("could not open file")?,
                };
                let ini_source = ini_source::IniSource::new(ini_filepath);
                let fixed_version =
                    ini_source.get_fixed_version(&tag.semver, Some(&tag.identifier))?;
                Ok(fixed_version)
            }
        }
    }
}

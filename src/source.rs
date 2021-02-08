use crate::version;

pub type FixedVersionResult = Result<version::FixedVersion, String>;

pub trait VersionSource {
    fn get_fixed_version(
        &self,
        requested_version: &semver::VersionReq,
        identifier: Option<&str>,
    ) -> FixedVersionResult;
}

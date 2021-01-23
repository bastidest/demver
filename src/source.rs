use crate::version;

pub trait VersionSource {
    fn get_available_versions(
        &self,
        version_range: semver::VersionReq,
        component: Option<String>,
    ) -> Vec<version::FixedVersion>;
}

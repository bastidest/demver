use crate::version;

pub enum FixedVersionResult {
    Err(String),
    None,
    Ok(version::FixedVersion),
}

impl FixedVersionResult {
    pub fn unwrap(&self) -> &version::FixedVersion {
        match self {
            Self::Ok(v) => return v,
            Self::Err(_) | Self::None => panic!("called `FixedVersionResult::unwrap()` on a `None` or `Err` value")
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok(_) => true,
            Self::Err(_) | Self::None => false,
        }
    }
}

pub trait VersionSource {
    fn get_fixed_version(
        &self,
        requested_version: &semver::VersionReq,
        identifier: Option<&str>,
    ) -> FixedVersionResult;
}

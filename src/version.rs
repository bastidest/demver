use std::cmp::Ordering;

#[derive(Debug, Clone, Eq)]
pub struct FixedVersion {
    pub semver: semver::Version,
    pub hash: String,
}

impl FixedVersion {
    pub fn get_semver(&self) -> &semver::Version {
        &self.semver
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }
}

impl PartialOrd for FixedVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FixedVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.semver.cmp(&other.semver)
    }
}

impl PartialEq for FixedVersion {
    fn eq(&self, other: &Self) -> bool {
        self.semver == other.semver
    }
}

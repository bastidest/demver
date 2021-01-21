pub mod syntax {
    use regex::Regex;
    
    #[derive(Debug)]
    struct ParsedTag {
        semver: String,
        source: String,
        identifier: String,
        current_version: String,
        timestamp: String,
    }

    impl ParsedTag {
        fn new(
            semver: &str,
            source: &str,
            identifier: &str,
            current_version: &str,
            timestamp: &str,
        ) -> Self {
            Self {
                semver: semver.to_owned(),
                source: source.to_owned(),
                identifier: identifier.to_owned(),
                current_version: current_version.to_owned(),
                timestamp: timestamp.to_owned(),
            }
        }

        fn parse(unparsed: &str) -> Result<Self, String> {
            lazy_static! {
                static ref RE: Regex =
                    Regex::new(r"\[demver\((.+)\)\|(.+)\|(.+)\]\s([^\s]+)\s@\s([^\s]+)").unwrap();
            }

            let captures = match RE.captures(unparsed) {
                Some(c) => c,
                None => return Err("Regex does not match".to_string()),
            };

            let semver = match captures.get(1) {
                Some(s) => s,
                None => return Err("Unable to extract the semver from demver tag".to_string()),
            };
            let source = match captures.get(2) {
                Some(s) => s,
                None => return Err("Unable to extract the source from demver tag".to_string()),
            };
            let identifier = match captures.get(3) {
                Some(s) => s,
                None => return Err("Unable to extract the identifier from demver tag".to_string()),
            };
            let current_version = match captures.get(4) {
                Some(s) => s,
                None => return Err("Unable to extract the current version from demver tag".to_string()),
            };
            let timestamp = match captures.get(5) {
                Some(s) => s,
                None => return Err("Unable to extract the timestamp from demver tag".to_string()),
            };

            Ok(Self::new(
                semver.as_str(),
                source.as_str(),
                identifier.as_str(),
                current_version.as_str(),
                timestamp.as_str(),
            ))
        }
    }

    #[derive(Debug)]
    struct DemverTag {
        semver: semver::VersionReq,
        source: String,
        identifier: String,
        current_version: semver::Version,
        timestamp: String,
    }

    impl DemverTag {
        fn parse(unparsed: &ParsedTag) -> Result<Self, String> {
            let semver = match semver::VersionReq::parse(&unparsed.semver) {
                Ok(v) => v,
                Err(e) => return Err(format!("Failed to parse semver: {}", e))
            };
            let source = unparsed.source.clone();
            let identifier = unparsed.identifier.clone();
            let current_version = match semver::Version::parse(&unparsed.current_version) {
                Ok(v) => v,
                Err(e) => return Err(format!("Failed to parse semver: {}", e))
            };
            let timestamp = unparsed.timestamp.clone();

            Ok(DemverTag {
                semver,
                source,
                identifier,
                current_version,
                timestamp
            })
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_parsed_tag() {
            let test_string = "# [demver(^1.0.0)|file(fn=versions.ini,hash=sha512)|testapp] 1.0.0 @ 2020-12-05T18-18-09";

            let parsed_tag = ParsedTag::parse(test_string).unwrap();

            assert_eq!(parsed_tag.semver, "^1.0.0");
            assert_eq!(parsed_tag.source, "file(fn=versions.ini,hash=sha512)");
            assert_eq!(parsed_tag.identifier, "testapp");
            assert_eq!(parsed_tag.current_version, "1.0.0");
            assert_eq!(parsed_tag.timestamp, "2020-12-05T18-18-09");
        }

        #[test]
        fn parse_demver_tag() {
            let semver = "^1.0.0";
            let source = "file(fn=versions.ini,hash=sha512)";
            let identifier = "testapp";
            let current_version = "1.0.0";
            let timestamp = "2020-12-05T18-18-09";
            
            DemverTag::parse(&ParsedTag::new(
                semver,
                source,
                identifier,
                current_version,
                timestamp,
            )).unwrap();
        }
    }
}

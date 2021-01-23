use regex::Regex;

#[derive(Debug)]
struct TokenizedTag {
    semver: String,
    source: String,
    identifier: String,
    current_version: String,
    timestamp: String,
}

impl TokenizedTag {
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

    fn handle_captures(captures: &regex::Captures) -> Result<Self, String> {
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

    fn tokenize_all(unparsed: &str, max_nr_tags: usize) -> Vec<Result<Self, String>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"\[demver\((.+?)\)\|(.+?)\|(.+?)\]\s([^\s]+)\s@\s([^\s]+)").unwrap();
        }

        let mut ret = Vec::new();

        for cap in RE.captures_iter(unparsed) {
            ret.push(Self::handle_captures(&cap));
            if max_nr_tags != 0 && ret.len() >= max_nr_tags {
                break
            }
        }

        ret
    }

    fn tokenize_one(unparsed: &str) -> Result<Self, String> {
        let mut vec = Self::tokenize_all(unparsed, 1);

        if vec.len() < 1 {
            return Err("no match was found".to_owned());
        }

        vec.remove(0)
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
    fn parse(unparsed: &TokenizedTag) -> Result<Self, String> {
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
    
    const TEST_STRING: &str = "# [demver(^1.0.0)|file(fn=versions.ini,hash=sha512)|testapp] 1.0.0 @ 2020-12-05T18-18-09";

    #[test]
    fn tokenize_one_clean() {
        let tokenized_tag = TokenizedTag::tokenize_one(TEST_STRING).unwrap();

        assert_eq!(tokenized_tag.semver, "^1.0.0");
        assert_eq!(tokenized_tag.source, "file(fn=versions.ini,hash=sha512)");
        assert_eq!(tokenized_tag.identifier, "testapp");
        assert_eq!(tokenized_tag.current_version, "1.0.0");
        assert_eq!(tokenized_tag.timestamp, "2020-12-05T18-18-09");
    }

    #[test]
    fn tokenize_one_prefix_postfix() {
        let tokenized_tag = TokenizedTag::tokenize_one(&("foo bar ".to_owned() + TEST_STRING + " bla bla")).unwrap();

        assert_eq!(tokenized_tag.semver, "^1.0.0");
        assert_eq!(tokenized_tag.source, "file(fn=versions.ini,hash=sha512)");
        assert_eq!(tokenized_tag.identifier, "testapp");
        assert_eq!(tokenized_tag.current_version, "1.0.0");
        assert_eq!(tokenized_tag.timestamp, "2020-12-05T18-18-09");
    }

    #[test]
    fn tokenize_one_multiple_first() {
        let tokenized_tag = TokenizedTag::tokenize_one(&("foo bar ".to_owned() + TEST_STRING + " bla bla " + TEST_STRING)).unwrap();

        assert_eq!(tokenized_tag.semver, "^1.0.0");
        assert_eq!(tokenized_tag.source, "file(fn=versions.ini,hash=sha512)");
        assert_eq!(tokenized_tag.identifier, "testapp");
        assert_eq!(tokenized_tag.current_version, "1.0.0");
        assert_eq!(tokenized_tag.timestamp, "2020-12-05T18-18-09");
    }

    #[test]
    fn tokenize_all_clean() {
        let tokenized_tags = TokenizedTag::tokenize_all(&("foo bar ".to_owned() + TEST_STRING + " bla bla " + TEST_STRING), 0);

        assert_eq!(tokenized_tags.len(), 2);

        for tag in tokenized_tags {
            let unwrapped = tag.unwrap();
            assert_eq!(unwrapped.semver, "^1.0.0");
            assert_eq!(unwrapped.source, "file(fn=versions.ini,hash=sha512)");
            assert_eq!(unwrapped.identifier, "testapp");
            assert_eq!(unwrapped.current_version, "1.0.0");
            assert_eq!(unwrapped.timestamp, "2020-12-05T18-18-09");
        }
    }

    #[test]
    fn parse_demver_tag() {
        let semver = "^1.0.0";
        let source = "file(fn=versions.ini,hash=sha512)";
        let identifier = "testapp";
        let current_version = "1.0.0";
        let timestamp = "2020-12-05T18-18-09";
        
        DemverTag::parse(&TokenizedTag::new(
            semver,
            source,
            identifier,
            current_version,
            timestamp,
        )).unwrap();
    }
}

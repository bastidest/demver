use regex::Regex;

#[derive(Debug, Clone)]
pub struct TokenizedTag {
    version_req: String,
    version_source: String,
    identifier: String,
    current_version: String,
    timestamp: String,
    idx_start: usize,
    idx_end: usize,
    origin_filename: String,
}

impl TokenizedTag {
    fn new(
        version_req: &str,
        version_source: &str,
        identifier: &str,
        current_version: &str,
        timestamp: &str,
        idx_start: usize,
        idx_end: usize,
        origin_filename: &str,
    ) -> Self {
        Self {
            version_req: version_req.to_owned(),
            version_source: version_source.to_owned(),
            identifier: identifier.to_owned(),
            current_version: current_version.to_owned(),
            timestamp: timestamp.to_owned(),
            idx_start,
            idx_end,
            origin_filename: origin_filename.to_owned(),
        }
    }

    fn handle_captures(filename: &str, captures: &regex::Captures) -> Result<Self, String> {
        let entire_match = captures.get(0).unwrap();
        let start = entire_match.start();
        let end = entire_match.end();
        let semver = match captures.get(1) {
            Some(s) => s,
            None => return Err("Unable to extract the semver from demver tag".to_string()),
        };
        let version_source = match captures.get(2) {
            Some(s) => s,
            None => return Err("Unable to extract the version_source from demver tag".to_string()),
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
            version_source.as_str(),
            identifier.as_str(),
            current_version.as_str(),
            timestamp.as_str(),
            start,
            end,
            filename,
        ))
    }

    pub fn tokenize_all(
        filename: &str,
        unparsed: &str,
        max_nr_tags: usize,
    ) -> Vec<Result<Self, String>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"\[demver\((.+?)\)\|(.+?)\|(.+?)\]\s([^\s]+)\s@\s([^\s]+)").unwrap();
        }

        let mut ret = Vec::new();

        for cap in RE.captures_iter(unparsed) {
            ret.push(Self::handle_captures(filename, &cap));
            if max_nr_tags != 0 && ret.len() >= max_nr_tags {
                break;
            }
        }

        ret
    }

    fn tokenize_one(filename: &str, unparsed: &str) -> Result<Self, String> {
        let mut vec = Self::tokenize_all(filename, unparsed, 1);

        if vec.len() < 1 {
            return Err("no match was found".to_owned());
        }

        vec.remove(0)
    }
}

#[derive(Debug, PartialEq)]
pub struct FileSourceTag {
    pub filename: String,
}

impl FileSourceTag {
    fn parse(unparsed_arguments: &str) -> Result<Self, String> {
        match unparsed_arguments.len() {
            0 => Err("no filename was given".to_owned()),
            _ => Ok(FileSourceTag {
                filename: unparsed_arguments.to_owned(),
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SourceTag {
    File(FileSourceTag),
}

impl SourceTag {
    fn parse(unparsed: &str) -> Result<Self, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([^\(]+)(?:\((.+)\))?").unwrap();
        }

        let captures = match RE.captures(unparsed) {
            Some(c) => c,
            None => return Err("Source Tag could not be parsed".to_owned()),
        };

        let source_tag_type = captures.get(1).unwrap().as_str();
        let source_arguments = match captures.get(2) {
            Some(m) => m.as_str(),
            None => "",
        };

        match source_tag_type {
            "file" => {
                let file_source_tag = FileSourceTag::parse(source_arguments)?;
                Ok(SourceTag::File(file_source_tag))
            }
            t => Err(format!("unknown version_source tag type '{}'", t)),
        }
    }
}

#[derive(Debug)]
pub struct DemverTag {
    tokenized_tag: TokenizedTag,
    pub semver: semver::VersionReq,
    pub version_source: SourceTag,
    pub identifier: String,
    current_version: semver::Version,
    timestamp: String,
}

impl DemverTag {
    pub fn parse(unparsed: &TokenizedTag) -> Result<Self, String> {
        let semver = match semver::VersionReq::parse(&unparsed.version_req) {
            Ok(v) => v,
            Err(e) => return Err(format!("Failed to parse semver: {}", e)),
        };
        let version_source = SourceTag::parse(&unparsed.version_source)?;
        let identifier = unparsed.identifier.clone();
        let current_version = match semver::Version::parse(&unparsed.current_version) {
            Ok(v) => v,
            Err(e) => return Err(format!("Failed to parse semver: {}", e)),
        };
        let timestamp = unparsed.timestamp.clone();

        Ok(DemverTag {
            tokenized_tag: (*unparsed).clone(),
            semver,
            version_source,
            identifier,
            current_version,
            timestamp,
        })
    }

    pub fn get_raw_version_req(&self) -> &String {
        &self.tokenized_tag.version_req
    }

    pub fn get_raw_source(&self) -> &String {
        &self.tokenized_tag.version_source
    }

    pub fn get_origin_filename(&self) -> &String {
        &self.tokenized_tag.origin_filename
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRING: &str =
        "# [demver(^1.0.0)|file(versions.ini)|testapp] 1.0.0 @ 2020-12-05T18-18-09";
    const TEST_FILENAME: &str = "foobarfile.txt";
    #[test]
    fn tokenize_one_clean() {
        let sut = TokenizedTag::tokenize_one(TEST_FILENAME, TEST_STRING).unwrap();

        assert_eq!(sut.version_req, "^1.0.0");
        assert_eq!(sut.version_source, "file(versions.ini)");
        assert_eq!(sut.identifier, "testapp");
        assert_eq!(sut.current_version, "1.0.0");
        assert_eq!(sut.timestamp, "2020-12-05T18-18-09");
        assert_eq!(sut.idx_start, 2);
        assert_eq!(sut.idx_end, TEST_STRING.len());
    }

    #[test]
    fn tokenize_one_prefix_postfix() {
        let sut = TokenizedTag::tokenize_one(
            TEST_FILENAME,
            &("foo bar ".to_owned() + TEST_STRING + " bla bla"),
        )
        .unwrap();

        assert_eq!(sut.version_req, "^1.0.0");
        assert_eq!(sut.version_source, "file(versions.ini)");
        assert_eq!(sut.identifier, "testapp");
        assert_eq!(sut.current_version, "1.0.0");
        assert_eq!(sut.timestamp, "2020-12-05T18-18-09");
        assert_eq!(sut.idx_start, "foo bar ".len() + 2);
        assert_eq!(sut.idx_end, "foo bar ".len() + TEST_STRING.len());
    }

    #[test]
    fn tokenize_one_multiple_first() {
        let sut = TokenizedTag::tokenize_one(
            TEST_FILENAME,
            &("foo bar ".to_owned() + TEST_STRING + " bla bla " + TEST_STRING),
        )
        .unwrap();

        assert_eq!(sut.version_req, "^1.0.0");
        assert_eq!(sut.version_source, "file(versions.ini)");
        assert_eq!(sut.identifier, "testapp");
        assert_eq!(sut.current_version, "1.0.0");
        assert_eq!(sut.timestamp, "2020-12-05T18-18-09");
        assert_eq!(sut.idx_start, "foo bar ".len() + 2);
        assert_eq!(sut.idx_end, "foo bar ".len() + TEST_STRING.len());
    }

    #[test]
    fn tokenize_all_clean() {
        let sut = TokenizedTag::tokenize_all(
            TEST_FILENAME,
            &("foo bar ".to_owned() + TEST_STRING + " bla bla " + TEST_STRING),
            0,
        );

        assert_eq!(sut.len(), 2);

        for tag in &sut {
            let unwrapped = tag.as_ref().unwrap();
            assert_eq!(unwrapped.version_req, "^1.0.0");
            assert_eq!(unwrapped.version_source, "file(versions.ini)");
            assert_eq!(unwrapped.identifier, "testapp");
            assert_eq!(unwrapped.current_version, "1.0.0");
            assert_eq!(unwrapped.timestamp, "2020-12-05T18-18-09");
        }

        let sut1 = &sut[0].as_ref().unwrap();
        let sut2 = &sut[1].as_ref().unwrap();
        assert_eq!(sut1.idx_start, "foo bar ".len() + 2);
        assert_eq!(sut1.idx_end, "foo bar ".len() + TEST_STRING.len());
        assert_eq!(
            sut2.idx_start,
            "foo bar ".len() + TEST_STRING.len() + " bla bla ".len() + 2
        );
        assert_eq!(
            sut2.idx_end,
            "foo bar ".len() + TEST_STRING.len() + " bla bla ".len() + TEST_STRING.len()
        );
    }

    #[test]
    fn parse_demver_tag() {
        let version_req = "^1.0.0";
        let version_source = "file(versions.ini)";
        let identifier = "testapp";
        let current_version = "1.0.0";
        let timestamp = "2020-12-05T18-18-09";
        let sut = DemverTag::parse(&TokenizedTag::new(
            version_req,
            version_source,
            identifier,
            current_version,
            timestamp,
            0,
            0,
            TEST_FILENAME,
        ))
        .unwrap();

        // println!("{:?}", sut);
    }

    #[test]
    fn parse_file_source() {
        let version_source = "file(versions.ini)";

        let file_source = SourceTag::parse(version_source).unwrap();

        assert_eq!(
            file_source,
            SourceTag::File(FileSourceTag {
                filename: "versions.ini".to_owned()
            })
        )
    }

    #[test]
    fn parse_file_source_empty() {
        assert!(SourceTag::parse("file()").is_err());
    }

    #[test]
    fn parse_file_source_missing_attr() {
        assert!(SourceTag::parse("file").is_err());
    }
}

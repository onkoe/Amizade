use crate::types::{Command, OcsParsingError, ParsedOcsUrl, Scheme};

use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use url::Url;
use urlencoding::decode;

/// Takes in an `ocs://` URL and parses it. Provides a ParsedOcsUrl.
///
/// As-is, OCS links should be lowercase in their implementation.
/// ```
/// use ocs_custodian::parser::check_url;
/// assert!((check_url("OCS://INSTALL?URL=https%3A%2F%2Ffake.download%2Fa.mp3&TYPE=music").is_err()));
/// ```
///
pub fn check_url(url: String) -> Result<ParsedOcsUrl, OcsParsingError> {
    let decoded_url = decode(&url)?;
    let ocs_url = Url::parse(decoded_url.deref())?;

    // make a hashmap out of it
    let parameters: HashMap<String, String> = ocs_url.query_pairs().into_owned().collect();

    // create a new instance of ParsedOcsUrl to fill out
    let parsed_ocs_url = ParsedOcsUrl {
        ocs_url: ocs_url.clone(),
        scheme: ocs_url.scheme().try_into()?,
        command: match ocs_url.host_str() {
            None => return Err(OcsParsingError::NoOcsCommand),
            Some(cmd) => cmd.try_into()?,
        },
        download_url: match parameters.get("url") {
            None => return Err(OcsParsingError::NoDownloadUrl),
            Some(given) => url::Url::from_str(given)?,
        },
        install_type: match parameters.get("type") {
            None => return Err(OcsParsingError::NoInstallType),
            Some(install_type) => install_type.to_owned(), // TODO: do a prelim check if install type is known for installation
        },
        filename: parameters
            .get("filename")
            .map(|filename| filename.to_owned()),
    };

    Ok(parsed_ocs_url)
}

mod tests {
    #![allow(unused)]
    use crate::installer::install;
    use crate::parser::check_url;
    use crate::types::OcsParsingError;
    use crate::ParsedOcsUrl;
    use urlencoding::encode;
    use LinkParts::*;

    enum LinkParts {
        Scheme,
        Command,
        DownloadUrl,
        InstallType,
        Filename,
        NoChange,
    }

    /// Returns a good link... by default
    fn new_link(chosen: LinkParts, difference: &str) -> String {
        let mut scheme: &str = "ocs";
        let mut command: &str = "install";
        let mut download: String = "https%3A%2F%2Ffake.download%2Flocation.png".into(); //  :p
        let mut install_type: &str = "plasma_look_and_feel";
        let mut filename: String = "location55.png".into(); // :p

        match chosen {
            Scheme => scheme = difference,
            Command => command = difference,
            DownloadUrl => download = encode(difference).into(),
            InstallType => install_type = difference,
            Filename => filename = encode(difference).into(),
            NoChange => (),
        }

        format!("{scheme}://{command}?url={download}&type={install_type}&filename={filename}")
    }

    #[test]
    fn url_parse_good_link() {
        // A nice working link..!
        // Should download from https://fake.download/location.png
        let good_link = new_link(LinkParts::NoChange, "");
        let parsed_good_link = check_url(good_link.clone());

        // let's see if we can get our link back
        assert!(parsed_good_link.is_ok());
        assert_eq!(good_link, parsed_good_link.unwrap().to_string());
    }

    #[test]
    fn parse_bad_link() {
        // Evil, scary link.
        // Must be an error if they're like this :)
        assert!(check_url("sduigh:sdiguhcc8////s::;dij".into()).is_err());
    }

    #[test]
    fn parse_empty_link() {
        assert!(check_url("".into()).is_err());
    }

    #[test]
    fn url_parse_test_scheme() {
        // `abc` is not a vaild scheme!
        // Download: https://normal.link/with_thing.mp3
        let weird_scheme_link = "abc://download?url=https%3A%2F%2Fnormal.link%2Fwith_thing.mp3&type=music?filename=with_thing.mp3";
        let parsed_weird_link = check_url(weird_scheme_link);

        assert!(parsed_weird_link.is_err());
        assert_eq!(
            parsed_weird_link,
            Err(OcsParsingError::UnexpectedOcsScheme("abc".to_owned()))
        );

        // No scheme!
        let no_scheme_link = "download?url=https%3A%2F%2Fnormal.link%2Fwith_thing.mp3&type=music?filename=with_thing.mp3";
        let parsed_no_scheme = check_url(no_scheme_link);

        assert!(parsed_no_scheme.is_err());
        // Seems like NoOcsScheme won't be used much. Oh well!
        assert_eq!(
            parsed_no_scheme,
            Err(OcsParsingError::UrlParsingError(
                url::ParseError::RelativeUrlWithoutBase
            ))
        );

        // Extremely long scheme
        let looooong_scheme = "abcd".repeat(400);
        let long_scheme_link = format!("{looooong_scheme}://download?url=https%3A%2F%2Fnormal.link%2Fwith_thing.mp3&type=music?filename=with_thing.mp3");
        let parsed_long_scheme = check_url(long_scheme_link.as_str());

        assert!(parsed_long_scheme.is_err());
        assert_eq!(
            parsed_long_scheme,
            Err(OcsParsingError::UnexpectedOcsScheme(looooong_scheme))
        );

        // Crazy characters scheme
        let crazy_scheme_link = "#(*H(F*(DH*HS(*D))));ocs://\"://download?url=https%3A%2F%2Fnormal.link%2Fwith_thing.mp3&type=music?filename=with_thing.mp3";
        let parsed_crazy_scheme = check_url(crazy_scheme_link);

        assert!(parsed_crazy_scheme.is_err());
        // I don't think we'll ever predict that one...
    }
}

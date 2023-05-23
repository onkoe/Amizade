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
    fn parse_weird_link() {
        let weird = "abc";
        // A weird link
        // `abc` is (usually) not a valid part!
        assert_eq!(
            check_url(new_link(Scheme, weird)),
            Err(OcsParsingError::UnexpectedOcsScheme(weird.into()))
        );
        assert_eq!(
            check_url(new_link(Command, weird)),
            Err(OcsParsingError::UnexpectedOcsCommand(weird.into()))
        );
        assert_eq!(
            check_url(new_link(DownloadUrl, weird)),
            Err(OcsParsingError::UrlParsingError(
                url::ParseError::RelativeUrlWithoutBase
            ))
        );
        // ...
        // TODO: uncomment when check_url actually checks types
        // ...
        /*assert_eq!(
            check_url(new_link(InstallType, weird)),
            Err(OcsParsingError::UnknownInstallType(weird.into()))
        ); */
        assert!(check_url(new_link(Filename, weird)).is_ok());
    }

    #[test]
    fn parse_blank_link() {
        // A link with some missing section
        assert!(check_url(new_link(Scheme, "")).is_err());
        assert!(check_url(new_link(Command, "")).is_err());
        assert!(check_url(new_link(DownloadUrl, "")).is_err());
        // assert!(check_url(new_link(InstallType, "")).is_err()); TODO
        assert!(check_url(new_link(Filename, "")).is_ok());
    }

    #[test]
    fn parse_loooong_link() {
        let long = "abcd".repeat(400);

        // A link where some part is obviously too long.
        // (testing for a panic)
        assert!(check_url(new_link(Scheme, long.as_str())).is_err());
        assert!(check_url(new_link(Command, long.as_str())).is_err());
        assert!(check_url(new_link(DownloadUrl, format!("https://{long})").as_str())).is_ok());
        // assert!(check_url(new_link(InstallType, long.as_str())).is_err()); TODO
        assert!(check_url(new_link(Filename, long.as_str())).is_ok());
    }

    #[test]
    fn parse_crazy_link() {
        let crazy = "#(*H(F*(DH*HS(*D))));";

        // A link with some weiiiird characters
        // Again, the main goal is to just not panic. :)

        assert!(check_url(new_link(Scheme, crazy)).is_err());
        assert!(check_url(new_link(Command, crazy)).is_err());
        assert!(check_url(new_link(DownloadUrl, crazy)).is_err());
        // assert!(check_url(new_link(InstallType, crazy)).is_err()); TODO
        assert!(check_url(new_link(Filename, crazy)).is_ok());
    }
}

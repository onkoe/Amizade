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
pub fn check_url(url: &str) -> Result<ParsedOcsUrl, OcsParsingError> {
    let decoded_url = decode(url)?;
    let ocs_url = Url::parse(decoded_url.deref())?;

    // make a hashmap out of it
    let parameters: HashMap<String, String> = ocs_url.query_pairs().into_owned().collect();

    // create a new instance of ParsedOcsUrl to fill out
    let parsed_ocs_url = ParsedOcsUrl {
        ocs_url: ocs_url.clone(),
        scheme: match ocs_url.scheme().to_lowercase().as_str() {
            "ocs" => Scheme::Ocs,
            "ocss" => Scheme::Ocss,
            other_scheme => {
                return Err(OcsParsingError::UnexpectedOcsScheme(
                    other_scheme.to_owned(),
                ))
            }
        },
        command: {
            let pot = ocs_url.host_str();

            match pot {
                None => return Err(OcsParsingError::NoOcsCommand),
                Some(cmd) => match cmd.to_lowercase().as_str() {
                    "download" => Command::Download,
                    "install" => Command::Install,
                    other => return Err(OcsParsingError::UnexpectedOcsCommand(other.to_owned())),
                },
            }
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
    #[allow(unused)]
    use crate::parser::check_url;
    #[allow(unused)]
    use crate::types::OcsParsingError;
    #[allow(unused)]
    use crate::ParsedOcsUrl;

    #[test]
    fn url_parse_good_link() {
        // A nice working link..!
        // Should download from https://fake.download/location.png
        let good_link = "ocs://install?url=https%3A%2F%2Ffake.download%2Flocation.png&type=plasma_look_and_feel&filename=location55.png";
        let parsed_good_link = check_url("ocs://install?url=https%3A%2F%2Ffake.download%2Flocation.png&type=plasma_look_and_feel&filename=location55.png");

        // let's see if we can get our link back
        assert!(parsed_good_link.is_ok());
        assert_eq!(good_link, parsed_good_link.unwrap().to_string());
    }

    #[test]
    fn url_parse_bad_link() {
        // Evil, scary link.
        // Must be an error if they're like this :)
        let bad_link = "sduigh:sdiguhcc8////s::;dij";
        let parsed_bad_link = check_url(bad_link);

        assert!(parsed_bad_link.is_err());
    }

    #[test]
    fn url_parse_empty_link() {
        let parsed_empty_link = check_url("");
        assert!(parsed_empty_link.is_err());
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

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

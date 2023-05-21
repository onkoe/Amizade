pub mod handler {
    use std::collections::HashMap;
    use std::fmt::Display;
    use std::ops::Deref;
    use std::str::FromStr;
    use std::string::FromUtf8Error;
    use thiserror::Error;
    use url::Url;
    use urlencoding::decode;

    /// Represents one of many parsing errors that can occur when parsing OCS links.
    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum OcsParsingError {
        #[error(transparent)]
        UrlDecodeError(#[from] FromUtf8Error),
        #[error(transparent)]
        UrlParsingError(#[from] url::ParseError),
        #[error("No OCS Scheme was provided. Please try a link like: `ocs://...`")]
        NoOcsScheme,
        #[error("An unexpected OCS Scheme was provided: `{0}`. Instead, please use `ocs://...`")]
        UnexpectedOcsScheme(String),
        #[error("No OCS Command was provided. Try a link like: `ocs://install...`")]
        NoOcsCommand,
        #[error("An unexpected OCS Command was provided: `{0}`. Instead, please ask for either an `install` or a `download`.")]
        UnexpectedOcsCommand(String),
        #[error("An OCS link without a download URL was erroneously provided to the parser.")]
        NoDownloadUrl,
        #[error("An unknown install type was given: {0}")]
        UnknownInstallType(String),
        #[error("No install type was given.")]
        NoInstallType,
    }

    /// A representation of the most important elements of an OCS link.
    /// The original URL is included as `ocs_url`, and the download URL can
    /// be reached using the `download_url`.
    #[derive(Debug, PartialEq, Eq)]
    pub struct ParsedOcsUrl {
        ocs_url: Url, // the "full" url. e.g. ocs://etc
        scheme: Scheme,
        command: Command,
        download_url: Url,
        install_type: String, // include aliases
        filename: Option<String>,
    }

    impl ParsedOcsUrl {
        /// todo: get download link or something
        pub fn download() {
            todo!();
        }

        /// todo: return all info as json or whatever
        pub fn to_json() {
            todo!("probably gonna use serde..?");
        }

        /// todo: get screenshots if available
        pub fn get_screenshots() {
            todo!("no idea how to get these from Pling right now :(");
        }
    }

    impl Display for ParsedOcsUrl {
        /// Returns a ParsedOcsUrl back as a String.

        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}://{}?url={}&type={}",
                self.scheme,
                self.command,
                urlencoding::encode(self.download_url.as_str()),
                self.install_type,
            )?;

            // If we have a filename, add it to the link
            if self.filename.is_some() {
                write!(
                    f,
                    "&filename={}",
                    self.filename.as_ref().expect("test").as_str()
                )?;
            }

            // All good!
            Ok(())
        }
    }

    /// A representation of the OCS scheme. As of mid-2023, there's only ocs://
    /// available. ocss:// will represent a "secure" version of the protocol.
    #[derive(Debug, PartialEq, Eq)]
    enum Scheme {
        Ocs,
        Ocss,
    }

    impl Display for Scheme {
        /// Converts the enum to text.
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let text = match self {
                Scheme::Ocs => "ocs",
                Scheme::Ocss => "ocss",
            };

            write!(f, "{}", text)
        }
    }

    /// The intention of the URL - what the user asks you to do.
    /// Also known as a "host string" in general terms.
    #[derive(Debug, PartialEq, Eq)]
    enum Command {
        Download, // pass to client.
        Install,  // we must install it. indicate success/failure
    }

    impl Display for Command {
        /// Returns the command as a str.
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let text = match self {
                Command::Download => "download",
                Command::Install => "install",
            };

            write!(f, "{}", text)
        }
    }

    /// Takes in an `ocs://` URL and parses it. Provides a ParsedOcsUrl.
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
                        other_scheme.to_string(),
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
                        other => {
                            return Err(OcsParsingError::UnexpectedOcsCommand(other.to_string()))
                        }
                    },
                }
            },
            download_url: match parameters.get("url") {
                None => return Err(OcsParsingError::NoDownloadUrl),
                Some(given) => url::Url::from_str(given)?,
            },
            install_type: match parameters.get("type") {
                None => return Err(OcsParsingError::NoInstallType),
                Some(install_type) => install_type.to_string(), // TODO: do a prelim check if install type is known for installation
            },
            filename: parameters
                .get("filename")
                .map(|filename| filename.to_owned()),
        };

        Ok(parsed_ocs_url)
    }
}

pub mod installer {
    // TODO: make this do something
    pub fn install() {
        // todo!
    }
}

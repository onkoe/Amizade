use std::fmt::Display;
use std::string::FromUtf8Error;
use thiserror::Error;
use url::Url;

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
    pub ocs_url: Url, // the "full" url. e.g. ocs://etc
    pub scheme: Scheme,
    pub command: Command,
    pub download_url: Url,
    pub install_type: String, // include aliases
    pub filename: Option<String>,
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
pub enum Scheme {
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
pub enum Command {
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

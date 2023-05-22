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
    #[error(transparent)]
    InstallTypeError(#[from] InstallTypeError),
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

/// Represents which kind of file should be processed.
/// Helps when dealing with the many, MANY types of files Pling has to offer.
pub struct InstallType {
    install_type: dyn InstallStrategy,
}

/// Defines how any specific InstallType should install itself to the system.
trait InstallStrategy {
    fn get_install_path(&self) -> String;
}

/// Represents a failure to parse given install type data.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum InstallTypeError {
    #[error("No known install type matched the given prompt: {0}")]
    NoMatchingInstallType(String), // TODO
    #[error("The given alias, {0}, didn't fit any existing type.")]
    NoInstallTypeAlias(String),
}

// personal media
pub enum PersonalMedia {
    Bin,
    Books,
    Comics,
    Documents,
    Downloads,
    Music,
    Pictures,
    Videos,
    Wallpapers,
}

impl InstallStrategy for PersonalMedia {
    /// May use $APP_DATA to denote a place to sae files.
    /// $APP_DATA is defined as the where the application's configuration lives
    /// For example, `$XDG_DATA_HOME/amizade`, etc...
    // TODO: deal with $APP_DATA lmao
    fn get_install_path(&self) -> String {
        match self {
            PersonalMedia::Bin => "$HOME/.local/bin".to_owned(),
            PersonalMedia::Books => "$APP_DATA/books".to_owned(),
            PersonalMedia::Comics => "$APP_DATA/comics".to_owned(),
            PersonalMedia::Documents => "$HOME/Documents".to_owned(),
            PersonalMedia::Downloads => "$HOME/Downloads".to_owned(),
            PersonalMedia::Music => "$HOME/Music".to_owned(),
            PersonalMedia::Pictures => "$HOME/Pictures".to_owned(),
            PersonalMedia::Videos => "$HOME/Videos".to_owned(),
            PersonalMedia::Wallpapers => "$XDG_DATA_HOME/wallpapers".to_owned(),
        }
    }
}

// styling
pub enum Styling {
    ColorSchemes,
    Cursors,
    Emoticons,
    Fonts,
    Icons,
    Themes,
}

impl InstallStrategy for Styling {
    fn get_install_path(&self) -> String {
        match self {
            Styling::ColorSchemes => "$XDG_DATA_HOME/color-schemes".to_owned(),
            Styling::Cursors => "$HOME/.icons".to_owned(), // TODO: i think this should be `$HOME/.local/share/icons/` or `$XDG_DATA_HOME/icons`
            Styling::Emoticons => "$XDG_DATA_HOME/emoticons".to_owned(),
            Styling::Fonts => "$HOME/.fonts".to_owned(), // TODO: `$XDG_DATA_HOME/fonts/`
            Styling::Icons => "$XDG_DATA_HOME/icons".to_owned(), // TODO: `$XDG_DATA_HOME/icons/`
            Styling::Themes => "$HOME/.themes".to_owned(), // TODO: `$XDG_DATA_HOME/themes/`
        }
    }
}

// wm themes
pub enum WMThemes {
    CairoClockThemes,
    CinnamonApplets,
    CinnamonDesklets,
    CinnamonExtensions,
    EmeraldThemes,
    EnlightenmentBackgrounds,
    EnlightenmentThemes,
    FluxboxStyles,
    GNOMEShellExtensions,
    IceWMThemes,
    PekWMThemes,
}

impl InstallStrategy for WMThemes {
    fn get_install_path(&self) -> String {
        match self {
            WMThemes::CairoClockThemes => "$HOME/.cairo-clock/themes".to_owned(),
            WMThemes::CinnamonApplets => "$XDG_DATA_HOME/cinnamon/applets".to_owned(),
            WMThemes::CinnamonDesklets => "$XDG_DATA_HOME/cinnamon/desklets".to_owned(),
            WMThemes::CinnamonExtensions => "$XDG_DATA_HOME/cinnamon/extensions".to_owned(),
            WMThemes::EmeraldThemes => "$HOME/.emerald/themes".to_owned(),
            WMThemes::EnlightenmentBackgrounds => "$HOME/.e/e/backgrounds".to_owned(),
            WMThemes::EnlightenmentThemes => "$HOME/.e/e/themes".to_owned(),
            WMThemes::FluxboxStyles => "$HOME/.fluxbox/styles".to_owned(),
            WMThemes::GNOMEShellExtensions => "$XDG_DATA_HOME/gnome-shell/extensions".to_owned(),
            WMThemes::IceWMThemes => "$HOME/.icewm/themes".to_owned(),
            WMThemes::PekWMThemes => "$HOME/.pekwm/themes".to_owned(),
        }
    }
}

// kde themes
pub enum QtGeneral {
    AmarokScripts,
    AuroraeThemes,
    DekoratorThemes,
    KwinEffects,
    KwinScripts,
    KwinTabbox,
    PlasmaDesktopthemes,
    PlasmaLookAndFeel,
    PlasmaPlasmoids,
    QtCurve,
    YakuakeSkins,
}

impl InstallStrategy for QtGeneral {
    fn get_install_path(&self) -> String {
        match self {
            QtGeneral::AmarokScripts => "$KDEHOME/share/apps/amarok/scripts".to_owned(),
            QtGeneral::AuroraeThemes => "$XDG_DATA_HOME/aurorae/themes".to_owned(),
            QtGeneral::DekoratorThemes => "$XDG_DATA_HOME/deKorator/themes".to_owned(),
            QtGeneral::KwinEffects => "$XDG_DATA_HOME/kwin/effects".to_owned(),
            QtGeneral::KwinScripts => "$XDG_DATA_HOME/kwin/scripts".to_owned(),
            QtGeneral::KwinTabbox => "$XDG_DATA_HOME/kwin/tabbox".to_owned(),
            QtGeneral::PlasmaDesktopthemes => "$XDG_DATA_HOME/plasma/desktoptheme".to_owned(),
            QtGeneral::PlasmaLookAndFeel => "$XDG_DATA_HOME/plasma/look-and-feel".to_owned(),
            QtGeneral::PlasmaPlasmoids => "$XDG_DATA_HOME/plasma/plasmoids".to_owned(),
            QtGeneral::QtCurve => "$XDG_DATA_HOME/QtCurve".to_owned(),
            QtGeneral::YakuakeSkins => "$KDEHOME/share/apps/yakuake/skins".to_owned(),
        }
    }
}

// application specific
pub enum AppSpecific {
    NautiliusScripts,
}

impl InstallStrategy for AppSpecific {
    fn get_install_path(&self) -> String {
        match self {
            Self::NautiliusScripts => "$XDG_DATA_HOME/nautilus/scripts".to_owned(),
        }
    }
}
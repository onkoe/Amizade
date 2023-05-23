// This module handles the spammy install types for `ocs://` links on Pling.
//
// It may be a better idea to use `build.rs` and dynamically create a file from
// a given CSV. However, I don't like the fact that this could affect
// testing and make development a bit less obvious.
//
// If you think that you have a good way to do this without affecting runtime
// performance, please let me know in an issue. I'd love to take a look!
use thiserror::Error;

/// Represents which kind of file should be processed.
/// Helps when dealing with the many, MANY types of files Pling has to offer.
pub struct InstallType {
    install_type: dyn InstallStrategy,
}

/// Helps define how any specific InstallType should install itself to the system.
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
    /// May use $APP_DATA to denote a place to save files.
    /// $APP_DATA is defined as the where the application's configuration lives
    /// For example, `$XDG_DATA_HOME/amizade`, etc...
    // TODO: deal with $APP_DATA lmao
    fn get_install_path(&self) -> String {
        match self {
            Self::Bin => "$HOME/.local/bin".to_owned(),
            Self::Books => "$APP_DATA/books".to_owned(),
            Self::Comics => "$APP_DATA/comics".to_owned(),
            Self::Documents => "$HOME/Documents".to_owned(),
            Self::Downloads => "$HOME/Downloads".to_owned(),
            Self::Music => "$HOME/Music".to_owned(),
            Self::Pictures => "$HOME/Pictures".to_owned(),
            Self::Videos => "$HOME/Videos".to_owned(),
            Self::Wallpapers => "$XDG_DATA_HOME/wallpapers".to_owned(),
        }
    }
}

impl TryFrom<&str> for PersonalMedia {
    type Error = InstallTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "bin" => Ok(Self::Bin),
            "books" => Ok(Self::Books),
            "comics" => Ok(Self::Comics),
            "documents" => Ok(Self::Documents),
            "downloads" => Ok(Self::Downloads),
            "music" => Ok(Self::Music),
            "pictures" => Ok(Self::Pictures),
            "videos" => Ok(Self::Videos),
            "wallpapers" => Ok(Self::Wallpapers),
            other => Err(InstallTypeError::NoMatchingInstallType(other.into())),
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
            Self::ColorSchemes => "$XDG_DATA_HOME/color-schemes".to_owned(),
            Self::Cursors => "$HOME/.icons".to_owned(), // TODO: i think this should be `$HOME/.local/share/icons/` or `$XDG_DATA_HOME/icons`
            Self::Emoticons => "$XDG_DATA_HOME/emoticons".to_owned(),
            Self::Fonts => "$HOME/.fonts".to_owned(), // TODO: `$XDG_DATA_HOME/fonts/`
            Self::Icons => "$XDG_DATA_HOME/icons".to_owned(), // TODO: `$XDG_DATA_HOME/icons/`
            Self::Themes => "$HOME/.themes".to_owned(), // TODO: `$XDG_DATA_HOME/themes/`
        }
    }
}

impl TryFrom<&str> for Styling {
    type Error = InstallTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "color_schemes" => Ok(Self::ColorSchemes),
            "cursors" => Ok(Self::Cursors),
            "emoticons" => Ok(Self::Emoticons),
            "fonts" => Ok(Self::Fonts),
            "icons" => Ok(Self::Icons),
            "themes" => Ok(Self::Themes),
            other => Err(InstallTypeError::NoMatchingInstallType(other.into())),
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
            Self::CairoClockThemes => "$HOME/.cairo-clock/themes".to_owned(),
            Self::CinnamonApplets => "$XDG_DATA_HOME/cinnamon/applets".to_owned(),
            Self::CinnamonDesklets => "$XDG_DATA_HOME/cinnamon/desklets".to_owned(),
            Self::CinnamonExtensions => "$XDG_DATA_HOME/cinnamon/extensions".to_owned(),
            Self::EmeraldThemes => "$HOME/.emerald/themes".to_owned(),
            Self::EnlightenmentBackgrounds => "$HOME/.e/e/backgrounds".to_owned(),
            Self::EnlightenmentThemes => "$HOME/.e/e/themes".to_owned(),
            Self::FluxboxStyles => "$HOME/.fluxbox/styles".to_owned(),
            Self::GNOMEShellExtensions => "$XDG_DATA_HOME/gnome-shell/extensions".to_owned(),
            Self::IceWMThemes => "$HOME/.icewm/themes".to_owned(),
            Self::PekWMThemes => "$HOME/.pekwm/themes".to_owned(),
        }
    }
}

impl TryFrom<&str> for WMThemes {
    type Error = InstallTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "cairo_clock_themes" => Ok(Self::CairoClockThemes),
            "cinnamon_applets" => Ok(Self::CinnamonApplets),
            "cinnamon_desklets" => Ok(Self::CinnamonDesklets),
            "cinnamon_extensions" => Ok(Self::CinnamonExtensions),
            "emerald_themes" => Ok(Self::EmeraldThemes),
            "enlightenment_backgrounds" => Ok(Self::EnlightenmentBackgrounds),
            "enlightenment_themes" => Ok(Self::EnlightenmentThemes),
            "fluxbox_styles" => Ok(Self::FluxboxStyles),
            "gnome_shell_extensions" => Ok(Self::GNOMEShellExtensions),
            "icewm_themes" => Ok(Self::IceWMThemes),
            "pekwm_themes" => Ok(Self::PekWMThemes),
            other => Err(InstallTypeError::NoMatchingInstallType(other.into())),
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
            Self::AmarokScripts => "$KDEHOME/share/apps/amarok/scripts".to_owned(),
            Self::AuroraeThemes => "$XDG_DATA_HOME/aurorae/themes".to_owned(),
            Self::DekoratorThemes => "$XDG_DATA_HOME/deKorator/themes".to_owned(),
            Self::KwinEffects => "$XDG_DATA_HOME/kwin/effects".to_owned(),
            Self::KwinScripts => "$XDG_DATA_HOME/kwin/scripts".to_owned(),
            Self::KwinTabbox => "$XDG_DATA_HOME/kwin/tabbox".to_owned(),
            Self::PlasmaDesktopthemes => "$XDG_DATA_HOME/plasma/desktoptheme".to_owned(),
            Self::PlasmaLookAndFeel => "$XDG_DATA_HOME/plasma/look-and-feel".to_owned(),
            Self::PlasmaPlasmoids => "$XDG_DATA_HOME/plasma/plasmoids".to_owned(),
            Self::QtCurve => "$XDG_DATA_HOME/QtCurve".to_owned(),
            Self::YakuakeSkins => "$KDEHOME/share/apps/yakuake/skins".to_owned(),
        }
    }
}

impl TryFrom<&str> for QtGeneral {
    type Error = InstallTypeError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "" => Ok(Self::AmarokScripts),
            "" => Ok(Self::AuroraeThemes),
            "" => Ok(Self::DekoratorThemes),
            "" => Ok(Self::KwinEffects),
            "" => Ok(Self::KwinScripts),
            "" => Ok(Self::KwinTabbox),
            "" => Ok(Self::PlasmaDesktopthemes),
            "" => Ok(Self::PlasmaLookAndFeel),
            "" => Ok(),
            "" => Ok(),
            "" => Ok(),
            other => Err(InstallTypeError::NoMatchingInstallType(other.into())),
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

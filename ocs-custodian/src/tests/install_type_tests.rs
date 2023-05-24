#![allow(unused)]
use crate::types::install_type::{
    self, PersonalMedia::*, QtGeneral::*, Styling::*, WMThemes::*, *,
};

#[test]
fn some_try_from_action() {
    // personal media
    assert_eq!(PersonalMedia::try_from("bin"), Ok(Bin));
    assert_eq!(PersonalMedia::try_from("music"), Ok(Music));
    assert_eq!(PersonalMedia::try_from("pictures"), Ok(Pictures));
    assert!(PersonalMedia::try_from("farts").is_err());

    // styling
    assert_eq!(Styling::try_from("xfwm4_themes"), Ok(Themes));
    assert_eq!(Styling::try_from("openbox_themes"), Ok(Themes));
    assert_eq!(Styling::try_from("themes"), Ok(Themes));
    assert_eq!(Styling::try_from("icons"), Ok(Icons));
    assert!(Styling::try_from("bigger farts").is_err());
}

#[test]
fn install_paths_sheeeesh() {
    assert_eq!(
        GNOMEShellExtensions.get_install_path(),
        "$XDG_DATA_HOME/gnome-shell/extensions"
    );
    assert_eq!(KwinTabbox.get_install_path(), "$XDG_DATA_HOME/kwin/tabbox");
    assert_eq!(
        AppSpecific::NautiliusScripts.get_install_path(),
        "$XDG_DATA_HOME/nautilus/scripts"
    )
}

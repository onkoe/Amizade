#![allow(dead_code)]
use urlencoding::encode;
use LinkParts::*;

pub enum LinkParts {
    Scheme,
    Command,
    DownloadUrl,
    InstallType,
    Filename,
    NoChange,
}

/// Returns a good link... by default
pub fn new_link(chosen: LinkParts, difference: &str) -> String {
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

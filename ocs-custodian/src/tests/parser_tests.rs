#![allow(unused)]
use crate::parser::check_url;
use crate::tests::test_helpers::{new_link, LinkParts, LinkParts::*};
use crate::types::OcsParsingError;
use urlencoding::encode;

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

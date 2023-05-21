#[cfg(test)]
mod handler_tests {
    use handler::OcsParsingError;

    #[test]
    fn url_parse_good_link() {
        // A nice working link..!
        // Should download from https://fake.download/location.png
        let good_link = "ocs://install?url=https%3A%2F%2Ffake.download%2Flocation.png&type=plasma_look_and_feel&filename=location55.png";
        let parsed_good_link = ocs_custodian::handler::check_url("ocs://install?url=https%3A%2F%2Ffake.download%2Flocation.png&type=plasma_look_and_feel&filename=location55.png");

        // let's see if we can get our link back
        assert!(parsed_good_link.is_ok());
        assert_eq!(good_link, parsed_good_link.unwrap().to_string());
    }

    #[test]
    fn url_parse_bad_link() {
        // Evil, scary link.
        // Must be an error if they're like this :)
        let bad_link = "sduigh:sdiguhcc8////s::;dij";
        let parsed_bad_link = ocs_custodian::handler::check_url(bad_link);

        assert!(parsed_bad_link.is_err());
    }

    #[test]
    fn url_parse_test_scheme() {
        // `abc` is not a vaild scheme!
        // Download: https://normal.link/with_thing.mp3
        let weird_scheme_link = "abc://download?url=https%3A%2F%2Fnormal.link%2Fwith_thing.mp3&type=music?filename=with_thing.mp3";
        let parsed_weird_link = ocs_custodian::handler::check_url(weird_scheme_link);

        assert!(parsed_weird_link.is_err());
        assert_eq!(
            parsed_weird_link,
            Err(OcsParsingError::UnexpectedOcsScheme("abc".to_string()))
        );

        // No scheme!
        let no_scheme_link = "download?url=https%3A%2F%2Fnormal.link%2Fwith_thing.mp3&type=music?filename=with_thing.mp3";
        let parsed_no_scheme = ocs_custodian::handler::check_url(no_scheme_link);

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
        let parsed_long_scheme = ocs_custodian::handler::check_url(long_scheme_link.as_str());

        assert!(parsed_long_scheme.is_err());
        assert_eq!(
            parsed_long_scheme,
            Err(OcsParsingError::UnexpectedOcsScheme(looooong_scheme))
        );

        // Crazy characters scheme
    }
}

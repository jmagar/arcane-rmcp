use super::*;
use crate::config::ArcaneConfig;

#[test]
fn client_requires_arcane_config() {
    let result = ArcaneClient::new(&ArcaneConfig::default());
    assert!(result.is_err());
    let message = result
        .err()
        .expect("missing config should error")
        .to_string();
    assert!(message.contains("RUSTCANE_API_URL"));
}

#[test]
fn base_url_normalizes_api_suffix() {
    assert_eq!(
        normalize_base_url("https://arcane.test"),
        "https://arcane.test/api"
    );
    assert_eq!(
        normalize_base_url("https://arcane.test/api/"),
        "https://arcane.test/api"
    );
}

#[test]
fn path_segments_are_percent_encoded() {
    assert_eq!(encode_path_segment("abc/def"), "abc%2Fdef");
    assert_eq!(encode_path_segment("nginx:latest"), "nginx%3Alatest");
}

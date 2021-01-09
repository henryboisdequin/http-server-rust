use assert_cmd::prelude::*;
use reqwest::blocking::get;
use std::fs;
use std::process::Command;

// 'cargo test'
#[test]
fn test_default_path() {
    let expected_response =
        fs::read_to_string(format!("{}/public/index.html", env!("CARGO_MANIFEST_DIR"))).unwrap();
    let mut test_bin = Command::cargo_bin("server").unwrap().spawn().unwrap();
    let response = get("http://127.0.0.1:8080/").unwrap().text().unwrap();

    test_bin.kill().unwrap();
    assert_eq!(expected_response, response);
}

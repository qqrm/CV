use sitegen::{InlineStartError, read_inline_start};
use std::env;
use std::fs;

#[test]
#[serial_test::serial]
fn reads_inline_start_from_markdown() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    fs::create_dir_all("profiles/cv/en").unwrap();
    fs::write("profiles/cv/en/CV.MD", "* March 2024 - Present").unwrap();
    let result = read_inline_start();
    env::set_current_dir(original).unwrap();
    assert_eq!(result.unwrap(), (2024, 3));
}

#[test]
#[serial_test::serial]
fn read_inline_start_returns_error_for_invalid_file() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    fs::create_dir_all("profiles/cv/en").unwrap();
    fs::write("profiles/cv/en/CV.MD", "* Not a valid entry").unwrap();
    let result = read_inline_start();
    env::set_current_dir(original).unwrap();
    let err = result.expect_err("expected parse error");
    assert!(matches!(err, InlineStartError::Parse));
    assert_eq!(err.to_string(), "could not parse inline start");
}

#[test]
#[serial_test::serial]
fn read_inline_start_returns_error_for_invalid_month() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    fs::create_dir_all("profiles/cv/en").unwrap();
    fs::write("profiles/cv/en/CV.MD", "* Smarch 2024 - Present").unwrap();
    let result = read_inline_start();
    env::set_current_dir(original).unwrap();
    let err = result.expect_err("expected parse error");
    assert!(matches!(err, InlineStartError::Parse));
    assert_eq!(err.to_string(), "could not parse inline start");
}

#[test]
#[serial_test::serial]
fn read_inline_start_returns_error_when_file_missing() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    let result = read_inline_start();
    env::set_current_dir(original).unwrap();
    let err = result.expect_err("expected io error");
    assert!(matches!(err, InlineStartError::Io(_)));
    assert_eq!(err.to_string(), "failed to read profiles/cv/en/CV.MD");
}

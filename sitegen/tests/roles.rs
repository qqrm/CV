use sitegen::{RolesError, read_roles};
use std::env;
use std::fs;

#[test]
#[serial_test::serial]
fn read_roles_returns_default_when_missing() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    let roles = read_roles().expect("default roles");
    env::set_current_dir(original).unwrap();
    assert_eq!(roles.get("em"), Some(&"Engineering Manager".to_string()));
    assert_eq!(roles.len(), 1);
}

#[test]
#[serial_test::serial]
fn read_roles_merges_with_defaults() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    fs::write("roles.toml", "[roles]\nfoo='Bar'").unwrap();
    let roles = read_roles().expect("merged roles");
    env::set_current_dir(original).unwrap();
    assert_eq!(roles.get("em"), Some(&"Engineering Manager".to_string()));
    assert_eq!(roles.get("foo"), Some(&"Bar".to_string()));
    assert_eq!(roles.len(), 2);
}

#[test]
#[serial_test::serial]
fn read_roles_returns_error_for_invalid_file() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    fs::write("roles.toml", "[roles]\ninvalid").unwrap();
    let err = read_roles().expect_err("expected parse error");
    env::set_current_dir(original).unwrap();
    assert!(matches!(err, RolesError::Parse(_)));
    assert_eq!(err.to_string(), "could not parse roles.toml");
}

#[test]
#[serial_test::serial]
fn read_roles_returns_error_for_empty_title() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    fs::write("roles.toml", "[roles]\nem = ''").unwrap();
    let err = read_roles().expect_err("expected empty title error");
    env::set_current_dir(original).unwrap();
    match &err {
        RolesError::EmptyTitle { slug } => assert_eq!(slug, "em"),
        _ => panic!("unexpected error"),
    }
    assert_eq!(err.to_string(), "role 'em' has empty title");
}

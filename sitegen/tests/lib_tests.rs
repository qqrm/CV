use sitegen::{
    month_from_en,
    month_from_ru,
    read_inline_start,
    read_roles,
    InlineStartError,
    RolesError,
};
use std::env;
use std::fs;

#[test]
fn parses_english_months() {
    let months = [
        ("January", 1),
        ("February", 2),
        ("March", 3),
        ("April", 4),
        ("May", 5),
        ("June", 6),
        ("July", 7),
        ("August", 8),
        ("September", 9),
        ("October", 10),
        ("November", 11),
        ("December", 12),
    ];
    for (name, number) in months {
        assert_eq!(month_from_en(name), Some(number));
    }
}

#[test]
fn parses_russian_months() {
    let months = [
        ("Январь", 1),
        ("Февраль", 2),
        ("Март", 3),
        ("Апрель", 4),
        ("Май", 5),
        ("Июнь", 6),
        ("Июль", 7),
        ("Август", 8),
        ("Сентябрь", 9),
        ("Октябрь", 10),
        ("Ноябрь", 11),
        ("Декабрь", 12),
    ];
    for (name, number) in months {
        assert_eq!(month_from_ru(name), Some(number));
    }
}

#[test]
fn unknown_months_return_none() {
    assert_eq!(month_from_en("Smarch"), None);
    assert_eq!(month_from_ru("Смарч"), None);
}

#[test]
#[serial_test::serial]
fn reads_inline_start_from_markdown() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    fs::write("cv.md", "* March 2024 – Present").unwrap();
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
    fs::write("cv.md", "* Not a valid entry").unwrap();
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
    fs::write("cv.md", "* Smarch 2024 – Present").unwrap();
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
    assert_eq!(err.to_string(), "failed to read cv.md");
}

#[test]
#[serial_test::serial]
fn read_roles_returns_default_when_missing() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    let roles = read_roles().expect("default roles");
    env::set_current_dir(original).unwrap();
    assert_eq!(roles.get("tl"), Some(&"Team Lead".to_string()));
    assert_eq!(roles.get("tech"), Some(&"Tech Lead".to_string()));
    assert_eq!(roles.len(), 2);
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
    assert_eq!(roles.get("tl"), Some(&"Team Lead".to_string()));
    assert_eq!(roles.get("foo"), Some(&"Bar".to_string()));
    assert_eq!(roles.len(), 3);
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
    fs::write("roles.toml", "[roles]\ntl = ''").unwrap();
    let err = read_roles().expect_err("expected empty title error");
    env::set_current_dir(original).unwrap();
    match &err {
        RolesError::EmptyTitle { slug } => assert_eq!(slug, "tl"),
        _ => panic!("unexpected error"),
    }
    assert_eq!(err.to_string(), "role 'tl' has empty title");
}

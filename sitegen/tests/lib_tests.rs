use sitegen::{
    InlineStartError, RolesError, default_roles, month_from_en, month_from_ru, read_inline_start,
    read_roles,
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
fn read_inline_start_returns_error_for_invalid_file() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    fs::write("cv.md", "* Not a valid entry").unwrap();
    let result = read_inline_start();
    env::set_current_dir(original).unwrap();
    assert!(matches!(result, Err(InlineStartError::Parse)));
}

#[test]
fn read_inline_start_returns_error_when_file_missing() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    let result = read_inline_start();
    env::set_current_dir(original).unwrap();
    assert!(matches!(result, Err(InlineStartError::Io(_))));
}

#[test]
fn read_roles_returns_defaults_when_file_missing() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    let roles = read_roles().unwrap();
    env::set_current_dir(original).unwrap();
    assert_eq!(roles, default_roles());
}

#[test]
fn read_roles_parses_valid_file() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    fs::write("roles.toml", "[roles]\nfoo = \"Bar\"\n").unwrap();
    let roles = read_roles().unwrap();
    env::set_current_dir(original).unwrap();
    assert_eq!(roles.get("foo"), Some(&"Bar".to_string()));
}

#[test]
fn read_roles_returns_error_for_empty_title() {
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    fs::write("roles.toml", "[roles]\ntl = \"\"\n").unwrap();
    let result = read_roles();
    env::set_current_dir(original).unwrap();
    assert!(matches!(result, Err(RolesError::EmptyTitle(slug)) if slug == "tl"));
}

use sitegen::{month_from_en, month_from_ru, read_inline_start, InlineStartError};
use std::env;
use std::fs;
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn parses_english_month() {
    assert_eq!(month_from_en("March"), Some(3));
    assert_eq!(month_from_en("December"), Some(12));
}

#[test]
fn parses_russian_month() {
    assert_eq!(month_from_ru("Март"), Some(3));
    assert_eq!(month_from_ru("Декабрь"), Some(12));
}

#[test]
fn reads_inline_start_from_markdown() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    fs::write("cv.md", "* March 2024 – Present").unwrap();
    let result = read_inline_start();
    env::set_current_dir(original).unwrap();
    assert_eq!(result.unwrap(), (2024, 3));
}

#[test]
fn errors_when_file_missing() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    let result = read_inline_start();
    env::set_current_dir(original).unwrap();
    assert!(matches!(result, Err(InlineStartError::Io(_))));
}

#[test]
fn errors_on_invalid_format() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let dir = tempfile::tempdir().expect("temp dir");
    let original = env::current_dir().unwrap();
    env::set_current_dir(dir.path()).unwrap();
    fs::write("cv.md", "* March 2024 - ???").unwrap();
    let result = read_inline_start();
    env::set_current_dir(original).unwrap();
    assert!(matches!(result, Err(InlineStartError::InvalidFormat)));
}

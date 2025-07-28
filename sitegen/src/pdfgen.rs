use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

fn run_pandoc(src: &Path, dest: &Path, fmt: &str) -> io::Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    let status = Command::new("pandoc")
        .arg(src)
        .arg("-t")
        .arg(fmt)
        .arg("-s")
        .arg("-o")
        .arg(dest)
        .status()?;
    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "pandoc failed"));
    }
    Ok(())
}

fn ensure_symlink(path: &Path, target: &Path) -> io::Result<()> {
    if path.exists() {
        if path.is_symlink() {
            if let Ok(current) = fs::read_link(path) {
                let abs_current = fs::canonicalize(path.parent().unwrap().join(&current))?;
                let abs_target = fs::canonicalize(target)?;
                if abs_current == abs_target {
                    return Ok(());
                }
            }
        }
        fs::remove_file(path)?;
    }
    let rel = pathdiff::diff_paths(target, path.parent().unwrap())
        .unwrap_or_else(|| target.to_path_buf());
    #[cfg(unix)]
    std::os::unix::fs::symlink(&rel, path)?;
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&rel, path)?;
    Ok(())
}

pub fn generate() -> io::Result<()> {
    let root: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .into();
    let cv_files = [(root.join("cv.md"), "en"), (root.join("cv.ru.md"), "ru")];
    for (src, lang) in cv_files {
        run_pandoc(
            &src,
            &root
                .join("latex")
                .join(lang)
                .join(format!("Belyakov_{}.tex", lang)),
            "latex",
        )?;
        run_pandoc(
            &src,
            &root
                .join("typst")
                .join(lang)
                .join(format!("Belyakov_{}.typ", lang)),
            "typst",
        )?;
    }
    let avatar_src = root.join("content/avatar.jpg");
    for lang in ["en", "ru"] {
        ensure_symlink(
            &root.join("typst").join(lang).join("avatar.jpg"),
            &avatar_src,
        )?;
    }
    Ok(())
}

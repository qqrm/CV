use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const BASE_URL: &str = "https://qqrm.github.io/CV/";
const ROUTE_PATHS: [&str; 6] = ["", "ru/", "rust-developer/", "rust-developer/ru/", "cto/", "cto/ru/"];
const PDF_MAPPINGS: [(&str, &str); 12] = [
    ("typst/en/Belyakov_en_light.pdf", "Belyakov_en_light.pdf"),
    ("typst/en/Belyakov_en_dark.pdf", "Belyakov_en_dark.pdf"),
    ("typst/ru/Belyakov_ru_light.pdf", "Belyakov_ru_light.pdf"),
    ("typst/ru/Belyakov_ru_dark.pdf", "Belyakov_ru_dark.pdf"),
    (
        "typst/en/Belyakov_rustdev_en_light.pdf",
        "Belyakov_rustdev_en_light.pdf",
    ),
    (
        "typst/en/Belyakov_rustdev_en_dark.pdf",
        "Belyakov_rustdev_en_dark.pdf",
    ),
    (
        "typst/ru/Belyakov_rustdev_ru_light.pdf",
        "Belyakov_rustdev_ru_light.pdf",
    ),
    (
        "typst/ru/Belyakov_rustdev_ru_dark.pdf",
        "Belyakov_rustdev_ru_dark.pdf",
    ),
    ("typst/en/Belyakov_cto_en_light.pdf", "Belyakov_cto_en_light.pdf"),
    ("typst/en/Belyakov_cto_en_dark.pdf", "Belyakov_cto_en_dark.pdf"),
    ("typst/ru/Belyakov_cto_ru_light.pdf", "Belyakov_cto_ru_light.pdf"),
    ("typst/ru/Belyakov_cto_ru_dark.pdf", "Belyakov_cto_ru_dark.pdf"),
];

fn parse_out_dir(args: &[String]) -> Result<PathBuf, String> {
    let mut out = PathBuf::from("dist-assets");
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let value = args
                    .get(i + 1)
                    .ok_or_else(|| String::from("missing value for --out"))?;
                out = PathBuf::from(value);
                i += 2;
            }
            unknown => {
                return Err(format!(
                    "unknown argument: {unknown} (usage: generate [--out <DIR>])"
                ));
            }
        }
    }

    Ok(out)
}

fn ensure_valid_pdf(path: &Path) -> Result<(), Box<dyn Error>> {
    let bytes = fs::read(path)?;
    if bytes.len() < 4 || &bytes[..4] != b"%PDF" {
        return Err(format!("invalid PDF signature: {}", path.display()).into());
    }
    Ok(())
}

fn write_sitemap(out_dir: &Path) -> Result<(), Box<dyn Error>> {
    let content = ROUTE_PATHS
        .iter()
        .map(|path| format!("{BASE_URL}{path}"))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(out_dir.join("sitemap.txt"), format!("{content}\n"))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let out_dir = parse_out_dir(&args).map_err(|msg| format!("{msg}"))?;

    fs::create_dir_all(&out_dir)?;

    for (source, output_name) in PDF_MAPPINGS {
        let source_path = Path::new(source);
        if !source_path.exists() {
            return Err(format!("missing PDF: {}", source_path.display()).into());
        }
        ensure_valid_pdf(source_path)?;
        fs::copy(source_path, out_dir.join(output_name))?;
    }

    write_sitemap(&out_dir)?;
    Ok(())
}

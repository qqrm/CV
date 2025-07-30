use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CvJson {
    en_markdown: String,
    ru_markdown: String,
    roles: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct Roles {
    roles: BTreeMap<String, String>,
}

fn main() -> std::io::Result<()> {
    let en_markdown = fs::read_to_string("cv.md")?;
    let ru_markdown = fs::read_to_string("cv.ru.md")?;
    let roles_content = fs::read_to_string("roles.toml")?;
    let roles: Roles = toml::from_str(&roles_content).expect("invalid roles.toml");

    let cv = CvJson {
        en_markdown,
        ru_markdown,
        roles: roles.roles.clone(),
    };
    fs::write("cv.json", serde_json::to_string_pretty(&cv).unwrap())?;

    generate_templates(&cv)?;
    Ok(())
}

fn generate_templates(cv: &CvJson) -> std::io::Result<()> {
    let template_en = fs::read_to_string("typst/en/Belyakov_en.typ")?;
    let template_ru = fs::read_to_string("typst/ru/Belyakov_ru.typ")?;

    for (slug, role) in &cv.roles {
        let dir_te = Path::new("typst/en").join(slug);
        let dir_tr = Path::new("typst/ru").join(slug);
        fs::create_dir_all(&dir_te)?;
        fs::create_dir_all(&dir_tr)?;

        fs::write(dir_te.join("Belyakov_en.typ"), template_en.replace("Rust Team Lead", role))?;
        fs::write(dir_tr.join("Belyakov_ru.typ"), template_ru.replace("Rust Team Lead", role))?;
    }
    Ok(())
}

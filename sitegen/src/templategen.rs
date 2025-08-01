use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

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
    let base = fs::read_to_string("template/base.typ")?;

    let en_body = fs::read_to_string("typst/en/Belyakov_en.typ")?
        .lines()
        .skip(9)
        .collect::<Vec<_>>()
        .join("\n");
    let ru_body = fs::read_to_string("typst/ru/Belyakov_ru.typ")?
        .lines()
        .skip(9)
        .collect::<Vec<_>>()
        .join("\n");

    let template_en = base
        .replace("{{NAME}}", "Alexey Leonidovich Belyakov")
        .replace("{{BODY}}", &en_body);
    let template_ru = base
        .replace("{{NAME}}", "Алексей Леонидович Беляков")
        .replace("{{BODY}}", &ru_body);

    for (slug, role) in &cv.roles {
        let dir_te = Path::new("typst/en").join(slug);
        let dir_tr = Path::new("typst/ru").join(slug);
        fs::create_dir_all(&dir_te)?;
        fs::create_dir_all(&dir_tr)?;

        fs::write(
            dir_te.join("Belyakov_en.typ"),
            template_en.replace("{{ROLE}}", role),
        )?;
        fs::write(
            dir_tr.join("Belyakov_ru.typ"),
            template_ru.replace("{{ROLE}}", role),
        )?;
    }
    Ok(())
}

use sitegen::parser::RolesFile;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    fs::read_to_string("cv.md")?;
    fs::read_to_string("cv.ru.md")?;
    let content = fs::read_to_string("roles.toml")?;
    toml::from_str::<RolesFile>(&content)?;
    println!("Validation successful");
    Ok(())
}

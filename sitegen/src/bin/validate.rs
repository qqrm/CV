use log::info;
use sitegen::parser::RolesFile;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting validation");
    fs::read_to_string("cv.md")?;
    fs::read_to_string("cv.ru.md")?;
    let content = fs::read_to_string("roles.toml")?;
    toml::from_str::<RolesFile>(&content)?;
    info!("Validation successful");
    Ok(())
}

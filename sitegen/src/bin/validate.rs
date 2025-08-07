use log::info;
use sitegen::parser::read_roles;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting validation");
    fs::read_to_string("cv.md")?;
    fs::read_to_string("cv.ru.md")?;
    fs::read_to_string("roles.toml")?;
    read_roles()?;
    info!("Validation successful");
    Ok(())
}

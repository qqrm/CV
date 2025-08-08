use log::info;
use sitegen::parser::read_roles;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting validation");
    fs::read_to_string("CV.MD")?;
    fs::read_to_string("CV_RU.MD")?;
    fs::read_to_string("roles.toml")?;
    read_roles()?;
    info!("Validation successful");
    Ok(())
}

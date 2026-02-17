use log::info;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting validation");
    fs::read_to_string("profiles/cv/en/CV.MD")?;
    fs::read_to_string("profiles/cv/ru/CV_RU.MD")?;
    fs::read_to_string("profiles/rust-developer/en/CV.MD")?;
    fs::read_to_string("profiles/rust-developer/ru/CV_RU.MD")?;
    info!("Validation successful");
    Ok(())
}

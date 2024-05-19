use reqwest;
use std::error::Error;

pub fn request() -> Result<(), Box<dyn Error>> {
    let url = "http://www.rustinaction.com/";
    let mut response = reqwest::get(url)?;

    let content = response.text()?;
    println!("{}", content);

    Ok(())
}

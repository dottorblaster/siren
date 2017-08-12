use std::fs::File;
use std::io::Read;
use std::io::Error as IoError;
use std::error::Error;

#[macro_use]
extern crate serde_derive;

mod parse_config;
mod execute;

fn read_sirenfile() -> Result<String, IoError> {
    let mut sirenfile = File::open("Sirenfile.json")?;
    let mut string_json = String::new();
    sirenfile.read_to_string(&mut string_json)?;

    Ok(string_json)
}

fn main() {
    let configstring = match read_sirenfile() {
        Ok(jsoncontent) => jsoncontent,
        Err(err) => {
            println!("Error! {}", err.description());
            String::new()
        }
    };
    let conf = parse_config::string_to_config(configstring);
    execute::run(conf.tasks);
}

#[macro_use]
extern crate serde_derive;
extern crate clap;

use std::error::Error;
use std::fs::File;
use std::io::Error as IoError;
use std::io::Read;

use clap::{App, Arg};

mod execute;
mod parse_config;
mod task_output;

fn parentpath(path: String) -> String {
    let mut v: Vec<&str> = path.split("/").collect();
    let len = v.len();
    v.remove(len - 1);
    let retval: String = v.join("/");
    retval
}

fn read_sirenfile(sirenfile_path: String) -> Result<String, IoError> {
    let mut sirenfile = File::open(sirenfile_path)?;
    let mut string_json = String::new();
    sirenfile.read_to_string(&mut string_json)?;

    Ok(string_json)
}

fn main() {
    let matches = App::new("Siren")
        .version("1.1.1")
        .author("Alessio Biancalana <dottorblaster@gmail.com>")
        .about("Your tiny friendly rusty neighborhood monitoring CLI tool")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets a custom Sirenfile")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("json-output")
                .short("j")
                .long("json-output")
                .value_name("JSON")
                .help("Enable JSON output")
                .takes_value(false),
        )
        .get_matches();

    let sirenfile_path = matches
        .value_of("file")
        .unwrap_or("./Sirenfile.json")
        .to_owned();
    let output_json = matches.is_present("json-output");

    let configstring = match read_sirenfile(sirenfile_path) {
        Ok(jsoncontent) => jsoncontent,
        Err(err) => {
            println!("Error! {}", err.description());
            String::new()
        }
    };
    let conf = parse_config::string_to_config(configstring);
    let cwd_path = match conf.switch_cwd {
        true => parentpath(
            matches
                .value_of("file")
                .unwrap_or("./Sirenfile.json")
                .to_owned(),
        ),
        false => String::from("."),
    };
    execute::run(conf.tasks, cwd_path, output_json);
}

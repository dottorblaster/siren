use std::fs::File;
use std::io::Read;
use std::io::Error as IoError;

fn read_sirenfile() -> Result<String, IoError> {

    let mut sirenfile = File::open("Sirenfile.yml")?;
    let mut string_yaml = String::new();
    sirenfile.read_to_string(&mut string_yaml)?;

    Ok(string_yaml)
}

fn main() {
    let config = match read_sirenfile() {
        Ok(yamlcontent) => yamlcontent,
        Err(err) => String::new()
    };
    println!("{}", config);
}

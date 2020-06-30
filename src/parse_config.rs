extern crate serde;
extern crate serde_json;
use self::serde_json::Error;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub command: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub switch_cwd: bool,
    pub tasks: Vec<Task>,
}

fn configparse(confstring: String) -> Result<Config, Error> {
    let c: Config = serde_json::from_str(&confstring)?;
    Ok(c)
}

pub fn string_to_config(confstring: String) -> Config {
    let configuration = match configparse(confstring) {
        Ok(c) => c,
        Err(err) => {
            println!("Error parsing Sirenfile!\n{}", err.to_string());
            Config {
                switch_cwd: false,
                tasks: Vec::new(),
            }
        }
    };

    configuration
}

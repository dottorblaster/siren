extern crate serde;
extern crate serde_json;
use self::serde_json::Error;

#[derive(Serialize, Deserialize)]
pub struct Task {
    name: String,
    description: String,
    command: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    switch_cwd: bool,
    tasks: Vec<Task>,
}

fn configparse(confstring: String) -> Result<Config, Error> {
    let c: Config = serde_json::from_str(&confstring)?;
    Ok(c)
}

pub fn string_to_config(confstring: String) -> Config {
    let configuration = match configparse(confstring) {
        Ok(c) => c,
        Err(err) => Config { switch_cwd: true, tasks: Vec::new() }
    };

    println!("{}", configuration.switch_cwd);

    configuration
}

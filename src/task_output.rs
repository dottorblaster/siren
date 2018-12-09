extern crate serde;
extern crate serde_json;

#[derive(Serialize, Clone)]
pub struct TaskOutput {
    outcome: String,
    code: String,
    name: String,
    description: String,
    command: String,
}

pub type Tasks = Vec<TaskOutput>;

#[derive(Serialize, Clone)]
pub struct Output {
    pub tasks: Vec<TaskOutput>,
}

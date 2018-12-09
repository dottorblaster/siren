extern crate serde;
extern crate serde_json;

#[derive(Serialize, Clone)]
pub struct TaskOutput {
    pub outcome: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub command: String,
}

pub type Tasks = Vec<TaskOutput>;

#[derive(Serialize, Clone)]
pub struct SerializableOutput {
    pub tasks: Vec<TaskOutput>,
}

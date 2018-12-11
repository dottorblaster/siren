extern crate serde;
extern crate serde_json;

use parse_config::Task;
use std::process::Output;

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

pub fn build_task_output(output: Output, task: Task) -> TaskOutput {
    TaskOutput {
        outcome: String::from_utf8(output.stdout).unwrap(),
        code: output.status.code().unwrap().to_string(),
        name: task.name,
        description: task.description,
        command: task.command,
    }
}

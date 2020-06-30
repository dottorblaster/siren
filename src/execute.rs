extern crate ansi_term;
extern crate serde_json;

use parse_config::Task;
use std::fs;
use std::process::Command;
use std::process::Output;
use std::sync::{Arc, Mutex};
use std::thread;
use task_output;
use task_output::SerializableOutput;

use self::ansi_term::ANSIString;
use self::ansi_term::Colour::{Black, Green, Red, Yellow};

fn task_success(task: Task, output: Output, json: bool) {
    if json == false {
        let stdout = ANSIString::from(String::from_utf8(output.stdout).unwrap());
        println!(
            "{} {}\n{}\n",
            Black.bold().on(Green).paint("  SUCCESS  "),
            Yellow.paint(format!("{}", task.name)),
            stdout
        );
    }
}

fn task_failure(task: Task, output: Output, json: bool) {
    if json == false {
        let stderr = ANSIString::from(String::from_utf8(output.stderr).unwrap());
        println!(
            "{} {}\n{}\n",
            Black.bold().on(Red).paint("  FAIL  "),
            Yellow.paint(format!("{}", task.name)),
            stderr
        );
    }
}

fn print_json(outputs: Arc<Mutex<Vec<task_output::TaskOutput>>>, json: bool) {
    if json == true {
        let slice = &*outputs.lock().unwrap();
        let serializable_output = SerializableOutput {
            tasks: slice.to_vec(),
        };
        println!("{}", serde_json::to_string(&serializable_output).unwrap());
    }
}

pub fn run(tasks: Vec<Task>, cwd_path: String, json_output: bool) -> bool {
    let outputs = Arc::new(Mutex::new(task_output::Tasks::with_capacity(tasks.len())));
    let mut handles = Vec::with_capacity(tasks.len());
    println!("\n");
    for task in &tasks {
        let (data, path) = (task.clone(), cwd_path.clone());
        let outputs = Arc::clone(&outputs);
        let child = thread::spawn(move || {
            let local_task = data.clone();
            let task_data = data.clone();
            let mut iter = local_task.command.split_whitespace();
            let current_dir = path.clone();
            let command_canon_path = format!(
                "{:?}/{}",
                fs::canonicalize(path).unwrap(),
                iter.nth(0).unwrap()
            )
            .replace("\"", "");
            let command_output = Command::new(command_canon_path)
                .args(iter)
                .current_dir(current_dir)
                .output()
                .expect("command failed");
            let cloned_output = command_output.clone();
            let mut list = outputs.lock().unwrap();
            list.push(task_output::build_task_output(cloned_output, task_data));
            match command_output.status.code() {
                Some(0) => task_success(data, command_output, json_output),
                Some(_) => task_failure(data, command_output, json_output),
                None => println!("Process terminated by signal"),
            }
        });
        handles.push(child);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    print_json(outputs, json_output);
    true
}

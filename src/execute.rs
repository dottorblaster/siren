extern crate ansi_term;

use parse_config::Task;
use std::thread;
use std::process::Command;
use std::process::Output;

use self::ansi_term::Colour::{Red, Green, Yellow, Black};
use self::ansi_term::ANSIString;

fn task_success(task: Task, output: Output) {
    let stdout = ANSIString::from(String::from_utf8(output.stdout).unwrap());
    println!(
        "{} {}\n{}\n",
        Black.bold().on(Green).paint("  SUCCESS  "),
        Yellow.paint(format!("{}", task.name)),
        stdout
    );
}

fn task_failure(task: Task, output: Output) {
    let stderr = ANSIString::from(String::from_utf8(output.stderr).unwrap());
    println!(
        "{} {}\n{}\n",
        Black.bold().on(Red).paint("  FAIL  "),
        Yellow.paint(format!("{}", task.name)),
        stderr
    );
}

pub fn run(tasks: Vec<Task>, cwd_path: String) -> bool {
    let mut handles = Vec::with_capacity(tasks.len());
    println!("\n");
    for task in &tasks {
        let (data, path) = (task.clone(), cwd_path.clone());
        let child = thread::spawn(move || {
            let local_task = data.clone();
            let mut iter = local_task.command.split_whitespace();
            let output = Command::new(iter.nth(0).unwrap())
                .args(iter)
                .current_dir(path)
                .output()
                .expect("command failed");
            match output.status.code() {
                Some(0) => task_success(data, output),
                Some(_) => task_failure(data, output),
                None => println!("Process terminated by signal")
            }
        });
        handles.push(child);
    }
    for handle in handles { handle.join().unwrap(); }
    true
}
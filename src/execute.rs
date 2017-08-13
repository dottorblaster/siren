extern crate ansi_term;

use parse_config::Task;
use std::process::Command;
use std::process::Output;

use self::ansi_term::Colour::{Red, Green, Yellow};
use self::ansi_term::ANSIString;

fn task_success(task: &Task, output: Output) {
    let stdout = ANSIString::from(String::from_utf8(output.stdout).unwrap());
    println!(
        "{} {}\n{}",
        Yellow.paint(format!("{}", task.name)),
        Green.paint("success!"),
        stdout
    );
}

fn task_failure(task: &Task, output: Output) {
    let stdout = ANSIString::from(String::from_utf8(output.stdout).unwrap());
    println!(
        "{} {}\n{}",
        Yellow.paint(format!("{}", task.name)),
        Red.paint("fail!"),
        stdout
    );
}

pub fn run(tasks: Vec<Task>, cwd_path: String) -> bool {
    let mut i = tasks.iter();
    loop {
        match i.next() {
            Some(task) => {
                let mut iter = task.command.split_whitespace();
                let output = Command::new(iter.nth(0).unwrap())
                    .args(iter)
                    .current_dir(&cwd_path)
                    .output()
                    .expect("command failed");
                match output.status.code() {
                    Some(0) => task_success(task, output),
                    Some(_) => task_failure(task, output),
                    None => println!("Process terminated by signal")
                }
            },
            None => { break }
        }
    }
    true
}
use parse_config::Task;
use std::process::Command;
use std::process::Output;

pub fn run(tasks: Vec<Task>) -> bool {
    let mut i = tasks.iter();
    loop {
        match i.next() {
            Some(task) => {
                let mut iter = task.command.split_whitespace();
                let output = Command::new(iter.nth(0).unwrap())
                    .args(iter)
                    .output()
                    .expect("command failed");
                match output.status.code() {
                    Some(0) => println!("Process successful"),
                    Some(_) => println!("Process error"),
                    None => println!("Process terminated by signal")
                }
            },
            None => { break }
        }
    }
    true
}
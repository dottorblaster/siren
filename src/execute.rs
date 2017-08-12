use parse_config::Task;
use std::process::Command;

pub fn run(tasks: Vec<Task>) -> bool {
    let mut i = tasks.iter();
    loop {
        match i.next() {
            Some(task) => {
                let mut iter = task.command.split_whitespace();
                let output = Command::new(iter.nth(0).unwrap())
                    .args(iter)
                    .output()
                    .spawn()
                    .expect("command failed");
                // Process command output here and give a feedback
            },
            None => { break }
        }
    }
    true
}
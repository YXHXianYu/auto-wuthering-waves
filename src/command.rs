use std::process::Command;
use crate::command_println;

pub fn run_command(command: Vec<&str>) {
    command_println!("Running command: {:?}", command);

    let _ = Command::new(command[0])
        .args(&command[1..])
        .output()
        .expect("Failed to run command.");
}

pub fn run_command_async(command: Vec<&str>) {
    command_println!("Running command asynchronously: {:?}", command);

    Command::new(command[0])
        .args(&command[1..])
        .spawn()
        .expect("Failed to run command.");
}

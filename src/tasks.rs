use std::io::{self, Read};
use crate::prelude::*;
use chrono::Local;
use crate::{task_println, welcome_println};

pub fn work() {
    welcome_println!("Welcome to auto-wuthering-waves!");

    do_daily_task();

    println!("Press any key to continue...");
    let _ = io::stdin().read_exact(&mut [0u8]).unwrap();
}

fn do_daily_task() {
    if check_if_executed_today() {
        task_println!("Today's task has been executed. Skip.");
        return;
    }
    task_println!("Today's task has not been executed. Start.");

    start_ww_launcher();

    let controller = PcControllerWrapper::new();

    controller.output_all_windows();

    controller.start_game();

    update_record_of_execution();

    task_println!("Daily task finished.");

    // end_emulator();
}

impl PcControllerWrapper {
    fn output_all_windows(&self) {
        let windows = self.pc_controller.get_all_windows().unwrap();
        println!("All Windows:");
        for w in windows.iter() {
            println!("\t{:?}", w);
        }
    }

    fn start_game(&self) {
        task_println!("Starting game.");

        let pic = self.pc_controller.screencap().unwrap();
        let target = open_image("start_game.png");

        let (x, y) = template_match(&pic, &target);

        println!("Clicking at ({}, {}).", x, y);

        self.pc_controller.left_click(x, y).unwrap();
    }
}

fn check_if_executed_today() -> bool {
    let config = get_config();
    let today = Local::now().date_naive().to_string();
    let record_of_execution = config.record_of_execution.clone();

    if record_of_execution.is_empty() {
        return false;
    } else if record_of_execution.last().unwrap() == &today {
        return true;
    } else {
        return false;
    }
}

fn update_record_of_execution() {
    task_println!("Updating record of execution.");
    let today = Local::now().date_naive().to_string();
    let mut config = get_config();
    config.record_of_execution.push(today);
    Config::update(config).unwrap();
}

fn get_input_from_stdin() -> String {
    println!("Waiting for your input: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn start_ww_launcher() {
    let ww_launcher_full_path = format!("{}/{}", get_config().ww_launcher_path, get_config().ww_launcher_name);
    let ww_launcher_full_path_with_quotes = format!("\"{}\"", ww_launcher_full_path);

    if cfg!(target_os = "windows") {
        task_println!("Starting Wuthering Waves launcher in Windows platform. Use PowerShell for running as administrator!");
        run_command_async(vec![
            "powershell",
            "-Command",
            "Start-Process",
            "-FilePath",
            ww_launcher_full_path_with_quotes.as_str(),
            "-Verb",
            "runAs",
        ]);
    } else {
        task_println!("Starting Wuthering Waves launcher in non-Windows platform.");
        run_command_async(vec![
            ww_launcher_full_path.as_str(),
        ]);
        return;
    }

    let wait_time = get_config().ww_launcher_wait_time;
    sleep(wait_time);

    task_println!("{} seconds passed. Wuthering Waves launcher is launched.", wait_time);
}
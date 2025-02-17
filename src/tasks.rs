use std::io::{self, Read};
use anyhow::{Error, Ok};
use crate::{error_println, prelude::*};
use chrono::Local;
use crate::{task_println, welcome_println};

pub fn work() {
    welcome_println!("Welcome to auto-wuthering-waves!");

    if !is_admin() {
        error_println!("This tool should be run as administrator.");
        press_any_key_to_continue();
        return;
    }

    welcome_println!("This tool needs to occupy the screen / keyboard / mouse to work properly.");
    welcome_println!("And it should be run as administrator.");
    welcome_println!("Whats'more, remember to close the game before running this tool.");
    press_any_key_to_continue();

    match do_daily_task() {
        Result::Ok(_) => {
            welcome_println!("Task finished successfully.");
        }
        Err(e) => {
            error_println!("Task failed: {:?}", e);
            sleep(10.0); // 10 seconds
        }
    };

    press_any_key_to_continue();
}

fn do_daily_task() -> Result<(), Error> {
    if check_if_executed_today() {
        task_println!("Today's task has been executed. Skip.");
        return Ok(());
    }
    task_println!("Today's task has not been executed. Start.");

    start_ww_launcher();

    let controller = PcControllerWrapper::new();

    controller.output_all_windows()?;

    controller.start_game()?;

    update_record_of_execution();

    task_println!("Daily task finished.");

    // end_emulator();

    Ok(())
}

impl PcControllerWrapper {
    fn output_all_windows(&self) -> Result<(), Error> {
        let windows = self.pc_controller.get_all_windows().unwrap();
        println!("All Windows:");
        for w in windows.iter() {
            println!("\t{:?}", w);
        }
        Ok(())
    }

    fn start_game(&self) -> Result<(), Error> {
        task_println!("Starting game.");

        task_println!("Clicking start game button.");
        self.find_and_click("start_game_button.png")?;
        sleep(get_config().game_start_wait_time);

        task_println!("Clicking login button.");
        self.find_and_click_until_default( "login_button.png")?;

        task_println!("Game started.");
        Ok(())
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

fn press_any_key_to_continue() {
    println!("Press any key to continue...");
    let _ = io::stdin().read_exact(&mut [0u8]).unwrap();
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
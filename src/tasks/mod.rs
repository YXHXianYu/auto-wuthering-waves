mod base_actions;
mod advanced_actions;

use std::io;
use anyhow::{Error, Ok};
use crate::{error_println, prelude::*};
use chrono::Local;
use crate::{task_println, welcome_println};

pub fn work() {
    welcome_println!("Welcome to auto-wuthering-waves!");

    if !is_admin() {
        run_myself_as_admin();
        println!("Auto run as admin.");
        sleep(5.0);
        return;
    }

    welcome_println!("This tool needs to occupy the screen / keyboard / mouse to work properly.");
    welcome_println!("And it should be run as administrator.");
    welcome_println!("Whats'more, remember to close the game before running this tool.");
    welcome_println!("");
    welcome_println!("Press enter to continue...");
    press_enter_to_continue();
    sleep(5.0); // 5 seconds

    match do_daily_task() {
        Result::Ok(_) => {
            welcome_println!("Task finished successfully.");
        }
        Err(e) => {
            error_println!("Task failed: {:?}", e);
            sleep(10.0); // 10 seconds
        }
    };

    press_enter_to_continue();
}

fn do_daily_task() -> Result<(), Error> {
    if check_if_executed_today() {
        task_println!("Today's task has been executed. Skip.");
        return Ok(());
    }
    task_println!("Today's task has not been executed. Start.");

    start_ww_launcher();

    let controller = PcControllerWrapper::new();

    controller.start_game()?;
    controller.get_monthly_card_reward()?;
    controller.heal_myself()?;
    controller.collect_character_exp()?;
    controller.use_stamina_prop()?;
    controller.collect_character_exp()?;
    controller.heal_myself()?;
    controller.collect_weapon_exp()?;
    controller.use_arbitrary_prop()?;
    controller.upgrade_weapon()?;
    controller.complete_synthesis_once()?;
    controller.collect_daily_tasks_rewards()?;
    controller.collect_pass_daily_tasks_rewards()?;

    update_record_of_execution();
    task_println!("Daily task finished.");
    Ok(())
}

#[allow(dead_code)]
impl PcControllerWrapper {
    fn output_all_windows(&self) -> Result<(), Error> {
        let windows = self.get_all_windows()?;
        println!("All Windows:");
        for w in windows.iter() {
            println!("\t{:?}", w);
        }
        Ok(())
    }

    fn start_game(&self) -> Result<(), Error> {
        task_println!("Starting game.");

        task_println!("Clicking start game button.");
        // self.fcuds("start_game_button.png", get_config().game_start_wait_time)?;
        self.click(1327, 750)?;
        sleep(get_config().game_start_wait_time);

        task_println!("Clicking login button.");
        match self.fcus( "login_button.png", get_config().retry_wait_time, 1, get_config().game_start_wait_time_2) {
            Result::Ok(_) => task_println!("Login button found."),
            Err(_) => task_println!("Login button not found. Assume already logged in."),
        }

        self.click_any_position_and_sleep(get_config().game_start_wait_time_3)?;

        task_println!("Game started.");
        Ok(())
    }

    fn get_monthly_card_reward(&self) -> Result<(), Error> {
        task_println!("Getting monthly card reward.");

        self.click_any_position_and_sleep(get_config().wait_time)?;
        self.click_any_position_and_sleep(get_config().wait_time)?;
        self.click_any_position_and_sleep(get_config().wait_time)?;
        self.click_any_position_and_sleep(get_config().wait_time)?;
        self.click_any_position_and_sleep(get_config().wait_time)?;

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

fn press_enter_to_continue() {
    println!("Press enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

#[allow(dead_code)]
fn get_input_from_stdin() -> String {
    println!("Waiting for your input: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn start_ww_launcher() {
    let ww_launcher_full_path = format!("{}/{}", get_config().ww_launcher_path, get_config().ww_launcher_name);

    run_command_async(vec![
        ww_launcher_full_path.as_str(),
    ]);

    let wait_time = get_config().ww_launcher_wait_time;
    sleep(wait_time);

    task_println!("{} seconds passed. Wuthering Waves launcher is launched.", wait_time);
}

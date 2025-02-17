use anyhow::{anyhow, Error};
use enigo::Key;
use image::DynamicImage;
use crate::{controller_println, prelude::*};

#[allow(dead_code)]
impl PcControllerWrapper {

    pub fn collect_character_exp(&self) -> Result<(), Error> {
        controller_println!("Fighting and collecting character exp");

        self.enter_simulation_playground()?;

        controller_println!("Entered the simulation playground");

        self.fcuds("simulation_playground_start_button.png", get_config().wait_time)?;
        self.fcuds("simulation_playground_start_button_2.png", get_config().wait_time_long)?;
        for _ in 0..9 {
            self.move_forward()?;
            sleep(get_config().wait_time_short);
        }
        self.interact()?;

        // start fight

        Ok(())
    }

    pub fn fight(&self) -> Result<(), Error> {
        controller_println!("Fighting");

        self.lock_or_unlock()?; // lock
        sleep(get_config().wait_time_short);

        for _ in 0..20 {
            self.attack()?;
            sleep(get_config().wait_time_short);
            self.resonance_skill()?;
            sleep(get_config().wait_time_short);
            self.attack()?;
            sleep(get_config().wait_time_short);
            self.resonance_liberation()?;
            sleep(get_config().wait_time_short);
            self.attack()?;
            sleep(get_config().wait_time_short);
            self.resonance_skill()?;
            sleep(get_config().wait_time_short);
            self.attack()?;
            sleep(get_config().wait_time_short);
            self.echo_skill()?;
            sleep(get_config().wait_time_short);
        }

        Ok(())
    }

    pub fn search_and_go_to_the_target(&self, target: DynamicImage, until: DynamicImage) -> Result<(), Error> {
        controller_println!("Searching and going to the target");

        self.lock_or_unlock()?; // 视角回正
        sleep(get_config().wait_time_short);

        let width = self.pc_controller.get_screen_size().0 as i32;
        let x_center = width / 2;
        let x_min = x_center - (get_config().search_dead_zone_x * width as f64) as i32;
        let x_max = x_center + (get_config().search_dead_zone_x * width as f64) as i32;

        let height = self.pc_controller.get_screen_size().1 as i32;
        let y_center = height / 2;
        let y_min = 0;
        let y_max = y_center + (get_config().search_dead_zone_y * height as f64) as i32;

        for _ in 0..get_config().search_max_times {
            let screencap = self.pc_controller.screencap()?;

            // whether found the target
            if template_match(&screencap, &until).is_some() {
                return Ok(());
            }

            // try to find the target
            match template_match(&screencap, &target) {
                Some((x, y, _)) => {
                    if (x_min..x_max).contains(&x) && (y_min..y_max).contains(&y) {
                        self.move_forward()?;
                    } else {
                        if (x - x_center) > 0 {
                            self.turn_right()?;
                        } else {
                            self.turn_left()?;
                        }
                    }
                }
                None => {}
            }
        }

        Err(anyhow!("Not found target image!"))
    }

    // 模拟训练场，用于获得角色经验、武器经验、贝币
    pub fn enter_simulation_playground(&self) -> Result<(), Error> {
        controller_println!("Entering simulation playground");

        self.open_guidebook()?;
        sleep(get_config().wait_time);
        self.fcuds("guidebook_tab_3.png", get_config().wait_time)?;
        self.fcuds("guidebook_tab_3_sub_2.png", get_config().wait_time)?;
        self.click(1725, 280)?;
        sleep(get_config().wait_time);
        self.fcuds("guidebook_confirm_quick_travel.png", get_config().wait_time_load_map)?;

        // move to YanYan
        self.move_forward()?;
        sleep(get_config().wait_time_short);
        self.move_forward()?;
        sleep(get_config().wait_time_short);
        self.move_forward()?;
        sleep(get_config().wait_time_short);
        self.move_forward()?;
        sleep(get_config().wait_time_short);

        // Chat to him
        self.interact()?;
        sleep(get_config().wait_time);
        for _ in 0..5 {
            self.click_any_position_and_sleep(get_config().wait_time_short)?;
        }
        self.interact()?;
        sleep(get_config().wait_time);
        for _ in 0..3 {
            self.click_any_position_and_sleep(get_config().wait_time_short)?;
        }

        Ok(())
    }

    pub fn complete_daily_task(&self) -> Result<(), Error> {
        unimplemented!("每日任务种类太多，需要索敌，无法实现自动化");

        controller_println!("Completing daily task");
        
        self.open_guidebook()?;
        sleep(get_config().wait_time);
        self.click(1673, 356)?;
        sleep(get_config().wait_time);
        self.guidebook_confirm()?;
        // TODO: 每日任务种类太多，需要索敌，无法实现自动化

        Ok(())
    }

    pub fn guidebook_confirm(&self) -> Result<(), Error> {
        controller_println!("Guidebook confirm");
        
        self.fcuds("complete_daily_task_button_1.png", get_config().wait_time)?;
        self.fcuds("complete_daily_task_button_2.png", get_config().wait_time_load_map)?;

        Ok(())
    }

    // MARK: test

    pub fn test_turn_around(&self) -> Result<(), Error> {
        controller_println!("Testing turn around");
        // sleep(6.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_right()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        self.turn_left()?;
        println!("Location: {:?}", self.pc_controller.location()?);
        sleep(1.0);

        controller_println!("Test Done.");
        Ok(())
    }
}
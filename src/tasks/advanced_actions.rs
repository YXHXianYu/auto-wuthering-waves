use aah_controller::PcControllerTrait;
use anyhow::{anyhow, Error};
use image::DynamicImage;
use crate::{controller_println, prelude::*};

// simplify R
type R = Result<(), Error>;

#[allow(dead_code)]
impl PcControllerWrapper {

    pub fn collect_pass_daily_tasks_rewards(&self) -> R {
        controller_println!("Collecting pass daily tasks rewards");

        self.open_pioneer_podcast()?;
        sleep(get_config().wait_time);
        self.fcuds("pass/tab_2.png", get_config().wait_time)?;
        self.fcuds("pass/get.png", get_config().wait_time)?;
        self.press_escape()?;
        sleep(get_config().wait_time);

        Ok(())
    }

    pub fn collect_daily_tasks_rewards(&self) -> R {
        controller_println!("Collecting daily tasks rewards");

        self.open_guidebook()?;
        sleep(get_config().wait_time);
        for _ in 0..5 {
            match self.find(open_image("guidebook/collect_daily_rewards_button_1.png")?) {
                Ok(_) => {

                },
                Err(_) => { break; },
            };
            
            sleep(get_config().wait_time_short);
        }
        self.fcuds("guidebook/collect_daily_rewards_button_2.png", get_config().wait_time)?;
        self.press_escape()?;
        sleep(get_config().wait_time);
        self.press_escape()?;
        sleep(get_config().wait_time);

        Ok(())
    }

    pub fn collect_character_exp(&self) -> R {
        controller_println!("Collecting character exp");

        self.enter_simulation_playground()?;
        controller_println!("Entered the simulation playground");

        self.simulation_playground_common_action()
    }

    pub fn collect_weapon_exp(&self) -> R {
        controller_println!("Collecting weapon exp");

        self.enter_simulation_playground()?;
        controller_println!("Entered the simulation playground");

        self.fcuds("simulation_playground_tab_2.png", get_config().wait_time)?;
        
        self.simulation_playground_common_action()
    }

    pub fn collect_coin_exp(&self) -> R {
        controller_println!("Collecting weapon exp");

        self.enter_simulation_playground()?;
        controller_println!("Entered the simulation playground");

        self.fcuds("simulation_playground_tab_3.png", get_config().wait_time)?;
        
        self.simulation_playground_common_action()
    }

    pub fn heal_myself(&self) -> R {
        controller_println!("Healing myself");

        self.open_map()?;
        sleep(get_config().wait_time);
        self.click(1812, 680)?;
        sleep(get_config().wait_time);
        self.fcuds("beacon.png", get_config().wait_time)?;
        self.fcuds("quick_travel_button.png", get_config().wait_time_load_map)?;

        Ok(())
    }

    pub fn simulation_playground_common_action(&self) -> R {
        self.fcuds("simulation_playground_start_button.png", get_config().wait_time_long)?;
        self.fcuds("simulation_playground_start_button_2.png", get_config().wait_time_long)?;

        // run mode
        self.switch_to_run_mode()?;

        for _ in 0..3 {
            self.move_forward()?;
            sleep(get_config().wait_time_short);
        }
        self.interact()?;
        sleep(get_config().wait_time_short);

        self.fight(
            open_image("search_targets/reward/target.png")?,
        )?;

        self.search_and_go_to_the_target(
            open_image("search_targets/reward/target.png")?,
            open_image("search_targets/reward/until.png")?,
        )?;
        self.interact()?;
        sleep(get_config().wait_time);
        self.fcuds("simulation_playground_collect_reward.png", get_config().wait_time)?;
        self.fcuds("simulation_playground_exit.png", get_config().wait_time_load_map)?;

        Ok(())
    }

    pub fn fight(&self, until: DynamicImage) -> R {
        controller_println!("Fighting");

        self.lock_or_unlock()?; // lock
        sleep(get_config().wait_time_short);

        for _ in 0..get_config().search_max_times {
            self.attack()?;
            sleep(get_config().wait_time_short);
            self.attack()?;
            sleep(get_config().wait_time_short);
            self.resonance_skill()?;
            sleep(get_config().wait_time_short);
            self.attack()?;
            sleep(get_config().wait_time_short);
            self.attack()?;
            sleep(get_config().wait_time_short);
            self.resonance_liberation()?;
            sleep(get_config().wait_time_short);
            self.attack()?;
            sleep(get_config().wait_time_short);
            self.attack()?;
            sleep(get_config().wait_time_short);

            let screencap = self.pc_controller.screencap()?;
            if template_match(&screencap, &until).is_some() {
                return Ok(());
            }
        }

        Ok(())
    }

    pub fn search_and_go_to_the_target(&self, target: DynamicImage, until: DynamicImage) -> R {
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
    pub fn enter_simulation_playground(&self) -> R {
        controller_println!("Entering simulation playground");

        self.open_guidebook()?;
        sleep(get_config().wait_time);
        self.fcuds("guidebook_tab_3.png", get_config().wait_time)?;
        self.fcuds("guidebook_tab_3_sub_2.png", get_config().wait_time)?;
        self.click(1725, 280)?;
        sleep(get_config().wait_time);
        self.fcuds("quick_travel_button.png", get_config().wait_time_load_map)?;

        // move to YanYan
        self.move_forward()?;

        // chat to YanYan
        for _ in 0..9 {
            self.interact()?;
            sleep(get_config().wait_time);
        }

        Ok(())
    }

    pub fn complete_synthesis_once(&self) -> R {
        const COMPLETE_SYNTHESIS_ONCE_MATCH_THRESHOLD: f32 = 0.001;

        controller_println!("Completing synthesis once");

        self.open_guidebook()?;
        sleep(get_config().wait_time);

        for _ in 0..4 {
            // 滑动每日任务表，试图找到"合成"任务
            PcControllerTrait::swipe(
                self.pc_controller.as_ref(),
                1010,
                675,
                1010,
                200,
                2.0,
            )?;

            // 尝试匹配
            match self.find(open_image("guidebook/complete_synthesis_once.png")?) {
                Ok((_, y, v)) => {
                    if v > COMPLETE_SYNTHESIS_ONCE_MATCH_THRESHOLD { continue; }
                    self.click(1673, y)?;
                    break;
                },
                Err(_) => { continue; },
            };
        }

        self.fcuds("map/QianWang.png", get_config().wait_time)?;
        self.fcuds("map/QueRen.png", get_config().wait_time)?;
        self.pc_controller.move_mouse_absolute(1, 1)?;
        sleep(get_config().wait_time_short);
        let _ = self.fcus("map/QueRen.png", 0.0, 1, get_config().wait_time);
        sleep(get_config().wait_time_load_map);

        self.switch_to_run_mode()?;

        for _ in 0..12 {
            self.move_forward()?;
            sleep(get_config().wait_time_short);
        }
        sleep(get_config().wait_time);
        self.search_and_go_to_the_target(
            open_image("search_targets/synthesis/target.png")?,
            open_image("search_targets/synthesis/until.png")?
        )?;
        self.interact()?;
        sleep(get_config().wait_time);
        self.fcuds("sub_ui/synthesis_start.png", get_config().wait_time)?;
        self.click_any_position_and_sleep(get_config().wait_time)?;
        self.press_escape()?;
        sleep(get_config().wait_time);

        Ok(())
    }

    pub fn complete_daily_task(&self) -> R {
        unimplemented!("每日任务种类太多，需要索敌，无法实现自动化");

        // controller_println!("Completing daily task");
        
        // self.open_guidebook()?;
        // sleep(get_config().wait_time);
        // self.click(1673, 356)?;
        // sleep(get_config().wait_time);
        // self.guidebook_confirm()?;
        // // TODO: 每日任务种类太多，需要索敌，无法实现自动化

        Ok(())
    }

    pub fn guidebook_confirm(&self) -> R {
        controller_println!("Guidebook confirm");
        
        self.fcuds("complete_daily_task_button_1.png", get_config().wait_time)?;
        self.fcuds("complete_daily_task_button_2.png", get_config().wait_time_load_map)?;

        Ok(())
    }

    pub fn back_to_default_ui(&self) -> R {
        controller_println!("Back to default UI");

        for _ in 0..3 {
            let res = self.find(open_image("main_menu.png")?);

            self.press_escape()?;
            sleep(get_config().wait_time);

            if res.is_ok() {
                break;
            }
        }

        Ok(())
    }

    pub fn switch_to_run_mode(&self) -> R {
        const SWITCH_TO_RUN_MODE_MATCH_THRESHOLD: f32 = 0.05;

        controller_println!("Switch to run mode");

        self.click_ctrl()?;
        sleep(get_config().wait_time_short);
        for _ in 0..10 {
            if self.find(open_image("other/BenPao.png")?)
                    .is_ok_and(|(_, _, v)| v < SWITCH_TO_RUN_MODE_MATCH_THRESHOLD) {
                break;
            }
            self.click_ctrl()?;
            sleep(get_config().wait_time_short);
        }

        Ok(())
    }

    // MARK: test

    pub fn test_turn_around(&self) -> R {
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
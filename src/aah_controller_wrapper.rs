use aah_controller::{PcControllerTrait, WindowInfo};
use image::DynamicImage;
use anyhow::{anyhow, Error};

use crate::{controller_println, prelude::*};

pub struct PcControllerWrapper {
    pub pc_controller: Box<dyn PcControllerTrait + Sync + Send>,
}

impl PcControllerWrapper {
    pub fn new() -> Self {
        let pc_controller = aah_controller::pc_controller::create_pc_controller().unwrap();
        Self {
            pc_controller,
        }
    }

    pub fn get_all_windows(&self) -> Result<Vec<WindowInfo>, Error> {
        Ok(self.pc_controller.get_all_windows()?)
    }

    pub fn find(&self, target: DynamicImage) -> Result<(i32, i32, f32), Error> {
        let pic = self.pc_controller.screencap().unwrap();
        match template_match(&pic, &target) {
            Some((x, y, v)) => Ok((x, y, v)),
            None => Err(anyhow!("Not found target image!")),
        }
    }

    pub fn find_and_click(&self, str: &str) -> Result<(), Error> {
        let target = open_image(str)?;
        let ans = self.find(target);
        match ans {
            Ok((x, y, _)) => {
                controller_println!("Clicking at ({}, {}) for \"{}\"", x, y, str);
                self.pc_controller.left_click(x, y).unwrap();
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    pub fn find_and_click_until(&self, str: &str, wait_time: f64, retry_times: i32) -> Result<(), Error> {
        for idx in 0..retry_times {
            controller_println!("Trying to find target image: \"{}\" for the {}-th time", str, idx + 1);
            if self.find_and_click(str).is_ok() {
                controller_println!("Found target image: \"{}\"", str);
                return Ok(());
            }
            sleep(wait_time);
        }
        controller_println!("Not found target image: \"{}\"", str);
        Err(anyhow!("Not found target image!"))
    }
    
    pub fn find_and_click_until_default(&self, str: &str) -> Result<(), Error> {
        self.find_and_click_until(str, get_config().retry_wait_time, get_config().retry_max_times)
    }

    pub fn find_and_click_util_and_sleep(&self, str: &str, wait_time: f64, retry_times: i32, sleep_time: f64) -> Result<(), Error> {
        self.find_and_click_until(str, wait_time, retry_times)?;
        sleep(sleep_time);
        Ok(())
    }

    pub fn find_and_click_until_default_and_sleep(&self, str: &str, sleep_time: f64) -> Result<(), Error> {
        self.find_and_click_until(str, get_config().retry_wait_time, get_config().retry_max_times)?;
        sleep(sleep_time);
        Ok(())
    }

    pub fn fcus(&self, str: &str, wait_time: f64, retry_times: i32, sleep_time: f64) -> Result<(), Error> {
        self.find_and_click_util_and_sleep(str, wait_time, retry_times, sleep_time)
    }

    pub fn fcuds(&self, str: &str, sleep_time: f64) -> Result<(), Error> {
        self.find_and_click_until_default_and_sleep(str, sleep_time)
    }

    pub fn click_any_position_and_sleep(&self, wait_time: f64) -> Result<(), Error> {
        let (width, height) = self.pc_controller.screen_size();
        self.pc_controller.left_click((width / 2) as i32, (height as f64 * 0.8) as i32)?;
        sleep(wait_time);
        Ok(())
    }
}
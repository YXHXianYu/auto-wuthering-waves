use aah_controller::PcControllerTrait;
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

    pub fn find(&self, target: DynamicImage) -> Result<(i32, i32), Error> {
        let pic = self.pc_controller.screencap().unwrap();
        match template_match(&pic, &target) {
            Some((x, y)) => Ok((x, y)),
            None => Err(anyhow!("Not found target image!")),
        }
    }

    pub fn find_and_click(&self, str: &str) -> Result<(), Error> {
        let target = open_image(str);
        let ans = self.find(target);
        match ans {
            Ok((x, y)) => {
                controller_println!("Clicking at ({}, {}) for \"{}\"", x, y, str);
                self.pc_controller.left_click(x, y).unwrap();
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    pub fn find_and_click_until_default(&self, str: &str) -> Result<(), Error> {
        self.find_and_click_until(str, get_config().retry_wait_time, get_config().retry_max_times)
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
}
use anyhow::Error;
use enigo::Key;
use crate::{controller_println, prelude::*};

#[allow(dead_code)]
impl PcControllerWrapper {
    pub fn click(&self, x: i32, y: i32) -> Result<(), Error> {
        controller_println!("Clicking");
        self.pc_controller.left_click(x, y)?;
        Ok(())
    }

    pub fn turn_left(&self) -> Result<(), Error> {
        controller_println!("Turning left");
        let tar_x = (self.pc_controller.screen_size().0 as f64 * get_config().turn_sensitive) as i32;
        self.pc_controller.move_mouse_relative(-tar_x, 0)?;
        sleep(get_config().wait_time_tick);
        self.display_cursor()?;
        Ok(())
    }

    pub fn turn_right(&self) -> Result<(), Error> {
        controller_println!("Turning right");
        let tar_x = (self.pc_controller.screen_size().0 as f64 * get_config().turn_sensitive) as i32;
        self.pc_controller.move_mouse_relative(tar_x, 0)?;
        sleep(get_config().wait_time_tick);
        self.display_cursor()?;
        Ok(())
    }

    pub fn location(&self) -> Result<(i32, i32), Error> {
        controller_println!("Getting location");
        let (x, y) = self.pc_controller.location()?;
        Ok((x, y))
    }

    pub fn move_forward(&self) -> Result<(), Error> {
        controller_println!("Moving forward");
        self.pc_controller.key_press(Key::W)?;
        sleep(get_config().move_wait_time);
        self.pc_controller.key_release(Key::W)?;
        Ok(())
    }

    pub fn move_backward(&self) -> Result<(), Error> {
        controller_println!("Moving backward");
        self.pc_controller.key_press(Key::S)?;
        sleep(get_config().move_wait_time);
        self.pc_controller.key_release(Key::S)?;
        Ok(())
    }

    pub fn move_left(&self) -> Result<(), Error> {
        controller_println!("Moving left");
        self.pc_controller.key_press(Key::A)?;
        sleep(get_config().move_wait_time);
        self.pc_controller.key_release(Key::A)?;
        Ok(())
    }

    pub fn move_right(&self) -> Result<(), Error> {
        controller_println!("Moving right");
        self.pc_controller.key_press(Key::D)?;
        sleep(get_config().move_wait_time);
        self.pc_controller.key_release(Key::D)?;
        Ok(())
    }

    // 锁定敌人
    pub fn lock_or_unlock(&self) -> Result<(), Error> {
        controller_println!("Locking or unlocking");
        self.pc_controller.middle_click(1000, 500)?;
        Ok(())
    }

    // 共鸣技能
    pub fn resonance_skill(&self) -> Result<(), Error> {
        controller_println!("Using resonance skill");
        self.pc_controller.key_click(Key::E)?;
        Ok(())
    }

    // 共鸣解放
    pub fn resonance_liberation(&self) -> Result<(), Error> {
        controller_println!("Using resonance liberation");
        self.pc_controller.key_click(Key::R)?;
        Ok(())
    }

    pub fn echo_skill(&self) -> Result<(), Error> {
        controller_println!("Using echo skill");
        self.pc_controller.key_click(Key::Q)?;
        Ok(())
    }

    pub fn jump(&self) -> Result<(), Error> {
        controller_println!("Jumping");
        self.pc_controller.key_click(Key::Space)?;
        Ok(())
    }

    pub fn attack(&self) -> Result<(), Error> {
        controller_println!("Attacking");
        self.pc_controller.left_click(1000, 500)?;
        Ok(())
    }

    pub fn interact(&self) -> Result<(), Error> {
        controller_println!("Interacting");
        self.pc_controller.key_click(Key::F)?;
        Ok(())
    }

    // MARK: Menu

    pub fn open_mail(&self) -> Result<(), Error> {
        controller_println!("Opening mail");
        self.pc_controller.key_click(Key::N)?;
        Ok(())
    }

    pub fn open_map(&self) -> Result<(), Error> {
        controller_println!("Opening map");
        self.pc_controller.key_click(Key::M)?;
        Ok(())
    }

    pub fn open_events(&self) -> Result<(), Error> {
        controller_println!("Opening events");
        self.pc_controller.key_click(Key::F1)?;
        Ok(())
    }

    pub fn open_guidebook(&self) -> Result<(), Error> {
        controller_println!("Opening guidebook");
        self.pc_controller.key_click(Key::F2)?;
        Ok(())
    }

    pub fn open_convene(&self) -> Result<(), Error> {
        controller_println!("Opening convene");
        self.pc_controller.key_click(Key::F3)?;
        Ok(())
    }

    pub fn open_pioneer_podcast(&self) -> Result<(), Error> {
        controller_println!("Opening pioneer podcast");
        self.pc_controller.key_click(Key::F4)?;
        Ok(())
    }

    pub fn display_cursor(&self) -> Result<(), Error> {
        controller_println!("Displaying cursor");
        self.pc_controller.key_click(Key::Alt)?;
        Ok(())
    }

}
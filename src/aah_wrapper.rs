use std::{path::Path, sync::Arc};
use aah_core::{resource::Resource, AAH};
use std::time::Duration;
use crate::{adb_println, sleep::sleep, task_println};

pub struct AahWrapper {
    pub aah: AAH,
}

impl AahWrapper {
    pub fn new() -> Self {
        let resource_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../resources");
        task_println!("Loading resource from {:?}", resource_path);

        let resource = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(Resource::try_init_by_specific_dir(resource_path))
            .expect("failed to load resource");

        let aah = AAH::connect_with_pc_controller(
            Arc::new(resource.into())
        ).expect("failed to connect to the device");

        Self { aah }
    }

    pub fn click(&self, x: u32, y: u32, wait_time: f64) {
        // adb_println!("Clicking screen at ({}, {})...", x, y);
    
        // FIXME: whether to use `click` or `click_scaled`?
        if let Err(e) = self.aah.click(x, y) {
            adb_println!("Failed to click screen: {:?}", e);
        }
    
        sleep(wait_time);
    }
    
    pub fn swipe(&self, x1: u32, y1: u32, x2: u32, y2: u32, wait_time: f64) {
        // adb_println!("Swiping from ({}, {}) to ({}, {})...", x1, y1, x2, y2);
    
        let res = self.aah.swipe(
            (x1, y1), (x2 as i32, y2 as i32), Duration::from_secs_f32(0.5), 0.0, 0.0
        );
        if let Err(e) = res {
            adb_println!("Failed to swipe: {:?}", e);
        }
        
        sleep(wait_time);
    }    
}
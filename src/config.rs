use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // launcher
    pub ww_launcher_path: String,
    pub ww_launcher_name: String,
    pub ww_launcher_wait_time: f64,
    pub ww_launcher_title: String,
    
    // wait time
    pub game_start_wait_time: f64,
    pub game_start_wait_time_2: f64,
    pub game_start_wait_time_3: f64,

    // common wait time
    pub wait_time_long: f64,
    pub wait_time: f64,
    pub wait_time_short: f64,
    pub wait_time_load_map: f64,
    pub wait_time_tick: f64,
    
    // retry
    pub retry_wait_time: f64,
    pub retry_max_times: i32,

    // search
    pub search_max_times: i32,
    pub search_dead_zone_x: f64,
    pub search_dead_zone_y: f64,

    // fight
    pub fight_max_rounds: i32,

    // action
    pub turn_sensitive: f64,
    pub move_wait_time: f64,
    pub swipe_time: f64,

    // record
    pub record_of_execution: Vec<String>,
}

const ENABLE_SAVE_CONFIG: bool = false;
const CONFIG_NAME: &str = "ww_config.json";

impl Default for Config {
    fn default() -> Self {
        Config {
            ww_launcher_path: "E:/Game/Wuthering Waves".to_string(),
            ww_launcher_name: "launcher.exe".to_string(),
            ww_launcher_wait_time: 5.0,
            ww_launcher_title: "鸣潮".to_string(),

            game_start_wait_time: 39.0,
            game_start_wait_time_2: 5.0,
            game_start_wait_time_3: 15.0,

            wait_time_long: 5.0,
            wait_time: 2.0,
            wait_time_short: 0.5,
            wait_time_load_map: 13.0,
            wait_time_tick: 0.05,

            retry_wait_time: 5.0,
            retry_max_times: 3,

            search_max_times: 100,
            search_dead_zone_x: 0.10,
            search_dead_zone_y: 0.15,

            fight_max_rounds: 20,

            turn_sensitive: 0.07, // %
            move_wait_time: 0.5,
            swipe_time: 0.5,

            record_of_execution: vec![],
        }
    }
}

pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let config = load_config().unwrap_or_default();
    Mutex::new(config)
});

pub fn get_config() -> Config {
    CONFIG.lock().unwrap().clone()
}

impl Config {
    pub fn get() -> Config {
        CONFIG.lock().unwrap().clone()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {

        if !ENABLE_SAVE_CONFIG {
            return Ok(());
        }

        let config_path = CONFIG_NAME;
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(config_path, config_str)?;
        Ok(())
    }

    pub fn update(config: Config) -> Result<(), Box<dyn std::error::Error>> {
        let mut current = CONFIG.lock().unwrap();
        *current = config;
        current.save()?;
        Ok(())
    }
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = CONFIG_NAME;
    if !Path::new(config_path).exists() {
        let default_config = Config::default();
        default_config.save()?;
        return Ok(default_config);
    }

    let config_str = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&config_str)?;
    Ok(config)
}
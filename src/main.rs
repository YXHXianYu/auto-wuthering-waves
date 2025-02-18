#[allow(unused_imports)]
use auto_wuthering_waves::prelude::*;

fn main() {
    work();
    // test_controller();
}

pub fn test_controller() {
    let controller = PcControllerWrapper::new();

    if !is_admin() {
        run_myself_as_admin();
        println!("Auto run as admin.");
        sleep(5.0);
        return;
    }

    println!("Wait 5 seconds to continue.");
    sleep(5.0);

    // controller.back_to_default_ui().unwrap();
    controller.complete_synthesis_once().unwrap();

    println!("Sleep 1000 secs. Press ctrl+C to exit.");
    sleep(1000.0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_template_match() {
        // let tar = open_image("login_button.png");
        // let picture = open_image("screencap.png");
    
        let tar = open_image("start_game_button.png").unwrap();
        let picture = open_image("screencap3.png").unwrap();
    
        let res = template_match(&picture, &tar);
    
        match res {
            Some((x, y, _)) => {
                println!("Found at ({}, {})", x, y);
            }
            None => {
                println!("Not found.");
            }
        }
    }
}

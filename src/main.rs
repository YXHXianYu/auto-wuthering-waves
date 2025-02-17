use auto_wuthering_waves::prelude::*;

fn main() {
    work();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_template_match() {
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

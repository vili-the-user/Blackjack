use figlet_rs::FIGfont;
use utils::notification;

use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

mod game;
pub mod save;
pub mod utils;
pub mod input;

fn main() -> io::Result<()> {
    // Create new big font for the intro
    let title = "Blackjack";
    let font = FIGfont::standard().unwrap();
    let big_text = font.convert(title);

    // If the creation of the font was successful print it
    if let Some(big_text) = big_text {
        println!("{big_text}");
    } else {
        println!("{title}");
    }
    sleep(Duration::from_secs(1));

    menu();

    Ok(())
}

/// Main menu of the game
fn menu() {
    'main_menu: loop {
        println!("---");
        println!("Main menu");
        println!("1. New game");
        println!("2. Continue");
        println!("3. Exit");

        loop {
            // Get user input
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            // Clear input to prevent bugs
            print!("\x1B[A\r\x1B[K");
            io::stdout().flush().unwrap();
        
            // Get MenuOption from input if it's valid
            let option = match input.trim().parse::<u8>() {
                Ok(num) => match input::MainMenuOptions::try_from(num) {
                    Ok(option) => option,
                    Err(err) => {
                        notification(&format!("{}", err), 1);
                        continue;
                    }
                },
                Err(_) => {
                    notification("Input must be a number", 1);
                    continue;
                }
            };
            
            match option {
                input::MainMenuOptions::NewGame => {
                    game::new_game();
                    break;
                },
                input::MainMenuOptions::Continue => {
                    game::load_game();
                    break;
                },
                input::MainMenuOptions::Exit => break 'main_menu,
            };
        }
    }
}

use figlet_rs::FIGfont;

use std::io;
use std::thread::sleep;
use std::time::Duration;

mod game;

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

    // Create variable for user input integer
    let mut input_int: u8;

    loop {
        println!("---");
        println!("Main menu");
        println!("1. New game");
        println!("2. Continue");
        println!("3. Exit");
    
        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
    
        // Check if input is valid
        input_int = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input a number between 1 and 3");
                sleep(Duration::from_secs(1));
                continue;
            }
        };
        if input_int > 3 {
            println!("Input a number between 1 and 3");
            sleep(Duration::from_secs(1));
            continue;
        }

        if input_int == 1 {
            game::new_game();
        }
    
        else if input_int == 2 {
            game::load_game();
        }
    
        else if input_int == 3 {
            break;
        }
    }
}

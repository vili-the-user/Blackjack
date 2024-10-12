use whoami::fallible::realname;

use std::io;
use std::thread::sleep;
use std::time::Duration;

/// Player struct
struct Player {
    name: String,
    wealth: u32
}

/// Starts a game with new stats
pub fn new_game() {

    // Create user name variable
    let user_name: String;

    // Get user name from user's PC
    match realname() {
        Ok(name) => user_name = name,
        Err(_) => {
            user_name = String::from("User");
        }
    };

    // Create new player with the user name
    let mut player: Player = Player {
        name: String::from(user_name),
        wealth: 10
    };

    println!("Created new save as {}", player.name);

    // Start new game loop
    game(&mut player);
}

/// Main game loop
fn game(player: &mut Player) {
    loop {
        println!("---");
        println!("You have {}$", player.wealth);
        println!("Place your bet");

        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Check if input is valid
        let bet: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input a number greater than 0");
                sleep(Duration::from_secs(1));
                continue;
            }
        };
        if bet > player.wealth {
            println!("You don't have that much money");
            sleep(Duration::from_secs(1));
            continue;
        }

        println!("You are betting {bet}$");

        // Remove bet from player's wealth
        player.wealth -= bet;
    }
}
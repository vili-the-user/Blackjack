use whoami::fallible::realname;

use std::io;

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
    game(player);
}

/// Main game loop
fn game(player: Player) {
    loop {
        println!("---");
        println!("You have {}$", player.wealth);
        println!("Place your bet");

        loop {
            // Get user input
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let bet: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Input a number greater than 0");
                    continue;
                }
            };
            
            break;
        }

        println!("")
    }
}
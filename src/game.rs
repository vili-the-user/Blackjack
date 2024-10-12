use whoami::fallible::realname;

use std::io;
use std::thread::sleep;
use std::time::Duration;

use rand::thread_rng;
use rand::seq::SliceRandom;

/// Player struct
struct Player {
    name: String,
    wealth: u32
}

/// Card struct
struct Card {
    num: String,
    icon: char
}

// Create constant arrays for card icons and numbers
const ICON_ARRAY: [char; 4] = ['♠', '♣', '♥', '♦'];
const NUM_ARRAY: [&str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

/// Creates a sorted list of cards
/// 
/// # Returns
/// 
/// Vec of Card objects
fn create_deck_vec() -> Vec<Card> {

    // Create new deck vector
    let mut deck_vec: Vec<Card> = Vec::new();

    // Add one card of each type to the vector
    for icon in ICON_ARRAY {
        for num in NUM_ARRAY {

            deck_vec.push(Card {
                num: String::from(num),
                icon: icon
            });
        }
    }

    return deck_vec;
}

/// Shuffles passed deck of cards
fn shuffle_deck(deck_vec: &mut Vec<Card>) {
    deck_vec.shuffle(&mut thread_rng());
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

    // Create new deck
    let mut deck = create_deck_vec();

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

        // Shuffle deck
        shuffle_deck(&mut deck);
    }
}
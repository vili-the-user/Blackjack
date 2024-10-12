use rand::seq::index;
use whoami::fallible::realname;

use std::cmp;
use std::fmt;
use std::io::{Write, Read};
use std::thread::sleep;
use std::time::Duration;
use std::{io, u8};
use std::fs::File;

use rand::seq::SliceRandom;
use rand::thread_rng;
use bincode::{serialize, deserialize, Error};

use serde::{Serialize, Deserialize};

fn save(player: &Player) -> Result<(), Error> {
    // Serialize player to a binary format
    let encoded: Vec<u8> = serialize(player)?;

    // Write binary data to a file
    let mut file = File::create("save.blackjack")?;
    file.write_all(&encoded)?;

    Ok(())
}

fn load() -> Result<Player, Box<dyn std::error::Error>> {
    let mut file = File::open("save.blackjack")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let player: Player = deserialize(&buffer)?;
    Ok(player)
}

/// Player struct
#[derive(Serialize, Deserialize)]
struct Player {
    name: String,
    wealth: u32
}

/// Card struct
struct Card {
    num_str: String,
    icon: char,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.num_str, self.icon)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.num_str, self.icon)
    }
}

// Create constant arrays for card icons and numbers
const ICON_ARRAY: [char; 4] = ['♠', '♣', '♥', '♦'];
const NUM_ARRAY: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];

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
                num_str: String::from(num),
                icon: icon,
            });
        }
    }

    return deck_vec;
}

/// Shuffles passed deck of cards
fn shuffle_deck(deck_vec: &mut Vec<Card>) {
    deck_vec.shuffle(&mut thread_rng());

    println!("Shuffling...");
    sleep(Duration::from_secs(2));
}

/// Calculates the total value of a vec of cards
///
/// # Returns
///
/// u8
fn cards_value(hand_vec: &Vec<Card>) -> u8 {
    // Create variables for sum of card values and amount of aces
    let mut total: u8 = 0;
    let mut aces_amt: u8 = 0;

    // Go through each card in the hand and add up their values
    for card in hand_vec {
        if card.num_str == "A" {
            total += 11;
            aces_amt += 1;
        } else if ["10", "J", "Q", "K"].contains(&card.num_str.as_str()) {
            total += 10;
        } else {
            // Parse integer from card num string
            let value: u8 = match card.num_str.parse() {
                Ok(num) => num,
                Err(_) => {
                    continue;
                }
            };
            total += value;
        }
    }
    // If there are aces in the hand and the total value exceeds 21, the aces' value changes from 11 to 1
    while total > 21 && aces_amt > 0 {
        total -= 10;
        aces_amt -= 1;
    }

    return total;
}

// Deals a card to a hand vec from the deck vec
fn deal_cards(to_vec: &mut Vec<Card>, from_vec: &mut Vec<Card>, amt_cards: u8) {
    for _ in 1..=amt_cards {
        // Create variable for dealt card
        let card: Card;

        // Pop the card from the deck
        match from_vec.pop() {
            Some(card_popped) => card = card_popped,
            None => {
                panic!("Deck empty, cannot deal cards");
            }
        };

        sleep(Duration::from_millis(200));

        to_vec.push(card);
    }
}

/// Prints the current hands and bet
fn print_game_state(player_hand: &Vec<Card>, dealer_hand: &Vec<Card>, dealer_turn: bool) {
    // Calculate total values for both player's and dealer's hands
    let player_hand_value: u8 = cards_value(&player_hand);
    let dealer_hand_value: u8 = cards_value(&dealer_hand);

    if dealer_turn {
        print!("\r\x1B[6A\x1B[KYour cards: {:?} ({player_hand_value})\tDealer's cards: {:?} ({dealer_hand_value})\r\x1B[6B", player_hand, dealer_hand);
        io::stdout().flush().unwrap();
    } else {
        print!("\r\x1B[6A\x1B[KYour cards: {:?} ({player_hand_value})\tDealer's cards: [{}, ??] (??)\x1B[6B\r", player_hand, dealer_hand[0]);
        io::stdout().flush().unwrap();
    }
    sleep(Duration::from_secs(1));
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
        wealth: 10,
    };

    // Create new save file
    match save(&player) {
        Ok(_) => { println!("Created new save as {}", player.name); },
        Err(_) => { 
            println!("An error occurred when saving");
            return;
        }
    };

    // Clear terminal
    print!("\x1B[2J\x1B[1;1H");

    // Start new game loop
    game(&mut player);

    // Save player to the file again after loop ends
    match save(&player) {
        Ok(_) => { println!("Saved"); },
            Err(_) => { 
                println!("An error occurred when saving");
                return;
            }
    };
}

pub fn load_game() {

    // Get player object from file
    let mut player = match load() {
        Ok(player) => { println!("Loaded save file created by {}", player.name); player },
        Err(_) => {
            println!("Save file is corrupted or doesn't exist. Make sure the save file and the app are in the same location");
            return;
        } 
    };

    // Clear terminal
    print!("\x1B[2J\x1B[1;1H");

    // Start game loop
    game(&mut player);

    // Save player to the file again after loop ends
    match save(&player) {
        Ok(_) => { println!("Saved"); },
        Err(_) => { 
            println!("An error occurred when saving");
            return;
        }
    };
}

/// Main game loop
fn game(player: &mut Player) {
    // Create new deck and shuffle it
    let mut deck = create_deck_vec();
    shuffle_deck(&mut deck);

    while player.wealth > 0 && player.wealth < u32::MAX {
        match save(&player) {
            Ok(_) => { println!("Saved"); },
            Err(_) => { 
                println!("An error occurred when saving");
                return;
            }
        };

        println!("\n---");
        println!("You have ${}", player.wealth);
        println!("Place your bet");

        // Get user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Check if input is valid
        let mut bet: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input a whole number greater than 0");
                sleep(Duration::from_secs(1));
                continue;
            }
        };
        if bet > player.wealth {
            println!("You don't have that much money");
            sleep(Duration::from_secs(1));
            continue;
        }

        println!("You are betting ${bet}");

        // Remove bet from player's wealth
        player.wealth = cmp::max(0, cmp::min(u32::MAX, player.wealth - bet));

        // Shuffle deck if less than half of cards are left
        if deck.len() < 26 {
            deck = create_deck_vec();
            shuffle_deck(&mut deck);
        }

        // Create empty hand vec for player and dealer
        let mut player_hand: Vec<Card> = Vec::new();
        let mut dealer_hand: Vec<Card> = Vec::new();

        // Deal cards to both
        deal_cards(&mut player_hand, &mut deck, 2);
        deal_cards(&mut dealer_hand, &mut deck, 2);

        // If both player and dealer get blackjack
        if cards_value(&player_hand) == 21 && cards_value(&dealer_hand) == 21 {
            player.wealth = cmp::max(0, cmp::min(u32::MAX, player.wealth + bet));

            println!("\n--- DRAW ---");
            println!("You and dealer both got a blackjack. You get {bet}$ back");

            continue;
        }

        // If player gets blackjack
        if cards_value(&player_hand) == 21 {
            player.wealth = cmp::max(0, cmp::min(u32::MAX, player.wealth + bet * 2));

            println!("\n--- YOU WON ---");
            println!("You got a blackjack. Won {}$", bet * 2);

            continue;
        }

        // If dealer gets blackjack
        if cards_value(&dealer_hand) == 21 {
            println!("\n--- YOU LOST ---");
            println!("Dealer got a blackjack");

            continue;
        }

        // Player's turn
        println!("\n--- YOUR TURN | BET: ${bet} ---");
        println!("\n---");
        println!("What do you want to do?");
        println!("1. Hit");
        println!("2. Stand");
        println!("3. Double down");

        // Print current game state
        print_game_state(&player_hand, &dealer_hand, false);
        while cards_value(&player_hand) <= 21 {
            // Create variable for user input integer
            let mut input_int: u8;

            let mut index: u8 = 0;
            loop {
                index += 1;

                // Get user input
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                // Clear input to prevent bugs
                print!("\x1B[A\r\x1B[K");

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
                if input_int == 3 {
                
                    if player.wealth < bet {
                        println!("You don't have enough money to double down");
                        sleep(Duration::from_secs(1));
                        continue;
                    }
                    else if index > 1 {
                        println!("You can't double down after hitting");
                        sleep(Duration::from_secs(1));
                        continue;
                    }
                }
                break;
            }

            if input_int == 1 {
                deal_cards(&mut player_hand, &mut deck, 1);

                // Print game state
                print_game_state(&player_hand, &dealer_hand, false);
            } else if input_int == 2 {
                break;
            } else if input_int == 3 {
                // Double down allows player to only hit once with double the bet
                // Reduce bet again from player's wealth to compensate for doubled bet
                player.wealth = cmp::max(0, cmp::min(u32::MAX, player.wealth - bet));
                bet *= 2;

                deal_cards(&mut player_hand, &mut deck, 1);

                // Print game state
                print_game_state(&player_hand, &dealer_hand, false);

                break;
            }
        }

        // If player gets more than 21
        if cards_value(&player_hand) > 21 {
            println!("\n--- YOU LOST ---");
            println!("You busted");

            continue;
        }

        // Dealer's turn
        print!("\r\x1B[7A\x1B[K--- DEALER'S TURN | BET: ${bet} ---\x1B[7B");
        io::stdout().flush().unwrap();

        // Print current game state
        print_game_state(&player_hand, &dealer_hand, true);
        while cards_value(&dealer_hand) <= 21 {
            sleep(Duration::from_secs(1));

            // Dealer stands on 17 or greater
            if cards_value(&dealer_hand) < 17
                && cards_value(&dealer_hand) <= cards_value(&player_hand)
            {
                deal_cards(&mut dealer_hand, &mut deck, 1);
                // Print game state
                print_game_state(&player_hand, &dealer_hand, true);
                sleep(Duration::from_secs(1));
            } else {
                sleep(Duration::from_secs(1));
                break;
            }
        }

        // If dealer gets more than 21
        if cards_value(&dealer_hand) > 21 {
            player.wealth = cmp::max(0, cmp::min(u32::MAX, player.wealth + bet * 2));

            println!("\n--- YOU WON ---");
            println!("Dealer busted. You won ${}", bet * 2);

            continue;
        }

        // If player and dealer have same value
        if cards_value(&player_hand) == cards_value(&dealer_hand) {
            player.wealth = cmp::max(0, cmp::min(u32::MAX, player.wealth + bet));

            println!("\n--- DRAW ---");
            println!("You and dealer got hands of same value. You get ${bet} back");

            continue;
        }

        // If player has more value than dealer
        if cards_value(&player_hand) > cards_value(&dealer_hand) {
            player.wealth = cmp::max(0, cmp::min(u32::MAX, player.wealth + bet * 2));

            println!("\n--- YOU WON ---");
            println!("You were closer to 21. You won ${}", bet * 2);

            continue;
        }

        // If dealer has more value than player
        if cards_value(&player_hand) < cards_value(&dealer_hand) {
            println!("\n--- YOU LOST ---");
            println!("Dealer was closer to 21.");

            continue;
        }
    }

    if player.wealth == 0 {
        println!("You ran out of money. Returning to main menu...");
        sleep(Duration::from_secs(2));
    }

    if player.wealth == u32::MAX {
        println!("You have too much money. The casino can't provide for further wins. Returning to main menu...");
        sleep(Duration::from_secs(2));
    }
}

use whoami::fallible::realname;

use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use std::{io, u8};

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::save::{save, load, Player};
use crate::utils::{notification, clear_terminal};

// Create constant arrays for card icons and numbers
const SUIT_ARRAY: [char; 4] = ['♠', '♣', '♥', '♦'];
const NUM_ARRAY: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];

/// Creates a sorted list of cards
///
/// # Returns
///
/// Vec of strings
fn create_deck_vec() -> Vec<String> {
    // Create new deck vector
    let mut deck_vec: Vec<String> = Vec::new();

    // Add one card of each type to the vector
    for suit in SUIT_ARRAY {
        for num in NUM_ARRAY {

            // Fuse card number and suit together
            let mut card = String::from(num);
            card.push(suit);

            // Push card String to deck vec
            deck_vec.push(card);
        }
    }

    return deck_vec;
}

/// Shuffles passed deck of cards
fn shuffle_deck(deck_vec: &mut Vec<String>) {
    deck_vec.shuffle(&mut thread_rng());

    println!("Shuffling...");
    sleep(Duration::from_secs(2));
}

/// Calculates the total value of a vec of cards
///
/// # Returns
///
/// u8
fn cards_value(hand_vec: &Vec<String>) -> u8 {
    // Create variables for sum of card values and amount of aces
    let mut total: u8 = 0;
    let mut aces_amt: u8 = 0;

    // Go through each card in the hand and add up their values
    for card in hand_vec {

        // Create clone of card to assume ownership
        let mut card_value = card.clone();

        // Pop out the suit from the card so that only the value can be read
        match card_value.pop() {
            Some(_) => {},
            None    => { println!("Cannot pop card suit because the card is empty"); }
        };

        // Add card value to the total according to their number
        if card_value == "A" {
            total += 11;
            aces_amt += 1;
        } else if ["10", "J", "Q", "K"].contains(&card_value.as_str()) {
            total += 10;
        } else {
            // Parse integer from card num string
            let value: u8 = match card_value.parse() {
                Ok(num) => num,
                Err(_)      => { println!("Cannot parse integer from card value because it is unknown"); continue; }
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

/// Deals a card to a hand vec from the deck vec
/// 
/// # Returns
/// 
/// Ok or an Err if the deck is empty
fn deal_cards(to_vec: &mut Vec<String>, from_vec: &mut Vec<String>, amt_cards: u8) -> Result<(), String> {
    for _ in 1..=amt_cards {

        // Check if deck is empty
        if from_vec.is_empty() {
            return Err(String::from("Given deck is empty, cannot deal cards"));
        }

        // Pop the card from the deck
        let dealt_card = match from_vec.pop() {
            Some(card) => card,
            None               => {
                // If for some reason the deck is still empty despite the earlier check, return an Err
                return Err(String::from("Deck was unexpectedly emptied while dealing")) 
            }
        };

        to_vec.push(dealt_card);
    }
    Ok(())
}

/// Prints the current hands and bet
fn print_game_state(player_hand: &Vec<String>, dealer_hand: &Vec<String>, dealer_turn: bool) {
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

    clear_terminal();

    // Create new save file
    match save(&player) {
        Ok(_) => { println!("Created new save as {}", player.name); },
        Err(_) => { 
            notification("An error occurred when saving", 2);

            return;
        }
    };

    // Start new game loop
    match game(&mut player) {
        Ok(_) => {
            clear_terminal();
        },
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    // Save player to the file again after loop ends
    match save(&player) {
        Ok(_) => { notification("Saved", 1); },
            Err(_) => { 
                notification("An error occurred when saving", 2);

                return;
            }
    };
}

pub fn load_game() {

    clear_terminal();

    // Get player object from file
    let mut player = match load() {
        Ok(player) => { println!("Loaded save file created by {}", player.name); player },
        Err(err) => {
            notification(&format!("{}", err), 2);

            return;
        } 
    };

    // Start game loop
    match game(&mut player) {
        Ok(_) => {
            clear_terminal();
        },
        Err(err) => {
            notification(&format!("{}", err), 2);
            return;
        }
    };

    // Save player to the file again after loop ends
    match save(&player) {
        Ok(_) => { notification("Saved", 1); },
        Err(_) => { 
            notification("An error occurred when saving", 2);
            return;
        }
    };
}

/// Main game loop
fn game(player: &mut Player) -> Result<(), String> {
    // Create new deck and shuffle it
    let mut deck = create_deck_vec();
    shuffle_deck(&mut deck);

    while player.wealth > 0 && player.wealth < u16::MAX {
        match save(&player) {
            Ok(_) => { notification("Saved", 1); },
            Err(_) => { 
                return Err(String::from("An error occurred while saving. Returning to main menu..."));
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
        let mut bet: u16 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                notification("Input a whole number greater than 0", 1);
                continue;
            }
        };
        if bet > player.wealth {
            notification("You don't have that much money", 1);

            continue;
        }

        println!("You are betting ${bet}");

        // Remove bet from player's wealth
        player.wealth = player.wealth.saturating_sub(bet);

        // Shuffle deck if less than half of cards are left
        if deck.len() < 26 {
            deck = create_deck_vec();
            shuffle_deck(&mut deck);
        }

        // Create empty hand vec for player and dealer
        let mut player_hand: Vec<String> = Vec::new();
        let mut dealer_hand: Vec<String> = Vec::new();

        // Deal cards to both
        deal_cards(&mut player_hand, &mut deck, 2)?;
        deal_cards(&mut dealer_hand, &mut deck, 2)?;

        // Print game state before player's turn starts to show cards in case of blackjack
        println!("\n--- YOUR TURN | BET: ${bet} ---");
        println!("\n---");
        println!("What do you want to do?");
        println!("1. Hit");
        println!("2. Stand");
        println!("3. Double down");

        print_game_state(&player_hand, &dealer_hand, false);

        // If both player and dealer get blackjack
        if cards_value(&player_hand) == 21 && cards_value(&dealer_hand) == 21 {
            player.wealth = player.wealth.saturating_add(bet);

            print_game_state(&player_hand, &dealer_hand, true);

            println!("\n--- DRAW ---");
            println!("You and dealer both got a blackjack. You get {bet}$ back");

            continue;
        }

        // If player gets blackjack
        if cards_value(&player_hand) == 21 {
            player.wealth = player.wealth.saturating_add(bet * 2);

            print_game_state(&player_hand, &dealer_hand, true);

            println!("\n--- YOU WON ---");
            println!("You got a blackjack. Won {}$", bet * 2);

            continue;
        }

        // If dealer gets blackjack
        if cards_value(&dealer_hand) == 21 {
            print_game_state(&player_hand, &dealer_hand, true);

            println!("\n--- YOU LOST ---");
            println!("Dealer got a blackjack");

            continue;
        }

        // Player's turn
        let mut index: u8 = 0;
        while cards_value(&player_hand) <= 21 {
            // Create variable for user input integer
            let mut input_int: u8;

            // Add to index
            index += 1;

            loop {
                // Get user input
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                // Clear input to prevent bugs
                print!("\x1B[A\r\x1B[K");
                io::stdout().flush().unwrap();

                // Check if input is valid
                input_int = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        notification("Input a number between 1 and 3", 1);

                        continue;
                    }
                };
                if input_int > 3 {
                    notification("Input a number between 1 and 3", 1);

                    continue;
                }
                if input_int == 3 {            
                    if index > 1 {
                        notification("You can't double down after hitting", 1);

                        continue;
                    }
                    else if player.wealth < bet {
                        notification("You don't have enough money to double down", 1);

                        continue;
                    }
                }
                break;
            }

            if input_int == 1 {
                deal_cards(&mut player_hand, &mut deck, 1)?;

                // Print game state
                print_game_state(&player_hand, &dealer_hand, false);
            } else if input_int == 2 {
                break;
            } else if input_int == 3 {
                // Double down allows player to only hit once with double the bet
                // Reduce bet again from player's wealth to compensate for doubled bet
                player.wealth = player.wealth.saturating_sub(bet);
                bet *= 2;

                deal_cards(&mut player_hand, &mut deck, 1)?;

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
                deal_cards(&mut dealer_hand, &mut deck, 1)?;
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
            player.wealth = player.wealth.saturating_add(bet * 2);

            println!("\n--- YOU WON ---");
            println!("Dealer busted. You won ${}", bet * 2);

            continue;
        }

        // If player and dealer have same value
        if cards_value(&player_hand) == cards_value(&dealer_hand) {
            player.wealth = player.wealth.saturating_add(bet);

            println!("\n--- DRAW ---");
            println!("You and dealer got hands of same value. You get ${bet} back");

            continue;
        }

        // If player has more value than dealer
        if cards_value(&player_hand) > cards_value(&dealer_hand) {
            player.wealth = player.wealth.saturating_add(bet * 2);

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

    // If player runs out of money or somehow gets to u32 max, go back to main menu
    if player.wealth == 0 {
        println!("You ran out of money. Returning to main menu...");
        sleep(Duration::from_secs(2));
    }

    if player.wealth == u16::MAX {
        println!("You have too much money. The casino can't provide for further wins. Returning to main menu...");
        sleep(Duration::from_secs(2));
    }

    Ok(())
}

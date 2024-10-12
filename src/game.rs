use whoami::fallible::realname;

use std::fmt;

use std::{io, u8};
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
    num_str: String,
    icon: char
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
                num_str: String::from(num),
                icon: icon
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
            None                  => {
                println!("Deck empty, cannot deal cards");
                return;
            }
        };

        sleep(Duration::from_millis(200));

        to_vec.push(card);
    }
}

/// Prints the current hands and bet
fn print_game_state(player_hand: &Vec<Card>, dealer_hand: &Vec<Card>, bet: u32, show_dealer_hand: bool) {
    // Calculate total values for both player's and dealer's hands
    let player_hand_value: u8 = cards_value(&player_hand);
    let dealer_hand_value: u8 = cards_value(&dealer_hand);

    if show_dealer_hand {
        print!("Your cards: {:?} ({player_hand_value})\tDealer's cards: {:?} ({dealer_hand_value})\n", player_hand, dealer_hand);
    } else {
        print!("Your cards: {:?} ({player_hand_value})\tDealer's cards: [{}, ??] (??)\n", player_hand, dealer_hand[0]);
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

        println!("You are betting {bet}$");

        // Remove bet from player's wealth
        player.wealth -= bet;

        // Shuffle deck
        shuffle_deck(&mut deck);

        // Create empty hand vec for player and dealer
        let mut player_hand: Vec<Card> = Vec::new();
        let mut dealer_hand: Vec<Card> = Vec::new();

        // Deal cards to both
        deal_cards(&mut player_hand, &mut deck, 2);
        deal_cards(&mut dealer_hand, &mut deck, 2);

        // If both player and dealer get blackjack
        if cards_value(&player_hand) == 21 && cards_value(&dealer_hand) == 21 {
            player.wealth += bet;
            
            println!("--- DRAW ---");
            println!("You and dealer both got a blackjack. You get {bet}$ back");
            print_game_state(&player_hand, &dealer_hand, bet, true);

            continue;
        }

        // If player gets blackjack
        if cards_value(&player_hand) == 21 {
            player.wealth += bet * 2;
            
            println!("--- YOU WON ---");
            println!("You got a blackjack. Won {}$", bet * 2);
            print_game_state(&player_hand, &dealer_hand, bet, true);

            continue;
        }

        // If dealer gets blackjack
        if cards_value(&player_hand) == 21 {            
            println!("--- YOU LOST ---");
            println!("Dealer got a blackjack");
            print_game_state(&player_hand, &dealer_hand, bet, true);

            continue;
        }

        // Player's turn
        while cards_value(&player_hand) <= 21 {
            // Print current game state
            println!("--- BET: {bet}$ ---");
            print_game_state(&player_hand, &dealer_hand, bet, false);

            println!("---");
            println!("What do you want to do?");
            println!("1. Hit");
            println!("2. Stand");

            // Create variable for user input integer
            let mut input_int: u8;

            loop {
                // Get user input
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                // Check if input is valid
                input_int = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Input a number between 1 and 2");
                        sleep(Duration::from_secs(1));
                        continue;
                    }
                };
                if input_int > 2 {
                    println!("Input a number between 1 and 2");
                    sleep(Duration::from_secs(1));
                    continue;
                }
                break;
            }

            if input_int == 1 {
                deal_cards(&mut player_hand, &mut deck, 1);
            }
            else if input_int == 2 {
                break;
            }
        }

        // If player gets more than 21
        if cards_value(&player_hand) > 21 {
            println!("--- YOU LOST ---");
            println!("Your cards went over 21");
            print_game_state(&player_hand, &dealer_hand, bet, true);

            continue;
        }

        // Dealer's turn
        while cards_value(&dealer_hand) <= 21 {
            // Print current game state
            println!("--- BET: {bet}$ ---");
            print_game_state(&player_hand, &dealer_hand, bet, true);

            println!("---");
            println!("Dealer's turn:");
            sleep(Duration::from_secs(1));

            // Dealer stands on 17 or greater
            if cards_value(&dealer_hand) < 17 {
                deal_cards(&mut dealer_hand, &mut deck, 1);
                println!("Hit");
                sleep(Duration::from_secs(1));
            } else {
                println!("Stand");
                sleep(Duration::from_secs(1));
                break;
            }
        }

        // If dealer gets more than 21
        if cards_value(&dealer_hand) > 21 {
            player.wealth += bet * 2;

            println!("--- YOU WON ---");
            println!("Dealer's cards went over 21");
            print_game_state(&player_hand, &dealer_hand, bet, true);

            continue;
        }

        // If player and dealer have same value
        if cards_value(&player_hand) == cards_value(&dealer_hand) {
            player.wealth += bet;
            
            println!("--- DRAW ---");
            println!("You and dealer got hands of same value. You get {bet}$ back");
            print_game_state(&player_hand, &dealer_hand, bet, true);

            continue;
        }

        // If player has more value than dealer
        if cards_value(&player_hand) > cards_value(&dealer_hand) {
            player.wealth += bet * 2;
            
            println!("--- YOU WON ---");
            println!("You were closer to 21. You won {}$", bet * 2);
            print_game_state(&player_hand, &dealer_hand, bet, true);

            continue;
        }

        // If dealer has more value than player
        if cards_value(&player_hand) < cards_value(&dealer_hand) {
            println!("--- YOU LOST ---");
            println!("Dealer was closer to 21.");
            print_game_state(&player_hand, &dealer_hand, bet, true);

            continue;
        }
    }
}
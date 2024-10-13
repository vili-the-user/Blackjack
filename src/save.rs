use std::io::{Write, Read};
use std::fs::File;

use bincode::{serialize, deserialize, Error};

use serde::{Serialize, Deserialize};

/// Serializes and writes player data to a file. If file doesn't exist, new one is created.
/// 
/// # Returns
/// 
/// Ok or Err if serialization failed, file creation failed or writing to file failed
pub fn save(player: &Player) -> Result<(), Error> {
    // Serialize player to a binary format
    let encoded: Vec<u8> = serialize(player)?;

    // Write binary data to a file
    let mut file = File::create("save.blackjack")?;
    file.write_all(&encoded)?;

    Ok(())
}

/// Reads the save file and deserializes player object
/// 
/// # Returns
/// 
/// Ok containing player object or Err if file doesn't exist, failed to read file or failed to deserialize data
pub fn load() -> Result<Player, String> {
    let mut file = match File::open("save.blackjack") {
        Ok(f) => f,
        Err(_) => {
            return Err(String::from("Couldn't find save file"));
        }
    };
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => {},
        Err(_) => {
            return Err(String::from("Failed to read save file"));
        }
    };
    
    let player: Player = match deserialize(&buffer) {
        Ok(p) => p,
        Err(_) => {
            return Err(String::from("Deserialization failed. Save file is corrupted"))
        }
    };
    
    Ok(player)
}

/// Player struct
#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub wealth: u32
}
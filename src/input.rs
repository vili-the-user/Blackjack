use std::convert::TryFrom;

/// Enum for all main menu choices
pub enum MainMenuOptions {
    NewGame,
    Continue,
    Exit
}

impl TryFrom<u8> for MainMenuOptions {
    /// Compares passed number and returns respective MenuOptions value
    /// 
    /// # Returns
    /// 
    /// MenuOptions if passed number is within 1-3, otherwise None
    fn try_from(num: u8) -> Result<Self, Self::Error> {
        match num {
            1 => Ok(MainMenuOptions::NewGame),
            2 => Ok(MainMenuOptions::Continue),
            3 => Ok(MainMenuOptions::Exit),
            _ => Err(String::from(format!("No option for number {}", num)))
        }
    } 

    type Error = String;
}

/// Enum for all in-game choices
pub enum InGameOptions {
    Hit,
    Stand,
    DoubleDown
}

impl TryFrom<u8> for InGameOptions {
    /// Compares passed number and returns respective GameOptions value
    /// 
    /// # Returns
    /// 
    /// MenuOptions if passed number is within 1-3, otherwise None
    fn try_from(num: u8) -> Result<Self, Self::Error> {
        match num {
            1 => Ok(InGameOptions::Hit),
            2 => Ok(InGameOptions::Stand),
            3 => Ok(InGameOptions::DoubleDown),
            _ => Err(String::from(format!("No option for number {}", num)))
        }
    } 

    type Error = String;
}
use std::thread::sleep;
use std::time::Duration;
use std::io;
use std::io::Write;

/// Prints a temporary message for entered amount of seconds
pub fn notification(msg: &str, sec: u64) {
    println!("{msg}");
    sleep(Duration::from_secs(sec));
    print!("\x1B[A\r\x1B[K");
    io::stdout().flush().unwrap();
}

/// Clears terminal completely
pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
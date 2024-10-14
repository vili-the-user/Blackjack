use std::thread::sleep;
use std::time::Duration;
use std::io;
use std::io::Write;

/// Enum contains durations for notifications. This keeps the durations consistent.
pub enum NotificationDuration {
    Short,
    Long
}

impl NotificationDuration {
    /// Converts a NotificationDuration to a Duration
    fn as_duration(&self) -> Duration {
        match self {
            NotificationDuration::Short => Duration::from_secs(1),
            NotificationDuration::Long => Duration::from_secs(2)
        }
    }
}

/// Prints a temporary message for entered amount of seconds
pub fn notification(msg: &str, duration: NotificationDuration) {
    println!("{msg}");
    sleep(duration.as_duration());
    print!("\x1B[A\r\x1B[K");
    io::stdout().flush().unwrap();
}

/// Clears terminal completely
pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}
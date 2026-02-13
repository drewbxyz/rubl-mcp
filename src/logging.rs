//! Stderr logging utilities.
//!
//! MCP clients capture stderr for debugging, making it useful for
//! diagnosing server issues.

const PREFIX: &str = "rubl";

pub fn info(message: impl std::fmt::Display) {
    eprintln!("[{}] {}", PREFIX, message);
}

pub fn error(message: impl std::fmt::Display) {
    eprintln!("[{}] ERROR: {}", PREFIX, message);
}

// Used by panic hook to capture crashes.
pub fn panic(message: impl std::fmt::Display) {
    eprintln!("[{}] PANIC: {}", PREFIX, message);
}

#[allow(dead_code)]
pub fn warn(message: impl std::fmt::Display) {
    eprintln!("[{}] WARN: {}", PREFIX, message);
}

use std::io;

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};


/// Helper struct to handle edge-cases involving
/// the enabling and disabling of raw mode.
///
/// Namely, this struct automatically disables
/// raw mode when it is dropped, which could prove
/// useful in situations where a function could return
/// early.
pub struct RawMode;
impl RawMode {
    pub fn enable() -> io::Result<Self> {
        enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        disable_raw_mode().expect("failed to disable raw mode")
    }
}
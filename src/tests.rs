use crossterm::terminal::is_raw_mode_enabled;

use crate::raw::RawMode;

#[test]
fn test_raw_mode() {
    let raw = RawMode::enable().unwrap();
    assert_eq!(is_raw_mode_enabled().unwrap(), true);

    drop(raw);
    assert_eq!(is_raw_mode_enabled().unwrap(), false);
}

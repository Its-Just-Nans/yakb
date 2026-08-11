#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use bladvak::{Bladvak, MainResult};
use yakb::YakbApp;

fn main() -> MainResult {
    Bladvak::<YakbApp>::bladvak_main()
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use serde::{Deserialize, Serialize};

const CAPITAL_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SIMPLE_CHARS: &str = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz";
const SPECIAL_CHARS: &str = "!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*";
const NUMBERS: &str = "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    use_capital: bool,
    use_simple: bool,
    use_special: bool,
    use_number: bool,
    length: i8,
}

#[tauri::command]
fn generate_password(config: Config) -> String {
    let mut password = String::new();
    let mut reg_pattern = String::new();

    loop {
        password.clear();
        reg_pattern.clear();

        if config.use_capital {
            password.push_str(CAPITAL_CHARS);
            reg_pattern.push_str("[A-Z]+")
        }
        if config.use_simple {
            password.push_str(SIMPLE_CHARS);
            reg_pattern.push_str("[a-z]+")
        }
        if config.use_special {
            password.push_str(SPECIAL_CHARS);
            reg_pattern.push_str("[!@#$%^&*]+")
        }
        if config.use_number {
            password.push_str(NUMBERS);
            reg_pattern.push_str("[0-9]+")
        }

        let re = Regex::new(&reg_pattern[..]).unwrap();

        unsafe {
            password.as_mut_vec().shuffle(&mut thread_rng());
        }

        password = password[0..(config.length as usize)].to_string();

        if re.is_match(&password) {
            break;
        }
    }

    password
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

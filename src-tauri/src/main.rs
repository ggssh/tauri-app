#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Mihayou, {}", name)
}

#[tauri::command]
fn get_vec(len: u32) -> Vec<i32> {
    // dbg!(len);
    let mut v = vec![];
    for i in 0..len {
        v.push(i as i32);
    }
    v
}

#[tauri::command]
fn get_map(len: u32) -> HashMap<u32, String> {
    let mut m = HashMap::new();
    for i in 0..len {
        m.insert(i, (i + 1).to_string());
    }
    m
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![greet, get_vec, get_map])
        .run(context)
        .expect("error while running tauri application");
}

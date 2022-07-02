#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use home::home_dir;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use tauri::*;

#[tauri::command]
fn add_task(content: String) {
    let home_path = home_dir().expect("unable to read home directory");
    let mut path = PathBuf::new();
    path.push(home_path);
    path.push("tasks.txt");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Error while opening tasks file");
    writeln!(file, "{}", content).expect("Error while writing in the tasks file");
}

#[tauri::command]
fn hide_window(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    let menu_item = app.tray_handle().get_item("toggle");
    window.hide().expect("unable to hide window");
    menu_item.set_title("Show").expect("unable to show window");
}

#[tauri::command]
fn show_window(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    let menu_item = app.tray_handle().get_item("toggle");
    window.show().expect("unable to show window");
    window.center().expect("unable to center window");
    menu_item
        .set_title("Hide")
        .expect("unable to set window title");
}

fn make_tray() -> SystemTray {
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle".to_string(), "Hide"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    return SystemTray::new().with_menu(menu);
}

fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        if id.as_str() == "quit" {
            std::process::exit(0);
        }
        if id.as_str() == "toggle" {
            let window = app.get_window("main").unwrap();
            let menu_item = app.tray_handle().get_item("toggle");
            if window.is_visible().unwrap() {
                window.hide().expect("unable to hide window");
                menu_item
                    .set_title("Show")
                    .expect("unable to set window title");
            } else {
                window.show().expect("unable to show window");
                window.center().expect("unable to center window");
                menu_item.set_title("Hide").expect("unable to hide window");
            }
        }
    }
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .system_tray(make_tray())
        .on_system_tray_event(handle_tray_event)
        .invoke_handler(tauri::generate_handler![add_task, hide_window, show_window])
        .run(context)
        .expect("error while running tauri application");
}

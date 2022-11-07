#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{sync::{Arc, Mutex}, time::Duration};

use tauri::{State, Manager};
use tokio::time::sleep;

#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app|{
          let app_handle = app.handle();
          tauri::async_runtime::spawn(async move {
            loop {
                sleep(Duration::from_secs(2)).await;
                println!("Sending backend-ping");
                app_handle.emit_all("backend-ping","ping").unwrap();
            }
          });  

          Ok(())
        })
        .manage(Counter(Default::default()))
        .invoke_handler(tauri::generate_handler![hello_world, add_count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn hello_world() -> String {
    "Hello World - time to sleep!".to_string()
}

#[tauri::command]
fn add_count(num: i32, counter: State<'_, Counter>) -> String {
    let mut val = counter.0.lock().unwrap();
    *val += num;
    format!("{val}")
}


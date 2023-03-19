// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use std::process::Command;

use csv::ReaderBuilder;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read() -> String {
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("C:\\Users\\cambi\\Downloads\\hashtag_donaldtrump.csv").unwrap();

     // Contador de registros
    let mut count = 0;

     // Iterar sobre cada registro en el archivo CSV
    for _ in csv_reader.records() {
         // Desempaquetar el resultado de cada registro
        //  let _record = result?;
 
         // Incrementar el contador de registros
         count += 1;
     }
 
     println!();
    
    format!("Hay {} registros en el archivo CSV", count)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, read])  
        .run(tauri::generate_context!())
        .unwrap();
    println!("END MAIN");
}
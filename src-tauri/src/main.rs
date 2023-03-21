// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use csv::ReaderBuilder;
use serde_json::{Map, Value};
use tauri::{api::process::Command, Manager, LogicalSize, Size};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn open_projects(handle: tauri::AppHandle) {
//     println!("Open projects");
//     let parent = handle.get_window("main").unwrap();
//     // parent.set_ignore_cursor_events(true).expect("Failed to set true ignore cursor events");
//     let _projects_window = tauri::WindowBuilder::new(&handle, "projects", tauri::WindowUrl::External("https://google.com".parse().unwrap()))//tauri::WindowUrl::App("/projects".into())
//         .title("Projects")
//         .resizable(false)
//         .center()
//         .parent_window(parent.hwnd().unwrap())
//         .owner_window(parent.hwnd().unwrap())
//         .build()
//         .unwrap();
//     _projects_window.open_devtools();
//     // _projects_window.on_window_event(move |event| {
//     //     match event {
//     //         WindowEvent::Destroyed => {
//     //             parent.set_ignore_cursor_events(false).expect("Failed to set false ignore cursor events");
//     //         },
//     //         _ => {},
//     //     }
//     // });
// }

// #[tauri::command]
// fn create_project(handle: tauri::AppHandle) {
//     println!("Create project");
//     let parent = handle.get_window("main").unwrap();
//     let projects = handle.get_window("projects").unwrap();
//     projects.close().expect("Failed to close projects window");
//     parent.set_ignore_cursor_events(true).expect("Failed to set true ignore cursor events");
//     let _window = tauri::WindowBuilder::new(&handle, "create_project", tauri::WindowUrl::App("/projects/create".into()))
//         .title("Create project")
//         .resizable(false)
//         .center()
//         .parent_window(parent.hwnd().unwrap())
//         .owner_window(parent.hwnd().unwrap())
//         .build()
//         .unwrap();
//     _window.on_window_event(move |event| {
//         match event {
//             WindowEvent::Destroyed => {
//                 parent.set_ignore_cursor_events(false).expect("Failed to set false ignore cursor events");
//             },
//             _ => {},
//         }
//     });
// }

#[tauri::command]
async fn create_project(id: &str, file_path: &str) -> Result<String, ()> {
    print!("PROJECT ID: {}", id);
    let client = reqwest::Client::new();
    let mut csv_reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_path(file_path)
        .unwrap();

    let headers:Vec<String> = csv_reader.headers().unwrap().iter().map(|header| header.to_string()).collect();
    let records = csv_reader.records();

    for record in records {
        let _record = record.expect("Failed to unwrap record");
        let mut map = Map::new();
        let mut index = 0;
        for header in headers.as_slice() {
            let option = _record.get(index);
            match option {
                Some(val) => {
                    map.insert(header.to_string(), Value::String(val.to_string()));
                    ()
                },
                None => {
                    map.insert(header.to_string(), Value::Null);
                    ()
                },
            }
            index = index + 1;
        }
        let json_string = serde_json::to_string(&map).unwrap();
        let _response = client.post("http://127.0.0.1:8000/key/table")
            .basic_auth("root", Some("root"))
            .header("Accept", "application/json")
            .header("NS", "main")
            .header("DB", id)
            .body(json_string)
            .send().await;
        drop(_response);
        drop(index);
        drop(_record);
    }
    drop(headers);
    drop(csv_reader);
    drop(client);
    print!("DONE");
    Ok("DONE".to_string())
}

#[tauri::command]
fn resize(handle: tauri::AppHandle, h: f64, w: f64) {
    let main = handle.get_window("main").unwrap();
    let size = Size::Logical(LogicalSize { width: w, height: h });
    main.set_min_size(Some(size)).expect("Failed set_min_size");
    main.set_size(size).expect("Failed set_size");
    main.set_resizable(false).expect("Failed set_size");
    main.center().expect("Failed center");
    
    // let projects = handle.get_window("projects").unwrap();
    // projects.close().expect("Failed to close projects window");
    // parent.set_ignore_cursor_events(true).expect("Failed to set true ignore cursor events");
    // let _window = tauri::WindowBuilder::new(&handle, "create_project", tauri::WindowUrl::App("/projects/create".into()))
    //     .title("Create project")
    //     .resizable(false)
    //     .center()
    //     .parent_window(parent.hwnd().unwrap())
    //     .owner_window(parent.hwnd().unwrap())
    //     .build()
    //     .unwrap();
    // _window.on_window_event(move |event| {
    //     match event {
    //         WindowEvent::Destroyed => {
    //             parent.set_ignore_cursor_events(false).expect("Failed to set false ignore cursor events");
    //         },
    //         _ => {},
    //     }
    // });
}

// #[tauri::command]
// fn read_config_files(app_handle: tauri::AppHandle) -> Vec<DiskEntry> {
//     let path = app_handle.path_resolver().app_data_dir().unwrap().join("projects");
//     if !path.exists() {
//         create_dir(path).expect("Failed to create the projects directory");
//         return Vec::<DiskEntry>::new()
//     }
//     let read_result = read_dir(path, false);

//     read_result.unwrap()
// }

// #[tauri::command]
// fn read_columns(path: &str) -> Vec<String> {
//     let mut csv_reader = ReaderBuilder::new()
//         .has_headers(true)
//         .from_path(path)
//         .unwrap();
    
//     let headers = csv_reader.headers().unwrap();

//     let iter = headers.iter().map(|header| header.to_string()).collect();
//     iter
// }

// #[tauri::command]
// fn read() -> String {
//     let mut csv_reader = ReaderBuilder::new()
//         .has_headers(true)
//         .from_path("C:\\Users\\cambi\\Downloads\\hashtag_donaldtrump.csv")
//         .unwrap();

//     // Contador de registros
//     let mut count = 0;

//     // Iterar sobre cada registro en el archivo CSV
//     for _ in csv_reader.records() {
//         // Desempaquetar el resultado de cada registro
//         //  let _record = result?;

//         // Incrementar el contador de registros
//         count += 1;
//     }

//     println!();

//     format!("Hay {} registros en el archivo CSV", count)
// }

fn main() {
    // tauri::async_runtime::spawn(async move {
    //     // read events such as stdout
    //     while let Some(event) = rx.recv().await {
    //         if let CommandEvent::Stdout(line) = event {
    //             window
    //                 .emit("message", Some(format!("'{}'", line)))
    //                 .expect("failed to emit event");
    //             // write to stdin
    //             child.write("message from Rust\n".as_bytes()).unwrap();
    //         }
    //     }
    // });

    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app.path_resolver().app_data_dir().unwrap();
            let path_dir = data_dir.join("data");
            println!("Current data dir: {}", path_dir.display());
            Command::new_sidecar("surreal")
            .expect("failed to create `surreal` binary command")
            .args(["start", "--log", "debug", "--user", "root", "--pass", "root", format!("file://{}", path_dir.display()).as_str()])
            .spawn()
            .expect("Failed to spawn sidecar");
            println!("Spawned DB");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_project, resize])
        .run(tauri::generate_context!())
        .unwrap();
    
}

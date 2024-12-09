use std::fs::File;
use std::io::Read;
use std::io::Write;
use tauri_plugin_dialog::DialogExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// TODO: Have this function return an object: two attributes, a success flag and
// an error message
#[tauri::command]
fn save_file(file_name: &str, file_contents: &str) -> bool {
    if file_contents.len() == 0 {
        return false;
    }

    println!("Saving file...");

    let mut file = match File::create("Test.txt") {
        Err(_) => return false,
        Ok(file) => file,
    };

    return match file.write_all(file_contents.as_bytes()) {
        Err(_) => false,
        Ok(_) => true,
    };
}

#[tauri::command]
async fn open_folder(app_handle: tauri::AppHandle) -> Option<Vec<String>> {
    let mut files: Vec<String> = Vec::new();

    println!("Opening Folder dialog");

    // Prompt user to select a folder
    app_handle
        .dialog()
        .file()
        .pick_folder(|f| {
            f
            .unwrap()
            .as_path()
            .unwrap()
            .read_dir()
            .unwrap()
            .for_each(|dir_res| {
                let s = dir_res
                    .unwrap()
                    .file_name()
                    .into_string();

                if let Ok(filename) = s {
                    &files.push(filename);
                }

            });
        });

    return Some(files);
}

#[tauri::command]
fn open_file(file_name: &str) -> String {
    let mut file = match File::open(file_name) {
        Err(_) => return "Cannot open file".to_string(),
        Ok(handle) => handle,
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Err(_) => "Cannfot read file contents".to_string(),
        Ok(_) => contents,
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(
            tauri::generate_handler![greet, save_file, open_folder]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

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
fn open_folder(folder_name: &str) -> Option<Vec<String>> {
    // Determine if item is a valid directory path, if not, return None
    let files: Vec<String> = Vec::new();

    // Attempt to read folder contents and load them into the 'files' collection to return
    return Some(files);
}

async fn open_folder_via_dialog(app_handle: tauri::AppHandle) -> Option<Vec<String>> {
    // Prompt user to select a folder
    let folder_name = app_handle.dialog().file().blocking_pick_folder();


    if let Option::None = folder_name {
        return None;
    }

    return open_folder("test");
}

#[tauri::command]
fn open_file(file_name: &str) -> String {
    let mut file = match File::open(file_name) {
        Err(_) => return "Cannot open file".to_string(),
        Ok(handle) => handle,
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Err(_) => "Cannot read file contents".to_string(),
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

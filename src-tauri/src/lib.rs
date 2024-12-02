use std::fs::File;
use std::io::Read;
use std::io::Write;
use tauri_plugin_dialog;

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
fn save_file_via_dialog(file_contents: &str) -> bool {
    // TODO: Call 'set_parent' here to bind the new dialog to the main window

    // let mut was_successful;


    // tauri::AppHandle::dialog()
    //     .file()
    //     .set_title("Choose a file to save to")
    //     .save_file(|path| {
    //         // 'path' is a file path, chosen by the user. It is an Option
    //         // handl
    //         if let Some(p) = path {
    //             was_successful = p
    //                 .as_path()
    //                 .and_then(|file_path| { file_path.to_str() })
    //                 .and_then(|filename| {
    //                     save_file(filename, file_contents);
    //                     true
    //                 })
    //                 .ok_or_else(false);
    //         }
    //         else {
    //             was_successful = false;
    //         }
    //     });

    // return was_successful;
    return false;
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
        .invoke_handler(tauri::generate_handler![greet, save_file, open_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

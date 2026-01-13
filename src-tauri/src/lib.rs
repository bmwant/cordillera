mod massif;

use massif::MassifData;

#[tauri::command]
fn parse_massif(path: String) -> Result<MassifData, String> {
    massif::parse_massif_file(&path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![parse_massif])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

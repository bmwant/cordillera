mod massif;

use massif::MassifData;
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::Emitter;

#[tauri::command]
fn parse_massif(path: String) -> Result<MassifData, String> {
    massif::parse_massif_file(&path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let open_file = MenuItemBuilder::new("Open File...")
                .id("open_file")
                .accelerator("CmdOrCtrl+O")
                .build(app)?;

            let file_menu = SubmenuBuilder::new(app, "File")
                .item(&open_file)
                .build()?;

            let menu = MenuBuilder::new(app)
                .item(&file_menu)
                .build()?;

            app.set_menu(menu)?;

            Ok(())
        })
        .on_menu_event(|app, event| {
            if event.id() == "open_file" {
                let _ = app.emit("menu-open-file", ());
            }
        })
        .invoke_handler(tauri::generate_handler![parse_massif])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

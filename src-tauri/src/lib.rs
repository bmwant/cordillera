mod massif;

use massif::MassifData;
use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder};
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
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let open_file = MenuItemBuilder::new("Open File...")
                .id("open_file")
                .accelerator("CmdOrCtrl+O")
                .build(app)?;

            let export_svg = MenuItemBuilder::new("Export to SVG...")
                .id("export_svg")
                .accelerator("CmdOrCtrl+E")
                .build(app)?;

            let file_menu = SubmenuBuilder::new(app, "File")
                .item(&open_file)
                .item(&export_svg)
                .separator()
                .item(&PredefinedMenuItem::quit(app, Some("Exit"))?)
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
            } else if event.id() == "export_svg" {
                let _ = app.emit("menu-export-svg", ());
            }
        })
        .invoke_handler(tauri::generate_handler![parse_massif])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn format_for(text: String, filename: String) -> Result<String, String> {
    let result = autocorrect::format_for(&text, &filename);

    if result.has_error() {
        return Err(result.error);
    }

    return Ok(result.out);
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![format_for])
        .run(context)
        .expect("error while running tauri application");
}

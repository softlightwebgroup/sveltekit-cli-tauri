// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod model;
mod commands;
mod db;
mod vars;
mod functions;

fn main() {
    tauri::Builder::default()
        .manage(db::DatabaseConnection::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_projects,
            commands::save_project,
            commands::get_files,
            commands::get_commands,
            commands::run_command,
            commands::get_routes,
            commands::create_route,
            commands::create_component,
            commands::create_store,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

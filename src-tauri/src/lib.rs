pub mod auth;
pub mod commands;
pub mod database;
pub mod models;
pub mod server;
pub mod services;

use database::DbState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_state = DbState::new().expect("Failed to initialize database");

    // Start local embedded server for Android companion
    server::start_local_api_server();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(db_state)
        .invoke_handler(tauri::generate_handler![
            commands::login,
            commands::get_dashboard_stats,
            commands::get_active_cash_session,
            commands::open_cash_session,
            commands::add_cash_movement,
            commands::close_cash_session,
            commands::list_cash_movements,
            commands::search_products,
            commands::save_product,
            commands::get_categories,
            commands::get_units,
            commands::process_sale,
            commands::list_sales,
            commands::add_expense,
            commands::list_expenses,
            commands::list_employees,
            commands::save_employee,
            commands::list_payrolls,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

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

    server::start_local_api_server();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(db_state)
        .invoke_handler(tauri::generate_handler![
            commands::login,
            commands::change_user_password,
            commands::get_user_by_qr,
            commands::get_dashboard_stats,
            commands::get_active_cash_session,
            commands::open_cash_session,
            commands::add_cash_movement,
            commands::close_cash_session,
            commands::list_cash_movements,
            commands::list_session_history,
            commands::search_products,
            commands::save_product,
            commands::delete_product,
            commands::get_categories,
            commands::save_category,
            commands::delete_category,
            commands::get_units,
            commands::process_sale,
            commands::list_sales,
            commands::get_sale_items,
            commands::hold_sale,
            commands::list_held_sales,
            commands::delete_held_sale,
            commands::list_customers,
            commands::save_customer,
            commands::delete_customer,
            commands::record_customer_debt_payment,
            commands::list_suppliers,
            commands::save_supplier,
            commands::delete_supplier,
            commands::create_purchase,
            commands::list_purchases,
            commands::add_expense,
            commands::list_expenses,
            commands::list_employees,
            commands::save_employee,
            commands::delete_employee,
            commands::list_payrolls,
            commands::get_all_settings,
            commands::set_setting,
            commands::set_multiple_settings,
            commands::get_hwid,
            commands::verify_license,
            commands::factory_reset,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
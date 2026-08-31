pub mod auth;
pub mod commands;
pub mod database;
pub mod models;
pub mod server;
pub mod services;

use database::DbState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_state = DbState::new().expect("Failed to initialize database");

    server::start_local_api_server();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // A second launch just focuses the existing window.
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.show();
                let _ = w.unminimize();
                let _ = w.set_focus();
            }
        }))
        .manage(db_state)
        .invoke_handler(tauri::generate_handler![
            commands::login,
            commands::get_active_users,
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
            commands::toggle_product_pin,
            commands::reorder_pinned_products,
            commands::process_sale,
            commands::create_sale,
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
            commands::record_supplier_debt_payment,
            commands::list_supplier_debt_payments,
            commands::create_purchase,
            commands::get_purchase_items,
            commands::delete_purchase,
            commands::list_purchases,
            commands::add_expense,
            commands::update_expense,
            commands::list_expenses,
            commands::delete_expense,
            commands::list_employees,
            commands::save_employee,
            commands::delete_employee,
            commands::list_payrolls,
            commands::record_employee_advance,
            commands::list_employee_advances,
            commands::get_all_settings,
            commands::set_setting,
            commands::set_multiple_settings,
            commands::send_telegram_message,
            commands::send_telegram_recap,
            commands::print_html_direct,
            commands::get_setting,
            commands::get_price_history,
            commands::clear_transaction_history,
            commands::get_hwid,
            commands::verify_license,
            commands::factory_reset,
            commands::test_scale_connection,
            commands::upload_product_to_scale,
            commands::upload_all_scalable_to_scale,
            commands::fetch_products_from_scale,
            commands::get_scale_sync_logs,
            commands::open_serial_cash_drawer,
            commands::delete_sale,
            commands::verify_admin_password,
            commands::save_unit,
            commands::backup_database,
            commands::restore_database,
            commands::get_app_version,
            commands::get_all_users,
            commands::get_all_roles,
            commands::create_user,
            commands::update_user,
            commands::delete_user,
            commands::check_github_update,
            commands::set_autostart,
            commands::get_autostart,
            commands::list_printers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
use axum::{routing::get, Json, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

use std::sync::OnceLock;
static DIAG_DB: OnceLock<crate::database::DbState> = OnceLock::new();

pub fn set_diag_db(db: crate::database::DbState) {
    let _ = DIAG_DB.set(db);
}

pub fn start_local_api_server() {
    std::thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build();

        if let Ok(rt) = rt {
            rt.block_on(async {
                let app = Router::new()
                    .route(
                        "/api/diag/login",
                        get(|| async {
                            // Runs the login backend path in-process and
                            // reports latency: if this hangs, the DB mutex is
                            // stuck inside the app (reproduces the reported
                            // "keeps saying signing in" from outside).
                            let t0 = std::time::Instant::now();
                            let login_result = DIAG_DB.get().map(|db| {
                                crate::auth::authenticate_user(db, "admin", "admin")
                                    .map(|u| u.map(|x| x.username))
                            });
                            let login_ms = t0.elapsed().as_millis() as u64;
                            let t1 = std::time::Instant::now();
                            let users_result = DIAG_DB.get().map(|db| {
                                crate::auth::list_active_users(db).map(|u| u.len())
                            });
                            let users_ms = t1.elapsed().as_millis() as u64;
                            let t2 = std::time::Instant::now();
                            let settings_result = DIAG_DB.get().map(|db| {
                                crate::services::settings_service::get_all_settings(db)
                                    .map(|m| m.len())
                            });
                            let settings_ms = t2.elapsed().as_millis() as u64;
                            Json(serde_json::json!({
                                "login": {"user": login_result, "ms": login_ms},
                                "list_users": {"count": users_result, "ms": users_ms},
                                "get_all_settings": {"keys": settings_result, "ms": settings_ms},
                            }))
                        }),
                    )
                    .route(
                        "/api/status",
                        get(|| async {
                            Json(serde_json::json!({
                                "status": "online",
                                "server": "TitaouPosT Host",
                                "version": "0.1.0"
                            }))
                        }),
                    )
                    .layer(CorsLayer::permissive());

                let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
                println!("[Local Server] Listening on http://{}", addr);

                if let Ok(listener) = tokio::net::TcpListener::bind(addr).await {
                    let _ = axum::serve(listener, app).await;
                }
            });
        }
    });
}
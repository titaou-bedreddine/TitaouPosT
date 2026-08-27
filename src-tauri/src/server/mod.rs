use axum::{routing::get, Json, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

pub fn start_local_api_server() {
    std::thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build();

        if let Ok(rt) = rt {
            rt.block_on(async {
                let app = Router::new()
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
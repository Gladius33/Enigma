#[cfg(test)]
mod tests {
    use actix_web::{web, App, HttpServer, HttpResponse, Responder};
    use std::sync::Mutex;
    use std::collections::HashMap;
    use std::net::TcpListener;
    use crate::server::PublicIdentity;
    use crate::consensus::check_username_availability;

    async fn mock_check_handler(user_exists: bool) -> impl Responder {
        if user_exists {
            HttpResponse::Ok().body("EXISTS")
        } else {
            HttpResponse::NotFound().body("NOT_FOUND")
        }
    }

    fn spawn_mock_node(user_exists: bool) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = format!("http://{}", listener.local_addr().unwrap());

        std::thread::spawn(move || {
            let _ = actix_rt::System::new();
            let srv = HttpServer::new(move || {
                App::new().route(
                    "/check_user/{username}",
                    web::get().to(move |_path| mock_check_handler(user_exists)),
                )
            })
            .listen(listener)
            .unwrap()
            .run();

            let _ = actix_rt::System::run(srv);
        });

        addr
    }

    #[actix_rt::test]
    async fn test_check_username_availability_network() {
        let node1 = spawn_mock_node(false); // ne connaît pas l'utilisateur
        let node2 = spawn_mock_node(true);  // connaît l'utilisateur

        // 1st test : Username available → true
        let available = check_username_availability("ghost", &[node1.clone()]).await.unwrap();
        assert!(available);

        // 2nd test : username already exist → false
        let taken = check_username_availability("ghost", &[node1, node2]).await.unwrap();
        assert!(!taken);
    }
}

    #[actix_rt::test]
    async fn test_broadcast_identity() {
        use crate::consensus::broadcast_identity;
        use actix_web::HttpRequest;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use serde_json::Value;

        static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

        let sync_handler = |body: web::Bytes, _req: HttpRequest| async move {
            if let Ok(parsed) = serde_json::from_slice::<Value>(&body) {
                if parsed.get("testuser").is_some() {
                    CALL_COUNT.fetch_add(1, Ordering::SeqCst);
                }
            }
            HttpResponse::Ok().body("OK")
        };

        fn spawn_sync_node() -> String {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = format!("http://{}", listener.local_addr().unwrap());

            let _ = std::thread::spawn(move || {
                let _ = actix_rt::System::new();
                let srv = HttpServer::new(move || {
                    App::new().route("/sync", web::post().to(sync_handler))
                })
                .listen(listener)
                .unwrap()
                .run();
                let _ = actix_rt::System::run(srv);
            });

            addr
        }

        let mock_node1 = spawn_sync_node();
        let mock_node2 = spawn_sync_node();

        let identity = crate::server::PublicIdentity {
            username: "testuser".to_string(),
            public_key: "pub".to_string(),
            signature: "sig".to_string(),
            timestamp: 0,
        };

        let nodes = vec![mock_node1, mock_node2];
        let result = broadcast_identity(&identity, &nodes).await;

        assert!(result.is_ok());
        assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 2);
    }

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use crate::server::{AppState, PublicIdentity, PeerPresence};
    use std::collections::{HashMap, HashSet};
    use std::sync::Mutex;

    fn test_state() -> web::Data<AppState> {
        let dummy_config = crate::server::Config {
            node: crate::server::NodeConfig {
                mode: "public".to_string(),
                bind_address: "127.0.0.1".to_string(),
                bind_port: 1488,
                max_users: 10,
            },
            sync: crate::server::SyncConfig {
                enabled: false,
                initial_nodes: vec![],
            },
        };

        web::Data::new(AppState {
            known_users: Mutex::new(HashMap::new()),
            active_peers: Mutex::new(HashMap::new()),
            known_nodes: Mutex::new(HashSet::new()),
            config: dummy_config,
        })
    }

    #[actix_rt::test]
    async fn test_check_user() {
        let state = test_state();
        let identity = PublicIdentity {
            username: "testuser".to_string(),
            public_key: "key".to_string(),
            signature: "sig".to_string(),
            timestamp: 0,
        };

        state.known_users.lock().unwrap().insert("testuser".to_string(), identity);

        let app = test::init_service(
            App::new()
                .app_data(state.clone())
                .route("/check_user/{username}", web::get().to(crate::server::check_user))
        ).await;

        let req = test::TestRequest::get()
            .uri("/check_user/testuser")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let req_not_found = test::TestRequest::get()
            .uri("/check_user/unknown")
            .to_request();

        let resp_nf = test::call_service(&app, req_not_found).await;
        assert_eq!(resp_nf.status(), 404);
    }

    #[actix_rt::test]
    async fn test_announce() {
        let state = test_state();

        let app = test::init_service(
            App::new()
                .app_data(state.clone())
                .route("/announce", web::post().to(crate::server::announce))
        ).await;

        let peer = PeerPresence {
            ip: "1.2.3.4".to_string(),
            port: 1234,
            timestamp: chrono::Utc::now().timestamp() as u64,
        };

        let req = test::TestRequest::post()
            .uri("/announce")
            .set_json(&peer)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let peers = state.active_peers.lock().unwrap();
        assert!(peers.contains_key("1.2.3.4"));
    }
}

    #[actix_rt::test]
    async fn test_register_and_resolve() {
        let state = test_state();

        let app = test::init_service(
            App::new()
                .app_data(state.clone())
                .route("/register", web::post().to(crate::server::register))
                .route("/resolve/{username}", web::get().to(crate::server::resolve))
        ).await;

        let identity = PublicIdentity {
            username: "john_doe".to_string(),
            public_key: "pubkey123".to_string(),
            signature: "sigXYZ".to_string(),
            timestamp: chrono::Utc::now().timestamp() as u64,
        };

        // Register the identity
        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&identity)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        // Resolve the identity
        let resolve_req = test::TestRequest::get()
            .uri("/resolve/john_doe")
            .to_request();

        let resolve_resp = test::call_service(&app, resolve_req).await;
        assert_eq!(resolve_resp.status(), 200);

        // Try resolving unknown user
        let unknown = test::TestRequest::get()
            .uri("/resolve/unknown_user")
            .to_request();

        let unknown_resp = test::call_service(&app, unknown).await;
        assert_eq!(unknown_resp.status(), 404);
    }

    #[actix_rt::test]
    async fn test_sync() {
        let state = test_state();

        let app = test::init_service(
            App::new()
                .app_data(state.clone())
                .route("/sync", web::post().to(crate::server::sync))
        ).await;

        let mut identities = HashMap::new();
        let user = PublicIdentity {
            username: "sync_user".to_string(),
            public_key: "abc123".to_string(),
            signature: "sig".to_string(),
            timestamp: 123456789,
        };
        identities.insert("sync_user".to_string(), user.clone());

        let req = test::TestRequest::post()
            .uri("/sync")
            .set_json(&identities)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let users = state.known_users.lock().unwrap();
        assert!(users.contains_key("sync_user"));
        assert_eq!(users.get("sync_user").unwrap().public_key, "abc123");
    }

    #[actix_rt::test]
    async fn test_nodes() {
        let state = test_state();
        {
            let mut nodes = state.known_nodes.lock().unwrap();
            nodes.insert("https://node1.test:1488".to_string());
            nodes.insert("https://node2.test:1488".to_string());
        }

        let app = test::init_service(
            App::new()
                .app_data(state.clone())
                .route("/nodes", web::get().to(crate::server::nodes))
        ).await;

        let req = test::TestRequest::get()
            .uri("/nodes")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let body: Vec<String> = test::read_body_json(resp).await;
        assert!(body.contains(&"https://node1.test:1488".to_string()));
        assert!(body.contains(&"https://node2.test:1488".to_string()));
    }

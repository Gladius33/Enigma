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

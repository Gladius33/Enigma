use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::fs;
use tokio::time::{interval, Duration};

const PRESENCE_TTL_SECS: u64 = 300; // 5 minutes

// ===================== Configuration structures =====================

#[derive(Debug, Deserialize)]
struct Config {
    node: NodeConfig,
    sync: SyncConfig,
}

#[derive(Debug, Deserialize)]
struct NodeConfig {
    mode: String,
    bind_address: String,
    bind_port: u16,
    max_users: usize,
}

#[derive(Debug, Deserialize)]
struct SyncConfig {
    enabled: bool,
    initial_nodes: Vec<String>,
}

// ===================== Runtime data structures =====================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicIdentity {
    pub username: String,
    pub public_key: String,
    pub signature: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PeerPresence {
    pub ip: String,
    pub port: u16,
    pub timestamp: u64,
}

pub struct AppState {
    pub known_users: Mutex<HashMap<String, PublicIdentity>>,
    pub active_peers: Mutex<HashMap<String, PeerPresence>>,
    pub known_nodes: Mutex<HashSet<String>>,
    pub config: Config,
}

// ===================== Handlers =====================

async fn register(
    data: web::Data<AppState>,
    info: web::Json<PublicIdentity>,
) -> impl Responder {
    let mut users = data.known_users.lock().unwrap();
    if users.contains_key(&info.username) {
        return HttpResponse::Conflict().body("Username already exists");
    }
    users.insert(info.username.clone(), info.into_inner());
    HttpResponse::Ok().body("User registered")
}

async fn resolve(
    data: web::Data<AppState>,
    web::Path(username): web::Path<String>,
) -> impl Responder {
    let users = data.known_users.lock().unwrap();
    if let Some(identity) = users.get(&username) {
        HttpResponse::Ok().json(identity)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

async fn announce(
    data: web::Data<AppState>,
    info: web::Json<PeerPresence>,
) -> impl Responder {
    let mut peers = data.active_peers.lock().unwrap();
    peers.insert(info.ip.clone(), info.into_inner());
    HttpResponse::Ok().body("Peer announced")
}

async fn sync(
    data: web::Data<AppState>,
    info: web::Json<HashMap<String, PublicIdentity>>,
) -> impl Responder {
    let mut users = data.known_users.lock().unwrap();
    for (username, identity) in info.into_inner() {
        users.entry(username).or_insert(identity);
    }
    HttpResponse::Ok().body("Sync completed")
}

async fn nodes(data: web::Data<AppState>) -> impl Responder {
    let nodes = data.known_nodes.lock().unwrap();
    let list: Vec<String> = nodes.iter().cloned().collect();
    HttpResponse::Ok().json(list)
}

async fn check_user(
    data: web::Data<AppState>,
    web::Path(username): web::Path<String>,
) -> impl Responder {
    let users = data.known_users.lock().unwrap();
    if users.contains_key(&username) {
        HttpResponse::Ok().body("EXISTS")
    } else {
        HttpResponse::NotFound().body("NOT_FOUND")
    }
}

// ===================== Main =====================

pub async fn start_server() {
    let config_str = fs::read_to_string("nodes/config.toml").expect("Failed to read config.toml");
    let config: Config = toml::from_str(&config_str).expect("Invalid config format");

    let state = web::Data::new(AppState {
        known_users: Mutex::new(HashMap::new()),
        active_peers: Mutex::new(HashMap::new()),
        known_nodes: Mutex::new(config.sync.initial_nodes.iter().cloned().collect()),
        config: config.clone(),
    });

    // Spawn cleanup task for peer TTL
    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut cleaner = interval(Duration::from_secs(60));
        loop {
            cleaner.tick().await;
            let mut peers = state_clone.active_peers.lock().unwrap();
            let now = chrono::Utc::now().timestamp() as u64;
            peers.retain(|_ip, peer| now.saturating_sub(peer.timestamp) < PRESENCE_TTL_SECS);
        }
    });

    // Start server
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/register", web::post().to(register))
            .route("/resolve/{username}", web::get().to(resolve))
            .route("/announce", web::post().to(announce))
            .route("/sync", web::post().to(sync))
            .route("/nodes", web::get().to(nodes))
            .route("/check_user/{username}", web::get().to(check_user))
    })
    .bind((config.node.bind_address.as_str(), config.node.bind_port))
    .expect("Failed to bind server")
    .run()
    .await
    .expect("Server failed");
}

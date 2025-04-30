use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::fs;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PublicIdentity {
    username: String,
    public_key: String,
    signature: String,
    timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PeerPresence {
    ip: String,
    port: u16,
    timestamp: u64,
}

struct AppState {
    known_users: Mutex<HashMap<String, PublicIdentity>>,
    active_peers: Mutex<HashMap<String, PeerPresence>>,
    known_nodes: Mutex<HashSet<String>>,
    config: Config,
}

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_str = fs::read_to_string("nodes/config.toml").expect("Failed to read config.toml");
    let config: Config = toml::from_str(&config_str).expect("Invalid config format");

    let state = web::Data::new(AppState {
        known_users: Mutex::new(HashMap::new()),
        active_peers: Mutex::new(HashMap::new()),
        known_nodes: Mutex::new(config.sync.initial_nodes.iter().cloned().collect()),
        config: config.clone(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/register", web::post().to(register))
            .route("/resolve/{username}", web::get().to(resolve))
            .route("/announce", web::post().to(announce))
            .route("/sync", web::post().to(sync))
            .route("/nodes", web::get().to(nodes))
    })
    .bind((config.node.bind_address.as_str(), config.node.bind_port))?
    .run()
    .await
}

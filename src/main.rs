use axum::Router;
use server::{
    events_server,
    user_server::{self},
};
use tokio::net::TcpListener;

mod db;
mod model;
mod server;
mod tests;

#[tokio::main]
async fn main() {
    // Roteador com Axum
    let mut app = user_server::functions::add_routes(Router::new());
    app = events_server::functions::add_routes(app);

    // Sobe o servidor
    let addr = "127.0.0.1:3000";
    let tcp_addr = TcpListener::bind(addr).await.unwrap();
    println!("Iniciando servidor em {}", addr);
    axum::serve(tcp_addr, app).await.unwrap();
}

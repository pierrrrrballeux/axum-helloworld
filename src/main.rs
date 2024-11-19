use axum::{
  extract::ConnectInfo,
  routing,
  Router
};
use std::net::SocketAddr;

async fn handler(ConnectInfo(ip): ConnectInfo<SocketAddr>) -> String {
  format!("IP: {ip}")
}

#[tokio::main]
async fn main(){

  let app = Router::new().route("/",routing::get(handler));
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

  axum::serve(
    listener,
    app.into_make_service_with_connect_info::<SocketAddr>()
  ).await.unwrap()
}
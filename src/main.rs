use axum::{
  extract::ConnectInfo,
  routing,
  response::Html,
  Router
};
use std::net::SocketAddr;
use tera::{Context,Tera};

async fn handler(ConnectInfo(ip): ConnectInfo<SocketAddr>) -> Html<String> {
  let tera = Tera::new("assets/*.html").unwrap();
  let mut ctx = Context::new();
  ctx.insert("IP", &ip.to_string());

  Html(tera.render("index.html", &ctx).unwrap().to_owned())
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
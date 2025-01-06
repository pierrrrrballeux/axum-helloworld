use axum::{
  extract::ConnectInfo, 
  response::{
    Html,
    Json,
    Redirect
  }, 
  routing::{get, post}, 
  Router
};
use std::net::SocketAddr;
use tera::{Context,Tera};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
  id: u32,
  username: String,

}

#[tokio::main]
async fn main(){
  let app = router();
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

  axum::serve(
    listener,
    app.into_make_service_with_connect_info::<SocketAddr>()
  ).await.unwrap()
}

fn router() -> Router {
  Router::new()
    .route("/",get(handler))
    .route("/redirect", get(|| async { Redirect::temporary("https://www.youtube.com/watch?v=dQw4w9WgXcQ") }))
    .route("/api/user", get(get_user))
}

async fn handler(ConnectInfo(ip): ConnectInfo<SocketAddr>) -> Html<String> {
  let tera = Tera::new("assets/*.html").unwrap();
  let mut ctx = Context::new();
  ctx.insert("IP", &ip.to_string());

  Html(tera.render("index.html", &ctx).unwrap().to_owned())
}

async fn get_user() -> Json<Response> {
  Json(Response {
    id: 1234,
    username: "Yamada Taro".to_string()
  })
}
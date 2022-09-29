use serde::{Deserialize};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use fast_qr::{QRBuilder, ECL};

#[derive(Deserialize,Clone)]
struct QueryParams {
  url: String,
}

#[get("/manifest")]
async fn mainfest(query_params: web::Query<QueryParams>) -> impl Responder {
let mainfest = format!("{{
    \"short_name\": \"myfreax\",
    \"name\": \"myfreax\",
    \"icons\": [
      {{
            \"src\": \"/assets/images/favicon-192.png\",
            \"type\": \"image/png\",
            \"sizes\": \"192x192\",
            \"purpose\": \"any maskable\"
            }},
        {{
            \"src\": \"/assets/images/favicon.png\",
            \"type\": \"image/png\",
            \"sizes\": \"512x512\",
            \"purpose\": \"any maskable\"
          }}
    ],
    \"start_url\": \"{}\",
    \"background_color\": \"#fff\",
    \"display\": \"standalone\",
    \"scope\": \".\",
    \"theme_color\": \"#2ea1f2\"
}}",query_params.url);
  HttpResponse::Ok().content_type::<String>("text/json".to_string()).body(mainfest)
}

#[get("/qrcode")]
async fn qrcode(query_params: web::Query<QueryParams>) -> impl Responder {
  let qrcode = QRBuilder::new(format!("{}", query_params.url))
    .ecl(ECL::H)
    .build();
  let svg = fast_qr::convert::svg::SvgBuilder::new()
    .shape(fast_qr::convert::svg::SvgShape::RoundedSquare)
    .build_qr(qrcode.unwrap());
  HttpResponse::Ok().content_type::<String>("image/svg+xml".to_string()).body(svg)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
    .service(qrcode)
    .service(mainfest)
  })
  .bind(("127.0.0.1", 8778))?
  .run()
  .await
}

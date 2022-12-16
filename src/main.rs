use actix_web::{ App, HttpServer};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::links::create_link)
            .service(api::links::get_all_links)
            .service(api::links::get_from_link)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

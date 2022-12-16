use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use settings::Settings;
use sqlx::mysql::MySqlPoolOptions;
use tera::{Context, Tera};

mod api;
mod settings;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing errors: {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

#[get("/")]
async fn index() -> impl Responder {
    let content = "Generating a tiny code format url";
    let mut data = Context::new();
    data.insert("content", content);
    let rendered = TEMPLATES.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let s = Settings::new().unwrap();
    let ip = s.server.get_ip();
    let url = s.database.url;
    let pool_size = s.database.pool_size;

    let pool = MySqlPoolOptions::new()
        .max_connections(pool_size)
        .connect(&url)
        .await?;

    let row: (i64, ) = sqlx::query_as("SELECT ?")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;
    let ret = row.0;
    println!("row is: {}", ret);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(api::links::create_link)
            .service(api::links::get_all_links)
            .service(api::links::get_from_link)
    })
        .bind(&ip)?
        .run()
        .await?;

    Ok(())
}

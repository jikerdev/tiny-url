use actix_web::{App, HttpServer, web};
use sqlx::mysql::MySqlPoolOptions;
use settings::Settings;

mod api;
mod settings;

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
            .service(api::links::create_link)
            .service(api::links::get_all_links)
            .service(api::links::get_from_link)
    })
        .bind(&ip)?
        .run()
        .await?;

    Ok(())
}

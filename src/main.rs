use actix_web::{App, HttpServer, web};
mod db;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::init_db().expect("Database initialization failed");
    println!("✅ Database ready");

    HttpServer::new(|| {
        App::new()
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

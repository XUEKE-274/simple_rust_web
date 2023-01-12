




mod service;

use actix_web::{App,HttpServer};
use simple_web_server::{get_person, add_person, get_person_list};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Web server start success >>> ");
    HttpServer::new(|| {
        App::new()
            .service(get_person)
            .service(add_person)
            .service(get_person_list)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


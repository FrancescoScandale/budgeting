use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use categories::Categories;

mod categories;
mod configdata;

async fn get_categories() -> impl Responder {
    // let items = state.items.lock().unwrap();
    let categories: Categories = Categories::init_categories();
    let items = categories.get_categories();
    HttpResponse::Ok().json(&*items)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/getcategories", web::get().to(get_categories))
    })
    .bind("127.0.0.1:5858")?
    .run()
    .await
}
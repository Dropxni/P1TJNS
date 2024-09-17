mod controllers; // Importacion de el módulo controllers

use actix_web::{web, App, HttpServer}; // Importacion de actix_web
use controllers::{register, update, delete}; // Importacion de los submódulos

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "127.0.0.1:8080";
    println!("Servidor corriendo en: http://{}", url);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(register::show_form)) // Muestra el formulario de registro
            .route("/register", web::post().to(register::register_user)) // Procesa el registro de usuario
            .route("/update_form", web::get().to(update::show_update_form)) // Muestra el formulario de actualización
            .route("/update_user", web::post().to(update::update_user)) // Procesa la actualización de usuario
            .route("/delete_form", web::get().to(delete::show_delete_form)) // Muestra el formulario de eliminación
            .route("/delete_user", web::post().to(delete::delete_user)) // Procesa la eliminación de usuario
    })
    .bind(url)?
    .run()
    .await
}

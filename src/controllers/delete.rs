use actix_web::{web, HttpResponse, Responder};
use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

// Estructura para capturar los datos del formulario de eliminación
#[derive(Deserialize)]
pub struct DeleteFormData {
    pub id: u32,
}

// Ruta para mostrar el formulario de eliminación
pub async fn show_delete_form() -> impl Responder {
    let html = include_str!("../templates/delete_user.html");
    HttpResponse::Ok().content_type("text/html").body(html)
}

// Ruta para procesar la eliminación de usuario
pub async fn delete_user(form: web::Form<DeleteFormData>) -> impl Responder {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está definida");

    let opts = Opts::from_url(&database_url).expect("URL de la base de datos no es válida");
    let pool = Pool::new(opts).expect("No se pudo crear el pool de conexiones");
    let mut conn = pool.get_conn().unwrap();

    // Elimina el usuario de la base de datos
    conn.exec_drop(
        r"DELETE FROM usuarios WHERE id=:id",
        params! {
            "id" => form.id,
        },
    ).unwrap();

    HttpResponse::Ok().body("Usuario eliminado con éxito.")
}

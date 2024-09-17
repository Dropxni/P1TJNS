use actix_web::{web, HttpResponse, Responder};
use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;
use log::error;
use actix_web::http::header;

// Estructura para capturar los datos del formulario de eliminación
#[derive(Deserialize)]
pub struct DeleteFormData {
    pub id: u32,
}

// Ruta para mostrar el formulario de eliminación
pub async fn show_delete_form() -> impl Responder {
    // Cargar la plantilla de eliminación
    let html = include_str!("../templates/delete_user.html");

    // Cargar el componente navbar
    let navbar = include_str!("../components/navbar.html");

    // Reemplazar {{navbar}} con el contenido real del navbar
    let html_with_navbar = html.replace("{{navbar}}", navbar);

    // Devolver la página con el navbar incluido
    HttpResponse::Ok().content_type("text/html").body(html_with_navbar)
}

// Ruta para procesar la eliminación de usuario
pub async fn delete_user(form: web::Form<DeleteFormData>) -> impl Responder {
    dotenv().ok(); // Cargar las variables de entorno

    // Obtener la URL de la base de datos del archivo .env
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            error!("DATABASE_URL no está definida");
            return HttpResponse::InternalServerError().body("Error al conectarse a la base de datos.");
        }
    };

    // Crear las opciones de conexión
    let opts = match Opts::from_url(&database_url) {
        Ok(opts) => opts,
        Err(_) => {
            error!("URL de la base de datos no es válida");
            return HttpResponse::InternalServerError().body("Error al conectarse a la base de datos.");
        }
    };

    // Crear el pool de conexiones
    let pool = match Pool::new(opts) {
        Ok(pool) => pool,
        Err(_) => {
            error!("No se pudo crear el pool de conexiones");
            return HttpResponse::InternalServerError().body("Error al conectarse a la base de datos.");
        }
    };

    // Obtener una conexión
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(_) => {
            error!("No se pudo obtener una conexión de la base de datos");
            return HttpResponse::InternalServerError().body("Error al conectarse a la base de datos.");
        }
    };

    // Ejecutar la consulta de eliminación
    match conn.exec_drop(
        r"DELETE FROM usuarios WHERE id=:id",
        params! {
            "id" => form.id,
        },
    ) {
        Ok(_) => {
            // Redirigir al formulario de eliminación con un parámetro de éxito
            HttpResponse::Found()
                .append_header((header::LOCATION, "/delete_form?success=true"))
                .finish()
        },
        Err(_) => {
            error!("No se pudo eliminar el usuario de la base de datos");
            HttpResponse::InternalServerError().body("Error al eliminar el usuario.")
        }
    }
}

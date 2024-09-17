use actix_web::{web, HttpResponse, Responder};
use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;
use bcrypt::{hash, DEFAULT_COST};
use std::env;
use dotenv::dotenv;
use log::error;
use actix_web::http::header;

// Estructura para capturar los datos del formulario de actualización
#[derive(Deserialize)]
pub struct UpdateFormData {
    pub id: u32,
    pub nombre: String,
    pub email: String,
    pub password: String,
    pub rol: String,
}

// Ruta para mostrar el formulario de actualización
pub async fn show_update_form() -> impl Responder {
    // Cargar la plantilla de actualización
    let html = include_str!("../templates/update_user.html");

    // Cargar el componente navbar
    let navbar = include_str!("../components/navbar.html");

    // Reemplazar {{navbar}} con el contenido real del navbar
    let html_with_navbar = html.replace("{{navbar}}", navbar);

    // Devolver la página con el navbar incluido
    HttpResponse::Ok().content_type("text/html").body(html_with_navbar)
}

// Ruta para procesar la actualización de usuario
pub async fn update_user(form: web::Form<UpdateFormData>) -> impl Responder {
    dotenv().ok(); // Cargar variables de entorno como la conexión a la base de datos

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

    // Obtener la conexión
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(_) => {
            error!("No se pudo obtener una conexión de la base de datos");
            return HttpResponse::InternalServerError().body("Error al conectarse a la base de datos.");
        }
    };

    // Encriptar la nueva contraseña con bcrypt
    let hashed_password = match hash(&form.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            error!("Error al encriptar la contraseña");
            return HttpResponse::InternalServerError().body("Error al procesar la solicitud.");
        }
    };

    // Ejecutar la consulta de actualización
    match conn.exec_drop(
        r"UPDATE usuarios SET nombre=:nombre, email=:email, password=:password, rol=:rol WHERE id=:id",
        params! {
            "id" => form.id,
            "nombre" => &form.nombre,
            "email" => &form.email,
            "password" => &hashed_password,
            "rol" => &form.rol,
        },
    ) {
        Ok(_) => {
            // Redirigir a la página principal con un parámetro de éxito
            HttpResponse::Found()
                .append_header((header::LOCATION, "/update_form?success=true"))
                .finish()
        },
        Err(_) => {
            error!("No se pudo actualizar el usuario en la base de datos");
            HttpResponse::InternalServerError().body("Error al actualizar el usuario.")
        }
    }
}

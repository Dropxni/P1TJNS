use actix_web::{web, HttpResponse, Responder};
use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;
use bcrypt::{hash, DEFAULT_COST};
use std::env;
use dotenv::dotenv;
use log::error;
use actix_web::http::header;

// Estructura para capturar los datos del formulario de registro
#[derive(Deserialize)]
pub struct FormData {
    pub nombre: String,
    pub email: String,
    pub password: String,
    pub rol: String,
}

// Ruta para mostrar el formulario de registro
pub async fn show_form() -> impl Responder {
    // Cargar la plantilla de registro
    let html = include_str!("../templates/register.html");
    
    // Cargar el componente navbar
    let navbar = include_str!("../components/navbar.html");

    // Reemplazar {{navbar}} con el contenido real del navbar
    let html_with_navbar = html.replace("{{navbar}}", navbar);

    // Devolver la página con el navbar incluido
    HttpResponse::Ok().content_type("text/html").body(html_with_navbar)
}

// Ruta para procesar el formulario de registro
pub async fn register_user(form: web::Form<FormData>) -> impl Responder {
    dotenv().ok();

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

    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(_) => {
            error!("No se pudo obtener una conexión de la base de datos");
            return HttpResponse::InternalServerError().body("Error al conectarse a la base de datos.");
        }
    };

    // Encriptar la contraseña con bcrypt
    let hashed_password = match hash(&form.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            error!("No se pudo encriptar la contraseña");
            return HttpResponse::InternalServerError().body("Error al procesar la solicitud.");
        }
    };

    // Ejecutar la consulta de inserción
    match conn.exec_drop(
        r"INSERT INTO usuarios (nombre, email, password, rol) VALUES (:nombre, :email, :password, :rol)",
        params! {
            "nombre" => &form.nombre,
            "email" => &form.email,
            "password" => &hashed_password,
            "rol" => &form.rol,
        },
    ) {
        Ok(_) => {
            // Redirigir a la página de registro con un parámetro de éxito
            HttpResponse::Found()
                .append_header((header::LOCATION, "/?success=true"))
                .finish()
        },
        Err(_) => {
            error!("No se pudo insertar el usuario en la base de datos");
            HttpResponse::InternalServerError().body("Error al registrar el usuario.")
        }
    }
}

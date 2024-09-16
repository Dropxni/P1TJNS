use actix_web::{web, HttpResponse, Responder};
use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;
use bcrypt::{hash, DEFAULT_COST};
use std::env;
use dotenv::dotenv;

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
    let html = include_str!("../templates/register.html");
    HttpResponse::Ok().content_type("text/html").body(html)
}

// Ruta para procesar el formulario de registro
pub async fn register_user(form: web::Form<FormData>) -> impl Responder {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está definida");

    let opts = Opts::from_url(&database_url).expect("URL de la base de datos no es válida");
    let pool = Pool::new(opts).expect("No se pudo crear el pool de conexiones");
    let mut conn = pool.get_conn().unwrap();

    // Encripta la contraseña
    let hashed_password = hash(&form.password, DEFAULT_COST).unwrap();

    // Inserta los datos en la base de datos
    conn.exec_drop(
        r"INSERT INTO usuarios (nombre, email, password, rol) VALUES (:nombre, :email, :password, :rol)",
        params! {
            "nombre" => &form.nombre,
            "email" => &form.email,
            "password" => &hashed_password,
            "rol" => &form.rol,
        },
    ).unwrap();

    HttpResponse::Ok().body("Usuario registrado con éxito.")
}

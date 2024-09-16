use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;
use bcrypt::{hash, DEFAULT_COST};

// Estructura para capturar los datos del formulario
#[derive(Deserialize)]
struct FormData {
    nombre: String,
    email: String,
    password: String,
    rol: String,
}

// Ruta para mostrar el formulario de registro
async fn show_form() -> impl Responder {
    // Carga el archivo HTML desde la carpeta templates dentro de src
    let html = include_str!("templates/register.html");
    HttpResponse::Ok().content_type("text/html").body(html)
}

// Ruta para procesar el formulario de registro
async fn register_user(form: web::Form<FormData>) -> impl Responder {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está definida");

    // Conexión a la base de datos
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
            "password" => &hashed_password,  // Almacena la contraseña encriptada
            "rol" => &form.rol,
        },
    ).unwrap();

    HttpResponse::Ok().body("Usuario registrado con éxito.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(show_form)) // Muestra el formulario
            .route("/register", web::post().to(register_user)) // Procesa el formulario
    })
    .bind("127.0.0.1:8080")? // Ejecuta en localhost:8080
    .run()
    .await
}

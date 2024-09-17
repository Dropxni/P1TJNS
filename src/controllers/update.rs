use actix_web::{web, HttpResponse, Responder};
use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;
use bcrypt::{hash, DEFAULT_COST};
use std::env;
use dotenv::dotenv;

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
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está definida");

    let opts = Opts::from_url(&database_url).expect("URL de la base de datos no es válida");
    let pool = Pool::new(opts).expect("No se pudo crear el pool de conexiones");
    let mut conn = pool.get_conn().unwrap();

    // Encripta la nueva contraseña
    let hashed_password = hash(&form.password, DEFAULT_COST).unwrap();

    // Actualiza los datos del usuario en la base de datos
    conn.exec_drop(
        r"UPDATE usuarios SET nombre=:nombre, email=:email, password=:password, rol=:rol WHERE id=:id",
        params! {
            "id" => form.id,
            "nombre" => &form.nombre,
            "email" => &form.email,
            "password" => &hashed_password,
            "rol" => &form.rol,
        },
    ).unwrap();

    HttpResponse::Ok().body("Usuario actualizado con éxito.")
}

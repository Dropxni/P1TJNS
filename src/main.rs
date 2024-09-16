use mysql::*;
use mysql::prelude::*;
use std::env;
use dotenv::dotenv;

fn main() {
    // Carga las variables de entorno desde el archivo `.env`
    dotenv().ok();

    // Carga la variable de entorno DATABASE_URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está definida");

    // Usa OptsBuilder para construir las opciones de conexión
    let opts = Opts::from_url(&database_url).expect("URL de la base de datos no es válida");
    let pool = Pool::new(opts).expect("No se pudo crear el pool de conexiones");

    // Obtiene una conexión del pool
    let mut conn = pool.get_conn().unwrap();

    // Ejecuta una consulta para crear la tabla si no existe
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS users (
            id INT PRIMARY KEY AUTO_INCREMENT,
            name VARCHAR(50),
            age INT
        )"
    ).unwrap();

    println!("Conexión a la base de datos establecida y tabla 'users' creada.");
}

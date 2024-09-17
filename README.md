**Tejones Dev**

Integrantes:

_José Manuel Ambrosio Agudo_
_Oscar Emmanuel Santos Lopez_

**Estructura del Proyecto**

P1TJNS/
├── Cargo.toml                # Archivo de configuración de Rust y dependencias
├── src/
│   ├── main.rs               # Archivo principal de la aplicación
│   ├── controllers/
│   │   ├── register.rs       # Lógica de registro de usuarios
│   │   ├── update.rs         # Lógica de actualización de usuarios
│   │   ├── delete.rs         # Lógica de eliminación de usuarios
│   ├── templates/
│   │   ├── register.html     # Formulario HTML para registrar usuarios
│   │   ├── update_user.html  # Formulario HTML para actualizar usuarios
│   │   ├── delete_user.html  # Formulario HTML para eliminar usuarios
│   └── components/
│       └── navbar.html       # Componente HTML para el navbar (barra de navegación)
├── .env                      # URL de la base de datos

**Base de Datos**

_Script de la base de datos_

CREATE DATABASE gestion_usuarios;
USE gestion_usuarios;
CREATE TABLE usuarios (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(100),
    email VARCHAR(100) UNIQUE,
    password VARCHAR(255),
    rol VARCHAR(50)
);

Tabla: usuarios
-----------------------------
| id        | INT, PK, AUTO_INCREMENT |
| nombre    | VARCHAR(100)             |
| email     | VARCHAR(100), UNIQUE     |
| password  | VARCHAR(255)             |
| rol       | VARCHAR(50)              |
-----------------------------

- id: Clave primaria (PK) que se incrementa automáticamente.
- nombre: Nombre del usuario (hasta 100 caracteres).
- email: Dirección de correo única para cada usuario (hasta 100 caracteres).
- password: Contraseña encriptada del usuario.
- rol: Rol del usuario (por ejemplo, admin o usuario).

**Nota**

_Para ejecutar el proyecto, favor de cambiar las credenciales del archivo .env_
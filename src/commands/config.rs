use crate::config::cli::get_configs;
use colored::*;
use std::fs::{self, OpenOptions};
use std::io::Read;
use std::process::exit;

pub fn ensure_config_exists() {
    let config_dir = dirs::home_dir().unwrap().join(".config/r-projects");
    let config_file = config_dir.join("config");

    // Verificar si el directorio existe, si no, crearlo
    if !config_dir.exists() {
        if let Err(e) = fs::create_dir_all(&config_dir) {
            eprintln!("{} {}", "Error al crear el directorio:".red(), e);
            exit(1);
        }
    }

    // Verificar si el archivo config existe, si no, crearlo
    if !config_file.exists() {
        if let Err(e) = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&config_file)
        {
            eprintln!("{} {}", "Error al crear el archivo config:".red(), e);
            exit(1);
        }
    }
}

pub fn config(key: &str, value: &str) {
    ensure_config_exists();

    // Verificar si la clave está permitida
    if !get_configs().contains(&key) {
        eprintln!("{}", "La clave no está permitida".red());
        exit(1);
    }

    let config_dir = dirs::home_dir().unwrap().join(".config/r-projects");
    let config_file = config_dir.join("config");

    // Leer el archivo config y cargar su contenido
    let mut content = String::new();
    if let Err(e) = fs::File::open(&config_file).and_then(|mut f| f.read_to_string(&mut content)) {
        eprintln!("{} {}", "Error al leer el archivo config:".red(), e);
        exit(1);
    }

    let mut found = false;
    let mut updated_content = String::new();

    // Procesar cada línea del archivo
    for line in content.lines() {
        if line.starts_with(key) {
            // Reemplazar la línea si la clave existe
            updated_content.push_str(&format!("{}={}\n", key, value));
            found = true;
        } else {
            // Si la clave no existe, agregar la línea sin modificaciones
            updated_content.push_str(&(line.to_string() + "\n"));
        }
    }

    // Si no se encontró la clave, agregarla
    if !found {
        updated_content.push_str(&format!("{}={}\n", key, value));
    }

    // Guardar los cambios de nuevo en el archivo config
    if let Err(e) = fs::write(&config_file, updated_content) {
        eprintln!("{} {}", "Error al escribir en el archivo config:".red(), e);
        exit(1);
    }
}

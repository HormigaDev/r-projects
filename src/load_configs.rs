use crate::commands::ensure_config_exists;
use colored::*;
use dirs;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

pub fn load_config() -> HashMap<String, String> {
    ensure_config_exists();
    let config_dir = dirs::config_dir().expect("No se pudo obtener el directorio de configuración");
    let config_path = config_dir.join("r-projects/config");

    let file = File::open(config_path).unwrap_or_else(|_| {
        eprintln!(
            "{}",
            "ERROR: No se pudo abrir el archivo de configuración".red()
        );
        exit(1);
    });

    let reader = BufReader::new(file);
    let mut config_map = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap_or_else(|_| {
            eprintln!("{}", "ERROR: No se pudo leer una línea del archivo".red());
            exit(1);
        });
        if let Some(eq_pos) = line.find('=') {
            let key = line[0..eq_pos].trim().to_string();
            let value = line[eq_pos + 1..].trim().to_string();
            config_map.insert(key, value);
        }
    }

    config_map
}

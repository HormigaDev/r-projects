use crate::config::cli::get_templates;
use colored::*;

pub fn list() {
    let templates: Vec<&str> = get_templates().keys().map(|&key| key).collect();

    println!(
        "{}",
        "Estas son todas las plantillas disponibles:".bright_blue()
    );
    for template in templates {
        println!("{}", template.yellow());
    }
}

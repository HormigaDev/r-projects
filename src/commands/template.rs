use crate::config::cli::get_templates;
use colored::*;
use std::fs::{self, File};
use std::io::{Read, Result, Write};
use std::path::Path;
use std::process::{exit, Command, ExitStatus};

pub fn template(id: &str, name: &str) {
    let templates = get_templates();

    if let Some(ssh) = templates.get(id) {
        let mut args = ["clone", ssh, name];
        if name.is_empty() {
            args = ["clone", ssh, id];
        }
        if run_command("git", &args).success() {
            println!(
                "{}",
                "La plantilla ha sido descargada correctamente".green()
            );

            #[allow(unused_assignments)]
            let mut project_dir = "";
            if name.is_empty() {
                project_dir = id;
            } else {
                project_dir = name;
            }

            let git_dir = format!("{}/.git", project_dir);
            if fs::remove_dir_all(&git_dir).is_ok() {
                println!("{}", "Eliminado el directorio .git".yellow());
                if run_command("git", &["init", project_dir]).success() {
                    println!("{}", "Inicializado el repositorio".green());
                    let cargo_toml_path = format!("{}/Cargo.toml", project_dir);
                    match replace_in_file(&cargo_toml_path, "cli-template", project_dir) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!(
                                "{} {}",
                                "Ocurrió un error al escribir en el archivo Cargo.toml:".red(),
                                e
                            );
                            exit(1);
                        }
                    };
                    let main_path = format!("{}/src/main.rs", project_dir);
                    match replace_in_file(&main_path, "app-name", project_dir) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!(
                                "{} {}",
                                "Ocurrió un error al escribir en el archivo main.rs:".red(),
                                e
                            );
                            exit(1);
                        }
                    };
                } else {
                    eprintln!(
                        "{}",
                        "Ocurrió un error al intentar iniciar el repositorio".red()
                    );
                    exit(1);
                }
            } else {
                eprintln!(
                    "{}",
                    "Error al intentar eliminar el directorio .git del proyecto".red()
                );
                exit(1);
            }
        }
    } else {
        eprintln!(
            "{}",
            format!(
                "La plantilla con el ID '{}' no existe o no está disponible",
                id
            )
            .red()
        );
        exit(1);
    }
}

fn run_command(command: &str, args: &[&str]) -> ExitStatus {
    match Command::new(command).args(args).status() {
        Ok(status) => status,
        Err(e) => {
            eprintln!(
                "{} {}",
                format!("Error al ejecutar el comando '{}':", command).red(),
                e
            );
            exit(1);
        }
    }
}

fn replace_in_file(file_path: &str, target: &str, replacement: &str) -> Result<()> {
    let path = Path::new(file_path);

    // Leer el contenido del archivo
    let mut content = String::new();
    File::open(&path)?.read_to_string(&mut content)?;

    // Reemplazar el texto
    let new_content = content.replace(target, replacement);

    // Escribir el contenido actualizado de vuelta al archivo
    let mut file = File::create(&path)?;
    file.write_all(new_content.as_bytes())?;

    Ok(())
}

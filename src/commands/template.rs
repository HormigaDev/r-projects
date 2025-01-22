use colored::*;
use std::fs;
use std::process::{exit, Command, ExitStatus};

pub fn template(user: &str, repository_name: &str, name: &str, remove_git: bool) {
    let repo_url = format!("git@github.com:{}/{}.git", user, repository_name);
    let folder_name = if name.is_empty() {
        repository_name
    } else {
        name
    };

    // Clonamos el repositorio
    let args = ["clone", &repo_url, folder_name];
    if run_command("git", &args).success() {
        println!(
            "{}",
            "La plantilla ha sido descargada correctamente".green()
        );

        // Si remove_git es false, eliminamos el directorio .git
        if !remove_git {
            let git_dir = format!("{}/.git", folder_name);

            // Eliminamos el directorio .git
            if fs::remove_dir_all(&git_dir).is_ok() {
                println!("{}", "Eliminado el directorio .git".yellow());

                // Inicializamos un nuevo repositorio
                if run_command("git", &["init", folder_name]).success() {
                    println!("{}", "Inicializado el repositorio".green());
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
            format!("Error al clonar el repositorio '{}'", repository_name).red()
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

use crate::config::cli::get_templates;
use colored::*;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Repository {
    name: String,
    html_url: String,
}

async fn fetch_repositories(user: &str) -> Result<Vec<Repository>, Error> {
    let url = format!("https://api.github.com/users/{}/repos", user);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "template-list-cli")
        .send()
        .await?
        .json::<Vec<Repository>>()
        .await?;
    Ok(response)
}

pub async fn list(user: &str) {
    if !user.is_empty() {
        println!("{}", "Buscando repositorios...".yellow());
        match fetch_repositories(user).await {
            Ok(repos) => {
                println!(
                    "{}",
                    format!("Repositorios públicos del usuario '{}':", user).bright_blue()
                );
                for repo in repos {
                    println!("{} - {}", repo.name.yellow(), repo.html_url.bright_green());
                }
            }
            Err(err) => eprintln!(
                "{}",
                format!("Error al obtener repositorios: {}", err).red()
            ),
        }
        return;
    }

    // Comportamiento por defecto
    let templates: Vec<&str> = get_templates().keys().map(|&key| key).collect();

    println!(
        "{}",
        "Estas son todas las plantillas disponibles del usuario HormigaDev:".bright_blue()
    );
    for template in templates {
        println!("{}", template.yellow());
    }
}

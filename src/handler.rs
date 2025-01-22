use crate::commands;
use crate::load_configs::load_config;
use clap::ArgMatches;
use colored::*;
use std::process::exit;

pub async fn execute(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("template", sub_matches)) => {
            let configs = load_config();
            let default_user = match configs.get("default.user").cloned() {
                Some(user) => user,
                None => String::new(),
            };
            let default_keep = match configs.get("default.keep").cloned() {
                Some(keep) => keep.trim() == "true",
                None => false,
            };

            let repo = sub_matches
                .value_of("repo")
                .unwrap_or_else(|| {
                    eprintln!("{}", "Por favor informe el nombre del repositorio".red());
                    exit(1);
                })
                .trim();
            let user = sub_matches.value_of("user").unwrap_or_else(|| {
                if !default_user.is_empty() {
                    return &default_user;
                } else {
                    eprintln!("{}", "Por favor informe el nombre del repositorio".red());
                    exit(1);
                }
            });
            let name = sub_matches.value_of("name").unwrap_or_else(|| "").trim();
            let keep = if sub_matches.is_present("nokeep") {
                false
            } else if sub_matches.is_present("keep") {
                true
            } else {
                default_keep
            };
            commands::template(user, repo, name, keep);
        }
        Some(("list", sub_matches)) => {
            let configs = load_config();
            let default_user = match configs.get("default.user").cloned() {
                Some(user) => user,
                None => String::new(),
            };
            let user = sub_matches
                .value_of("user")
                .unwrap_or_else(|| &default_user)
                .trim();
            commands::list(user).await;
        }
        Some(("config", sub_matches)) => {
            let key = sub_matches
                .value_of("key")
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        "Por favor informe el identificador (clave) de la configuración".red()
                    );
                    exit(1);
                })
                .trim();
            let value = sub_matches
                .value_of("value")
                .unwrap_or_else(|| {
                    eprintln!("{}", "Por favor informe el valor de la configuración".red());
                    exit(1);
                })
                .trim();

            commands::config(key, value);
        }
        _ => {
            eprintln!("{}", "Unknown or unsupported command".red());
            exit(1);
        }
    }
}

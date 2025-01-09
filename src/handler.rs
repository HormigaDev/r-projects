use crate::commands;
use clap::ArgMatches;
use colored::*;
use std::process::exit;

pub fn execute(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("template", sub_matches)) => {
            let id = sub_matches
                .value_of("identifier")
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        "Por favor informe el identificador de la plantilla".red()
                    );
                    exit(1);
                })
                .trim();
            let name = sub_matches.value_of("name").unwrap_or_else(|| "").trim();
            commands::template(id, name);
        }
        Some(("list", _)) => {
            commands::list();
        }
        _ => {
            eprintln!("{}", "Unknown or unsupported command".red());
            exit(1);
        }
    }
}

use clap::{App, Arg, Command};
use dotenv::dotenv;

mod commands;
mod config;
mod handler;
mod load_configs;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut matches = App::new("r-projects")
        .version("0.1.1")
        .author("HormigaDev <hormigadev7@gmail.com>")
        .about("Un CLI para clonar rapidamente las plantillas de proyectos personales mas comunes");

    for option in config::cli::get_options() {
        let (subcommand, about, args) = option;
        let mut subcmd = Command::new(subcommand);
        subcmd = subcmd.about(about);
        if args.len() > 0 {
            for arg in args {
                let (name, s, required, takes) = arg;
                subcmd = subcmd.arg(
                    Arg::new(name)
                        .short(s)
                        .long(name)
                        .takes_value(takes)
                        .required(required),
                );
            }
        }
        matches = matches.subcommand(subcmd);
    }

    let matches = matches.get_matches();
    handler::execute(matches).await;
}

use std::collections::HashMap;

pub mod cli {
    use crate::config::HashMap;

    pub fn get_options() -> Vec<(
        &'static str,
        &'static str,
        Vec<(&'static str, char, bool, bool)>,
    )> {
        const REQUIRED: bool = true;
        const OPTIONAL: bool = false;
        const TAKES: bool = true;
        const NOTAKES: bool = false;

        vec![
            (
                "template",
                "Este comando clona una de todas las plantillas disponibles con el nombre identificador especificado",
                vec![
                    ("repo", 'r', REQUIRED, TAKES),
                    ("user", 'u', OPTIONAL, TAKES),
                    ("name", 'n', OPTIONAL, TAKES),
                    ("keep", 'k', OPTIONAL, NOTAKES),
                    ("nokeep", 'K', OPTIONAL, NOTAKES),
                ]
            ),
            (
                "list",
                "Este comando lista todas las plantillas disponibles para clonar",
                vec![
                    ("user", 'u', OPTIONAL, TAKES),
                ]
            ),
            (
                "config",
                "Este comando permite editar las configuraciones del CLI",
                vec![
                    ("key", 'k', REQUIRED, TAKES),
                    ("value", 'v', REQUIRED, TAKES),
                    ("list", 'l', OPTIONAL, NOTAKES),
                ]
            )
        ]
    }

    pub fn get_templates() -> HashMap<&'static str, (&'static str, &'static str)> {
        HashMap::from([
            (
                "cli-template",
                ("git@github.com:HormigaDev/cli-template.git", "rust"),
            ),
            (
                "bot-template-ts",
                ("git@github.com:HormigaDev/bot-template.git", "typescript"),
            ),
            (
                "nest-api-template",
                (
                    "git@github.com:HormigaDev/nest-api-template.git",
                    "typescript",
                ),
            ),
        ])
    }

    pub fn get_configs() -> [&'static str; 3] {
        ["default.user", "default.keep", "auth.token"]
    }
}

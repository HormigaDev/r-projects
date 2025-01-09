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
        // const NOTAKES: bool = false;

        vec![
            (
                "template",
                "Este comando clona una de todas las plantillas disponibles con el nombre identificador especificado",
                vec![
                    ("identifier", 'i', REQUIRED, TAKES),
                    ("name", 'n', OPTIONAL, TAKES),
                ]
            ),
            (
                "list",
                "Este comando lista todas las plantillas disponibles para clonar",
                vec![]
            )
        ]
    }

    pub fn get_templates() -> HashMap<&'static str, &'static str> {
        HashMap::from([
            ("cli-template", "git@github.com:HormigaDev/cli-template.git"),
            (
                "bot-template-ts",
                "git@github.com:HormigaDev/bot-template.git",
            ),
        ])
    }
}

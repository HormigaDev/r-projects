# Rust CLI Template

Es una plantilla simple de un CLI hecho en Rust

---

El archivo `src/config.rs` contiene la plantilla de como se estructuran los comandos en el CLI: un nombre, una descripción, y una lista de argumentos

---

El archivo `src/handler.rs` contiene la plantilla de donde se manejarán los comandos y se controla cuales comandos se capturan.

---

El archivo `src/commands/mod.rs` es el archivo donde se importan todos los comandos y se exportan para usarlos en el handler;

---

En la carpeta `src/commands/` puedes colocar tus comandos e importarlos al `mod.rs` para que puedan ser usados en otros archivos

# @nodalia/egui

Genera el crate Rust **`nodalia-theme`** ([`crate/`](crate)) desde los tokens de NDL: constantes de color por modo, densidades, métricas y movimiento, y, con la *feature* `egui`, un `egui::Style` construido con ellas.

**Compilado y probado contra egui 0.37; sin verificar visualmente en una app real.** Ver [docs/integration/egui.md](../../docs/integration/egui.md). Versión `0.1.0-alpha.1`.

`crate/src/tokens.rs` está generado y **sí se guarda en el repositorio**, para que una app Rust pueda depender del crate por git sin necesitar Node. No se edita a mano: se regenera con `npm run build -w @nodalia/egui`, y un test falla si deja de coincidir con los tokens. Lo mismo vale para `crate/src/icons/data.rs`, los iconos de [`@nodalia/icons`](../icons): `nodalia_theme::icons::get("trim")` da sus piezas SVG y, con la *feature* `egui`, `icons::paint` los dibuja en un `egui::Painter` (estilo normal o grueso, y con el acento en las piezas que lo piden).

Para una cabecera de ventana dibujada por la aplicación, `nodalia_theme::window_controls::show` pinta los tres controles NDL y devuelve la acción elegida. El host reserva el espacio, da las etiquetas accesibles, informa del foco y la maximización, y decide cómo aplicar cada acción.

Las apps de oficina (Letra, Celdas, Lámina) comparten la vista Archivo con `nodalia_theme::office`: `file_view` dibuja la isla de navegación (Atrás, las entradas de la app y un grupo abajo, Cuenta y Preferencias) y la isla de la página; `page`, `section`, `settings`/`setting`, `templates` y `recents` dibujan Inicio y Preferencias. El módulo solo dibuja: devuelve la entrada, la plantilla o el archivo reciente elegido y cada app ejecuta sus propios comandos.

Las apps de ingeniería (Torno, Vortia, Forja) comparten `nodalia_theme::engineering` (feature `engineering`): IBM Plex Sans y Mono incrustadas (`install_fonts` o `add_fonts`), la escala tipográfica de la suite, las medidas de la ventana y las piezas de panel (`island`, `panel_header`, `properties`/`property`, `value`, `tree_row`, `ribbon_tab`, `ribbon_group`, `ribbon_tool`). Especificación: [docs/suites/engineering.md](../../docs/suites/engineering.md).

Las apps creativas (Lienzo, Pluma y, más adelante, Toma) comparten `nodalia_theme::creator` (feature `creator`): el selector de espacio de trabajo (`workspace_selector`, con espacios, guardados y acciones), los botones de diálogo (`primary_button`, `secondary_button`), el botón de cerrar, la navegación de Preferencias (`nav_row`), el carril de Inicio (`home_row`), las pestañas (`tab`), los campos (`field_frame`, `search_field`) y las esquinas de isla (`island`, `round_island_corners`). Especificación: [docs/suites/creator.md](../../docs/suites/creator.md).

# @nodalia/egui

Genera el crate Rust **`nodalia-theme`** ([`crate/`](crate)) desde los tokens de NDL: constantes de color por modo, densidades, métricas y movimiento, y, con la *feature* `egui`, un `egui::Style` construido con ellas.

**Compilado y probado contra egui 0.37; sin verificar visualmente en una app real.** Ver [docs/integration/egui.md](../../docs/integration/egui.md). Versión `0.1.0-alpha.1`.

`crate/src/tokens.rs` está generado y **sí se guarda en el repositorio**, para que una app Rust pueda depender del crate por git sin necesitar Node. No se edita a mano: se regenera con `npm run build -w @nodalia/egui`, y un test falla si deja de coincidir con los tokens. Lo mismo vale para `crate/src/icons/data.rs`, los iconos de [`@nodalia/icons`](../icons): `nodalia_theme::icons::get("trim")` da sus piezas SVG y, con la *feature* `egui`, `icons::paint` los dibuja en un `egui::Painter` (estilo normal o grueso, y con el acento en las piezas que lo piden).

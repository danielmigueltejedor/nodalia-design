# @nodalia/gtk

NDL para GTK 4, sin libadwaita.

- Hojas de color por modo `nodalia-{light,dark,light-hc,dark-hc}.css` y de métricas por densidad `nodalia-metrics-{compact,regular,comfortable}.css`, generadas desde los tokens.
- `nodalia-gtk.css`, la hoja de widgets (`src/`), que solo lee variables.
- `crate/`: el crate de Rust `nodalia-gtk`, con las hojas incrustadas y los widgets adaptativos.

`npm run build` escribe las hojas en `dist/` y en `crate/src/css/` (esta copia se versiona para que el crate se use desde git sin Node). Ver [docs/integration/gtk.md](../../docs/integration/gtk.md). Versión `0.1.0-alpha.1`.

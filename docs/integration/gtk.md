# Integración GTK4 / libadwaita

> **Sin verificar en un runtime GTK real.** Las hojas se generan y se comprueban estructuralmente, pero aún no se han cargado en una app. Trátalo como un punto de partida y comunica cualquier problema.

## Qué hace

Define los **colores con nombre públicos** de libadwaita (`accent_bg_color`, `window_bg_color`, `headerbar_bg_color`, `sidebar_bg_color`, `destructive_bg_color`, etc.) a partir de los tokens. libadwaita los usa para colorear sus widgets. NDL no toca selectores de widgets ni nodos internos, que son API privada.

Forma, espaciado y tipografía los sigue gobernando libadwaita. NDL aporta color e identidad.

## Ficheros

`nodalia-light.css`, `nodalia-dark.css`, `nodalia-light-hc.css`, `nodalia-dark-hc.css`. Los modos de alto contraste cumplen 7:1 en texto.

## Uso (Rust, gtk4-rs)

```rust
use adw::prelude::*;

let provider = gtk::CssProvider::new();
provider.load_from_string(include_str!("nodalia-dark.css")); // GTK 4.12+: load_from_string
gtk::style_context_add_provider_for_display(
    &gdk::Display::default().expect("display"),
    &provider,
    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
);
```

Elige el fichero según `adw::StyleManager::default()` (`is_dark()` y `is_high_contrast()`) y vuelve a cargarlo cuando cambie (`connect_dark_notify`, `connect_high_contrast_notify`).

Para incluirlo, instala `@nodalia/gtk` y copia los ficheros a tu recurso en el **build** (no a mano), por ejemplo con un script de `build.rs` o `gresource` que lea `node_modules/@nodalia/gtk/dist/`.

## Requisitos

libadwaita ≥ 1.6 (variables CSS de color con nombre). Las apps que dependan de una versión anterior pueden seguir usando `@define-color`, que las hojas también emiten, pero conviene comprobarlo.

## Limitaciones

- No verificado en ejecución.
- No incluye iconos; usa los del tema del sistema. Ver [iconografía](../iconography.md).
- No cambia radios ni tipografía de libadwaita.
- Las densidades no se aplican en GTK (libadwaita no las expone).

# Actualizaciones dentro de la app

Todas las apps Nodalia se actualizan desde dentro con el mismo flujo, el que estrenó Vortia. La lógica vive en `packages/update`, el crate `nodalia-update`: no dibuja nada y no depende de ningún toolkit, así que lo usan por igual las apps GTK y las de egui. Cada app pone el texto y la ventana.

## El flujo

1. **Comprobar.** `check(&App)` pide `releases/latest` a GitHub y compara el tag con la versión que corre. Devuelve `UpToDate` o `Available(Release)` con las notas, la página de la release y el paquete de esta máquina.
2. **Descargar.** `download(&App, &Release, progreso)` baja el paquete a `~/.cache/<app>/update` (en macOS, `~/Library/Caches/<app>/update`) y calcula su SHA-256 mientras escribe. Solo lo conserva si coincide con el `digest` que GitHub publica para ese archivo; una release sin digest no se instala.
3. **Aplicar.** `apply(&App, paquete, antes_de_reiniciar)` sustituye la instalación y la arranca de nuevo. Si `antes_de_reiniciar` devuelve un motivo (trabajo sin guardar), no se toca nada. Con `Ok`, la app debe salir de inmediato con `std::process::exit(0)`.

## Qué se puede actualizar en el sitio

| Instalación | Qué hace |
|---|---|
| AppImage (`$APPIMAGE`) | Aparta el AppImage actual, copia el nuevo en su lugar, lo arranca y borra la copia; si algo falla, lo devuelve |
| macOS (`Nombre.app`) | Comprueba que el `.app` del DMG esté firmado por el mismo editor que el instalado y delega el cambio en un script que espera a que la app salga |
| Flatpak, `.deb`, `.rpm`, AUR, compilación de desarrollo | No se toca: `install_kind()` devuelve `Managed` y la app abre la página de la release |

Los DMG sin certificado de Apple están firmados ad hoc: se pueden actualizar entre sí si el identificador del bundle coincide. Con Developer ID, el equipo debe coincidir.

Tras un fallo en macOS, el script deja un código que `take_last_failure()` lee al siguiente arranque; la app lo traduce a un mensaje y nunca falla en silencio. El registro del script está en `update_dir(app)/update.log`.

## Nombres de los paquetes

El crate elige el paquete por el nombre del asset, sin distinguir mayúsculas: `macos-arm64.dmg` en Apple Silicon (o `macos-universal.dmg`) y `x86_64.AppImage` en Linux. Es el mismo esquema que Vortia: `Nombre-X.Y.Z-x86_64.AppImage`, `Nombre-X.Y.Z-macos-arm64.dmg`. Los `.sha256` que acompañan a cada asset se ignoran.

## Lo que pone la app

- Un elemento «Buscar actualizaciones» en el menú principal y, si quiere, una comprobación al arrancar (una vez al día basta).
- Una ventana con la versión nueva, las notas y un botón de instalar con barra de progreso, o «Abrir página de la release» cuando `install_kind()` es `Managed`.
- Todo el trabajo en un hilo aparte: las llamadas bloquean.

Uso desde Rust:

```toml
nodalia-update = { git = "https://github.com/danielmigueltejedor/nodalia-design", rev = "…" }
```

```rust
const APP: nodalia_update::App = nodalia_update::App {
    name: "Vinilo",
    repo: "danielmigueltejedor/vinilo",
    bundle: "Vinilo.app",
    version: env!("CARGO_PKG_VERSION"),
    edition: None,
};
```

## Varias ediciones en la misma release

Si una release lleva dos compilaciones de la misma app para la misma plataforma (Vinilo en macOS: `Native` y `Nodalia`), cada edición declara `edition: Some("native")` o `Some("nodalia")` y en macOS solo se ofrece el disco llamado `<edición>-macos-arm64.dmg`. Así una edición nunca propone instalar la otra. El `bundle` es el de su edición (`Vinilo Native.app`, `Vinilo Nodalia.app`) y el identificador de la app también difiere, de modo que la comprobación de firma sigue exigiendo el mismo identificador que el instalado.

## Apps egui: la ventana ya hecha

Las apps egui no tienen que dibujar nada: con la característica `egui`, `nodalia_update::egui::Updater` lleva el flujo entero y sus ventanas modales, en español o en inglés.

```toml
nodalia-update = { git = "https://github.com/danielmigueltejedor/nodalia-design", rev = "…", features = ["egui"] }
```

```rust
let mut updater = nodalia_update::egui::Updater::new(APP).with_accent([0xe8, 0x57, 0x3f]);

// Una vez arrancada la app (cada fotograma vale: solo actúa la primera vez).
updater.check_at_start(ctx);
// Desde «Buscar actualizaciones…».
updater.check_now(ctx);
// Cada fotograma.
match updater.show(ctx, nodalia_update::Language::from_code(idioma), seguro_cerrar) {
    nodalia_update::egui::Action::OpenUrl(url) => abrir(url),
    nodalia_update::egui::Action::Exit => std::process::exit(0),
    nodalia_update::egui::Action::None => {}
}
```

- La comprobación al arrancar espera 20 segundos, se hace como mucho una vez al día y solo donde la app puede sustituirse; si no encuentra nada, o falla, no dice nada.
- `seguro_cerrar` es si la app puede salir sin perder nada (trabajo sin guardar, diálogos, tareas). Mientras sea `false`, la versión descargada espera y la ventana explica por qué.
- `is_open()` dice si hay una ventana abierta: son modales, así que las teclas no deben llegar al documento de detrás.
- Las notas de la release se muestran en el idioma de la app si están escritas con los encabezados `**Español**` e `**English**` (`notes_in`).

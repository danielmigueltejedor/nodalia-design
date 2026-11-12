# Actualizaciones dentro de la app

Todas las apps Nodalia se actualizan desde dentro con el mismo flujo, el que estrenó Vortia. La lógica vive en `packages/update`, el crate `nodalia-update`: no dibuja nada y no depende de ningún toolkit, así que lo usan por igual las apps GTK y las de egui. Cada app pone el texto y la ventana.

## El flujo

1. **Comprobar.** `check(&App)` pide `releases/latest` a GitHub y compara el tag con la versión que corre. Devuelve `UpToDate` o `Available(Release)` con las notas, la página de la release y el paquete de esta máquina.
2. **Descargar.** `download(&App, &Release, progreso)` baja el paquete a `~/.cache/<app>/update` (en macOS, `~/Library/Caches/<app>/update`) y calcula su SHA-256 mientras escribe. Solo lo conserva si coincide con el `digest` que GitHub publica para ese archivo (y deja ese digest junto al paquete, en `<paquete>.verified-sha256`); una release sin digest no se instala. En macOS el ayudante vuelve a comprobarlo: copia el disco a una carpeta privada, calcula su SHA-256 y solo entonces monta la copia, de modo que un disco cambiado después de la descarga no se instala.
3. **Aplicar.** `apply(&App, paquete, antes_de_reiniciar)` sustituye la instalación y la arranca de nuevo. Si `antes_de_reiniciar` devuelve un motivo (trabajo sin guardar), no se toca nada. Con `Ok`, la app debe salir de inmediato con `std::process::exit(0)`.

## Qué se puede actualizar en el sitio

| Instalación | Qué hace |
|---|---|
| AppImage (`$APPIMAGE`) | Aparta el AppImage actual, copia el nuevo en su lugar, lo arranca y, si aguanta los primeros segundos (o termina bien), borra la copia; si algo falla o se cierra con error, devuelve el anterior |
| macOS (`Nombre.app`) | Comprueba que el `.app` del DMG esté firmado por el mismo editor que el instalado y delega el cambio en un script que espera a que la app salga |
| Flatpak, `.deb`, `.rpm`, AUR, compilación de desarrollo | No se toca: `install_kind()` devuelve `Managed` y la app abre la página de la release |

Los DMG sin certificado de Apple están firmados ad hoc: se pueden actualizar entre sí si el identificador del bundle coincide. Con Developer ID, el equipo debe coincidir.

Tras un fallo en macOS, el script deja un código que `take_last_failure()` lee al siguiente arranque; la app lo traduce a un mensaje y nunca falla en silencio. El registro del script está en `update_dir(app)/update.log`.

## Nombres de los paquetes

El crate elige el paquete por el final del nombre del asset, sin distinguir mayúsculas: `macos-arm64.dmg` en Apple Silicon (o `macos-universal.dmg`) y `x86_64.AppImage` en Linux x86-64 (`aarch64.AppImage` en ARM; nunca el de otra arquitectura). Un `.zsync` o `.sha256` junto al paquete no es el paquete. Es el mismo esquema que Vortia: `Nombre-X.Y.Z-x86_64.AppImage`, `Nombre-X.Y.Z-macos-arm64.dmg`. Los `.sha256` que acompañan a cada asset se ignoran.

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

Si una release lleva dos compilaciones de la misma app para la misma plataforma (Vinilo en macOS: `Native` y `Nodalia`), cada edición declara `edition: Some("native")` o `Some("nodalia")` y en macOS solo se ofrece el disco llamado `<versión>-<edición>-macos-arm64.dmg` (la versión completa antes de la edición evita que `native` coincida con `non-native`). Así una edición nunca propone instalar la otra. El `bundle` es el de su edición (`Vinilo Native.app`, `Vinilo Nodalia.app`) y el identificador de la app también difiere, de modo que la comprobación de firma sigue exigiendo el mismo identificador que el instalado.

## Versiones, firma y recuperación

- Una pre-release (`2.0.0-beta`) es anterior a su versión estable (`2.0.0`), como en semver.
- En macOS el ayudante vuelve a comprobar la firma y el publicador de **la copia que va a instalar** (no solo la del disco montado antes), y no toca la app instalada si no coinciden. Si no puede devolver la versión anterior, la deja junto a la app y dice dónde en `update.log`. Tras abrir la nueva espera unos 5 s y solo borra la anterior si el proceso sigue vivo; si ha caído al arrancar, restaura la anterior y la vuelve a abrir (resultado `launch`; para la app llega como `Failure::Other`, porque la API no distingue un arranque fallido de otros fallos).
- Solo se lanza un ayudante de macOS por proceso.
- Las variables `NODALIA_UPDATE_*` que dirigen el ayudante en las pruebas (`OPEN`, `WAIT_STEPS`, `GRACE_STEPS`, `SKIP_SIGNATURE`) solo las respetan las compilaciones de depuración; una compilación publicada las ignora, de modo que nadie que controle el entorno de la app puede desactivar la comprobación de firma.
- Una app firmada ad hoc no tiene publicador: la coincidencia de identificador no lo demuestra. En ese caso lo que protege es el `sha256` que GitHub publica para el disco, comprobado antes de instalar; con un certificado de desarrollador, además, el publicador debe coincidir.
- Una instalación gestionada (Flatpak, `.deb`…) recibe `Check::Available` aunque la release no tenga paquete para ella, para poder abrir la página de la release.

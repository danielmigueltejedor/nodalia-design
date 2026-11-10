# Integración GTK 4

NDL en GTK es GTK 4 puro, sin libadwaita. Se reparte en dos piezas:

- **`@nodalia/gtk`**: las hojas de estilo.
- **`nodalia-gtk`** (crate de Rust en `packages/gtk/crate/`): instala las hojas, sigue las preferencias del sistema y aporta los widgets adaptativos que GTK no trae.

> **Estado.** Compila y sus tests pasan; CI comprueba que GTK analiza cada hoja. El aspecto y el comportamiento se están revisando en Vinilo 2.0, la primera app que lo usa. Comunica cualquier problema.

## Hojas

| Fichero | Contenido | Cuándo cambia |
| --- | --- | --- |
| `nodalia-light.css`, `nodalia-dark.css`, `nodalia-light-hc.css`, `nodalia-dark-hc.css` | `--ndl-color-*`, `--ndl-elevation-*` y los colores con nombre de libadwaita (`window_bg_color`, `accent_bg_color`…) | Con el modo |
| `nodalia-metrics-compact.css`, `nodalia-metrics-regular.css`, `nodalia-metrics-comfortable.css` | `--ndl-space-*`, `--ndl-radius-*`, `--ndl-font-*`, `--ndl-duration-*`, `--ndl-easing-*`, `--ndl-size-*`, `--ndl-component-*`, `--ndl-density-*` | Con la densidad |
| `nodalia-gtk.css` | Los widgets: nodos CSS documentados de GTK 4, clases de estilo públicas y los nodos `ndl-*` del crate | Nunca: solo lee variables |

Las dos primeras se generan desde los tokens; la tercera está escrita a mano en `packages/gtk/src/` y los tests comprueban que no tiene literales de color y que cada variable que usa existe. Necesitan GTK 4.16+ por `var()`. Las dimensiones van en `px`, porque GTK no tiene un tamaño de letra raíz con el que escalar `rem`.

Los colores con nombre de libadwaita se siguen emitiendo para que la hoja propia de una app pueda usar `@window_bg_color` y compañía, y para las apps que aún dependen de libadwaita.

## El crate `nodalia-gtk`

Requiere GTK ≥ 4.20 (lee `gtk-interface-color-scheme` y `gtk-interface-contrast`). Lleva las hojas incrustadas, así que se usa directamente desde git, sin Node:

```toml
[dependencies]
nodalia-gtk = { git = "https://github.com/danielmigueltejedor/nodalia-design", rev = "<commit>" }
```

Fija siempre un `rev` o una etiqueta: el crate aún es alfa.

```rust
use nodalia_gtk as ndl;
use ndl::prelude::*;

app.connect_startup(|_| ndl::init());          // instala las hojas en la pantalla
ndl::StyleManager::default().set_density(ndl::Density::Compact);
```

`StyleManager` sigue al sistema en claro, oscuro y alto contraste, y deja forzar uno (`set_color_scheme`), cambiar la densidad y sustituir el color de acento (`set_accent`).

### Widgets

Los nombres y los métodos siguen a los de libadwaita cuando el trabajo es el mismo, para que portar una app sea sobre todo cambiar el prefijo.

| Grupo | Widgets |
| --- | --- |
| Ventanas | `ApplicationWindow`, `Window`, `BreakpointBin`, `Breakpoint`, `BreakpointCondition` |
| Barras | `ToolbarView`, `HeaderBar`, `WindowTitle` |
| Navegación y disposición | `NavigationView`, `NavigationPage`, `OverlaySplitView`, `BottomSheet`, `Clamp`, `ClampScrollable` |
| Diálogos | `Dialog`, `AlertDialog`, `PreferencesDialog`, `AboutDialog`, `ShortcutsDialog` |
| Filas y preferencias | `ActionRow`, `SwitchRow`, `ComboRow`, `SpinRow`, `EntryRow`, `PasswordEntryRow`, `ButtonRow`, `PreferencesGroup`, `PreferencesPage` |
| Otros | `StatusPage`, `Spinner`, `Avatar`, `Toast`, `ToastOverlay`, `TimedAnimation` |

Diferencias con libadwaita que conviene conocer:

- Los diálogos se presentan dentro de la ventana, sobre un velo, cuando la ventana es de este crate; en otra ventana abren una propia.
- `PreferencesDialog`, `AboutDialog`, `ShortcutsDialog`, `AlertDialog` y `Toast` son tipos de Rust, no GObject: no se meten en un `view!` de relm4, se crean en código.
- `ToastOverlay` no implementa los rasgos de contenedor de relm4; en un `view!` se usa `set_child`.
- Los iconos son los del tema del sistema. Ver [iconografía](../iconography.md).

## Comprobación en un GTK real

```sh
python3 packages/gtk/tools/check-gtk.py
```

Carga cada hoja en GTK 4, prueba cada variable en la propiedad donde se usaría y estiliza widgets reales con todo, porque GTK solo valida un `var()` al calcular el estilo. Falla con cualquier error o aviso. Necesita PyGObject con GTK 4 y una pantalla (`xvfb-run` en CI).

## Uso sin el crate

Una app GTK que no sea de Rust carga las tres hojas (modo, métricas y `nodalia-gtk.css`) con prioridad de aplicación, y cambia la de modo cuando cambien las preferencias del sistema. Los nodos `ndl-*` no existirán, y sus reglas no hacen nada.

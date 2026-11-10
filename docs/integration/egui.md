# Integración Rust / egui

> **Compilado y probado contra egui 0.37; sin verificar visualmente en una app real.** Los tests comprueban que el estilo se construye en los cuatro modos y las tres densidades, que cada valor sale de los tokens y que los pares de color que egui dibuja cumplen contraste. Nadie ha mirado todavía una app completa con este estilo.

El adaptador es un crate Rust, `nodalia-theme`, en [`packages/egui/crate`](../../packages/egui/crate). Su fichero de tokens se genera desde `@nodalia/tokens`.

## Dependencia

Mientras no se publique en un registro, se toma por git **fijando el commit**:

```toml
[dependencies]
nodalia-theme = { git = "https://github.com/danielmigueltejedor/nodalia-design", rev = "<commit>", features = ["egui"] }
```

El crate pide `egui = "0.37"` sin *features*. Una app que sustituye egui por un fork con `[patch.crates-io]` (como hacen las apps basadas en los motores craft) sigue usando su fork: el parche se aplica también a esta dependencia.

Sin la *feature* `egui` el crate no tiene dependencias y sirve para cualquier otro toolkit Rust.

## Uso

```rust
use nodalia_theme::for_egui::{self, Appearance};
use nodalia_theme::{Density, Mode};

// Al arrancar: seguir la preferencia claro/oscuro del sistema.
for_egui::apply_system(&ctx, &Appearance::default());

// O fijar un modo, una densidad y movimiento reducido.
for_egui::apply(&ctx, &Appearance { mode: Mode::DarkHc, density: Density::COMPACT, reduced_motion: true });
```

Para widgets propios, los tokens están como constantes:

```rust
use nodalia_theme::{for_egui::color, radius, space, Mode};

let palette = Mode::Dark.palette();
let fill = color(palette.status_success_bg);
let padding = space::MD;
```

## Acento de la app

```rust
use nodalia_theme::Accent;

for_egui::apply_system(&ctx, &Appearance { accent: Accent::Vortia, ..Appearance::default() });
let palette = Mode::Dark.palette_with(Accent::Vortia);   // para widgets propios
```

Solo cambian los colores de acento; el resto de la paleta es el mismo en todas las apps.

## Qué se aplica a egui

| egui | Token |
| --- | --- |
| `panel_fill`, `window_fill` | `color.surface.panel`, `color.surface.overlay` |
| Controles en reposo, encima y pulsados | `color.control.bg`, `bg-hover`, `bg-pressed`, con bordes `color.border.*` |
| Texto y texto débil | `color.text.primary`, `color.text.secondary` |
| Selección y contorno de foco | `color.selection.bg` y `color.accent.text` a 2 px |
| Campos de texto | `color.control.field-bg` |
| Enlaces, avisos y errores | `color.text.link`, `color.status.warning.fg`, `color.status.danger.fg` |
| Radios de control, menú y ventana | `component.button.radius`, `radius.md`, `radius.lg` |
| Sombras de ventana y de menú | `elevation.overlay`, `elevation.raised` |
| Altura de control, relleno y separación | `density.*` |
| Tamaños de texto | `font.size.small`, `body`, `title` |
| Duración de animaciones | `motion.state` (0 con movimiento reducido) |

## Límites

- **Es un mapeo, no una copia.** egui tiene menos huecos de estilo que tokens tiene NDL. No expresa radios por componente, el desplazamiento del anillo de foco, curvas de easing ni pesos de fuente: quedan como constantes para widgets propios.
- **El foco comparte color con la selección.** egui usa el mismo trazo para ambos.
- **No hay componentes.** Solo estilo: barras de herramientas, inspectores o árboles con el aspecto de NDL habrá que construirlos en cada app o en un crate común posterior.
- **No instala fuentes.** Solo fija tamaños; la familia es la que cargue la app. NDL no distribuye Inter ni Sora.
- **Alto contraste y movimiento reducido** los decide la app: egui no expone esas preferencias del sistema.
- egui cambia su API de estilo entre versiones menores. El crate fija `0.37`; otra versión exige revisar `for_egui.rs`.

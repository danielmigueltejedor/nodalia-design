# Nodalia Design Language (NDL)

Lenguaje de diseño de las aplicaciones de Nodalia: una especificación única, *design tokens* en el estándar abierto DTCG y adaptadores por tecnología, para que Forja, Pluma, Lienzo, Vortia, Letra, Lámina o Cifra se reconozcan como parte del mismo ecosistema en Linux, macOS y Windows.

> **Estado: `0.1.0-alpha.1`.** Las APIs pueden cambiar entre versiones alfa. Qué funciona hoy, qué está verificado y qué falta está en [docs/status.md](docs/status.md); no hay nada presentado como terminado que no lo esté.

## Qué contiene

| Paquete | Qué es | Estado |
| --- | --- | --- |
| [`@nodalia/tokens`](packages/tokens) | 190 tokens DTCG (primitivos, semánticos, de componente), un acento por app derivado de un solo color, cuatro modos de apariencia, validación de contraste WCAG y generador de CSS | Operativo |
| [`@nodalia/web`](packages/web) | Componentes CSS sin framework + comportamientos ARIA (pestañas, árbol, barra de herramientas, separador, menú contextual, tooltip) | Operativo |
| [`@nodalia/icons`](packages/icons) | 255 iconos originales de trazo para cualquier app: SVG sueltos, *sprite*, índice y página de vista previa | Operativo; 55 de ellos en uso en Vortia 0.5.0, y el tema de iconos completo de nodalia-gtk (Vinilo 2.0) |
| [`@nodalia/gtk`](packages/gtk) | Hojas de estilo GTK4 / libadwaita (colores con nombre públicos) generadas desde los tokens | Generado y probado estructuralmente; **sin verificar en un runtime GTK real** |
| [`@nodalia/qml`](packages/qml) | Singleton `NodaliaTheme` para Qt Quick generado desde los tokens | Generado y probado estructuralmente; **sin verificar en Qt** |
| [`@nodalia/qtwidgets`](packages/qtwidgets) | Tokens aplanados, plantilla de hoja de estilos y hojas rellenas para Qt Widgets | En uso en Vortia 0.5.0 (Qt 6, macOS); **sin probar en Linux ni Windows** |
| [`@nodalia/egui`](packages/egui) | Crate Rust `nodalia-theme`: tokens como constantes y un `egui::Style` generado desde ellos | Compilado y probado contra egui 0.37; **sin verificar visualmente en una app real** |
| [`@nodalia/catalog`](apps/catalog) | Catálogo visual interactivo y tres demostraciones de referencia (productividad, editor creativo, ingeniería) | Operativo (no se publica) |

## Probarlo

Requiere Node.js 20 o superior (se desarrolla con 22).

```bash
npm install
npm run build
npm run serve -w @nodalia/catalog   # http://127.0.0.1:4318/
```

El catálogo permite cambiar tema (sistema, claro, oscuro), contraste, densidad, escala y movimiento, y muestra los estados de interacción de los controles.

## Verificación

```bash
npm run validate      # tokens: alias, capas, contraste WCAG; CSS: sin valores arbitrarios
npm test              # compila y ejecuta los tests unitarios de todos los paquetes
npm run check:repro   # dos compilaciones limpias deben producir ficheros idénticos
npm run test:rust     # crate nodalia-theme, con y sin egui (necesita Rust)
npm run test:browser  # Chromium: cascada de modos, axe-core, teclado, demostraciones
npm run ci            # todo lo anterior
```

Para los tests de navegador hace falta Chromium (`npx playwright-core install chromium`, o `NDL_CHROMIUM=/ruta/a/chromium`).

## Cómo se usa

- **Web:** `@import '@nodalia/web/nodalia.css'` y `class="ndl-button"`. Ver [docs/integration/web.md](docs/integration/web.md).
- **GTK4 / libadwaita:** cargar `nodalia-light.css` o `nodalia-dark.css` según `AdwStyleManager`. Ver [docs/integration/gtk.md](docs/integration/gtk.md).
- **Qt Quick:** importar el módulo `Nodalia` y enlazar `NodaliaTheme.mode`. Ver [docs/integration/qml.md](docs/integration/qml.md).
- **Rust / egui:** crate `nodalia-theme` y `for_egui::apply_system(&ctx, …)`. Ver [docs/integration/egui.md](docs/integration/egui.md).
- **Qt Widgets:** estilo Fusion + `nodalia-<modo>-<densidad>.qss`. Ver [docs/integration/qt-widgets.md](docs/integration/qt-widgets.md).
- **Otros:** [docs/integration/other.md](docs/integration/other.md).
- **Adoptarlo en una app existente, sin migrar de golpe:** [docs/migration.md](docs/migration.md).

## Documentación

[Lenguaje de diseño](docs/design-language.md) · [Arquitectura](docs/architecture.md) · [Tokens](docs/tokens.md) · [Componentes](docs/components.md) · [Iconografía](docs/iconography.md) · [Migración](docs/migration.md) · [Versionado y compatibilidad](docs/versioning.md) · [Estado y pendientes](docs/status.md) · [Contribuir](CONTRIBUTING.md)

## Licencia

MIT. Ver [LICENSE](LICENSE). El repositorio no incluye fuentes, iconos ni imágenes de terceros.

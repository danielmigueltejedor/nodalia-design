# Changelog

Formato basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/). Versionado según [docs/versioning.md](docs/versioning.md).

## [Sin publicar]

### Añadido
- `@nodalia/qtwidgets`: tokens aplanados, plantilla de hoja de estilos y doce hojas rellenas para Qt Widgets. Integrado en Vortia 0.5.0; sin probar en Linux ni Windows.
- `@nodalia/egui`: genera el crate Rust `nodalia-theme` (tokens como constantes; con la *feature* `egui`, `egui::Style` y `Visuals` para los cuatro modos y las tres densidades). Probado contra egui 0.37; sin verificar visualmente en una app.

## [0.1.0-alpha.0]

Primera versión: infraestructura y conjunto inicial.

### Añadido
- `@nodalia/tokens`: 190 tokens DTCG en tres capas; modos claro, oscuro y alto contraste (claro y oscuro); densidades compacta, regular y cómoda; validación de alias, capas y contraste WCAG; generador de CSS.
- `@nodalia/web`: componentes CSS (botones, campos de texto y numéricos, selector, casilla, opción, interruptor, deslizador, progreso, actividad, etiquetas, tooltip, menú, diálogo, notificación, búsqueda, barra de título, barra lateral, barra de herramientas, pestañas, paneles y separador, inspector, listas, tabla, árbol, estado vacío, barra de estado, área de trabajo, marco de visor) y comportamientos ARIA.
- `@nodalia/gtk` y `@nodalia/qml`: adaptadores generados desde los tokens.
- `@nodalia/catalog`: catálogo interactivo y demostraciones de productividad, editor creativo e ingeniería.
- Tests unitarios, de navegador (cascada de modos, axe-core, teclado, demostraciones), presupuesto de tamaño y chequeo de builds reproducibles. CI y release en borrador.

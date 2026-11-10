# Changelog

Formato basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/). Versionado según [docs/versioning.md](docs/versioning.md).

## [Sin publicar]

### Añadido
- `@nodalia/icons`: 192 iconos nuevos (247 en total) en diez grupos más: general, medios, texto, datos, diapositivas, diseño, foto, correo, modelado y documento. Mismas normas de dibujo. `dist/preview.html` muestra el conjunto completo.
- `@nodalia/gtk`: hojas de métricas `nodalia-metrics-{compact,regular,comfortable}.css` con espaciado, radios, tipografía, movimiento y tamaños de componente como variables `--ndl-*` (requiere GTK 4.16+), y `--ndl-elevation-raised` / `--ndl-elevation-overlay` en las hojas de cada modo. Siguen sin selectores de widgets: la app decide qué widget usa cada valor.
- `@nodalia/gtk`: `nodalia-gtk.css`, hoja de widgets para GTK 4 que solo lee variables `--ndl-*`.
- `nodalia-gtk`: crate de Rust para GTK 4.20 sin libadwaita. Instala las hojas, sigue las preferencias de claro, oscuro y contraste, y aporta navegación, barras, panel lateral, hoja inferior, diálogos, puntos de ruptura, filas de preferencias, páginas de estado y avisos con la API de libadwaita.
- `@nodalia/gtk`: `tools/check-gtk.py` carga todas las hojas en un GTK 4 real, estiliza widgets con ellas y falla con cualquier error; corre en CI.

### Cambiado
- NDL en GTK ya no se apoya en libadwaita (principio 9 y regla 4 de `AGENTS.md`). Las hojas siguen emitiendo sus colores con nombre.

## [0.1.0-alpha.1]

Primera versión etiquetada: una app puede fijarla y descargar su paquete desde la release.

### Añadido
- `@nodalia/icons`: 55 iconos originales de trazo (rejilla de 24 px, trazo de 1,75 px), como SVG sueltos en `currentColor`, en negro para toolkits que tiñen por opacidad, *sprite* e índice. Nacieron en Vortia 0.5.0.
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

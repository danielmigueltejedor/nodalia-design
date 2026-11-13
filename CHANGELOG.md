# Changelog

Formato basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/). Versionado según [docs/versioning.md](docs/versioning.md).

## [Sin publicar]

### Añadido
- `@nodalia/icons`: 142 iconos de dibujo vectorial (501 en total), hechos para Pluma y compartidos con todas las apps: herramientas (selección de grupo, punto de ancla, curvatura, tipos de texto, formas, cuchilla, cortes, licuar, anchura, generador de formas, perspectiva, malla, fusión, rociador de símbolos, mesa de trabajo), Buscatrazos completo, degradados, extremos y uniones de trazo, alineación del trazo, alinear y distribuir, reglas de relleno y paneles (muestras, estilos gráficos, apariencia, efectos, acciones).
- **Acento por app.** Cada app aporta un color clave en `accents.json` y NDL deriva una rampa con la misma luminancia, paso a paso, que la de por defecto, así que el contraste es idéntico y se valida en la compilación. 17 apps incluidas. Disponible en CSS (`data-accent`), en el crate Rust (`Accent`, `Mode::palette_with`) y en `nodalia-qt.json` (grupo `accent`). `dist/accents.json` lista rampas y colores resultantes.
- `@nodalia/icons`: 192 iconos nuevos (247 en total) en diez grupos más: general, medios, texto, datos, diapositivas, diseño, foto, correo, modelado y documento. Mismas normas de dibujo. `dist/preview.html` muestra el conjunto completo.
- `@nodalia/gtk`: hojas de métricas `nodalia-metrics-{compact,regular,comfortable}.css` con espaciado, radios, tipografía, movimiento y tamaños de componente como variables `--ndl-*` (requiere GTK 4.16+), y `--ndl-elevation-raised` / `--ndl-elevation-overlay` en las hojas de cada modo. Siguen sin selectores de widgets: la app decide qué widget usa cada valor.
- `@nodalia/gtk`: `nodalia-gtk.css`, hoja de widgets para GTK 4 que solo lee variables `--ndl-*`.
- `nodalia-gtk`: crate de Rust para GTK 4.20 sin libadwaita. Instala las hojas, sigue las preferencias de claro, oscuro y contraste, y aporta navegación, barras, panel lateral, hoja inferior, diálogos, puntos de ruptura, filas de preferencias, páginas de estado y avisos con la API de libadwaita.
- `@nodalia/gtk`: `tools/check-gtk.py` carga todas las hojas en un GTK 4 real, estiliza widgets con ellas y falla con cualquier error; corre en CI.

- `nodalia-update` (Windows): las instalaciones hechas con el MSI se actualizan desde la app. Descarga el MSI de la versión, comprueba su SHA-256, espera a que la app se cierre, lo instala con `msiexec` en la misma carpeta y para el mismo usuario o equipo, y vuelve a abrir la app. Una copia portable (con `portable.txt`) recibe el aviso y la página de la versión.
- `nodalia-update`: CI ejecuta el crate en Linux, macOS y Windows, con la instalación de cada sistema de principio a fin (imagen de disco firmada ad hoc y ayudante real en macOS, ayudante del MSI en Windows, AppImage en Linux).

### Cambiado
- `nodalia-update`: una app que no puede sustituirse donde está (carpeta sin permiso de escritura, la propia imagen de disco o una copia trasladada por macOS) ya no intenta instalar y falla: lo explica y abre la página de la versión. `InstallKind` gana `WindowsMsi`, `Portable` y `NotWritable`; `Failure` gana `Install` y `Cancelled` y pasa a ser `#[non_exhaustive]`.
- `nodalia-update`: un repositorio sin versiones publicadas responde «no hay actualizaciones» en vez de «The latest release has no version tag».
- `nodalia-gtk`: en macOS los semáforos del sistema quedan separados del borde de la isla tanto como del borde superior, con el mismo aire antes del título.
- NDL en GTK ya no se apoya en libadwaita (principio 9 y regla 4 de `AGENTS.md`). Las hojas siguen emitiendo sus colores con nombre.

### Corregido
- `nodalia-gtk`: los diálogos (`AlertDialog` y cualquier `Dialog` con `content-width`) ya no salen recortados. Medían su altura con el ancho del contenedor en vez de con su propio ancho, así que el texto que se parte en líneas ocupaba más de lo reservado y los botones quedaban fuera, sin poder pulsarse. Comprobado en una app real bajo Xvfb; sin prueba automática porque necesita una pantalla.
- `nodalia-gtk`: cambiar entre claro y oscuro ya no hace que GTK recargue su tema. El tema base es «Empty», que no tiene variante clara ni oscura que seguir, y `set_gtk_application_prefer_dark_theme` provocaba esa recarga (unos 10 ms por cambio y por app) sin aportar nada.
- `nodalia-gtk`: `EntryRow` y `PasswordEntryRow` con el campo vacío y sin foco muestran el título como marcador de posición, centrado en la fila, en lugar de un título pequeño sobre un hueco; al escribir o enfocar, el título sube sin cambiar la altura de la fila. El título de la fila de campo también queda alineado con el texto (heredaba el margen de los títulos de grupo).
- `nodalia-update` (macOS): la actualización dentro de la app ya no falla con «has an invalid code signature» en apps firmadas ad hoc. La comprobación usaba `codesign --verify --strict`, que rechaza los metadatos del Finder (`com.apple.FinderInfo`) que llevan todos los ficheros de una imagen de disco; ahora comprueba el sello sin `--strict` (firma, ficheros sellados y código anidado siguen verificándose). El error indica la app y el motivo que da `codesign`.

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

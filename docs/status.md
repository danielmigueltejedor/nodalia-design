# Estado y pendientes

Versión: `0.1.0-alpha.1`. Este documento es la fuente de verdad sobre qué está hecho. Si algo no aparece como verificado, no lo está.

## Qué funciona y cómo se ha comprobado

| Área | Estado | Comprobación |
| --- | --- | --- |
| Tokens DTCG (190) con capas primitivo → semántico → componente | Operativo | `npm run validate`: alias, ciclos, tipos, capas |
| Modos claro, oscuro, claro alto contraste, oscuro alto contraste | Operativo | Validación WCAG de pares texto/fondo (4,5:1 y 3:1; 7:1 y 4,5:1 en alto contraste) |
| Densidades compacta / regular / cómoda | Operativo | Tests de navegador: altura de control medida |
| Movimiento reducido (sistema y forzado) | Operativo | Tests de navegador: duraciones computadas a 0 ms |
| Salida CSS | Operativo | Matriz de 12 combinaciones de cascada en Chromium |
| Componentes web (`@nodalia/web`) | Operativo en Chromium | 53 tests de navegador, 16 ejecuciones de axe-core sin violaciones, teclado |
| Catálogo y 3 demostraciones | Operativo | Tests de navegador (humo, interacción y axe) |
| Iconos (`@nodalia/icons`, 247) | Operativo; 55 en uso en Vortia 0.5.0, el resto solo revisados en hoja de contactos | 7 tests: solo formas simples, sin color propio, dentro de la rejilla, *sprite* e índice completos |
| Qt Widgets (`@nodalia/qtwidgets`) | En uso en Vortia 0.5.0 (Qt 6, Fusion, macOS, claro y oscuro) | 8 tests: cada marcador resuelve en los 4 modos y 3 densidades, sin literales en la plantilla, contraste de los pares que dibuja; revisión visual con capturas de Vortia |
| Crate Rust `nodalia-theme` (`@nodalia/egui`) | Compila y pasa sus tests con egui 0.37 | 6 tests del generador (incluye que `tokens.rs` coincide con los tokens); 12 tests Rust: contraste de los pares que dibuja egui, métricas por densidad, movimiento reducido, aplicación a un `Context` |
| Compilación reproducible | Operativo | `npm run check:repro`: dos compilaciones limpias, SHA-256 idéntico en 19 ficheros |
| Linter de CSS (sin literales de color ni de tamaño, sin variables desconocidas) | Operativo | Tests unitarios del linter y ejecución sobre el CSS real |
| Presupuesto de tamaño | Operativo | CSS y comportamientos comprobados en gzip |

Totales en la última ejecución completa: 59 tests unitarios, 12 tests Rust y 53 de navegador, todos correctos.

## Qué NO está verificado

- **Qt Widgets fuera de macOS.** Solo se ha visto en Vortia sobre macOS; Linux, Windows y los modos de alto contraste están sin revisar.
- **egui en una app real.** El estilo se construye y se comprueba por valores, pero nadie ha revisado visualmente una app con él. La primera integración (Vortia en Rust) dirá qué hay que ajustar.
- **GTK4 / libadwaita.** `@nodalia/gtk` genera hojas con `@define-color` y solo nombres de color públicos de libadwaita. Se prueba que el contenido es el esperado, que solo se usan nombres documentados y que los pares críticos cumplen contraste. **No se ha cargado en un runtime GTK real** (no había libadwaita en el entorno de desarrollo). Hay que comprobarlo con una app de verdad antes de darlo por bueno.
- **Qt Quick.** `NodaliaTheme.qml` se genera y se prueba estructuralmente (paletas, formato `#AARRGGBB`, curvas de easing). **No se ha ejecutado con Qt.**
- **Otros navegadores.** Solo se ha probado Chromium. Firefox y WebKit están sin probar.
- **Lectores de pantalla reales.** axe-core detecta una parte de los problemas; no sustituye a una prueba manual con Orca, NVDA o VoiceOver.
- **Regresión visual por píxeles.** No se hace: las fuentes no se incluyen y los sustitutos del sistema difieren entre máquinas, así que las comparaciones darían falsos positivos. Lo que sí se comprueba es estructura, estilos computados, contraste y accesibilidad.
- **Windows y macOS.** El código es multiplataforma, pero solo se ha ejecutado en Linux.

## Limitaciones conocidas

- Los adaptadores de egui y Qt Widgets solo aportan estilo, no componentes. Los límites de Qt están en [Qt Widgets](integration/qt-widgets.md).
- No hay fuentes incluidas (los tokens nombran Inter y Sora, que caen a la del sistema si no están instaladas). Los 192 iconos añadidos después de Vortia no se han usado todavía en ninguna app: pueden necesitar ajustes al verlos en contexto. Ver [iconografía](iconography.md).
- Componentes no implementados: combobox con autocompletado, submenús, tablas ordenables, gestor de notificaciones, acoplamiento de paneles por arrastre. Ver [componentes](components.md).
- Capas, árbol de operaciones e historial aparecen como **composiciones en las demostraciones**, no como componentes empaquetados.
- Las acciones de GitHub usan etiquetas de versión mayor (`@v4`), no SHA fijados.
- No se han publicado paquetes; el flujo de release crea borradores y marca como pre-release cualquier etiqueta con guion.

## Pendiente, por prioridad

1. Integrar `nodalia-theme` en la primera app egui y revisar el resultado visualmente.
2. Revisar la hoja de Qt Widgets en Linux y Windows, y un token de altura para igualar campos numéricos y desplegables.
3. Verificar GTK y QML en apps reales y corregir lo que aparezca.
4. Integrar NDL en la primera app (ver [migración](migration.md)) y recoger fricciones.
5. Usar los iconos nuevos en las apps y corregir los que no funcionen; decidir la fuente incluida (ver [iconografía](iconography.md)).
6. Probar en Firefox y WebKit; añadir una pasada manual con lector de pantalla.
7. Componentes pendientes: combobox, submenús, tablas ordenables, notificaciones, acoplamiento.
8. Fijar acciones de CI por SHA; crear el proyecto en SonarQube y activar CodeRabbit si se desea.
9. Publicación de paquetes en un registro (solo con decisión explícita; hoy no hay publicación automática).

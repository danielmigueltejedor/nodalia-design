# Componentes

Estado de cada componente en la implementación web (`@nodalia/web`). GTK usa los widgets de libadwaita coloreados por NDL; Qt Quick expone tokens, no controles.

Leyenda: **Hecho** = implementado, con estados, accesibilidad y tests. **Parcial** = existe con límites indicados. **Pendiente** = no existe.

## Controles básicos

| Componente | Clase | Estado | Notas |
| --- | --- | --- | --- |
| Botón (primario, secundario, sutil, peligro) | `ndl-button` | Hecho | estados normal, hover, pulsado, foco, deshabilitado |
| Botón de icono / barra de herramientas | `ndl-toolbar` + `toolbar()` | Hecho | teclado con tabulación itinerante |
| Campo de texto | `ndl-input`, `ndl-field` | Hecho | etiqueta, ayuda y error asociados por ARIA |
| Búsqueda | `ndl-search` | Hecho | |
| Campo numérico con unidad | `ndl-numeric`, `ndl-unit-field` | Parcial | presentación y unidad; la conversión es de la app |
| Casilla / radio | `ndl-check`, `ndl-choice` | Hecho | área de pulsación de 24 px |
| Interruptor | `ndl-switch` | Hecho | |
| Control deslizante | `ndl-slider` + `slider()` | Hecho | |
| Desplegable (select nativo) | `ndl-select` | Parcial | envoltura estilizada del `<select>` nativo; sin combobox personalizado |
| Barra de progreso, indicador | `ndl-progress`, `ndl-spinner` | Hecho | indeterminado respeta movimiento reducido |
| Insignia | `ndl-badge` | Hecho | |
| Enlace | `ndl-link` | Hecho | |

## Estructura y navegación

| Componente | Clase | Estado | Notas |
| --- | --- | --- | --- |
| Ventana de la app | `ndl-app` | Hecho | |
| Barra de título y de estado | `ndl-titlebar`, `ndl-statusbar` | Hecho | |
| Barra lateral | `ndl-sidebar`, `ndl-nav-item` | Hecho | `aria-current` |
| Pestañas | `ndl-tabs`, `ndl-tab`, `ndl-tabpanel` + `tabs()` | Hecho | flechas, Inicio, Fin |
| Menú contextual | `ndl-menu` + `contextMenu()` | Parcial | sin submenús |
| Diálogo | `ndl-dialog` | Hecho | sobre `<dialog>` nativo |
| Toast | `ndl-toast`, `ndl-toast-region` | Parcial | región viva; sin gestor de cola |
| Tooltip | `ndl-tooltip` + `tooltips()` | Hecho | también con foco; Escape lo cierra |
| Lista | `ndl-list` | Hecho | |
| Tabla | `ndl-table` | Parcial | estilo y semántica; sin ordenación |
| Árbol | `ndl-tree` + `tree()` | Hecho | `aria-expanded`, teclado completo |
| Panel y grupo | `ndl-panel`, `ndl-group` | Hecho | |
| Separador redimensionable | `ndl-split`, `ndl-splitter` + `splitter()` | Hecho | teclado y puntero, `aria-valuenow` |
| Estado vacío | `ndl-empty` | Hecho | |

## Patrones profesionales

| Patrón | Estado | Dónde se ve |
| --- | --- | --- |
| Marco de visor / lienzo | Hecho (`ndl-viewport` + `ndl-workspace`) | demos creativa e ingeniería |
| Inspector paramétrico | Composición | demo de ingeniería (`ndl-prop`, `ndl-numeric`) |
| Árbol de operaciones | Composición | demo de ingeniería |
| Historial deshacer/rehacer | Composición | demos creativa e ingeniería |
| Capas | Composición | demo creativa |
| Importar / exportar con progreso | Composición | demos creativa e ingeniería |
| Lista de tareas con selección y confirmación | Composición | demo de productividad |
| Notificaciones con gestor | Pendiente | |
| Acoplamiento de paneles por arrastre | Pendiente | |
| Combobox con autocompletado | Pendiente | |
| Submenús | Pendiente | |
| Tabla ordenable | Pendiente | |

«Composición» significa que el patrón está montado con los componentes anteriores en una demostración funcional, pero no es un componente reutilizable empaquetado. Los datos y la lógica (capas, operaciones, unidades) son de las demos: NDL no incluye motores de CAD, render ni simulación.

## Contrato de un componente

- Solo consume tokens. El linter rechaza literales.
- Tiene estilo de foco por teclado y no elimina `outline`.
- Los estados son reales (`:hover`, `:disabled`, `aria-*`); `data-state` solo existe para forzarlos en el catálogo y en pruebas.
- Funciona en los cuatro modos y las tres densidades.
- Pasa axe-core en el catálogo.

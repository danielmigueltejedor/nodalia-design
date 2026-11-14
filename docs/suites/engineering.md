# Suite de ingeniería: Torno, Vortia y Forja

Las tres apps de ingeniería de Nodalia (Torno, CAD 3D paramétrico; Vortia, simulación CFD; Forja,
dibujo 2D) deben leerse como una sola suite: quien sabe usar una encuentra en el mismo sitio el
árbol del modelo, las propiedades, la búsqueda de comandos o las preferencias de las otras dos.
Sobre NDL (islas, tipografía con peso, menú hamburguesa, controles de ventana propios solo en
Linux, menú global por D-Bus, sin degradados ni brillos) esta página fija lo que comparten.

Las tres están hechas con **egui 0.37**, así que lo compartido es código, no solo una guía: el
módulo `nodalia_theme::engineering` (feature `engineering` del crate `nodalia-theme`). Lo que
todavía no está en el módulo está descrito aquí y cada app lo sigue a mano hasta que se suba.

Todo lo de esta página es diseño propio de Nodalia. No se copia código, medidas, paletas ni
dibujos de otras aplicaciones (en particular, nada del material de Torno bajo `LICENSE-BRAND`).

## Tipografía

Una sola familia para las tres: **IBM Plex Sans** (variable, pesos 400/500/600/700) para la
interfaz e **IBM Plex Mono** para valores. Las dos son SIL OFL 1.1 y van incrustadas en el crate
(`packages/egui/crate/fonts`, tomadas de `google/fonts`). Plex es técnica y legible a tamaño
pequeño, y su versión mono alinea cifras: lo que una app de ingeniería muestra todo el rato.

| Rol | Fuente | Tamaño | Peso | Dónde |
| --- | --- | --- | --- | --- |
| `caption()` | Plex Sans | 11 | 400 | nombre de grupo del ribbon, barra de estado, ayudas |
| `body()` | Plex Sans | 12,5 | 400 | filas del árbol, propiedades, menús |
| `label()` | Plex Sans | 12,5 | 500 | pestañas, etiquetas de herramienta, fila seleccionada |
| `panel_title()` | Plex Sans | 13 | 600 | «Navegador», «Propiedades», cabeceras de isla |
| `dialog_title()` | Plex Sans | 17 | 600 | título de diálogo |
| `page_title()` | Plex Sans | 26 | 700 | título de Inicio y de Preferencias |
| `value_font()` | Plex Mono | 12 | 400 | longitudes, coordenadas, residuos, línea de comandos, registros |

Reglas:

- Todo número que el usuario lee o escribe (cotas, coordenadas, unidades, iteraciones) va en
  Plex Mono, para que las columnas de cifras se alineen.
- Sin cursivas ni subrayados en la interfaz. El énfasis es el peso.
- Los alfabetos que Plex no cubre (CJK, árabe, tailandés…) siguen detrás de Plex como
  respaldo: `engineering::add_fonts` pone Plex delante y conserva lo que la app ya tenía.

Uso:

```rust
// App sin fuentes propias (Vortia):
nodalia_theme::engineering::install_fonts(ctx);

// App que ya monta sus definiciones con respaldos de idioma (Forja, Torno):
let mut fonts = build_app_fonts(language);
nodalia_theme::engineering::add_fonts(&mut fonts);
ctx.set_fonts(fonts);
nodalia_theme::engineering::apply_text_styles(ctx);
```

## La ventana

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ ◆      ⌂  Archivo  💾 ↶ ↷ │ Doc 1 × │ Doc 2 × │ +     🔍 Buscar comandos  ≡  ● ● ● │  barra de título 40
├──────────────────────────────────────────────────────────────────────────────┤
│ Espacio ▾ │ Modelo  Superficie  Inspeccionar  Vista …                        │  pestañas 28
│           │ [⬚][⬚][⬚] │ [⬚][⬚] │ [⬚][⬚][⬚][⬚] │ …                         │  ribbon 92
│           │   Crear     Modificar    Construir                               │
├────────────┬────────────────────────────────────────────────┬────────────────┤
│ Navegador  │                                                │ Propiedades    │
│ (isla 264) │            vista (lienzo, sin isla)            │ (isla 304)     │
│            │                            orientación ▣ ⌂     │                │
│            │         [ barra de vista flotante, píldora ]   │                │
├────────────┴────────────────────────────────────────────────┴────────────────┤
│ Historial / Modelo · Presentación │ x 12.500  y 4.003  mm │ rejilla forzado …│  barra de estado 30
└──────────────────────────────────────────────────────────────────────────────┘
```

Medidas en `engineering::layout`. Separación entre islas y con el borde: 8. Radio de isla: 12.

### Barra de título

De izquierda a derecha: el **icono de la app** (24, en lugar del nombre, que queda como
etiqueta accesible; `TitleBar::icon`); **Inicio** (casa); **Archivo** (menú de
documento: nuevo, abrir, guardar, exportar, imprimir); acceso rápido (guardar, deshacer, rehacer: cada app lista solo los que tiene, `TitleBar::quick_access`);
las **pestañas de documento**; a la derecha la **búsqueda de comandos** (mismo atajo en las tres:
`Ctrl+K` / `⌘K`), el **menú hamburguesa** (Preferencias, Atajos de teclado, Acerca de) y los
controles de ventana. En Linux, controles NDL redondos salvo que el usuario elija los del
escritorio; en macOS y Windows, los nativos. El menú de la app se exporta al menú global por
D-Bus; la barra de menús dentro de la ventana solo aparece si el escritorio no tiene menú global.

### Ribbon

- A la izquierda, el **selector de espacio de trabajo** (Torno: Diseño, Chapa, Ensamblaje…;
  Vortia: Caso; Forja: Dibujo 2D, Anotación…).
- Las **pestañas** del espacio (`ribbon_tab`): texto en peso medio, subrayado de acento en la
  activa, sin caja.
- Debajo, **grupos de herramientas** (`ribbon_group`): botones de 60×48 (`ribbon_tool`) con un
  icono CAD de 22 sobre la etiqueta, y el nombre del grupo en `caption` debajo, separados por una
  línea fina. Un grupo puede terminar en una flecha que despliega sus herramientas menos usadas.
- El ribbon se puede plegar a solo pestañas (doble clic en una pestaña), y el estado se recuerda.

### Navegador (izquierda)

Isla con cabecera (`panel_header`: título y, a la derecha, un recuento o un filtro) y el árbol
del modelo (`tree_row`): filas de 26, sangría de 16 por nivel, flecha de despliegue, icono de 16,
ojo de visibilidad al final. Seleccionado: tinte suave del acento, texto en acento y peso medio.
Oculto o suprimido: texto terciario.

- Torno: documento, orígenes, cuerpos, bocetos, operaciones.
- Vortia: caso, geometría, malla (regiones, parches), física (modelos, materiales), condiciones
  de contorno, ajustes del cálculo, resultados.
- Forja: capas, bloques, presentaciones (sustituye a la paleta de capas actual).

### Propiedades (derecha)

Isla plegable con cabecera y una rejilla de propiedades (`properties` + `property`): etiqueta a la
derecha de una columna de 112 en texto secundario, control a la izquierda, filas de 28. Valores de
solo lectura con `value` (Plex Mono). Secciones plegables con título en `label`.

- Torno: parámetros de la operación seleccionada y propiedades físicas.
- Vortia: el editor de la condición de contorno, el modelo o el esquema seleccionado.
- Forja: propiedades del objeto seleccionado (color, capa, tipo de línea…).

### Vista

El lienzo no va en isla: ocupa el centro entero. Arriba a la derecha, el indicador de
orientación (cubo o triedro) y el botón de vista inicial; abajo al centro, la **barra de vista
flotante** (una isla en píldora: órbita, encuadre, zoom, estilo visual, secciones). Ejes: X rojo,
Y verde, Z azul. Selección en el color de acento; preselección en el acento de *hover*.

### Barra de estado (abajo)

Una franja de 30 en `caption`: a la izquierda lo que es de la app (Torno: historial de
operaciones; Forja: pestañas Modelo/Presentación; Vortia: estado del cálculo y residuos), a la
derecha coordenadas y unidades en Plex Mono y los conmutadores de ayudas (rejilla, forzado,
ortogonal…).

## Inicio

Inicio es la vista de Archivo de la suite de oficina (`office::file_view`) dibujada con la tipografía
de la suite (`engineering::home_look`): isla de navegación a la izquierda y la página en una isla
al lado. Navegación, en este orden: **Inicio**, **Abrir**, **Ejemplos**, y abajo **Preferencias**.
Sin fila de Volver cuando no hay documento abierto.

La página de Inicio, de arriba abajo:

1. `home_header`: el icono de la app (52), su nombre en `page_title` y debajo la versión.
2. **Nuevo** (`start_cards`): tarjetas de 176×116 con un icono CAD en una insignia teñida del
   acento, el nombre y una línea con lo que trae. Torno: Pieza, Ensamblaje, Plano; Vortia:
   plantillas de caso por tipo de flujo y calor; Forja: Dibujo en mm, Dibujo en pulgadas.
3. **Recientes** (`recent_documents`): filas de 56 con miniatura (o el icono CAD del tipo de
   documento), nombre sobre carpeta, y la fecha en Plex Mono a la derecha.

Las variantes `office::file_view_with`, `start_cards_with` y `recent_documents_with` llaman a la
app con el id o el índice y la `Response` de cada elemento al dibujarlo, para registrarlos en su
automatización (agentes, pruebas).

## Preferencias

Un diálogo NDL (solo botón de cerrar) con navegación a la izquierda y una página que se desplaza.
Las mismas secciones, en el mismo orden, en las tres apps; una app omite la que no aplica:

1. **General**: idioma, al iniciar (Inicio o último documento), autoguardado.
2. **Apariencia**: tema (claro, oscuro, sistema; el selector solo vive aquí), densidad,
   controles de ventana (Linux).
3. **Unidades y precisión**: sistema de unidades, decimales, ángulos.
4. **Vista**: esquema de navegación del ratón (Nodalia, y estilos de otras herramientas
   descritos por comportamiento), fondo, rejilla, dispositivo 3D.
5. **Atajos de teclado**.
6. **Rendimiento**: GPU, suavizado, hilos (Vortia: núcleos del cálculo).
7. **Actualizaciones**.

## Diálogos

Ventanas NDL (`ndl_window` o equivalente) con controles integrados, nunca la barra de título de
GNOME. Título en `dialog_title`; cuerpo con la rejilla de propiedades; botones abajo a la
derecha, el principal el último y en acento. Un comando lanzado por programa (MCP, CLI, canal de
control) nunca abre un diálogo.

## Iconos

Iconos CAD en `packages/icons`, grupo `cad`: un mismo concepto es el mismo icono en las tres
apps (extruir, revolución, malla, sección, cota…). Estilo regular con piezas de acento; los
iconos de interfaz (casa, guardar, menú) en el estilo grueso de NDL. El conjunto volumétrico
propio que sustituye a los iconos de Torno bajo licencia de marca se añade ahí.

## Estado

| Pieza | En `engineering` | Torno | Vortia | Forja |
| --- | --- | --- | --- | --- |
| Fuentes y escala tipográfica | sí | pendiente (ya usa Plex) | pendiente | pendiente |
| Isla, cabecera de panel, propiedades | sí | pendiente | pendiente | pendiente |
| Fila de árbol | sí | pendiente | pendiente | pendiente |
| Pestaña, grupo y herramienta del ribbon | sí | pendiente | pendiente | pendiente |
| Barra de título (`title_bar`) | sí | — | — | — |
| Inicio (`home_look`, `home_header`, `start_cards`, `recent_documents`) | sí | — | — | en curso |
| Preferencias | descrita | — | — | — |
| Barra de estado (`status_bar`, `status_value`, `status_toggle`) | sí | — | — | — |
| Barra de vista flotante (`view_bar`) | sí | — | — | — |

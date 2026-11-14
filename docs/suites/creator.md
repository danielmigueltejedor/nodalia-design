# Suite creativa: Lienzo, Pluma y Toma

Las apps creativas de Nodalia (Lienzo, imagen; Pluma, ilustración vectorial; Toma, vídeo, más
adelante) son una familia: quien usa una encuentra en el mismo sitio y con el mismo aspecto el
selector de espacio de trabajo, la búsqueda, Inicio, Preferencias y los botones de cada diálogo.
Sobre NDL (islas, tipografía con peso, menú hamburguesa, controles de ventana propios solo en
Linux, menú global por D-Bus, sin degradados ni brillos) esta página fija lo que comparten.

Las tres están hechas con **egui 0.37**, así que lo compartido es código: el módulo
`nodalia_theme::creator` (feature `creator` del crate `nodalia-theme`). Una pieza que aparece en
más de una app se dibuja con ese módulo, nunca con una copia propia; si a una app le falta una
variante, se añade aquí y se sube el `rev` en las dos.

Los colores salen de la `Palette` de cada app (`Look::new(palette)`), así que un esquema propio
(Medium Dark) o el acento de cada app se conservan. La cara seminegrita la da cada app
(`Look::strong_family`).

## La ventana

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ ≡  ⌂  Archivo Edición …        documento — App        🔍  [Esenciales ▾]  ● ● ● │  barra 44
├──┬───────────────────────────────────────────────────────────┬──┬────────────┤
│▣ │ │ doc 1 × │ doc 2 × │                                     │▣ │ Propiedades│
│▣ │                                                           │▣ │ Capas  …   │
│  │                  lienzo (isla)                            │  │  (isla)    │
│  │                                                           │  │            │
├──┴───────────────────────────────────────────────────────────┴──┴────────────┤
│ barra de opciones / estado                                                    │
└──────────────────────────────────────────────────────────────────────────────┘
```

Medidas en `creator::layout`: barra 44, separación entre islas y con el borde 8, radio de isla 16,
radio de control 10, radio de campo 8, altura de control 28 y de botón 30. Los botones y las pestañas usan la cara media a 13 (`Look::medium_family`).

### Barra de la app

De izquierda a derecha: el **menú hamburguesa** (Acerca de, Preferencias, Atajos de teclado,
Salir), **Inicio** (casa), los menús de trabajo, el **título** centrado en el hueco libre (nombre del
documento y la app, en texto secundario), y al final la **búsqueda de comandos** (icono de 28) y
el **selector de espacio de trabajo** (`workspace_selector`, hasta 130 de ancho). En Linux siguen
los controles de ventana NDL redondos salvo que el usuario elija los del escritorio; en macOS y
Windows, los nativos. Una barra estrecha quita primero la búsqueda (también está en un menú) y
estrecha el selector. Sin selector de claro/oscuro: vive solo en Preferencias.

### Islas

Las herramientas, el carril de iconos, el grupo de paneles y el lienzo son islas sobre el fondo de
ventana. Toda isla tiene las cuatro esquinas redondeadas; donde un componente heredado pinta
recto, `round_island_corners` las redondea por fuera. Las pestañas de documento y de panel son
píldoras con el radio de control, sin líneas divisorias.

## Inicio

Una isla con un carril a la izquierda y la página al lado, **la misma pantalla en las tres**: la dibuja
`creator::home::view`, y cada app solo pone sus textos, su marca y sus archivos.

1. **Nuevo archivo** (`primary_button`) y **Abrir…** (`secondary_button`), de 130 de ancho.
2. Las filas **Inicio** y **Aprender** (`home_row`, iconos `home` y `sparkle`): la seleccionada es una
   píldora en el color de texto principal.
3. Abajo, los enlaces **Sitio web** (getnodalia.com/&lt;app&gt;) y **GitHub**.

La página lleva la marca y el nombre de la app (seminegrita a 24), una línea, y debajo
**Archivos recientes** (`home::recents`): tarjetas de 184 × 236 con miniatura (o el icono de archivo
si la app no tiene), nombre y carpeta, cerradas por una tarjeta de soltar; o una lista con el mismo
contenido. El cambio entre tarjetas y lista es de la app y se guarda. Sin recientes, una zona de
soltar ocupa su sitio. Lo que la app añada después (los tamaños de Pluma) va con
`home::section_heading`. Más estrecha que 640, los dos botones se ponen encima de la página y todo
se desplaza.

## Preferencias

Un diálogo NDL (solo botón de cerrar, `close_button`) con la búsqueda a todo el ancho arriba
(`search_field`), la navegación a la izquierda (`nav_row`: la sección seleccionada es una píldora
gris con texto seminegrita) y la página a la derecha. Abajo a la derecha **Aceptar**
(`primary_button`), **Cancelar** y **Aplicar** (`secondary_button`).

## Diálogos

- Título en seminegrita a la izquierda y `close_button` (círculo de 28) a la derecha, a la misma
  distancia del borde que el título.
- Botones abajo a la derecha. La acción principal, una sola, con `primary_button` y la última;
  todo lo demás con `secondary_button`. Ancho mínimo 72 para que se alineen, 30 de alto, como en Lienzo.
- Pestañas dentro de un diálogo (Herramientas / Comandos de menú) con `tab`.
- Campos, desplegables y tablas con `field_frame` (radio 8); casillas con radio 4.
- Un comando lanzado por programa (MCP, CLI, canal de control) nunca abre un diálogo.

## Estado

| Pieza | En `creator` | Lienzo | Pluma | Toma |
| --- | --- | --- | --- | --- |
| Selector de espacio de trabajo | sí | pendiente | sí | — |
| Botones principal y secundario | sí | pendiente | sí | — |
| Botón de cerrar de diálogo | sí | pendiente | sí | — |
| Navegación de Preferencias | sí | pendiente | sí | — |
| Carril de Inicio | sí | pendiente | sí | — |
| Pestañas, campos, búsqueda | sí | pendiente | sí | — |
| Esquinas de isla | sí | pendiente | sí | — |
| Pantalla de Inicio | sí | pendiente | pendiente | — |
| Barra de la app | descrita | — | — | — |

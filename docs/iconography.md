# Iconografía

**Estado: 55 iconos funcionales incluidos en `@nodalia/icons`.** Son dibujos originales de este proyecto (MIT). No se incluyen fuentes ni imágenes de terceros.

## El conjunto

`@nodalia/icons` cubre edición, archivos, ejecución, vista, croquis, simulación y resultados. Nació en Vortia 0.5.0; faltan los iconos propios de otras apps (texto, capas, medios, correo), que se añaden aquí cuando una app los necesite, no en la app.

| Salida | Para qué |
| --- | --- |
| `dist/svg/<nombre>.svg` | En `currentColor`: web y todo lo que herede el color del texto |
| `dist/mono/<nombre>.svg` | En negro: toolkits que tiñen por opacidad (motores de iconos de Qt, texturas en egui) |
| `dist/sprite.svg` | Todos como `<symbol id="ndl-<nombre>">` para `<use href>` |
| `dist/icons.json` | Nombres y grupos |

Para añadir o cambiar un icono se edita `packages/icons/src/icons.mjs` y se regenera; un SVG generado no se retoca.

## Cuatro categorías que no se mezclan

| Categoría | Qué es | Quién la define |
| --- | --- | --- |
| Iconos funcionales (simbólicos) | acciones y estados dentro de la interfaz (guardar, deshacer, capa visible…) | NDL y el sistema |
| Iconos de aplicación | identidad de cada app en el lanzador y el dock | cada app, con la marca Nodalia |
| Iconos de tipo de archivo | formatos propios (`.forja`, etc.) | cada app |
| Cursores | puntero del sistema y de herramientas | el tema de cursores del sistema; las apps solo definen cursores de herramienta propios |

## Nodalix OS

Nodalix OS conserva **Adwaita** como tema de iconos funcionales, su propio tema de iconos de aplicación y su tema de cursores. NDL no los sustituye ni los redefine. En GTK, las apps usan nombres de icono del tema (`document-save-symbolic`) para que cada sistema aporte su propio conjunto; los iconos propios de NDL solo cubren lo que el tema del sistema no tiene (por ejemplo, herramientas de modelado).

## Reglas de dibujo

- Rejilla de 24 px con 3 px de margen libre; coordenadas en múltiplos de 0,25 px.
- Trazo de 1,75 px, extremos y uniones redondeados, siempre `currentColor`: el icono hereda el color del texto y el contraste lo gobierna el token.
- Solo `path`, `circle` y `rect`. Sin transformaciones, clases ni estilos en línea.
- Los nodos (vértices, puntos de control) son círculos rellenos.
- Línea discontinua solo para lo que no es geometría real: ejes, guías, el segmento que falta.
- Relleno suave (18 %) solo para indicar una cara o un plano.
- Nunca solo icono si el significado no es evidente: etiqueta visible o `aria-label`/tooltip.
- Se dibujan originales. No se copian, trazan ni adaptan iconos de otros conjuntos.

Los tests del paquete comprueban las cuatro primeras reglas.

## Licencias

- Todo activo añadido al repositorio lleva autor, origen y licencia en un registro (`ATTRIBUTION.md`, que se crea con el primer activo).
- Licencias admisibles: las compatibles con MIT y con redistribución (MIT, Apache-2.0, BSD, ISC, CC0, CC-BY con atribución, OFL para fuentes).
- No se admiten activos sin licencia clara ni «uso libre solo personal».

## Fuentes

Los tokens nombran `Inter` y `Sora` con respaldo en la pila del sistema, pero **no se distribuyen**. Si se decide incluirlas, irán como paquete independiente con su licencia OFL y un registro de atribución.

## Pendiente

- Iconos para las apps que aún no tienen (texto, capas, medios, correo).
- Pipeline que convierta SVG a recurso GTK (`gresource`), Qt (`qrc`) y web (sprite) sin copiar a mano.
- Decisión sobre fuente propia.

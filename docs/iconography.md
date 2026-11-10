# Iconografía

**Estado: política definida, sin iconos incluidos.** `0.1.0-alpha.0` no incluye ningún icono, fuente ni imagen. Esto es deliberado: ningún activo se añade sin origen y licencia verificables.

## Cuatro categorías que no se mezclan

| Categoría | Qué es | Quién la define |
| --- | --- | --- |
| Iconos funcionales (simbólicos) | acciones y estados dentro de la interfaz (guardar, deshacer, capa visible…) | NDL y el sistema |
| Iconos de aplicación | identidad de cada app en el lanzador y el dock | cada app, con la marca Nodalia |
| Iconos de tipo de archivo | formatos propios (`.forja`, etc.) | cada app |
| Cursores | puntero del sistema y de herramientas | el tema de cursores del sistema; las apps solo definen cursores de herramienta propios |

## Nodalix OS

Nodalix OS conserva **Adwaita** como tema de iconos funcionales, su propio tema de iconos de aplicación y su tema de cursores. NDL no los sustituye ni los redefine. En GTK, las apps usan nombres de icono del tema (`document-save-symbolic`) para que cada sistema aporte su propio conjunto; los iconos propios de NDL solo cubren lo que el tema del sistema no tiene (por ejemplo, herramientas de modelado).

## Reglas para iconos funcionales propios

- Rejilla de 16 px, trazo de 1,5 px, esquinas redondeadas pequeñas, `currentColor` (el icono hereda el color del texto y el contraste lo gobierna el token).
- Formato SVG simbólico; un único archivo por icono.
- Nunca solo icono si el significado no es evidente: etiqueta visible o `aria-label`/tooltip.
- Se dibujan originales. No se copian, trazan ni adaptan iconos de otros sistemas sin licencia compatible.

## Licencias

- Todo activo añadido al repositorio lleva autor, origen y licencia en un registro (`ATTRIBUTION.md`, que se crea con el primer activo).
- Licencias admisibles: las compatibles con MIT y con redistribución (MIT, Apache-2.0, BSD, ISC, CC0, CC-BY con atribución, OFL para fuentes).
- No se admiten activos sin licencia clara ni «uso libre solo personal».

## Fuentes

Los tokens nombran `Inter` y `Sora` con respaldo en la pila del sistema, pero **no se distribuyen**. Si se decide incluirlas, irán como paquete independiente con su licencia OFL y un registro de atribución.

## Pendiente

- Decidir el conjunto inicial de iconos de herramientas de modelado y su proceso de dibujo.
- Pipeline que convierta SVG a recurso GTK (`gresource`), Qt (`qrc`) y web (sprite) sin copiar a mano.
- Decisión sobre fuente propia.

# @nodalia/icons

Los iconos de NDL: 359 dibujos originales de trazo sobre una rejilla de 24 px. Se definen en [`src/icons.mjs`](src/icons.mjs) y se generan como SVG sueltos, *sprite* e índice. `dist/preview.html` los muestra todos por grupo.

El grupo `cad` reúne las herramientas de dibujo técnico (trazar, modificar, acotar, capas, bloques, restricciones) que comparten las apps CAD de Nodalia, para que un mismo concepto tenga el mismo icono en todas. Los que ya existían en otros grupos (`line`, `circle`, `rectangle`, `fillet`, `chamfer`, `dimension`...) no se repiten. El campo `accent` de un icono lista las piezas que una app CAD pinta con su color de acento (la geometría que crea la herramienta); el resto de usos lo ignora y pinta el icono de un solo color.

Ver [docs/iconography.md](../../docs/iconography.md). Versión `0.1.0-alpha.1`. Licencia MIT, como el resto del repositorio.

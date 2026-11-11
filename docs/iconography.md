# Iconografía

**Estado: 255 iconos funcionales incluidos en `@nodalia/icons`.** Son dibujos originales de este proyecto (MIT). No se incluyen fuentes ni imágenes de terceros.

## El conjunto

`@nodalia/icons` está pensado para que cualquier app de Nodalia encuentre lo que necesita sin dibujar iconos propios:

| Grupo | Para qué |
| --- | --- |
| `general` | Navegación, estados, archivos, ajustes, cuenta, sistema |
| `edit`, `files`, `run`, `view` | Acciones comunes de edición, archivo, ejecución y vista |
| `text` | Formato de texto, párrafo, tablas, comentarios (Letra, Pliego) |
| `data` | Hojas de cálculo y datos (Celdas) |
| `slides` | Presentaciones (Lámina) |
| `design` | Dibujo vectorial e imagen: pluma, pincel, alinear, transformar (Pluma, Lienzo) |
| `photo` | Revelado: exposición, contraste, temperatura (Luz) |
| `media` | Reproducción, audio, vídeo y línea de tiempo (Vinilo, Toma, Pulso) |
| `mail` | Correo (Buzón) |
| `sketch`, `model` | Croquis y modelado 3D (Forja, Torno, Vortia) |
| `simulation`, `results` | Simulación y resultados (Vortia) |
| `document` | PDF y maquetación (Papel, Pliego) |

Los 55 primeros se han usado en Vortia 0.5.0. Los demás están dibujados con las mismas normas y revisados en hoja, pero **ninguna app los ha usado todavía**. Un icono que falte se añade aquí, no en la app.

| Salida | Para qué |
| --- | --- |
| `dist/svg/<nombre>.svg` | En `currentColor`: web y todo lo que herede el color del texto |
| `dist/mono/<nombre>.svg` | En negro: toolkits que tiñen por opacidad (motores de iconos de Qt, texturas en egui) |
| `dist/sprite.svg` | Todos como `<symbol id="ndl-<nombre>">` para `<use href>` |
| `dist/icons.json` | Nombres y grupos |
| `dist/preview.html` | Página con todos los iconos por grupo, en claro y oscuro |

Para añadir o cambiar un icono se edita `packages/icons/src/icons.mjs` y se regenera; un SVG generado no se retoca.

## Cuatro categorías que no se mezclan

| Categoría | Qué es | Quién la define |
| --- | --- | --- |
| Iconos funcionales (simbólicos) | acciones y estados dentro de la interfaz (guardar, deshacer, capa visible…) | NDL y el sistema |
| Iconos de aplicación | identidad de cada app en el lanzador y el dock | cada app, con la marca Nodalia |
| Iconos de tipo de archivo | formatos propios (`.forja`, etc.) | cada app |
| Cursores | puntero del sistema y de herramientas | el tema de cursores del sistema; las apps solo definen cursores de herramienta propios |

## GTK: el tema de iconos Nodalia

Una app de Nodalia se reconoce también por sus iconos, así que en GTK los iconos funcionales son los de NDL, no los del sistema. `nodalia-gtk` instala al iniciarse un tema de iconos llamado **Nodalia** y lo hace el de la app:

- Cada icono de `@nodalia/icons` está como `ndl-<nombre>-symbolic`.
- Los nombres estándar que piden las apps y el propio GTK (`window-close-symbolic`, `pan-down-symbolic`, `media-playback-start-symbolic`…) responden con el dibujo de NDL. La tabla está en [`packages/gtk/icon-aliases.json`](../packages/gtk/icon-aliases.json).
- Un nombre que NDL no dibuja cae al tema que tenía el sistema, luego a Adwaita y a hicolor (`Inherits`).

Por eso una app GTK sigue pidiendo nombres estándar: no hay que tocar cada llamada para que sus iconos sean los de NDL. Para un icono de NDL sin nombre estándar se pide `ndl-<nombre>-symbolic`.

En GTK los iconos van en el estilo **bold** de NDL: trazo de 2,25 y, en los iconos que lo indican con `fill` en `icons.mjs`, formas sólidas trazadas con el mismo color para que conserven las esquinas redondas (reproducir, pausa, estrella, nota musical, disco, persona, altavoz…). Es lo que da a las apps de Nodalia su carácter; el estilo regular (1,75, solo contornos) queda para documentos y herramientas densas. `@nodalia/icons/bold` produce el cuerpo bold de cada icono.

Los SVG para GTK se generan desde `icons.mjs` con las clases de recoloreado de GTK (`foreground-stroke`, `foreground-fill`, `transparent-fill`), de modo que el trazo toma el color del texto. El tema se escribe en la caché del usuario, porque GTK solo lee temas de iconos desde directorios.

Los iconos de aplicación y los cursores siguen siendo del sistema: Nodalix OS conserva su tema de iconos de aplicación y su tema de cursores.

## Reglas de dibujo

- Rejilla de 24 px con 3 px de margen libre; coordenadas en múltiplos de 0,25 px.
- Trazo de 1,75 px, extremos y uniones redondeados, siempre `currentColor`: el icono hereda el color del texto y el contraste lo gobierna el token.
- Solo `path`, `circle` y `rect`. Sin transformaciones, clases ni estilos en línea.
- En el estilo bold, `fill` lista los elementos que se rellenan; un icono puede dar un cuerpo `bold` propio cuando rellenar sus formas no se lee (un disco necesita su agujero).
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

Los tokens nombran `Inter` y `Sora`. Se distribuyen sin modificar en `packages/fonts/` con su licencia OFL-1.1 (ver su README); `nodalia-gtk` las lleva dentro y las registra para la app.

## Pendiente

- Revisar los iconos nuevos en contexto, dentro de cada app.
- Mostrar los iconos en el catálogo.
- Pipeline que convierta SVG a recurso Qt (`qrc`) sin copiar a mano. GTK ya tiene el suyo (el tema Nodalia) y la web el *sprite*.

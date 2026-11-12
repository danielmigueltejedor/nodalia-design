# Fondo de la ventana del DMG de macOS

`draw_background.py` dibuja, para cada app Nodalia, el fondo que muestra Finder al abrir su DMG.
Todas comparten la composición, así que se reconocen como familia, y cada una tiene un dibujo
propio que dice qué hace la app:

| App | Isla de la izquierda | Flecha entre los iconos |
|---|---|---|
| Celdas | cuadrícula con una selección y su tirador | fila de celdas |
| Forja | escuadra, arco y cota sobre papel milimetrado | línea de cota |
| Lámina | diapositivas apiladas con gráfico | miniaturas que crecen |
| Letra | «Aa», líneas de texto y cursor | puntos guía y cursor |
| Lienzo | capas con una pincelada y un pincel | pincelada |
| Luz | diafragma e histograma | deslizador |
| Papel | página con subrayado, firma y sello | marcador |
| Pliego | pliego de dos páginas y retícula de columnas | marcos enlazados |
| Pluma | curva de Bézier con manejadores y plumilla | trazado |
| Pulso | editor de curvas y fotogramas clave | fotogramas clave |
| Toma | visor, pistas de vídeo y forma de onda | fotogramas y «play» |
| Torno | pieza torneada con su eje, giro de revolución y cota | eje de revolución |
| Vinilo | disco con su brazo | flecha en el color de la app |
| Vortia | perfil alar con líneas de corriente | línea de corriente |

Reglas: tintas planas, sin degradados ni brillos; la isla derecha es blanca porque Finder pinta
las etiquetas de los iconos en negro también en modo oscuro; nada dentro de las cajas de 128 pt
de los iconos ni bajo sus etiquetas. El texto es Sora contorneada, así que el SVG se renderiza
igual en cualquier máquina sin fuentes instaladas.

## Uso

```sh
pip install fonttools
python3 tools/dmg-background/draw_background.py forja ../forja/packaging/macos/dmg/background.svg
python3 tools/dmg-background/draw_background.py --all /tmp/fondos   # todas
```

Cada app guarda el SVG generado en su repositorio y lo convierte en su propio formato (TIFF
HiDPI en las apps con `packaging/macos/dmg/generate.py`, PNG 1x/2x en Vinilo). Para cambiar un
dibujo se edita este script y se regenera, no el SVG.

# La identidad de Nodalia, para portarla a otro toolkit

Vinilo 2.0 (GTK) es la primera app con la identidad completa. Esta nota lista lo que otra app (Vortia en Qt, la web) tiene que replicar para ser de la misma familia. Los números salen de los tokens; aquí solo se dice dónde va cada uno.

## Forma

| Qué | Cómo | Token |
| --- | --- | --- |
| Marco de la ventana | El fondo de la ventana es el marco; nada se dibuja directamente sobre él salvo las islas | `color.surface.window` |
| Islas | Barra lateral, contenido y reproductor/barra inferior son superficies planas redondeadas, separadas 8 px entre sí y del borde | `color.surface.panel`, `component.panel.radius` (16), `component.window.gap` (8) |
| Ventana | Radio 20 | `component.window.radius` |
| Controles | Radio 10 (filas de menú, filas de la barra lateral, tooltips) | `component.control.radius` |
| Botones, búsqueda, *chips* | Píldoras | `component.button.radius`, `component.field.radius` |
| Sombras | Solo en lo que flota: menús, diálogos, lecturas en pantalla. Nada de degradados ni brillos | `elevation.overlay` |

## Color

- El acento (cian) se reserva para el progreso, el foco y la acción principal de una vista (`Reproducir`).
- La fila activa de la barra lateral se **invierte**: fondo del color del texto, texto del color del marco.
- El botón de reproducir es un círculo sólido en el color del texto.
- Bajo el puntero, cerrar se vuelve coral (`component.window.close`), minimizar amarillo anaranjado (glifo `component.window.minimize`, fondo `component.window.minimize-tint`) y maximizar cian (glifo `component.window.maximize`, fondo `component.window.maximize-tint`).
- Una acción peligrosa en un menú (cerrar sesión) va en `color.status.danger.fg`.

## Tipografía

| Qué | Cómo |
| --- | --- |
| Título de vista | Sora 800, 44 px, interletrado −0,04 em |
| Etiqueta de sección | 10,5 px, 700, versalitas, interletrado 0,13 em, `color.text.tertiary` |
| Título de lo que suena | 700 |
| Nombre en una lista | 600 |
| Fila de la barra lateral | 500; 700 la activa |
| Número junto a una etiqueta | `font-size.small`, cifras tabulares, `color.text.tertiary` |

## Iconos

- Estilo **bold**: trazo 2,25 y, en los que lo listan (`fill` en `icons.mjs`), formas sólidas con el mismo trazo para que las esquinas queden redondas. `@nodalia/icons/bold` da el cuerpo de cada icono.
- Todos los iconos de la interfaz son de NDL, también los que el toolkit dibuja por su cuenta (flechas, marcas de verificación, controles de ventana).
- Cada entrada de un menú lleva su icono.

## Componentes

| Componente | Comportamiento |
| --- | --- |
| Controles de ventana | Una cápsula de la altura de un botón de la cabecera con tres píldoras dentro; glifos `window-minimize`, `window-maximize`/`window-restore` (dos esquinas en diagonal: hacia fuera para maximizar, hacia el centro para restaurar), `window-close`. Círculos de 28 px, centrados en la cabecera, nunca estirados |
| Menús y *popovers* | Isla elevada, radio 16, borde de 1 px, sombra; icono en cada fila y atajo a la derecha |
| Lectura de volumen | Isla elevada con borde y sombra; barra gruesa en el color del texto |
| Ventana emergente de nivel (volumen) | Solo el deslizador vertical grueso, sin botones de paso |
| Hoja inferior (reproductor) | Arrastrar la barra hacia arriba la abre siguiendo al puntero; abierta, arrastrar hacia abajo la sigue. Al soltar, se abre o se cierra si recorrió un tercio de su altura; si no, vuelve |
| Diálogos | Isla, radio 16; Acerca de con el icono de la app y el nombre en Sora |
| Título de vista | Cada vista de lista abre con su nombre grande |
| Barra lateral | Etiquetas de sección en versalitas, icono y nombre en cada fila, número discreto a la derecha |

## Controles de ventana del sistema

Los controles de Nodalia existen solo en Linux. En macOS y Windows las apps Nodalia usan siempre los controles nativos del sistema, sin opción de cambiarlos. En Linux son los de serie, salvo que se fuerce otro: cada app deja elegir los del escritorio (GNOME, KDE…) en Preferencias y el cambio se aplica al reiniciar.

- GTK: `nodalia_gtk::set_window_controls(WindowControls::System)` antes de `nodalia_gtk::init()`. Los botones siguen la disposición del escritorio y los glifos de su tema de iconos, planos, sin los círculos ni los tintes de Nodalia. GTK 4 no admite decoraciones del servidor, así que la ventana sigue dibujando su cabecera.
- Qt: la ventana devuelve el marco al escritorio (decoración del servidor en KDE, la del tema de Qt en GNOME). Los menús siguen yendo a la barra superior si la hay; si no, a la barra de menús de la ventana. En Vortia es `ui/window-controls=system` y la variable `VORTIA_NATIVE_TITLEBAR=1` lo fuerza para una ejecución.

## Menú global

En Linux, cada app exporta su menú a la barra superior del shell (`org.gtk.Menus` / `org.gtk.Actions`): un menú con el nombre de la app (Acerca de, Preferencias, Atajos, Cerrar sesión, Salir) y luego los de su trabajo (Reproducción, Ver, Ventana, Ayuda). El menú principal dentro de la ventana se mantiene. En GTK lo hace `nodalia_gtk::global_menu` y la barra lo lee por Mutter (`gtk_menubar_object_path`), también en Wayland.

En Qt hay que ayudar: Qt registra su barra de menús en `com.canonical.AppMenu.Registrar` solo en X11. En Wayland la app exporta su `QMenuBar` por `com.canonical.dbusmenu` en una ruta propia (no `/MenuBar/N`, que Qt ya usa sin registrar) y la registra con el **pid del proceso** como id de ventana; la barra de Nodalix la empareja con la ventana por ese pid. La implementación de referencia es `ndl::exportGlobalMenu` en Vortia (`src/ui/src/GlobalMenu.cpp`): unos cientos de líneas sobre la API pública de QtDBus, sin cabeceras privadas.

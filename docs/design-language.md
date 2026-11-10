# Lenguaje de diseño

NDL describe cómo se ve y cómo se comporta una app de Nodalia. Es una identidad propia, sobria y pensada para herramientas de trabajo largo (modelado, edición, escritura, ingeniería).

## Principios

1. **Identidad propia.** Un acento cian, neutros ligeramente fríos, geometría contenida. No imita a ninguna plataforma ni a otros sistemas de diseño.
2. **Minimalismo funcional.** Cada elemento visual tiene una función. Si se quita y nadie lo echa en falta, sobraba.
3. **Claridad.** Una jerarquía legible a primera vista: superficies, bordes y peso tipográfico antes que adornos.
4. **Precisión.** Densidad adecuada a herramientas profesionales; campos numéricos con unidades; foco siempre visible.
5. **Fluidez.** Transiciones cortas (100–240 ms) que explican un cambio de estado, no lo decoran. Todo se desactiva con movimiento reducido.
6. **Accesibilidad.** Contraste verificado, foco visible, objetivos de pulsación de 24 px o más, teclado completo, alto contraste, escala de texto. No es una fase posterior.
7. **Adaptabilidad.** Claro, oscuro, sistema, alto contraste, tres densidades, escalado.
8. **Coherencia.** Un único conjunto de tokens para web, GTK y Qt.
9. **Integración nativa.** En GTK, NDL se apoya en libadwaita y no la sustituye; en cada escritorio respeta el comportamiento del sistema (preferencias de tema, contraste y movimiento).

## Lo que NDL evita

- Transparencias y desenfoques decorativos.
- Sombras que no expresan elevación real (solo `raised` y `overlay`).
- Degradados decorativos.
- Mayúsculas con espaciado, bordes dobles, relieves.
- Color como único portador de significado.

## Color

El acento (`color.accent.*`) se reserva para la acción principal, la selección y el foco. Los estados de estado (`success`, `warning`, `danger`, `info`) llevan siempre texto o icono además del color. Las superficies se apilan: ventana → panel → elevada → superpuesta.

Correcto: un botón primario por vista; selección en un árbol con fondo `accent-subtle` y marca de selección.
Incorrecto: colorear en acento todos los iconos de una barra; usar rojo para «cerrado» sin texto.

## Tipografía

Inter (interfaz) y Sora (títulos de pantalla) si están instaladas, con la pila del sistema como respaldo para que la app se vea nativa en cada escritorio; NDL no incluye las fuentes. Cuerpo de 13 px. Se usan pocos estilos y el peso, no el color, marca la jerarquía. Las cifras de campos numéricos usan numerales tabulares.

## Forma y espacio

Radios pequeños, bordes de 1 px, espaciado en una escala de 4 px. Los controles comparten altura según densidad para que alinear una fila de controles sea trivial.

## Foco

Anillo de 2 px en `border-focus` con desplazamiento; nunca se elimina `outline`. Un test comprueba que cada componente interactivo tiene estilo de foco por teclado.

## Movimiento

| Rol | Duración | Uso |
| --- | --- | --- |
| feedback | 100 ms | hover, pulsación |
| state | 160 ms | cambio de estado de un control |
| panel | 240 ms | abrir/cerrar panel |
| enter / exit | 160–240 ms | diálogos, toasts |

Con `prefers-reduced-motion: reduce` (o `data-motion="reduced"`) todas valen 0 ms. Ninguna información depende de una animación.

## Apariencia

`setAppearance({ theme, contrast, density, motion })` (web) o las propiedades de `NodaliaTheme` (Qt) cambian modo, contraste, densidad y movimiento en caliente. Sin ajustes explícitos se siguen las preferencias del sistema.

## Cómo comprobar que algo respeta NDL

- Todos los colores, tamaños, radios y duraciones salen de tokens (el linter lo exige para el CSS del paquete).
- Se puede usar con teclado y con el foco siempre visible.
- Funciona en los cuatro modos y las tres densidades sin cambios de maquetación.
- Con movimiento reducido no pierde información.

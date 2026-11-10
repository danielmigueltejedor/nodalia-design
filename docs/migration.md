# Adopción gradual en una app existente

NDL está pensado para adoptarse por capas, sin reescribir la interfaz de golpe y sin tocar el resto del ecosistema. Este repositorio **no modifica ninguna otra app**: cada app decide cuándo y cuánto adopta.

## Principios

- **Fija la versión.** Cada app depende de una versión exacta (`0.1.0-alpha.0`, no `^0.1`). Mientras NDL esté en alfa, cualquier versión puede cambiar nombres de token.
- **Empieza por el color.** Es lo que da coherencia visual con menos riesgo.
- **No copies ficheros.** Se consume como paquete; si falta algo, se pide un token, no se duplica un valor.
- **Un cambio visible = una PR de la app**, con captura antes/después y comprobación de contraste.

## Pasos recomendados

1. **Elige la integración** según la tecnología de la app: [web](integration/web.md), [GTK](integration/gtk.md), [QML](integration/qml.md), [egui](integration/egui.md) u [otros](integration/other.md).
2. **Cuadro de equivalencias.** Lista los colores, tamaños y radios que la app usa hoy y asígnales un token semántico. Lo que no encaje señala un token que falta o un valor propio que quizá no deba existir.
3. **Sustituye valores, no estructura.** Primero los colores y la tipografía; después espaciado y radios; al final los componentes.
4. **Apariencia.** Conecta tema, contraste, densidad y movimiento a las preferencias del sistema (y a las de la app si las tiene).
5. **Comprueba.** Contraste, foco visible, navegación por teclado, modo oscuro, alto contraste y movimiento reducido.
6. **Retira lo antiguo** solo cuando nada lo use.

## Orden sugerido de apps

1. Una app con GTK/libadwaita (por ejemplo, Brújula o Vinilo): la integración es solo cargar una hoja de estilos, así que da feedback rápido sobre el adaptador GTK, que es el menos verificado.
2. Una app web (Nodalia Web), que valida el paquete `@nodalia/web` en producción.
3. Las apps egui, con el crate `nodalia-theme` ([integración](integration/egui.md)).

La elección de la «primera app» es de Daniel y del equipo; esto es solo una recomendación basada en el riesgo.

## Comunicar cambios que rompen

Mientras la versión sea `0.x`, un cambio que rompe se anota en `CHANGELOG.md` bajo **Changed** o **Removed** con el nombre antiguo, el nuevo y cómo migrar. Los tokens renombrados se mantienen como alias deprecado durante al menos una versión menor cuando sea posible. Ver [versionado](versioning.md).

## Si una app necesita desviarse

Una desviación puntual es legítima (una herramienta con un color propio, por ejemplo). Debe:

- usar un token semántico propio de la app, no un literal suelto;
- cumplir el mismo contraste;
- documentarse en la app, y, si se repite en dos apps, proponerse como token de NDL.

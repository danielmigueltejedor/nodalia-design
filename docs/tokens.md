# Tokens

Los tokens siguen el formato abierto [DTCG](https://design-tokens.github.io/community-group/format/). Están en `packages/tokens/src/`.

## Tipos usados

`color`, `dimension`, `duration`, `cubicBezier`, `fontFamily`, `fontWeight`, `shadow`, `typography`, `transition`, `number`.

## Modos

Un token semántico de color puede declarar valores por modo en `$extensions`:

| Modo | Cuándo | Respaldo |
| --- | --- | --- |
| `light` | valor base | — |
| `dark` | `prefers-color-scheme: dark` o `data-theme="dark"` | base |
| `light-hc` | `prefers-contrast: more` o `data-contrast="more"` | base |
| `dark-hc` | ambos | `dark`, luego base |

El generador emite bloques completos para cada modo (no deltas), con selectores tanto de preferencia del sistema como de atributo explícito. El atributo explícito gana sobre la preferencia del sistema; esa precedencia está cubierta por la matriz de 12 combinaciones de `tests/browser/appearance.test.mjs`.

## Colores

- Neutros 0–1000 con ligero tono azulado. Acento cian (`a300` `#8ADBE0` es el color de marca; `a600` `#12838C` es el acento por defecto en claro).
- El contraste se elige por par, no por escala: ver `CONTRAST_PAIRS` en `packages/tokens/lib/validate.mjs`. Texto normal 4,5:1 y componentes 3:1; en alto contraste 7:1 y 4,5:1. `npm run validate` falla si un par no cumple.
- Los estados nunca se distinguen solo por color: los componentes añaden forma o texto (icono de marca, subrayado, etiqueta).

## Acento por app

Cada app puede tener su propio acento, el color de su icono, sin salirse del sistema. La app solo aporta **un color clave** en `packages/tokens/src/accents.json`; NDL deriva de él la rampa completa:

- **Mismo tono** que el color clave.
- **Misma luminancia, paso a paso, que la rampa por defecto.** El contraste depende solo de la luminancia, así que cualquier acento cumple exactamente el contraste que cumple el cian por defecto. No hay que comprobarlo a mano: la compilación valida cada acento en los cuatro modos y falla si alguno no pasa.
- **Saturación relativa a la del color clave.** Un color clave gris (Papel) da un acento gris; uno intenso da un acento intenso.

Un acento solo cambia `color.accent.*`, `color.selection.bg`, `color.border.focus` y `color.text.link`. Superficies, texto, bordes y estados son iguales en todas las apps: eso es lo que las hace reconocerse como familia.

| Acento | Color clave | | Acento | Color clave |
| --- | --- | --- | --- | --- |
| `nodalia` (por defecto) | cian de marca | | `pliego` | `#3340D0` |
| `nodalix` | `#0A93B8` | | `pluma` | `#EE5A14` |
| `vortia` | `#0B55DB` | | `lienzo` | `#EE3B6B` |
| `brujula` | `#0C9A58` | | `luz` | `#F59300` |
| `vinilo` | `#5A27D6` | | `toma` | `#C2188F` |
| `buzon` | `#D4241B` | | `pulso` | `#6FB312` |
| `letra` | `#1C86D2` | | `forja` | `#96501F` |
| `celdas` | `#1D6B3B` | | `torno` | `#8C7419` |
| `lamina` | `#7C1F41` | | `papel` | `#68778F` |

Cómo se activa: `<html data-accent="vortia">` en web, `Accent::Vortia` en Rust, el grupo `accent` de `nodalia-qt.json` en Qt Widgets. GTK y Qt Quick no tienen acentos todavía.

Añadir una app: una línea en `accents.json` y `npm run build`.

## Tipografía

Familia de interfaz `Inter` con respaldo en la pila del sistema (`system-ui`, Segoe UI, Cantarell, Roboto…); familia de títulos `Sora` con respaldo en `Inter` y el sistema. **No se incluye ninguna fuente**: si Inter o Sora no están instaladas se usa el respaldo, así que el aspecto exacto depende de la máquina. Cuerpo 13 px (escritorio denso); estilos `display`, `headline`, `title`, `large`, `label`, `small`, `caption`, `code`.

## Espaciado, radios, bordes

Escala de espaciado de 0 a 48 px (`none`, `xxs`…`xxxl`). Radios pequeños y sobrios. Bordes `thin` (1 px) y `thick` (2 px; usado en foco).

## Elevación

Solo dos niveles (`raised`, `overlay`), con sombras sutiles. La jerarquía se consigue ante todo con superficies y bordes, no con sombras.

## Movimiento

Duraciones `instant`, `fast` (100 ms), `normal` (160 ms), `slow` (240 ms), `slower` (360 ms). Curvas `standard`, `enter`, `exit`. Roles semánticos: `feedback`, `state`, `panel`, `enter`, `exit`. Con movimiento reducido todas las duraciones valen 0 ms.

## Densidad y tamaños de control

| Densidad | Altura de control | Relleno X |
| --- | --- | --- |
| compact | 24 | 8 |
| regular | 32 | 12 |
| comfortable | 40 | 16 |

Los elementos pequeños pero interactivos (casillas, conmutadores) mantienen un área de pulsación de 24 px como mínimo (WCAG 2.2, 2.5.8).

## Puntos de corte

Definidos como tokens (`breakpoint.*`) para uso de las apps; CSS no admite variables en `@media`, así que no se inyectan en las consultas del propio paquete.

## Cómo añadir o cambiar un token

1. Edita el fichero de la capa correcta.
2. `npm run validate -w @nodalia/tokens`.
3. `npm run build && npm test`.
4. Si el cambio altera un valor existente, anótalo en `CHANGELOG.md` (ver [versionado](versioning.md)).

## Salidas

- `@nodalia/tokens/css` → variables `--ndl-*` con modos, densidades y movimiento.
- `@nodalia/tokens` → `tokens.json` (árbol DTCG fusionado).
- `@nodalia/tokens/resolved` → valores finales por modo, para generadores de otras tecnologías.
- `@nodalia/tokens/lib` → API para escribir un generador propio (`loadTokens`, `resolve`, `contrast`…).

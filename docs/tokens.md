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

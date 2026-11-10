# Otros toolkits

La mayoría de las apps de Nodalia usan **egui (Rust)**, que ya tiene adaptador: ver [egui](egui.md). Qt Widgets también: ver [Qt Widgets](qt-widgets.md). Para el resto de toolkits se pueden consumir los tokens hoy y generar el adaptador después.

## Opción A: usar los tokens resueltos

`@nodalia/tokens/resolved` es un JSON con el valor final de cada token por modo y densidad. Un script de build de la app puede leerlo y emitir constantes:

```js
import { loadTokens, resolve } from '@nodalia/tokens/lib';
```

`packages/qml/scripts/build.mjs`, `packages/gtk/scripts/build.mjs` y `packages/egui/scripts/build.mjs` son ejemplos cortos de generadores completos (menos de 100 líneas cada uno).

## Cómo proponer un adaptador nuevo

1. Que consuma `@nodalia/tokens/lib` o `resolved.json`, nunca valores copiados.
2. Que sea determinista (misma entrada, mismos bytes) para pasar `check:repro`.
3. Con tests de contenido y de contraste de los pares críticos.
4. Documentado aquí con sus límites y lo que **no** esté verificado.

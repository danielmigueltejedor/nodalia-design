# Otros toolkits (egui, Qt Widgets, nativos…)

La mayoría de las apps de Nodalia usan **egui (Rust)**; Vortia y Lienzo, **Qt 6 Widgets**. Ninguna tiene aún adaptador. Se pueden consumir los tokens hoy y generar el adaptador después.

## Opción A: usar los tokens resueltos

`@nodalia/tokens/resolved` es un JSON con el valor final de cada token por modo y densidad. Un script de build de la app puede leerlo y emitir constantes:

```js
import { loadTokens, resolve } from '@nodalia/tokens/lib';
```

`packages/qml/scripts/build.mjs` y `packages/gtk/scripts/build.mjs` son ejemplos cortos de generadores completos (menos de 100 líneas cada uno).

## egui (pendiente: generador oficial)

Plan: un generador que produzca un módulo Rust con `const` de color por modo y una función que construya `egui::Visuals` y `egui::Style`. Hoy puede hacerse a mano desde `resolved.json`, con la advertencia de que habrá que migrar cuando salga el generador.

## Qt Widgets (pendiente: generador oficial)

Plan: `QPalette` por modo y una hoja QSS mínima para lo que la paleta no cubre.

## Cómo proponer un adaptador nuevo

1. Que consuma `@nodalia/tokens/lib` o `resolved.json`, nunca valores copiados.
2. Que sea determinista (misma entrada, mismos bytes) para pasar `check:repro`.
3. Con tests de contenido y de contraste de los pares críticos.
4. Documentado aquí con sus límites y lo que **no** esté verificado.

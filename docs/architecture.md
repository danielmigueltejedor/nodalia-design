# Arquitectura

NDL es un monorepo de npm workspaces. Cada paquete se versiona y se consume por separado; ninguna app copia ficheros a mano.

```
tokens (DTCG)  ──►  @nodalia/tokens  ──►  CSS de variables
                         │
                         ├──►  @nodalia/gtk   (hojas libadwaita)
                         ├──►  @nodalia/qml   (singleton Qt Quick)
                         ├──►  @nodalia/egui  (crate Rust nodalia-theme)
                         ├──►  @nodalia/qtwidgets (tokens y hojas de estilos Qt)
                         └──►  @nodalia/web   (componentes + comportamientos)
                                       │
                                       └──►  @nodalia/catalog (catálogo y demos, no se publica)
```

## Paquetes

| Paquete | Entrada | Salida |
| --- | --- | --- |
| `@nodalia/tokens` | `src/*.tokens.json` (DTCG) | `dist/tokens.css`, `tokens.json`, `resolved.json` |
| `@nodalia/gtk` | `mapping.json` + tokens resueltos | `dist/nodalia-{light,dark,light-hc,dark-hc}.css` |
| `@nodalia/qml` | tokens resueltos | `dist/Nodalia/NodaliaTheme.qml` + `qmldir` |
| `@nodalia/qtwidgets` | `src/nodalia.qss` + tokens resueltos | `dist/nodalia-qt.json`, `nodalia.qss`, `nodalia-<modo>-<densidad>.qss` |
| `@nodalia/egui` | tokens resueltos | `crate/src/tokens.rs` (guardado en el repositorio) dentro del crate `nodalia-theme` |
| `@nodalia/web` | `src/components.css`, `src/behaviors.mjs` | `dist/nodalia.css` (con capas), `dist/behaviors.mjs` |
| `@nodalia/catalog` | `src/` | sitio estático en `dist/` |

Dependencias: `gtk`, `qml`, `egui`, `qtwidgets` y `web` dependen solo de `tokens`. `tokens` no depende de nada. No hay dependencias de ejecución externas; las de desarrollo son `playwright-core` y `axe-core`, con versión exacta.

## Flujo de generación

1. `load` lee los cuatro ficheros de tokens y los fusiona.
2. `resolve` sigue alias, detecta ciclos y aplica la cadena de modos (`dark-hc → dark → base`, `light-hc → base`).
3. `validate` comprueba tipos, capas (un componente no referencia primitivos), alias rotos y contraste.
4. Los renderizadores producen CSS, hojas GTK y QML. Son funciones puras: misma entrada, misma salida byte a byte (lo comprueba `check:repro`).

## Capas de tokens

- **Primitivos** (`primitive`): valores crudos y sin significado: paleta neutra 0–1000, acento, verde, ámbar, rojo, azul; espaciado, radios, tipografía, duraciones, curvas, puntos de corte.
- **Semánticos** (`semantic`): significado (`color.surface.panel`, `color.text.secondary`, `color.status.danger.fg`, roles de movimiento). Aquí viven los modos claro/oscuro/contraste.
- **Densidad** (`density`): alturas, rellenos y separaciones por densidad.
- **Componente** (`component`): decisiones concretas de cada control (`button`, `field`, `switch`, `slider`…) referenciando semánticos.

Las apps consumen semánticos y de componente. Referenciar primitivos desde código de app es una señal de que falta un token semántico.

## Web

El CSS usa `@layer ndl.tokens, ndl.components;`, de modo que una app puede sobrescribir cualquier regla sin `!important` ni especificidad. Todo selector empieza por `.ndl-`. Las variantes se expresan con atributos `data-*` (`data-variant`, `data-size`, `data-state`) y los estados reales (`:hover`, `:focus-visible`, `:disabled`, `aria-*`).

Los comportamientos (`behaviors.mjs`) son funciones independientes que añaden roles ARIA y teclado a un marcado dado. No crean DOM ni tienen estado global.

## GTK

Solo se generan colores con nombre públicos de libadwaita. No se usan selectores de widgets ni nodos internos: son API privada y cambian entre versiones. La forma, el espaciado y la tipografía los gobierna libadwaita, y NDL se ocupa del color.

## Qt Quick

Un único singleton `NodaliaTheme` con propiedades `mode`, `density` y `reducedMotion` y propiedades de solo lectura derivadas. El módulo no contiene controles; las apps los estilizan con estos valores.

## Qué queda fuera

NDL no incluye motores de CAD, render, simulación ni lógica de documentos. Los patrones profesionales (visor, capas, árbol de operaciones, inspector paramétrico, historial) son marcado y estilo; los datos y el comportamiento de dominio son de cada app.

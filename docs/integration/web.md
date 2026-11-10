# Integración web

```bash
npm install @nodalia/web@0.1.0-alpha.1   # versión exacta
```

Los paquetes no están publicados en un registro todavía. Mientras tanto, `npm pack` en este repositorio genera los tarballs (los adjunta el flujo de release) y se instalan con `npm install ./nodalia-web-0.1.0-alpha.1.tgz`.

## CSS

```css
@import '@nodalia/web/nodalia.css';
```

o, con un bundler, `import '@nodalia/web/nodalia.css'`. Un único fichero contiene los tokens (`@layer ndl.tokens`) y los componentes (`@layer ndl.components`). Tus estilos, sin capa o en una capa posterior, ganan sin `!important`.

## Acento de la app

```html
<html data-accent="vortia">
```

Cambia la rampa de acento y, con ella, botón primario, selección, foco y enlaces. Sin el atributo se usa el cian por defecto. Los nombres están en [tokens](../tokens.md#acento-por-app).

## Marcado

```html
<body class="ndl-app">
  <button class="ndl-button" data-variant="primary">Guardar</button>
  <label class="ndl-field">
    <span>Nombre</span>
    <input class="ndl-input">
  </label>
</body>
```

Todos los componentes y sus variantes están en el [catálogo](../components.md) y se ven en `npm run serve -w @nodalia/catalog`.

## Comportamientos

Los comportamientos añaden roles ARIA y teclado a marcado existente:

```js
import { tabs, tree, splitter, toolbar, slider, contextMenu, tooltips, setAppearance } from '@nodalia/web/behaviors';

tabs(document.querySelector('[role=tablist]'));
tooltips();                                   // todos los [data-ndl-tooltip]
setAppearance({ theme: 'dark', contrast: 'more', density: 'compact' });
```

`setAppearance` establece `data-theme`, `data-contrast`, `data-density` y `data-motion` en `<html>`. Sin ellos, se siguen las preferencias del sistema.

## Reglas

- Usa solo `.ndl-*` y `data-*` documentados. No dependas de la estructura interna.
- No fijes colores ni tamaños: usa `var(--ndl-…)`.
- Mantén las etiquetas accesibles: NDL no puede inventar el nombre de un control.
- Prueba tus pantallas con axe-core (la configuración de `tests/browser/a11y.test.mjs` es un punto de partida).

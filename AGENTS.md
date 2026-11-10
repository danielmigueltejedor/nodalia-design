# Instrucciones para agentes y colaboradores

Este repositorio es la infraestructura visual de Nodalia. Una regresión aquí llega a todas las aplicaciones, así que se trabaja con cuidado y se informa solo de lo que se ha ejecutado.

## Reglas que no se rompen

1. **Los tokens son la única fuente de valores visuales.** Ningún color, tamaño, radio, duración ni sombra literal en `packages/web/src/components.css`; `npm run validate` lo comprueba. Si falta un valor, se añade un token, no un literal.
2. **Capas de tokens.** Primitivos (`palette`, `space`, `radius`…) solo contienen literales. Los semánticos (`color.*`, `elevation`, `typography`, `motion`, `size`) solo referencian primitivos. Los de componente (`component.*`) solo referencian primitivos o semánticos, nunca otros de componente.
3. **Contraste y accesibilidad se verifican, no se afirman.** Cada color semántico nuevo entra en los pares de `packages/tokens/lib/validate.mjs` y debe pasar en los cuatro modos. No se baja un umbral ni se excluye una regla de axe para que pase CI. Si una regla es un falso positivo, se documenta el motivo junto a la exclusión.
4. **Nada de CSS que dependa de internos.** El adaptador GTK estiliza solo nodos CSS documentados en la referencia de widgets de GTK 4, clases de estilo públicas (`.flat`, `.suggested-action`, `.boxed-list`…) y los nodos `ndl-*` del crate `nodalia-gtk`; cada valor sale de una variable `--ndl-*`, nunca un literal (los tests de `@nodalia/gtk` lo comprueban). El adaptador Qt no redefine controles nativos globalmente.
5. **Sin recursos de terceros.** No se añaden fuentes, iconos, imágenes ni código sin licencia compatible, registrada en `docs/iconography.md` o en un `THIRD_PARTY.md`. Las fuentes no se incluyen en este repositorio.
6. **Los ficheros generados no se editan.** `dist/` está ignorado; el cambio se hace en la fuente o en el generador.
7. **Builds reproducibles.** `npm run check:repro` debe pasar. Los generadores no usan fechas, orden de directorios no determinista ni aleatoriedad.
8. **Honestidad de estado.** Lo no verificado en un runtime real (GTK, Qt, Firefox, WebKit) se declara como tal en `docs/status.md`. No se añaden componentes vacíos ni adaptadores que aparenten estar terminados.
9. **CI y políticas.** Los workflows y este fichero se revisan siempre por una persona. No se desactivan comprobaciones para conseguir verde.
10. **`main` cambia mediante pull request.**

## Idioma

Documentación en español. Identificadores, API pública, texto del catálogo y mensajes de error en inglés.

## Flujo de trabajo

```bash
npm install && npm run build
npm run validate && npm test            # rápido, sin navegador
npm run test:browser                    # necesita Chromium
```

Cambiar un token: editar `packages/tokens/src/*.tokens.json`, ejecutar `npm run validate`. Cambiar un componente: editar `components.css` o `behaviors.mjs`, añadir su caso al catálogo y un test en `tests/browser/`. Ver [CONTRIBUTING.md](CONTRIBUTING.md).

## Mapa

- `packages/tokens/src/` fuentes DTCG · `lib/` carga, resolución por modo, validación, generador CSS.
- `packages/{gtk,qml,web}/` adaptadores; consumen `@nodalia/tokens/lib`, nunca los JSON directamente.
- `apps/catalog/` catálogo y demostraciones; se construye desde los paquetes.
- `tests/browser/` pruebas con Chromium (cascada de modos, axe, teclado, demostraciones).

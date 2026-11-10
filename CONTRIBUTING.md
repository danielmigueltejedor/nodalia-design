# Contribuir

Gracias por ayudar. NDL es compartido por todas las aplicaciones de Nodalia, así que se exige más rigor que en una app.

## Antes de empezar

1. Abre o comenta una incidencia que describa el problema o la necesidad desde la perspectiva de una aplicación que consume NDL.
2. Lee [AGENTS.md](AGENTS.md): las reglas valen igual para personas y agentes.
3. Una persona responde de cada pull request, también de los generados con ayuda de herramientas.

## Entorno

```bash
npm install
npm run build && npm run validate && npm test
npx playwright-core install chromium && npm run test:browser
npm run serve -w @nodalia/catalog
```

## Qué debe incluir un cambio

| Cambio | Obligatorio |
| --- | --- |
| Token nuevo o modificado | Pasa `npm run validate` en los cuatro modos. Los colores nuevos entran en los pares de contraste. Se documenta en `docs/tokens.md` si cambia el contrato. |
| Componente nuevo | Estados (reposo, hover, pulsado, foco, deshabilitado) y atributos ARIA; entrada en el catálogo con la rejilla de estados; test de navegador de su teclado; axe sin violaciones en claro, oscuro y alto contraste; fila en `docs/components.md`. |
| Cambio que rompa la API | Entrada en el CHANGELOG bajo «Cambiado» o «Eliminado» con la migración; sigue [docs/versioning.md](docs/versioning.md). |
| Corrección de un fallo | Un test que falla sin la corrección. |

## Criterios de revisión

- ¿Hay algún valor visual fuera de un token?
- ¿Se ve bien en los cuatro modos, en las tres densidades y a 150 % de escala?
- ¿Funciona solo con teclado y con movimiento reducido?
- ¿Es la solución más pequeña que resuelve el problema? No se aceptan abstracciones sin un segundo uso real ni componentes sin uso.
- ¿El estado en `docs/status.md` sigue siendo cierto?

Las puntuaciones de SonarQube o los comentarios automáticos de CodeRabbit son señales, no el criterio: un aviso se resuelve o se explica, nunca se silencia sin motivo.

## Commits y ramas

Ramas desde `main`, pull request con descripción que se entienda sin contexto externo. Mensajes de commit en imperativo y específicos. No se sube a `main` directamente.

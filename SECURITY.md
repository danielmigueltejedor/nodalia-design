# Política de seguridad

## Versiones con soporte

NDL está en fase alfa. Solo se corrigen vulnerabilidades en la última versión publicada y en `main`.

## Qué se considera una vulnerabilidad

NDL es CSS, JavaScript sin dependencias de ejecución y ficheros generados. Los casos relevantes son, por ejemplo: `behaviors.mjs` que inserte HTML a partir de datos no confiables (hoy asigna siempre `textContent`), una dependencia de desarrollo con una vulnerabilidad conocida que afecte a la cadena de compilación, o un workflow de CI/release con permisos excesivos.

## Cómo informar

No abras una incidencia pública. Usa los avisos de seguridad privados de GitHub del repositorio («Security» → «Report a vulnerability»). Incluye la versión, los pasos para reproducirlo y el impacto. Se acusa recibo en un plazo razonable y se coordina la divulgación una vez publicada la corrección.

## Cadena de suministro

- Sin dependencias de ejecución. Las de desarrollo (`playwright-core`, `axe-core`) están fijadas a versión exacta y el `package-lock.json` está versionado.
- El catálogo no carga recursos externos (comprobado por test).
- El workflow de release crea solo un borrador; una persona lo revisa y lo publica.

# Versionado y compatibilidad

## Esquema

[SemVer 2.0](https://semver.org/lang/es/). Los cuatro paquetes publicables (`tokens`, `web`, `gtk`, `qml`, `qtwidgets`) y el crate `nodalia-theme` comparten número de versión en esta etapa para evitar combinaciones ambiguas; el flujo de release comprueba que todos coinciden con la etiqueta.

- `0.1.0-alpha.N`: pre-release. Cualquier cosa puede cambiar.
- `0.x.0` (cuando haya beta/estable en `0.x`): un cambio de la `x` puede romper; el `CHANGELOG` lo dice.
- `1.0.0`: se congela la API pública descrita abajo.

No existe publicación estable automática. Una etiqueta `vX.Y.Z-algo` crea un **borrador** de release marcado como pre-release; publicarlo es una decisión manual.

## Qué es API pública

- Nombres de tokens (`color.text.primary` → `--ndl-color-text-primary`, `NodaliaTheme.colorTextPrimary`).
- Nombres de clases `.ndl-*` y atributos `data-*` documentados, y las funciones exportadas por `@nodalia/web/behaviors`.
- Nombres de ficheros generados (`nodalia-<modo>.css`, `NodaliaTheme.qml`) y las rutas de `exports`.
- La API de `@nodalia/tokens/lib`.

**No** es API pública: valores concretos de color/tamaño (pueden afinarse sin cambio mayor mientras mantengan contraste), clases sin documentar, estructura interna de `dist/` más allá de lo exportado.

## Qué es un cambio que rompe

- Renombrar o eliminar un token, una clase o una función.
- Cambiar el significado semántico de un token.
- Cambiar el marcado que un componente exige.

Un ajuste de valor que mantiene el contraste y el significado es compatible.

## Deprecación

1. Se marca como *Deprecated* en `CHANGELOG.md`, con el reemplazo.
2. El nombre antiguo sigue funcionando (alias) al menos una versión menor.
3. Se elimina en la siguiente versión mayor (o menor, mientras sea `0.x`) anotándolo en *Removed*.

## Compatibilidad soportada

| Plataforma | Versión mínima prevista | Verificado |
| --- | --- | --- |
| Node (herramientas) | 20 | Node 22 |
| Navegador (web) | Navegadores con capas CSS (`@layer`), propiedades lógicas y la propiedad `scale` (aprox. 2022 en adelante); no medido | Chromium |
| GTK / libadwaita | libadwaita 1.6 | **no** |
| Qt | Qt 6 (QtQuick) | **no** |

Las versiones mínimas de GTK y Qt son las que justifican las funciones usadas por los adaptadores; no se han comprobado en ejecución. Ver [estado](status.md).

## Reproducibilidad

Las salidas son deterministas: la misma entrada produce los mismos bytes. `npm run check:repro` compila dos veces desde cero y compara huellas SHA-256. Las dependencias están fijadas en `package-lock.json`.

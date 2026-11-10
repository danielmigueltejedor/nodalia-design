# Integración Qt Widgets

> **En uso en Vortia 0.5.0** (Qt 6, estilo Fusion, macOS, modos claro y oscuro, densidad regular). **Sin probar** en Linux ni Windows, con otros estilos base, ni en los modos de alto contraste. Los límites de abajo salieron de esa integración.

`@nodalia/qtwidgets` genera en `dist/`:

| Archivo | Para qué |
| --- | --- |
| `nodalia-qt.json` | Tokens en dos niveles (`grupo.nombre`): colores por modo, métricas por densidad, espaciado, radios, tipografía |
| `nodalia.qss` | Plantilla de hoja de estilos con marcadores `{{grupo.nombre}}` |
| `nodalia-<modo>-<densidad>.qss` | La plantilla ya rellena, para apps que no quieran rellenarla ellas |

## Uso mínimo

```cpp
app.setStyle(QStyleFactory::create("Fusion"));
QFile sheet(":/ndl/nodalia-dark-regular.qss");
sheet.open(QIODevice::ReadOnly);
app.setStyleSheet(QString::fromUtf8(sheet.readAll()));
```

Y una `QPalette` con los mismos tokens, porque el dibujo propio de la app (gráficas, lienzos) y parte de Fusion leen la paleta, no la hoja:

| `QPalette` | Token |
| --- | --- |
| `Window` | `color.surfaceWindow` |
| `Base` | `color.surfacePanel` |
| `AlternateBase` | `color.surfaceSunken` |
| `Button` | `color.controlBg` |
| `WindowText`, `Text`, `ButtonText` | `color.textPrimary` (desactivado: `color.textDisabled`) |
| `PlaceholderText` | `color.textTertiary` |
| `Midlight`, `Mid`, `Dark` | `color.borderSubtle`, `borderDefault`, `borderStrong` |
| `Highlight`, `HighlightedText` | `color.accentDefault`, `color.textOnAccent` |
| `Link` | `color.textLink` |
| `ToolTipBase`, `ToolTipText` | `color.surfaceInverse`, `color.textInverse` |

## Rellenar la plantilla en la app

Si la app quiere cambiar de modo o densidad en caliente, carga `nodalia-qt.json` y `nodalia.qss` y sustituye cada `{{grupo.nombre}}`: `color.*` se busca bajo el modo activo, `density.*` bajo la densidad activa y el resto directamente. La app añade después su propia hoja, con la misma sintaxis, para sus nombres de objeto.

Variantes de botón: `button->setProperty("ndlVariant", "primary" | "subtle" | "danger")`. El botón por defecto de un diálogo ya toma el acento.

## Lo que la app tiene que poner

- **Iconos de los subcontroles.** NDL no distribuye iconos: la app asigna `image:` a `QComboBox::down-arrow`, `QAbstractSpinBox::up-arrow` / `::down-arrow`, `QMenu::right-arrow`, `QTabBar::close-button` y `QCheckBox::indicator:checked`.
- **Color de esos iconos.** Una hoja de estilos no tiñe imágenes: un icono negro sigue negro en modo oscuro. Hace falta un archivo por modo, o generarlos al aplicar el tema (Vortia escribe copias teñidas en la caché).

## Límites encontrados en Qt

- **El texto de un `QSpinBox` ignora el relleno de la hoja**, tanto el del propio control como el de su `QLineEdit` interior. Solución: `spin->findChild<QLineEdit*>()->setTextMargins(izquierda, 0, 0, 0)` al pulir el widget.
- **Los `QSpinBox` salen unos píxeles más altos** que los `QComboBox` con el mismo `min-height`.
- **Los márgenes de un `QToolBar` no obedecen a la hoja**: vienen del estilo base y se restablecen en cada cambio de estilo. Para alinear una barra con un panel hay que fijarlos en código después de aplicar la hoja.
- **Sin anillo de foco desplazado.** El foco se marca con borde de 2 px en `color.borderFocus`; Qt no tiene `outline-offset`.
- **Sin sombras.** Las hojas de Qt no las dibujan; la elevación se expresa solo con superficie y borde.
- **Sin transiciones.** Los tokens de movimiento no tienen equivalente en hojas de estilos.
- La plantilla solo estiliza clases estándar. No hay componentes compuestos (inspector, árbol de operaciones…).

# Integración Qt Quick (QML)

> **Sin verificar con Qt.** El singleton se genera y se prueba estructuralmente. No se ha ejecutado en una app Qt.

> **Aplica solo a apps Qt Quick.** Vortia y Lienzo usan Qt Widgets, para los que este adaptador no sirve directamente; falta un generador de `QPalette`/QSS (ver [estado](../status.md)).

## Uso

Añade `node_modules/@nodalia/qml/dist` (o la copia generada en tu build) a la ruta de importación QML y:

```qml
import Nodalia 1.0

Rectangle {
    color: NodaliaTheme.colorSurfacePanel
    Text { color: NodaliaTheme.colorTextPrimary }
}
```

Enlaza el estado una vez, al arrancar:

```qml
Component.onCompleted: {
    NodaliaTheme.mode = Qt.styleHints.colorScheme === Qt.Dark ? "dark" : "light"   // Qt 6.5+
    NodaliaTheme.density = "regular"
}
```

Para alto contraste usa `"light-hc"` / `"dark-hc"`; el sistema no siempre expone esa preferencia a Qt, así que normalmente la decide la app.

## Qué contiene

- Propiedades de color por rol (`colorSurface…`, `colorText…`, `colorBorder…`, `colorAccent…`, `colorControl…`, `colorStatus…`) como `color`, derivadas del modo activo.
- Densidades (`controlHeight`, `gap`…) en el mapa `densities`.
- Duraciones (`durationFast`…, `motion*Duration`) y curvas de easing como arrays para `Easing.BezierSpline`; las duraciones valen 0 con `reducedMotion: true`.
- Los colores usan el formato `#AARRGGBB` de Qt.

## Limitaciones

- No incluye controles ni estilo para Qt Quick Controls; solo valores.
- No verificado en ejecución.

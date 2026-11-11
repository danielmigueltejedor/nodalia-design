#!/usr/bin/env python3
"""Loads every generated stylesheet into a real GTK 4 and fails on any parser error.

The unit tests check what the sheets say; this checks that GTK agrees. It needs PyGObject
with GTK 4 (and libadwaita, if installed, so its named colours are in play) and a display,
or GDK_BACKEND=broadway / a headless compositor (CI uses xvfb-run):

    python3 packages/gtk/tools/check-gtk.py

It also loads a probe sheet that reads every --ndl-* variable on a real widget, so a variable
that GTK cannot substitute into the property it is meant for shows up as an error here.
"""
import os
import pathlib
import re
import sys

import gi

# Xvfb has no GL, and GTK aborts trying to make a GL renderer; parsing needs none.
os.environ.setdefault("GSK_RENDERER", "cairo")
os.environ.setdefault("GDK_DISABLE", "gl,vulkan")

gi.require_version("Gtk", "4.0")
gi.require_version("Gdk", "4.0")
from gi.repository import Gdk, GLib, Gtk  # noqa: E402

try:
    gi.require_version("Adw", "1")
    from gi.repository import Adw  # noqa: E402
except (ImportError, ValueError):
    Adw = None

# The crate's copy: committed, and identical to dist/ (the unit tests check), so no build is needed.
DIST = pathlib.Path(__file__).resolve().parent.parent / "crate" / "src" / "css"

# Which property each kind of variable is probed with. A variable is only useful if GTK accepts
# it where an app would put it, so the probe uses the same property.
PROBES = [
    (re.compile(r"--ndl-(color|palette)-|--ndl-component-window-(close|minimize|maximize)"), "color"),
    (re.compile(r"--ndl-elevation-"), "box-shadow"),
    (re.compile(r"--ndl-(radius|component-.*radius)"), "border-radius"),
    (re.compile(r"--ndl-font-family-"), "font-family"),
    (re.compile(r"--ndl-font-size-|--ndl-font-line-height-"), "font-size"),
    (re.compile(r"--ndl-font-weight-"), "font-weight"),
    (re.compile(r"--ndl-(duration|component-spinner-period)"), "transition-duration"),
    (re.compile(r"--ndl-easing-"), "transition-timing-function"),
    (re.compile(r"--ndl-"), "min-height"),
]


def load(css, label, errors):
    provider = Gtk.CssProvider()

    def on_error(_provider, section, error):
        loc = section.get_start_location()
        errors.append(f"{label}:{loc.lines + 1}:{loc.line_chars + 1}: {error.message}")

    provider.connect("parsing-error", on_error)
    provider.load_from_string(css)
    return provider


def main():
    if Adw is not None:
        Adw.init()
    else:
        Gtk.init()
    errors = []
    sheets = sorted(DIST.glob("nodalia-*.css"))
    if not sheets:
        print("no stylesheets in dist/: run `npm run build` first", file=sys.stderr)
        return 2
    names = set()
    for sheet in sheets:
        text = sheet.read_text()
        load(text, sheet.name, errors)
        names.update(re.findall(r"^\s*(--ndl-[a-z0-9-]+):", text, re.M))

    probe = []
    for i, name in enumerate(sorted(names)):
        prop = next(p for pattern, p in PROBES if pattern.search(name))
        probe.append(f".ndl-probe-{i} {{ {prop}: var({name}); }}")
    probe_provider = load("\n".join(probe), "probe.css", errors)

    # var() is only checked when a style is computed, so style real widgets with everything.
    display = Gdk.Display.get_default()
    providers = [load(s.read_text(), s.name, []) for s in sheets] + [probe_provider]
    for p in providers:
        Gtk.StyleContext.add_provider_for_display(display, p, Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION)
    GLib.log_set_handler("Gtk", GLib.LogLevelFlags.LEVEL_WARNING | GLib.LogLevelFlags.LEVEL_CRITICAL,
                         lambda _d, _l, msg, *_: errors.append(f"computed: {msg}"))
    box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL)
    for widget in (Gtk.Button(label="b"), Gtk.Entry(), Gtk.Switch(), Gtk.CheckButton(label="c"),
                   Gtk.Scale(), Gtk.ProgressBar(), Gtk.SpinButton(), Gtk.Label(label="l")):
        box.append(widget)
    for i in range(len(names)):
        box.append(Gtk.Label(label=str(i), css_classes=[f"ndl-probe-{i}"]))
    window = Gtk.Window(child=Gtk.ScrolledWindow(child=box))
    window.present()
    context = GLib.MainContext.default()
    for _ in range(200):
        context.iteration(False)
    window.destroy()

    version = f"GTK {Gtk.get_major_version()}.{Gtk.get_minor_version()}.{Gtk.get_micro_version()}"
    if Adw is not None:
        version += f", libadwaita {Adw.get_major_version()}.{Adw.get_minor_version()}.{Adw.get_micro_version()}"
    for e in errors:
        print(e, file=sys.stderr)
    print(f"{version}: {len(sheets)} sheets, {len(names)} variables probed, {len(errors)} errors")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())

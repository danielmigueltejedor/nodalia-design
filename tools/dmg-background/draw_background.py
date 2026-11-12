#!/usr/bin/env python3
"""Draw the Finder window background of a Nodalia app's macOS DMG, as an SVG.

    pip install fonttools
    python3 tools/dmg-background/draw_background.py <app> <out.svg>
    python3 tools/dmg-background/draw_background.py --all <dir>    # <dir>/<app>.svg for every app

Same layout for every app, so the family reads as one: two islands on the window colour, the
left one in the app's colour with a flat picture of what the app does, the right one white with
the name and the drag hint. The arrow between the two icons speaks the app's language too.
The text is outlined from Sora (packages/fonts/sora, OFL-1.1), so rendering needs no fonts and
every machine draws the same pixels. The output is byte-for-byte reproducible.

The layout matches the craft-style apps' packaging/macos/dmg/: 660 x 400 pt content, a 128 pt
app icon at (326, 205) and Applications at (574, 205). Finder draws icon labels in black in
light and dark mode, so the right island stays white.
"""
import io
import math
import sys
from pathlib import Path

from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.transformPen import TransformPen
from fontTools.ttLib import TTFont
from fontTools.varLib import instancer

SORA = Path(__file__).resolve().parents[2] / "packages/fonts/sora/Sora[wght].ttf"
_fonts = {}


def sora(weight):
    if weight not in _fonts:
        f = TTFont(str(SORA))
        f = instancer.instantiateVariableFont(f, {"wght": weight})
        buf = io.BytesIO()
        f.save(buf)
        buf.seek(0)
        _fonts[weight] = TTFont(buf)
    return _fonts[weight]


def text(s, x, y, size, weight, fill, anchor="start", tracking=0.0):
    """Outlined text as one <path>; y is the baseline."""
    f = sora(weight)
    cmap = f.getBestCmap()
    gs = f.getGlyphSet()
    upm = f["head"].unitsPerEm
    k = size / upm
    names = [cmap.get(ord(c), ".notdef") for c in s]
    adv = [gs[n].width * k + tracking for n in names]
    width = sum(adv) - (tracking if names else 0)
    if anchor == "middle":
        x -= width / 2
    elif anchor == "end":
        x -= width
    pen = SVGPathPen(gs)
    cx = x
    for n, a in zip(names, adv):
        tp = TransformPen(pen, (k, 0, 0, -k, cx, y))
        gs[n].draw(tp)
        cx += a
    return f'<path d="{pen.getCommands()}" fill="{fill}"/>'


WINDOW = "#E4E6EB"
PANEL = "#FFFFFF"
INK = "#15171E"
TERTIARY = "#636A7E"
W, H = 660, 400
GAP = 8
R = 16
# Island geometry. Art island on the left, the install island on the right.
AX, AY, AW, AH = GAP, GAP, 224, H - 2 * GAP
IX = AX + AW + GAP
IW = W - IX - GAP
APP_POS, APPS_POS = (326, 205), (574, 205)


def arrowhead(x, y, color, size=7):
    return f'<path d="M{x},{y} L{x - size * 1.6},{y - size} L{x - size * 1.6},{y + size} Z" fill="{color}"/>'


# ---- motifs: drawn in the art island's own coordinates (0..224, 0..384) -------------------------

def m_celdas(c, d):
    o = [f'<rect width="{AW}" height="{AH}" fill="{c}"/>']
    cw, rh = 52, 26
    o.append(f'<rect x="0" y="0" width="{AW}" height="34" fill="{d}"/>')
    o.append(f'<rect x="0" y="0" width="26" height="{AH}" fill="{d}"/>')
    for x in range(26, AW, cw):
        o.append(f'<line x1="{x}" y1="0" x2="{x}" y2="{AH}" stroke="#FFFFFF" stroke-width="1.2"/>')
    for y in range(34, AH, rh):
        o.append(f'<line x1="0" y1="{y}" x2="{AW}" y2="{y}" stroke="#FFFFFF" stroke-width="1.2"/>')
    # a column of numbers as bars, a SUM cell and the selection with its fill handle
    for i, wv in enumerate((30, 18, 40, 24, 34, 12, 28)):
        y = 34 + rh * (i + 2)
        o.append(f'<rect x="{78 + 52 - 6 - wv}" y="{y + 9}" width="{wv}" height="8" rx="2" fill="#FFFFFF"/>')
    y = 34 + rh * 9
    o.append(f'<rect x="78" y="{y}" width="52" height="{rh}" fill="#FFFFFF"/>')
    o.append(f'<rect x="{78 + 52 - 6 - 36}" y="{y + 9}" width="36" height="8" rx="2" fill="{INK}"/>')
    sy = 34 + rh * 2
    o.append(f'<rect x="78" y="{sy}" width="104" height="{rh * 8}" fill="none" stroke="{INK}" stroke-width="3"/>')
    o.append(f'<rect x="{78 + 104 - 5}" y="{sy + rh * 8 - 5}" width="10" height="10" fill="{INK}" stroke="#FFFFFF" stroke-width="2"/>')
    return "".join(o)


def m_forja(c, d):
    o = [f'<rect width="{AW}" height="{AH}" fill="{c}"/>']
    for x in range(8, AW, 16):
        o.append(f'<line x1="{x}" y1="0" x2="{x}" y2="{AH}" stroke="{d}" stroke-width="0.8"/>')
    for y in range(8, AH, 16):
        o.append(f'<line x1="0" y1="{y}" x2="{AW}" y2="{y}" stroke="{d}" stroke-width="0.8"/>')
    # set square
    o.append('<path d="M28,92 L28,300 L196,300 Z" fill="none" stroke="#FFFFFF" stroke-width="6" stroke-linejoin="round"/>')
    o.append('<path d="M52,172 L52,276 L136,276 Z" fill="none" stroke="#FFFFFF" stroke-width="3" stroke-linejoin="round"/>')
    # arc from a centre with crosshair
    o.append(f'<path d="M120,72 A92,92 0 0 1 212,164" fill="none" stroke="{INK}" stroke-width="3"/>')
    o.append(f'<line x1="112" y1="164" x2="132" y2="164" stroke="{INK}" stroke-width="2"/>')
    o.append(f'<line x1="122" y1="154" x2="122" y2="174" stroke="{INK}" stroke-width="2"/>')
    o.append(f'<circle cx="122" cy="164" r="4" fill="{INK}"/>')
    # dimension line under the square
    o.append(f'<line x1="28" y1="318" x2="28" y2="340" stroke="{INK}" stroke-width="1.5"/>')
    o.append(f'<line x1="196" y1="318" x2="196" y2="340" stroke="{INK}" stroke-width="1.5"/>')
    o.append(f'<line x1="28" y1="332" x2="196" y2="332" stroke="{INK}" stroke-width="1.5"/>')
    o.append(f'<line x1="22" y1="338" x2="34" y2="326" stroke="{INK}" stroke-width="2.5"/>')
    o.append(f'<line x1="190" y1="338" x2="202" y2="326" stroke="{INK}" stroke-width="2.5"/>')
    return "".join(o)


def m_lamina(c, d):
    o = [f'<rect width="{AW}" height="{AH}" fill="{c}"/>']
    sw, sh = 168, 94.5
    for i, fill in enumerate((d, "#FFFFFF", "#FFFFFF")):
        x, y = 12 + i * 22, 70 + i * 66
        o.append(f'<rect x="{x}" y="{y}" width="{sw}" height="{sh}" rx="8" fill="{fill}" stroke="{INK}" stroke-width="{3 if i == 2 else 0}"/>')
        if i == 1:
            o.append(f'<rect x="{x + 14}" y="{y + 16}" width="70" height="9" rx="2" fill="{d}"/>')
            o.append(f'<rect x="{x + 14}" y="{y + 34}" width="110" height="5" rx="2" fill="{d}"/>')
            o.append(f'<rect x="{x + 14}" y="{y + 44}" width="90" height="5" rx="2" fill="{d}"/>')
    x, y = 12 + 44, 70 + 132
    o.append(f'<rect x="{x + 14}" y="{y + 14}" width="64" height="9" rx="2" fill="{INK}"/>')
    for j, bh in enumerate((24, 40, 56)):
        o.append(f'<rect x="{x + 18 + j * 22}" y="{y + 82 - bh}" width="14" height="{bh}" fill="{c}"/>')
    o.append(f'<circle cx="{x + 134}" cy="{y + 56}" r="18" fill="{INK}"/>')
    o.append(f'<path d="M{x + 134},{y + 56} L{x + 134},{y + 38} A18,18 0 0 1 {x + 152},{y + 56} Z" fill="{c}"/>')
    # presenter stand
    o.append(f'<line x1="{x + 84}" y1="{y + sh + 2}" x2="{x + 84}" y2="{y + sh + 46}" stroke="{INK}" stroke-width="5"/>')
    o.append(f'<line x1="{x + 54}" y1="{y + sh + 46}" x2="{x + 114}" y2="{y + sh + 46}" stroke="{INK}" stroke-width="5" stroke-linecap="round"/>')
    return "".join(o)


def m_letra(c, d):
    o = [f'<rect width="{AW}" height="{AH}" fill="{c}"/>']
    o.append(text("Aa", 112, 176, 132, 800, "#FFFFFF", anchor="middle"))
    # baseline and the caret
    o.append(f'<line x1="16" y1="176" x2="{AW - 16}" y2="176" stroke="{d}" stroke-width="1.5"/>')
    for i, wv in enumerate((176, 160, 182, 120)):
        y = 214 + i * 22
        o.append(f'<rect x="20" y="{y}" width="{wv}" height="8" rx="4" fill="{"#FFFFFF" if i != 2 else d}"/>')
    o.append(f'<rect x="146" y="{214 + 3 * 22 - 6}" width="4" height="20" fill="{INK}"/>')
    o.append(f'<rect x="20" y="{214 + 4 * 22 + 8}" width="64" height="8" rx="4" fill="{d}"/>')
    return "".join(o)


def m_lienzo(c, d):
    o = [f'<rect width="{AW}" height="{AH}" fill="{c}"/>']
    # three stacked layers seen at an angle
    def layer(cy, fill, stroke=None):
        pts = f"112,{cy - 46} 206,{cy} 112,{cy + 46} 18,{cy}"
        s = f' stroke="{stroke}" stroke-width="3" stroke-linejoin="round"' if stroke else ""
        return f'<polygon points="{pts}" fill="{fill}"{s}/>'
    o.append(layer(292, d))
    o.append(layer(252, "#FFFFFF"))
    o.append(layer(212, INK))
    # a brush stroke across the top layer
    o.append(f'<path d="M58,214 C84,180 118,236 150,196" fill="none" stroke="{c}" stroke-width="12" stroke-linecap="round"/>')
    # a round brush above it
    o.append(f'<path d="M150,60 L186,96 L132,150 C122,160 104,160 98,152 C104,146 104,128 114,118 Z" fill="#FFFFFF"/>')
    o.append(f'<path d="M160,50 L196,86" stroke="{INK}" stroke-width="10" stroke-linecap="round"/>')
    return "".join(o)


def m_luz(c, d):
    o = [f'<rect width="{AW}" height="{AH}" fill="{c}"/>']
    cx, cy, r = 112, 138, 84
    o.append(f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="{INK}"/>')
    o.append(f'<circle cx="{cx}" cy="{cy}" r="{r - 10}" fill="none" stroke="{d}" stroke-width="2"/>')
    # iris: six blades meeting around a hexagonal opening
    n, ri = 6, 30
    hexp = [(cx + ri * math.cos(math.radians(60 * i + 15)), cy + ri * math.sin(math.radians(60 * i + 15))) for i in range(n)]
    o.append('<polygon points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in hexp) + f'" fill="{c}"/>')
    for i in range(n):
        x, y = hexp[i]
        a = math.radians(60 * i + 15 + 60)
        o.append(f'<line x1="{x:.1f}" y1="{y:.1f}" x2="{x + 52 * math.cos(a):.1f}" y2="{y + 52 * math.sin(a):.1f}" stroke="{d}" stroke-width="2"/>')
    # histogram
    vals = [6, 10, 16, 26, 40, 58, 70, 64, 52, 46, 50, 62, 74, 66, 48, 30, 20, 12, 8, 5]
    bw = (AW - 32) / len(vals)
    pts = [(16, 352)] + [(16 + i * bw + bw / 2, 352 - v) for i, v in enumerate(vals)] + [(AW - 16, 352)]
    o.append('<polygon points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in pts) + f'" fill="{INK}"/>')
    o.append(f'<line x1="16" y1="352" x2="{AW - 16}" y2="352" stroke="{INK}" stroke-width="2"/>')
    return "".join(o)


def m_papel(c, d):
    o = [f'<rect width="{AW}" height="{AH}" fill="{c}"/>']
    x, y, w, h, f = 30, 46, 164, 232, 36
    o.append(f'<path d="M{x},{y} L{x + w - f},{y} L{x + w},{y + f} L{x + w},{y + h} L{x},{y + h} Z" fill="#FFFFFF"/>')
    o.append(f'<path d="M{x + w - f},{y} L{x + w - f},{y + f} L{x + w},{y + f} Z" fill="{d}"/>')
    o.append(f'<rect x="{x + 18}" y="{y + 22}" width="70" height="10" rx="2" fill="{INK}"/>')
    for i, wv in enumerate((128, 120, 0, 96, 128, 110)):
        yy = y + 52 + i * 18
        o.append(f'<rect x="{x + 18}" y="{yy}" width="{wv}" height="6" rx="3" fill="{d}"/>')
    # highlight over one line
    o.append(f'<rect x="{x + 14}" y="{y + 52 + 2 * 18 - 4}" width="110" height="14" rx="2" fill="{c}"/>')
    o.append(f'<rect x="{x + 18}" y="{y + 52 + 2 * 18}" width="102" height="6" rx="3" fill="{INK}"/>')
    # signature
    o.append(f'<line x1="{x + 18}" y1="{y + 210}" x2="{x + 110}" y2="{y + 210}" stroke="{d}" stroke-width="1.5"/>')
    o.append(f'<path d="M{x + 22},{y + 204} C{x + 34},{y + 170} {x + 42},{y + 214} {x + 54},{y + 190} S{x + 74},{y + 196} {x + 82},{y + 186} S{x + 96},{y + 200} {x + 108},{y + 188}" fill="none" stroke="{INK}" stroke-width="2.5" stroke-linecap="round"/>')
    # stamp
    o.append(f'<rect x="{x + 10}" y="{y + h + 26}" width="{w - 20}" height="30" rx="15" fill="{INK}"/>')
    o.append(text("PDF", x + w / 2, y + h + 46, 14, 700, "#FFFFFF", anchor="middle", tracking=2))
    return "".join(o)


def m_pliego(c, d):
    o = [f'<rect width="{AW}" height="{AH}" fill="{c}"/>']
    # a spread of two pages
    for px in (16, 114):
        o.append(f'<rect x="{px}" y="64" width="94" height="132" fill="#FFFFFF"/>')
    o.append(f'<line x1="112" y1="60" x2="112" y2="200" stroke="{INK}" stroke-width="2"/>')
    # left page: headline + two columns
    o.append(f'<rect x="24" y="74" width="66" height="10" rx="1" fill="{INK}"/>')
    for col in (24, 64):
        for i in range(8):
            o.append(f'<rect x="{col}" y="{94 + i * 11}" width="34" height="4" fill="{d}"/>')
    # right page: picture frame with the empty-frame cross, then text
    o.append(f'<rect x="122" y="74" width="78" height="56" fill="{c}"/>')
    o.append(f'<line x1="122" y1="74" x2="200" y2="130" stroke="#FFFFFF" stroke-width="1.5"/>')
    o.append(f'<line x1="200" y1="74" x2="122" y2="130" stroke="#FFFFFF" stroke-width="1.5"/>')
    for i in range(5):
        o.append(f'<rect x="122" y="{140 + i * 11}" width="{78 if i < 4 else 40}" height="4" fill="{d}"/>')
    # the column grid that runs down the panel
    for i in range(6):
        x = 16 + i * 39.2
        o.append(f'<rect x="{x:.1f}" y="230" width="31" height="130" fill="none" stroke="#FFFFFF" stroke-width="1.5"/>')
    for i in range(10):
        y = 244 + i * 12
        o.append(f'<line x1="16" y1="{y}" x2="{AW - 16}" y2="{y}" stroke="{d}" stroke-width="1"/>')
    return "".join(o)


def m_pluma(c, d):
    o = [f'<rect width="{AW}" height="{AH}" fill="{c}"/>']
    p0, c0, c1, p1 = (24, 300), (24, 120), (200, 280), (200, 96)
    o.append(f'<path d="M{p0[0]},{p0[1]} C{c0[0]},{c0[1]} {c1[0]},{c1[1]} {p1[0]},{p1[1]}" fill="none" stroke="#FFFFFF" stroke-width="6"/>')
    for a, b in ((p0, c0), (p1, c1)):
        o.append(f'<line x1="{a[0]}" y1="{a[1]}" x2="{b[0]}" y2="{b[1]}" stroke="{INK}" stroke-width="2"/>')
        o.append(f'<circle cx="{b[0]}" cy="{b[1]}" r="7" fill="{INK}"/>')
    for p in (p0, p1):
        o.append(f'<rect x="{p[0] - 9}" y="{p[1] - 9}" width="18" height="18" fill="#FFFFFF" stroke="{INK}" stroke-width="3"/>')
    # pen nib
    nx, ny = 112, 312
    o.append(f'<path d="M{nx},{ny + 44} L{nx - 26},{ny + 4} L{nx - 14},{ny - 22} L{nx + 14},{ny - 22} L{nx + 26},{ny + 4} Z" fill="{INK}"/>')
    o.append(f'<line x1="{nx}" y1="{ny + 44}" x2="{nx}" y2="{ny + 6}" stroke="{c}" stroke-width="3"/>')
    o.append(f'<circle cx="{nx}" cy="{ny + 2}" r="5" fill="{c}"/>')
    o.append(f'<rect x="{nx - 18}" y="{ny - 40}" width="36" height="14" rx="3" fill="{INK}"/>')
    return "".join(o)


def m_pulso(c, d):
    o = [f'<rect width="{AW}" height="{AH}" fill="{c}"/>']
    # graph editor: an ease curve on a grid
    gx, gy, gw, gh = 24, 48, 176, 176
    o.append(f'<rect x="{gx}" y="{gy}" width="{gw}" height="{gh}" fill="{INK}"/>')
    for i in range(1, 4):
        o.append(f'<line x1="{gx + gw * i / 4}" y1="{gy}" x2="{gx + gw * i / 4}" y2="{gy + gh}" stroke="#2E3342" stroke-width="1"/>')
        o.append(f'<line x1="{gx}" y1="{gy + gh * i / 4}" x2="{gx + gw}" y2="{gy + gh * i / 4}" stroke="#2E3342" stroke-width="1"/>')
    a, b = (gx + 14, gy + gh - 14), (gx + gw - 14, gy + 14)
    o.append(f'<path d="M{a[0]},{a[1]} C{a[0] + 110},{a[1]} {b[0] - 110},{b[1]} {b[0]},{b[1]}" fill="none" stroke="{c}" stroke-width="4"/>')
    for p, h in ((a, (a[0] + 110, a[1])), (b, (b[0] - 110, b[1]))):
        o.append(f'<line x1="{p[0]}" y1="{p[1]}" x2="{h[0]}" y2="{h[1]}" stroke="#FFFFFF" stroke-width="1.5"/>')
        o.append(f'<circle cx="{h[0]}" cy="{h[1]}" r="4.5" fill="#FFFFFF"/>')
    # timeline with keyframes
    for row in range(3):
        y = 262 + row * 34
        o.append(f'<rect x="16" y="{y - 12}" width="{AW - 32}" height="24" rx="6" fill="{d}"/>')
        for kx in ((40, 104, 176), (64, 150), (40, 88, 128, 184))[row]:
            o.append(f'<path d="M{kx},{y - 8} L{kx + 8},{y} L{kx},{y + 8} L{kx - 8},{y} Z" fill="{"#FFFFFF" if row != 1 else INK}"/>')
    o.append(f'<line x1="118" y1="244" x2="118" y2="{262 + 2 * 34 + 16}" stroke="{INK}" stroke-width="2.5"/>')
    o.append(f'<path d="M110,240 L126,240 L118,250 Z" fill="{INK}"/>')
    return "".join(o)


def m_toma(c, d):
    o = [f'<rect width="{AW}" height="{AH}" fill="{c}"/>']
    # viewer frame
    o.append(f'<rect x="16" y="40" width="192" height="108" rx="6" fill="{INK}"/>')
    o.append('<path d="M98,72 L134,94 L98,116 Z" fill="#FFFFFF"/>')
    # tracks
    clips = [
        [(16, 70, "#FFFFFF"), (90, 54, "#FFFFFF"), (148, 60, INK)],
        [(36, 96, d), (136, 72, d)],
        [(16, 120, INK), (140, 68, "#FFFFFF")],
    ]
    for r, row in enumerate(clips):
        y = 184 + r * 36
        for x, w, f in row:
            o.append(f'<rect x="{x}" y="{y}" width="{w}" height="28" rx="5" fill="{f}"/>')
    # audio waveform track
    y0 = 184 + 3 * 36 + 14
    for i in range(46):
        x = 18 + i * 4.2
        h = 3 + 11 * abs(math.sin(i * 0.55)) * (0.5 + 0.5 * abs(math.sin(i * 0.17)))
        o.append(f'<rect x="{x:.1f}" y="{y0 - h:.1f}" width="2.4" height="{2 * h:.1f}" rx="1.2" fill="{INK}"/>')
    # playhead
    o.append('<line x1="122" y1="172" x2="122" y2="352" stroke="#FFFFFF" stroke-width="2.5"/>')
    o.append('<path d="M114,166 L130,166 L130,174 L122,182 L114,174 Z" fill="#FFFFFF"/>')
    return "".join(o)


def m_vinilo(c, d):
    o = [f'<rect width="{AW}" height="{AH}" fill="{c}"/>']
    cx, cy, r = 96, 214, 150
    o.append(f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="{INK}"/>')
    for rr in range(r - 12, 52, -9):
        o.append(f'<circle cx="{cx}" cy="{cy}" r="{rr}" fill="none" stroke="#2E3342" stroke-width="1.2"/>')
    o.append(f'<circle cx="{cx}" cy="{cy}" r="46" fill="{c}"/>')
    o.append(f'<circle cx="{cx}" cy="{cy}" r="30" fill="none" stroke="{d}" stroke-width="2"/>')
    o.append(f'<circle cx="{cx}" cy="{cy}" r="6" fill="#FFFFFF"/>')
    # tonearm
    o.append('<circle cx="196" cy="54" r="14" fill="#FFFFFF"/>')
    o.append(f'<circle cx="196" cy="54" r="5" fill="{INK}"/>')
    o.append('<path d="M196,54 L196,170 L170,206" fill="none" stroke="#FFFFFF" stroke-width="6" stroke-linejoin="round" stroke-linecap="round"/>')
    o.append('<rect x="154" y="198" width="26" height="16" rx="3" fill="#FFFFFF" transform="rotate(-36 167 206)"/>')
    return "".join(o)


# ---- the arrow between the two icons, in each app's own language --------------------------------

AL, AR, AYc = 404, 494, 205  # 10 pt clear of both 128 pt icon boxes


def a_celdas(c):
    o = []
    for i in range(4):
        o.append(f'<rect x="{AL + i * 18}" y="{AYc - 7}" width="18" height="14" fill="{c if i == 3 else "none"}" stroke="{INK}" stroke-width="1.2"/>')
    o.append(arrowhead(AR, AYc, INK))
    return "".join(o)


def a_forja(c):
    return (f'<line x1="{AL}" y1="{AYc - 10}" x2="{AL}" y2="{AYc + 10}" stroke="{INK}" stroke-width="1"/>'
            f'<line x1="{AL}" y1="{AYc}" x2="{AR - 10}" y2="{AYc}" stroke="{INK}" stroke-width="1.2"/>'
            f'<line x1="{AL - 5}" y1="{AYc + 5}" x2="{AL + 5}" y2="{AYc - 5}" stroke="{c}" stroke-width="2.5"/>'
            + arrowhead(AR, AYc, INK))


def a_lamina(c):
    o = []
    for i, w in enumerate((16, 20, 26)):
        h = w * 9 / 16
        x = AL + (0, 22, 48)[i]
        o.append(f'<rect x="{x}" y="{AYc - h / 2:.1f}" width="{w}" height="{h:.1f}" rx="2" fill="{c if i == 2 else "none"}" stroke="{INK}" stroke-width="1.2"/>')
    o.append(arrowhead(AR, AYc, INK))
    return "".join(o)


def a_letra(c):
    o = [f'<circle cx="{AL + i * 7}" cy="{AYc}" r="1.4" fill="{INK}"/>' for i in range(10)]
    o.append(arrowhead(AR - 8, AYc, INK))
    o.append(f'<rect x="{AR - 3}" y="{AYc - 11}" width="3" height="22" fill="{c}"/>')
    return "".join(o)


def a_lienzo(c):
    return (f'<path d="M{AL},{AYc + 2} C{AL + 20},{AYc - 8} {AL + 50},{AYc + 8} {AR - 12},{AYc}" fill="none" stroke="{c}" stroke-width="6" stroke-linecap="round"/>'
            + arrowhead(AR, AYc, INK))


def a_luz(c):
    return (f'<line x1="{AL}" y1="{AYc}" x2="{AR - 12}" y2="{AYc}" stroke="{INK}" stroke-width="2" stroke-linecap="round"/>'
            f'<line x1="{AL}" y1="{AYc}" x2="{AL + 46}" y2="{AYc}" stroke="{c}" stroke-width="4" stroke-linecap="round"/>'
            f'<circle cx="{AL + 50}" cy="{AYc}" r="7" fill="#FFFFFF" stroke="{INK}" stroke-width="2"/>'
            + arrowhead(AR, AYc, INK))


def a_papel(c):
    return (f'<rect x="{AL}" y="{AYc - 6}" width="{AR - AL - 14}" height="12" rx="2" fill="{c}"/>'
            f'<line x1="{AL + 4}" y1="{AYc}" x2="{AR - 14}" y2="{AYc}" stroke="{INK}" stroke-width="2"/>'
            + arrowhead(AR, AYc, INK))


def a_pliego(c):
    return (f'<rect x="{AL}" y="{AYc - 9}" width="18" height="18" fill="none" stroke="{INK}" stroke-width="1.2"/>'
            f'<path d="M{AL + 13},{AYc + 2} L{AL + 18},{AYc} L{AL + 13},{AYc - 2} Z" fill="{INK}"/>'
            f'<line x1="{AL + 22}" y1="{AYc}" x2="{AR - 26}" y2="{AYc}" stroke="{c}" stroke-width="1.6" stroke-dasharray="3 3"/>'
            f'<rect x="{AR - 22}" y="{AYc - 9}" width="18" height="18" fill="none" stroke="{INK}" stroke-width="1.2"/>'
            + arrowhead(AR + 2, AYc, INK, 5))


def a_pluma(c):
    return (f'<rect x="{AL - 4}" y="{AYc + 4}" width="8" height="8" fill="#FFFFFF" stroke="{c}" stroke-width="1.6"/>'
            f'<path d="M{AL},{AYc + 8} C{AL + 26},{AYc - 26} {AL + 50},{AYc + 26} {AR - 10},{AYc}" fill="none" stroke="{INK}" stroke-width="1.6"/>'
            + arrowhead(AR, AYc, INK))


def a_pulso(c):
    o = []
    for i, x in enumerate((AL + 6, AL + 30, AL + 54)):
        o.append(f'<path d="M{x},{AYc - 7} L{x + 7},{AYc} L{x},{AYc + 7} L{x - 7},{AYc} Z" fill="{c if i == 2 else "none"}" stroke="{INK}" stroke-width="1.2"/>')
    o.append(arrowhead(AR, AYc, INK))
    return "".join(o)


def a_toma(c):
    o = [f'<rect x="{AL + i * 16}" y="{AYc - 7}" width="13" height="14" rx="2" fill="none" stroke="{INK}" stroke-width="1.2"/>' for i in range(4)]
    o.append(f'<path d="M{AL + 68},{AYc - 10} L{AR},{AYc} L{AL + 68},{AYc + 10} Z" fill="{c}"/>')
    return "".join(o)


def a_vinilo(c):
    return (f'<line x1="{AL}" y1="{AYc}" x2="{AR - 12}" y2="{AYc}" stroke="{c}" stroke-width="2.4" stroke-linecap="round"/>'
            + arrowhead(AR, AYc, c))


# Each colour is the middle of the app icon's background in nodalix-icon-theme, so the DMG
# window and the icon inside it read as one thing. The darker shade is for a second ink.
APPS = {
    # id: (name, tagline, colour, darker shade, motif, arrow)
    "celdas": ("Celdas", "Hojas de cálculo", "#2F9E5B", "#237A45", m_celdas, a_celdas),
    "forja": ("Forja", "Dibujo técnico y CAD", "#C47A45", "#A86536", m_forja, a_forja),
    "lamina": ("Lámina", "Presentaciones", "#B03C67", "#8C2D50", m_lamina, a_lamina),
    "letra": ("Letra", "Procesador de textos", "#3A9EE6", "#2479BD", m_letra, a_letra),
    "lienzo": ("Lienzo", "Edición de imagen", "#EE4F78", "#C7385E", m_lienzo, a_lienzo),
    "luz": ("Luz", "Revelado y biblioteca de fotos", "#F9A825", "#C9851A", m_luz, a_luz),
    "papel": ("Papel", "Documentos PDF", "#8794AD", "#5F6B84", m_papel, a_papel),
    "pliego": ("Pliego", "Maquetación de páginas", "#5A63E6", "#434BC0", m_pliego, a_pliego),
    "pluma": ("Pluma", "Ilustración vectorial", "#F2782F", "#C95F1E", m_pluma, a_pluma),
    "pulso": ("Pulso", "Animación y efectos visuales", "#7CC23A", "#5E9A27", m_pulso, a_pulso),
    "toma": ("Toma", "Edición de vídeo", "#DB45B5", "#B03091", m_toma, a_toma),
    "vinilo": ("Vinilo", "Apple Music en tu escritorio", "#8B4AE6", "#6B33C2", m_vinilo, a_vinilo),
}


def svg(app):
    name, tagline, c, d, motif, arrow = APPS[app]
    o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}">',
         f'<!-- {name} installer window: Nodalia islands, flat inks, text outlined from Sora (OFL). '
         f'Drawn by tools/dmg-background/draw_background.py in nodalia-design: change it there and regenerate. -->',
         f'<defs><clipPath id="art"><rect width="{AW}" height="{AH}" rx="{R}"/></clipPath></defs>',
         f'<rect width="{W}" height="{H}" fill="{WINDOW}"/>',
         f'<g transform="translate({AX},{AY})"><g clip-path="url(#art)">{motif(c, d)}</g></g>',
         f'<rect x="{IX}" y="{AY}" width="{IW}" height="{AH}" rx="{R}" fill="{PANEL}"/>',
         text(name, IX + 28, 64, 34, 800, INK),
         f'<rect x="{IX + 28}" y="76" width="22" height="4" rx="2" fill="{c}"/>',
         text(tagline, IX + 58, 81, 12.5, 500, TERTIARY),
         arrow(c),
         f'<line x1="{IX + 28}" y1="332.5" x2="{IX + IW - 28}" y2="332.5" stroke="{WINDOW}" stroke-width="1"/>',
         text("Arrastra la app a Aplicaciones para instalarla", IX + 28, 358, 11.5, 600, INK),
         text("Drag the app to Applications to install it", IX + 28, 374, 10, 500, TERTIARY),
         "</svg>"]
    return "\n".join(o) + "\n"


def main():
    args = sys.argv[1:]
    if len(args) == 2 and args[0] == "--all":
        out = Path(args[1])
        out.mkdir(parents=True, exist_ok=True)
        for app in APPS:
            (out / f"{app}.svg").write_text(svg(app))
    elif len(args) == 2 and args[0] in APPS:
        Path(args[1]).write_text(svg(args[0]))
    else:
        sys.exit(__doc__ + "\napps: " + ", ".join(APPS))


if __name__ == "__main__":
    main()

# Fonts

Nodalia Design uses two typefaces, both under the SIL Open Font License 1.1.

| Font | Use | Source | Licence |
| --- | --- | --- | --- |
| Sora (variable, `wght`) | View titles, section titles, big numbers | https://github.com/google/fonts/tree/main/ofl/sora | OFL-1.1, see `sora/OFL.txt` |
| Inter (variable, `opsz`,`wght`) | Body text and controls | https://github.com/google/fonts/tree/main/ofl/inter | OFL-1.1, see `inter/OFL.txt` |

Files are the unmodified variable fonts from the google/fonts repository. Reserved font names apply: do not rename or modify them.

The engineering suite (Torno, Vortia, Forja) uses **IBM Plex Sans** (variable, `wdth`,`wght`) and **IBM Plex Mono** Regular, OFL-1.1, from https://github.com/google/fonts/tree/main/ofl/ibmplexsans and `ofl/ibmplexmono` (fetched 2026-11-13; SHA-256 `3b031aa4216174205bd8471f88a49b91f093169e9e87bd5262242bc5967fe2e3` and `6a3412f058c7d8dfd9170c41e85ade48e5156ecb89356110ca57a0a27734af46`). They live inside the Rust crate, `packages/egui/crate/fonts/`, so a vendored copy of the crate carries them; see `docs/suites/engineering.md`. Reserved Font Name "Plex": unmodified files only.

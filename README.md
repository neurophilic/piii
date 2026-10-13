# piii lab

A science lab you work in by drawing. Everything is a **super**, a box with four ports, and supers are joined by **wires**. What a super holds opens to fill the canvas, and the rest of the lab waits as icons beside it.

Live at https://www.piii.app/

## Objectives

- **Graphical.** The lab is a canvas of supers and wires, not a stack of pages. You drag the supers, wire them together, lock and save the layout, and open any one to work in it. Every super has the same shape, a superellipse with n = 50.
- **Built in Rust.** The site is Rust compiled to WebAssembly, with no HTML files, no frameworks and no dependencies: `core` and `alloc` only. It draws every view, word and style itself. A few lines of JavaScript (`front/app/host.js`) lend it the page.
- **Science first.** The lab's first instrument is **ScholarPi**, which assesses research papers. A panel of independent AI models reads a manuscript. Deterministic checks audit its references, reproducibility and integrity. It is scored on eight open criteria (the **piX** index), and the report shows what holds it back. Each assessment is recorded on a Proof-of-Research ledger and earns **piQ**, a soulbound token.
- **Open and accountable.** The rubric and every weight are published (`/api/rubric`). The h-index and venue prestige are reported but never scored. Nothing optional happens before you choose (`/api/cookies`).
- **More instruments.** Lab, Library and Connect hold the tools still being rewritten in Rust: HAL-OS, Unigyro, FaceMace, the Map of Science, Neuro Frenzy and the MD plan.

## The canvas

```
 piii lab        [save] [lock] [chain] [reset]
                     │      │
      welcome ───────┘      ├──────────┐
      Tools  ═══▶ Explore ═══▶ Library │
      Connect      │            │      │
      Privacy     Lab   ═══▶  Project ─┘
                 [←] [↑]  [↓] [→]
```

Magenta wires carry data out of an output (▷ into ◁). Ink wires are sync (△) and condition (▽). The project map (`#/map`) shows the whole lab in the same notation: one prompt fanned out to every engine, and the assessment from manuscript to ledger.

## Layout

```
front/        what the browser gets
  app/        ui.wasm (built from back/rust/ui) and host.js
  fonts/ icon.svg manifest.webmanifest ScholarPi_Whitepaper.pdf zotero/
back/
  rust/ui/    the site: canvas, supers, wires, every window (no dependencies, no_std)
  rust/superai/   SuperAI, the lab's graphical language
  rust/numbers/   π, e, φ, γ, computed exactly
  api.py      the server and the science engines (FastAPI)
  tests/  scripts/tools.py  zotero/  deploy/  LICENSES/
nixpacks.toml railway.toml requirements.txt runtime.txt .github/
```

## Run and build

```bash
python back/scripts/tools.py run        # http://localhost:8000 (add GROQ_API_KEY to back/.env for the AI panel)
python back/scripts/tools.py build-ui   # the site to front/app/ui.wasm (rustup target add wasm32-unknown-unknown)
cd back && python -m pytest -q          # the tests
```

`build-ui --library DIR` builds without the wasm32 target, from the standard library's source.

## Deploy

Railway builds from the repository root with Nixpacks. `ui.wasm` is committed, so the build only installs Python. The app starts with `gunicorn api:app -c back/gunicorn.conf.py`. The healthcheck is `/api/health`, and data lives in `SCHOLARPI_DATA_DIR`. `back/.env.example` lists every setting. The repository is public: never commit `.env` or keys.

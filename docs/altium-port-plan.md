# Altium Designer → Rust (iced) 1:1 Port — Master Plan

**Target:** Rebuild Altium Designer in Rust using **iced 0.14** as the desktop framework,
matching Altium's menus, panels, toolbars, dashboards, pages, tabs, options, and tools.

**Status:** This is a *continuation* plan. The oxide-eda project already ships an iced
UI skeleton, a clean-room Altium file importer, and engine crates. Ghidra decompilation
of the Altium binaries is in progress. This plan turns that foundation into a full 1:1 port.

---

## 0. Executive Summary

| Item | State |
|---|---|
| Desktop framework | **iced 0.14** (canvas, wgpu, svg, image, tokio) — already in `Cargo.toml` |
| UI skeleton | **Exists** in `oxide-app`: canvas (camera/grid/input/draw), menu_bar, panels, keymap/catalog, dock, active_bar, app/{contracts,runtime,command,state,handlers,view,bootstrap,dispatch} |
| Altium file import | **Exists** in `oxide-altium-importer` (CFB/OLE2 + pipe-delimited records) for `.schdoc`, `.pcbdoc`, `.schlib`, `.pcblib`, `.intlib` |
| Engine crates | `oxide-engine`, `oxide-router`, `oxide-rules`, `oxide-sim`, `oxide-bom`, `oxide-output`, `oxide-gfx`, `oxide-renderer`, `oxide-widgets` |
| Reverse-engineered assets | **UI command catalog (3004 commands)** from `.rcs` files; Ghidra projects for X2.EXE, AltiumMS.exe, and core engine DLLs |

The plan below is organized so every phase ships a working, demoable artifact and the
reverse-engineered assets (UI catalog + decompiled code) directly drive the work.

---

## 1. Asset Inventory (what already exists)

### 1.1 UI skeleton — `oxide-app/src/`
```
canvas/
  camera.rs        # pan/zoom camera
  grid.rs          # grid/snap
  input/           # mouse/keyboard input handling
  draw/            # drawing primitives
menu_bar/
  view.rs          # menu bar (to be expanded to full Altium menu tree)
panels/
  components_panel, properties_parameters, waveform,
  element_properties, footprint_editor_properties, symbol_editor_properties,
  telecom, mcu_console
keymap/catalog/    # keyboard shortcut catalog
dock/              # dockable panels
active_bar/        # active tool bar
app/{contracts,runtime,command,state,handlers,view,bootstrap,dispatch}
```

### 1.2 Altium importer — `oxide-altium-importer/src/`
- `cfb.rs` — OLE2/CFB container reader
- `record.rs` — pipe-delimited property records (`|KEY=VALUE|...`) with typed getters:
  `get_i64`, `get_f64`, `get_bool`, `get_coord_mm` (raw = 1-mil; suffixes mm/mil/in)
- `sch_importer.rs`, `pcb_importer.rs`, `schlib_importer.rs`, `pcblib_importer.rs`, `intlib_importer.rs`, `rule_importer.rs`
- `verification.rs` — round-trip verification reports
- Tests: `importer_tests.rs`, `altium_parity_tests.rs`, `real_corpus_test.rs`

### 1.3 Reverse-engineered assets (this work)
- **UI command catalog** → `refernces/AltiumDesigner/decompiled/altium_ui_catalog.json` (3004 commands)
- **Ghidra projects** → `/home/jrad/tools/ghidra_projects/{altium_main, altium_ms, altium_analysis}`
  - `altium_main`: X2.EXE (main editor, 41 MB)
  - `altium_ms`: AltiumMS.exe (shell/manager, 10 MB)
  - `altium_analysis`: core engine DLLs — RouterEntry.dll, ArcPour.dll, GPUPourLib.dll,
    IntGeometryEntry.dll, unigraph.dll, rtree.dll, PolygonClipper.dll
- **Strings** → `decompiled/{X2.EXE.strings, AltiumMS.exe.strings}`

---

## 2. Reverse-Engineering Findings

### 2.1 UI command catalog (from `.rcs` resource files)
Altium defines its entire UI in text-based `.rcs` files. Each line is a command:
```
PL PLPCB:Undo   Command='PCB:Undo'   Caption='&Undo'   Image='UNDO.PNG'
               Shortcut1='Ctrl+Z'   Shortcut2='Alt+Backspace'   Params=''   Description='Undo'   End
```
Fields: `Command` (namespace:action), `Caption` (display text, `&` = mnemonic),
`Image` (icon), `Shortcut1/2`, `Params`, `Description`.

**Catalog counts (from `altium_ui_catalog.json`):**
| `.rcs` file | Commands | Scope |
|---|---|---|
| AdvPcb.rcs | 959 | PCB editor |
| advsch.rcs | 677 | Schematic editor |
| WorkspaceManager.rcs | 524 | Project/workspace management |
| CAMTASTIC.rcs | 274 | CAM/DFM |
| MultiBoardAssembly.rcs | 42 | Multi-board |
| + ~30 more (DRC, simulation, 3D, version control, …) | — | — |
| **Total** | **3004** | — |

**Command namespaces observed:** `PCB:*`, `@PLSCH:*` (interactive), `InteractiveProperties:*`,
`ExtensionManager:*`, `PCB:Import/Export`, etc.

> **This catalog is the single source of truth for 1:1 UI parity.** Every command,
caption, icon, and shortcut in it must be reproduced in the Rust app. It also reveals the
command architecture: a namespaced command registry with params + interactive modes.

### 2.2 File format (from importer + Ghidra)
- **Container:** OLE2/CFB (compound file) — `cfb.rs` reads it.
- **Records:** length-prefixed, pipe-delimited property lists (`|RECORD=1|LOCATION.X=100|...`).
- **Coordinates:** raw numbers are in 1-mil units; optional `mm`/`mil`/`in` suffixes.
- **File types:** `.schdoc` (schematic), `.pcbdoc` (PCB), `.schlib` (symbol lib),
  `.pcblib` (footprint lib), `.intlib` (integrated lib).

### 2.3 Core engine DLLs (Ghidra `altium_analysis` project)
| DLL | Role |
|---|---|
| RouterEntry.dll | Routing engine entry (manual + auto) |
| c3d_routing.dll | 3D routing (45 MB) |
| ArcPour.dll | Arc-based copper pour |
| GPUPourLib.dll | GPU-accelerated copper pour |
| IntGeometryEntry.dll | Integer/coordinate geometry |
| unigraph.dll | Graph algorithms (connectivity, nets) |
| rtree.dll | Spatial index (R-tree) for hit-test/culling |
| PolygonClipper.dll | Polygon clipping (pours, keepouts) |

> These map directly to the Rust engine crates: `oxide-router`, `oxide-rules`, `oxide-gfx`.

---

## 3. Target Architecture (Rust + iced)

```
oxide-eda/
├── crates/oxide-app          # iced frontend (existing skeleton → full Altium UI)
│   ├── ui_catalog/          # NEW: generated from altium_ui_catalog.json
│   ├── menu/                # NEW: full Altium menu tree (File/Edit/View/Design/Tools/Analysis/Help)
│   ├── toolbar/             # NEW: tool palettes per editor
│   ├── dock/                # existing → Altium dockable panels
│   ├── canvas/              # existing → PCB + schematic canvas
│   └── editors/{pcb,schematic,library}  # editor state machines
├── crates/oxide-engine       # NEW core: data model + command registry
│   ├── model/               # Board, Net, Trace, Via, Pour, Symbol, Footprint, Layer…
│   ├── command/             # namespaced command registry (PCB:*, @PLSCH:*)
│   └── undo/                # command-pattern history stack
├── crates/oxide-altium-importer  # existing → complete + export round-trip
├── crates/oxide-router / oxide-rules / oxide-sim / oxide-bom / oxide-output
└── crates/oxide-3d         # NEW: wgpu/bevy 3D viewer (isolated window)
```

**Key decisions:**
1. **Rendering:** `iced::canvas` for PCB + schematic viewports (pan/zoom/hit-test/culling).
2. **State:** central `EditorState` + iced `Message` (Flux); command registry drives all actions.
3. **Command architecture:** replicate Altium's namespaced commands (`PCB:Undo`, `@PLSCH:InteractiveZoomIn`) as a Rust `CommandRegistry` — this is what makes 1:1 menus/toolbars tractable.
4. **Undo/redo:** command pattern + history stack over `EditorState` (Altium uses `PCB:Undo`/`PCB:Redo`).
5. **3D:** isolated `wgpu`/`bevy` window (iced has no native 3D); ship a 2D isometric view first.
6. **Performance:** R-tree spatial index + viewport culling (mirrors Altium's `rtree.dll`).

---

## 4. Phased Roadmap

Each phase ends with a working, demoable artifact + an exit criterion.

### Phase 0 — Foundation & RE completion *(Weeks 1–3)*
**Goal:** Finish decompilation; turn the UI catalog into code.
- [ ] Complete Ghidra decompilation of X2.EXE + AltiumMS.exe → export functions to `decompiled/{X2.EXE,AltiumMS.exe}/` (use `decompile_exporter.py`).
- [ ] Decompile targeted functions: entry points, file-format handlers, routing/pour entry points.
- [ ] Generate a Rust `ui_catalog` module from `altium_ui_catalog.json` (commands → `#[derive]` structs + registry).
- [ ] Build the **command registry** prototype: `CommandRegistry` with namespaced commands + params.

*Exit:* iced app opens; command registry can dispatch any of the 3004 commands by name; decompiled C for X2.EXE/AltiumMS.exe is available.

### Phase 1 — Data model & file I/O *(Weeks 4–10)*
**Goal:** Complete import + add export round-trip.
- [ ] Finish `oxide-altium-importer`: verify all record types against Ghidra decompilation of the CFB streams.
- [ ] Add **exporters**: internal model → `.schdoc`/`.pcbdoc` (round-trip), plus Gerber/BOM/netlist/ODB++.
- [ ] Golden-file tests: import real Altium file → round-trip → diff.

*Exit:* Open a real Altium project; it renders correctly; save produces a valid file.

### Phase 2 — Editor core (canvas) *(Weeks 11–20)*
**Goal:** The canvas everything sits on.
- [ ] Pan/zoom/fit-to-view (extend existing `camera.rs`).
- [ ] Selection: single, bounding-box, by-net, by-layer.
- [ ] Manipulation: move/rotate/scale with grid snap.
- [ ] Undo/redo via command history.
- [ ] Layer visibility + stackup view.
- [ ] R-tree spatial index + culling (baseline: 10k objects @ 60 fps).

*Exit:* Load a PCB; pan/zoom/select/move/undo — smooth.

### Phase 3 — Schematic editor *(Weeks 21–35)*
**Goal:** Full schematic capture (from `advsch.rcs`, 677 commands).
- [ ] Symbol placement + rotation + pin editing (`@PLSCH:Interactive*` commands).
- [ ] Net connection: wires, net labels, power nets, ports.
- [ ] Page management (multi-page, page-to-page refs).
- [ ] Annotations: labels, notes, dimensions.
- [ ] Cross-probe foundation (schematic ↔ PCB stable IDs).
- [ ] Schematic DRC (unconnected pins, net conflicts).

*Exit:* Draw a multi-page schematic with nets/labels/annotations; round-trips to `.schdoc`.

### Phase 4 — PCB editor *(Weeks 36–55)*
**Goal:** Full PCB capture (from `AdvPcb.rcs`, 959 commands).
- [ ] Footprint placement + rotation.
- [ ] Manual trace routing (constraint-aware; port `RouterEntry.dll` logic).
- [ ] Via placement (drill, annular ring, layer span).
- [ ] Copper pour (port `ArcPour.dll`/`GPUPourLib.dll`; GPU path later).
- [ ] Layer management: copper/signal/plane/mask/solder/keepout/mechanical.
- [ ] Title block editing.
- [ ] PCB DRC integration (clearance, min width, pour violations).

*Exit:* Route a multi-layer board with pours/vias; DRC matches Altium.

### Phase 5 — Libraries & Component Manager *(Weeks 56–65)*
**Goal:** Edit + manage the component ecosystem.
- [ ] Symbol editor (pins, electrical connections, body).
- [ ] Footprint editor (pads, copper, keepout; IPC-2450).
- [ ] Component Manager (search, assign, replace, properties).
- [ ] IPC-2450 import/export.

*Exit:* Create a symbol + footprint, place on board, works end-to-end.

### Phase 6 — Analysis: DRC, Simulation, 3D *(Weeks 66–85)*
**Goal:** The "Analysis" menu becomes real.
- [ ] Full DRC engine (electrical + spacing + clearance; user-defined rules).
- [ ] Basic simulation (DC/AC) — subset first; full SPICE is a stretch goal.
- [ ] 3D viewer (isolated wgpu/bevy window): board + components, orbit/zoom.
- [ ] DFM analysis (from `CAMTASTIC.rcs`, 274 commands).

*Exit:* DRC report matches Altium; 3D viewer renders the loaded board; basic simulation runs.

### Phase 7 — Output & Manufacturing *(Weeks 86–95)*
**Goal:** Production-ready exports.
- [ ] Gerber (Gerber 9 + 2011) for all layers.
- [ ] BOM (CSV/Excel) with properties.
- [ ] Netlist export (multiple dialects).
- [ ] ODB++ export.
- [ ] Fabrication data (drill, copper, mask, solder).

*Exit:* Exports open cleanly in CAM tools and match Altium's output.

### Phase 8 — Parity & Polish *(Weeks 96+)*
**Goal:** "1:1" feel.
- [ ] Match exact UI layout, panel positions, dialog behavior (driven by `altium_ui_catalog.json`).
- [ ] Keyboard shortcut parity (all of them).
- [ ] Performance tuning (large boards, many components).
- [ ] Feature-parity test matrix: every Altium feature checked against the tool.

*Exit:* Side-by-side comparison shows no visible/functional gaps in the core workflow.

---

## 5. Technical Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| **File format** (`.schdoc`/`.pcbdoc`) CFB streams are complex | Med | High | Importer already parses CFB + pipe records; verify against Ghidra decompilation of the actual streams. Accept partial support early, expand over time. Golden-file tests catch regressions. |
| **3D rendering** in iced | Med | Med | Isolate 3D in a separate wgpu/bevy window; ship 2D isometric first, true 3D later. |
| **Auto-routing complexity** | High | Med | Ship manual routing first (Phase 4); port `RouterEntry.dll` logic for constraint-based auto-route in Phase 6. |
| **Performance on large boards** | Med | High | R-tree + culling from Phase 2; keep engine GUI-free for headless testing. |
| **Scope creep** (Altium has many features) | High | High | Phased exit criteria force incremental shipping; defer "nice-to-have" analysis to Phase 6/8. |

---

## 6. Milestones (demo checkpoints)

| Milestone | Deliverable |
|---|---|
| **M0** (Wk 3) | iced app opens; command registry dispatches all 3004 commands; X2.EXE/AltiumMS.exe decompiled C exported |
| **M1** (Wk 10) | Open a real Altium project; round-trip save works |
| **M2** (Wk 20) | Load + pan/zoom/select/move a PCB, undo/redo, smooth |
| **M3** (Wk 35) | Multi-page schematic with nets/labels/annotations |
| **M4** (Wk 55) | Multi-layer PCB with routing, pours, vias + DRC |
| **M5** (Wk 65) | Symbol/footprint editors + Component Manager |
| **M6** (Wk 85) | DRC report + 3D viewer + basic simulation |
| **M7** (Wk 95) | Gerber/BOM/netlist/ODB++ exports that open in CAM tools |
| **M8** (Wk 96+) | Full feature-parity matrix; no visible gaps in core workflow |

---

## 7. Recommended first step (start here)

1. **Finish the decompilation:** run `decompile_exporter.py` on the `altium_main` and `altium_ms` Ghidra projects once analysis completes → C output in `decompiled/{X2.EXE,AltiumMS.exe}/`.
2. **Build the command registry** from `altium_ui_catalog.json` (Phase 0) — this is the highest-leverage piece for 1:1 UI parity.
3. **Verify the importer** against real `.schdoc`/`.pcbdoc` files (add fixtures to `fixtures/`) and confirm round-trip.

These three artifacts (decompiled C + command registry + verified importer) de-risk the whole project before writing a single editor feature.

---

## Appendix A — Where the reverse-engineered assets live

| Asset | Path |
|---|---|
| UI command catalog (JSON) | `refernces/AltiumDesigner/decompiled/altium_ui_catalog.json` |
| X2.EXE strings | `refernces/AltiumDesigner/decompiled/X2.EXE.strings` |
| AltiumMS.exe strings | `refernces/AltiumDesigner/decompiled/AltiumMS.exe.strings` |
| Ghidra project: main editor | `/home/jrad/tools/ghidra_projects/altium_main` |
| Ghidra project: shell/manager | `/home/jrad/tools/ghidra_projects/altium_ms` |
| Ghidra project: core engine DLLs | `/home/jrad/tools/ghidra_projects/altium_analysis` |
| Decompilation output (to be generated) | `refernces/AltiumDesigner/decompiled/{X2.EXE,AltiumMS.exe}/` |

## Appendix B — Command namespace → Rust module mapping

| Altium namespace | Rust home |
|---|---|
| `PCB:*` | `oxide-app::editors::pcb` + `oxide-engine::command` |
| `@PLSCH:*` (interactive) | `oxide-app::editors::schematic` + `oxide-engine::command` |
| `InteractiveProperties:*` | `oxide-app::panels::element_properties` |
| `ExtensionManager:*` | `oxide-app::ui_catalog` (import/export extensions) |
| `PCB:Import/Export` | `oxide-altium-importer` + `oxide-output` |
| `WorkspaceManager:*` | `oxide-app::app::state` (project/workspace) |
| `CAMTASTIC:*` | `oxide-output` (DFM/CAM) |

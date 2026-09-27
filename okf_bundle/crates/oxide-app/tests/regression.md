---
okf_version: "0.2"
type: Module
title: regression
description: "Regression tests for v0.10–v0.12 walkthrough findings, split by"
resource: crates/oxide-app/tests/regression.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression
language: rust
---

# regression

Regression tests for v0.10–v0.12 walkthrough findings, split by

## Docstring

Regression tests for v0.10–v0.12 walkthrough findings, split by
subsystem (see #432 — the flat 6,475-line file grew append-site
conflicts on every parallel branch and had no answer to "where does
my test go").

These exercise dispatchers without spinning up the iced runtime —
`Oxide::new()` constructs the app, the test populates state
directly via the `pub` fields on `DocumentState` / `UiState`, then
`Oxide::update(Message::*)` routes through the same handler the
UI would. State changes (file system effects, `dirty_paths`,
tree state, etc.) are observed afterwards.

Closes the manual-walkthrough gap for items where the only
genuine UI dependency is the `rfd::AsyncFileDialog` picker — those
still need a human eye.

One test binary: `oxide-app` links `iced`/`wgpu`, so these stay
`mod`-included here rather than becoming separate
`tests/regression_*.rs` targets (each would be its own link step).
Each module below carries its own helpers and `use` lines — no
shared `support` module, since no helper is called from more than
one module (confirmed while doing the split).

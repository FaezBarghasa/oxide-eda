---
okf_version: "0.2"
type: Module
title: step_attach
description: STEP file attachment helper.
resource: crates/oxide-app/src/library/editor/footprint/step_attach.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/footprint/step_attach
language: rust
---

# step_attach

STEP file attachment helper.

## Docstring

STEP file attachment helper.

Handles the file-pick → SHA-256 → copy-into-`step/<hash>.step` flow
for [`oxide_library::StepAttachment`]. Also exposes the small
`view()` widget for the Footprint tab's Body 3D pane.

Per `v0.9-refactor-2-plan.md` §11 step F5: the attachment is
content-hashed so two MPNs sharing identical STEP geometry
de-duplicate to one file in `mylib.snxlib/step/`.

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/editor/footprint/step_attach/view.md) |
| related | [hash_hex](/crates/oxide-app/src/library/editor/footprint/step_attach/hash_hex.md) |
| related | [stash_step](/crates/oxide-app/src/library/editor/footprint/step_attach/stash_step.md) |
| related | [hash_hex_is_lowercase](/crates/oxide-app/src/library/editor/footprint/step_attach/hash_hex_is_lowercase.md) |
| related | [stash_step_writes_file_then_skips_on_dup](/crates/oxide-app/src/library/editor/footprint/step_attach/stash_step_writes_file_then_skips_on_dup.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
| related | [sha2](/_dependencies/cargo/sha2.md) |

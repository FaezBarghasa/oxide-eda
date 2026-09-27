---
okf_version: "0.2"
type: Function
title: stash_step
description: "Stash a freshly-uploaded STEP file under `<lib_root>/step/<hash>.step`"
resource: crates/oxide-app/src/library/editor/footprint/step_attach.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/footprint/step_attach/stash_step
language: rust
---

# stash_step

Stash a freshly-uploaded STEP file under `<lib_root>/step/<hash>.step`

## Signature

```rust
pub fn stash_step(lib_root: &Path, bytes: &[u8], filename: &str) -> Option<StepAttachment>
```

## Visibility

- `pub`

## Docstring

Stash a freshly-uploaded STEP file under `<lib_root>/step/<hash>.step`
and return the [`StepAttachment`] record to drop on the
`Footprint::step_attachment` field. Returns `None` when the IO
path fails (existing file collisions are NOT errors — content-hash
makes them no-ops by design).

## Source
Lines 149–179 in `crates/oxide-app/src/library/editor/footprint/step_attach.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [step_attach](/crates/oxide-app/src/library/editor/footprint/step_attach.md) |
| calls | [hash_hex](/crates/oxide-app/src/library/editor/footprint/step_attach/hash_hex.md) |
| calls | [atomic_write](/crates/oxide-app/src/app/dispatch/library/recovery/atomic_write.md) |
| called_by | [stash_step_writes_file_then_skips_on_dup](/crates/oxide-app/src/library/editor/footprint/step_attach/stash_step_writes_file_then_skips_on_dup.md) |

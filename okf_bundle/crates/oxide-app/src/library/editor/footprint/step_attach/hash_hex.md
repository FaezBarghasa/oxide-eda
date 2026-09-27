---
okf_version: "0.2"
type: Function
title: hash_hex
description: "SHA-256 hex of `bytes` — same routine the 3D upload flow uses, but"
resource: crates/oxide-app/src/library/editor/footprint/step_attach.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/footprint/step_attach/hash_hex
language: rust
---

# hash_hex

SHA-256 hex of `bytes` — same routine the 3D upload flow uses, but

## Signature

```rust
fn hash_hex(bytes: &[u8]) -> String
```

## Docstring

SHA-256 hex of `bytes` — same routine the 3D upload flow uses, but
kept private here so this module is self-contained.

## Source
Lines 124–142 in `crates/oxide-app/src/library/editor/footprint/step_attach.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [step_attach](/crates/oxide-app/src/library/editor/footprint/step_attach.md) |
| called_by | [hash_hex_is_lowercase](/crates/oxide-app/src/library/editor/footprint/step_attach/hash_hex_is_lowercase.md) |
| called_by | [stash_step](/crates/oxide-app/src/library/editor/footprint/step_attach/stash_step.md) |

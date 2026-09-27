---
okf_version: "0.2"
type: Function
title: create_custom_from_active
resource: crates/oxide-app/src/keymap/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/keymap/editor/create_custom_from_active
language: rust
---

# create_custom_from_active

## Signature

```rust
impl KeymapEditorModel { pub fn create_custom_from_active(
        &mut self,
        id: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<(), ProfileLoadError> }
```

## Visibility

- `pub`

## Source
Lines 102–113 in `crates/oxide-app/src/keymap/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/keymap/editor.md) |

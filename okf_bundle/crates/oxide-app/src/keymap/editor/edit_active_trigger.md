---
okf_version: "0.2"
type: Function
title: edit_active_trigger
resource: crates/oxide-app/src/keymap/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/keymap/editor/edit_active_trigger
language: rust
---

# edit_active_trigger

## Signature

```rust
impl KeymapEditorModel { pub fn edit_active_trigger(
        &mut self,
        command: AppCommandId,
        context: ShortcutContext,
        trigger_text: String,
    ) -> Result<(), ProfileLoadError> }
```

## Visibility

- `pub`

## Source
Lines 152–180 in `crates/oxide-app/src/keymap/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/keymap/editor.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |

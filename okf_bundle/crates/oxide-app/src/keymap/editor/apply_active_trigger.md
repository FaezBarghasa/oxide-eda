---
okf_version: "0.2"
type: Function
title: apply_active_trigger
resource: crates/oxide-app/src/keymap/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/keymap/editor/apply_active_trigger
language: rust
---

# apply_active_trigger

## Signature

```rust
impl KeymapEditorModel { fn apply_active_trigger(
        &mut self,
        command: AppCommandId,
        context: ShortcutContext,
        trigger: ShortcutTrigger,
    ) }
```

## Source
Lines 204–223 in `crates/oxide-app/src/keymap/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/keymap/editor.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [Command](/crates/oxide-engine/src/command/Command.md) |

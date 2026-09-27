---
okf_version: "0.2"
type: Function
title: proposed_for
description: "The proposed designator the preview promises for the sheet titled `sheet`."
resource: crates/oxide-app/src/app/view/dialogs/annotate_preview/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/proposed_for
language: rust
---

# proposed_for

The proposed designator the preview promises for the sheet titled `sheet`.

## Signature

```rust
fn proposed_for(app: &Oxide, sheet: &str) -> Option<String>
```

## Docstring

The proposed designator the preview promises for the sheet titled `sheet`.

## Source
Lines 88–93 in `crates/oxide-app/src/app/view/dialogs/annotate_preview/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [the_preview_promises_what_the_action_assigns](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/the_preview_promises_what_the_action_assigns.md) |

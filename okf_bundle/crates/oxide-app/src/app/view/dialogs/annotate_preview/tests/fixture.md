---
okf_version: "0.2"
type: Function
title: fixture
description: "A project listing only `a_root.snxsch`, which is open (and active) and"
resource: crates/oxide-app/src/app/view/dialogs/annotate_preview/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/fixture
language: rust
---

# fixture

A project listing only `a_root.snxsch`, which is open (and active) and

## Signature

```rust
fn fixture() -> (Oxide, PathBuf)
```

## Docstring

A project listing only `a_root.snxsch`, which is open (and active) and
references `z_child.snxsch` — a hierarchical child sitting unlisted and
unopened on disk. Both hold one unannotated `R?`. Plus a loose tab from
nowhere, also `R?`.

The names are deliberately adversarial to ordering: `a_root` sorts BEFORE
`z_child`, the opposite of what the old action's walk order gave the
active sheet (unopened sheets first, active engine unconditionally last).
A fixture where the active sheet already sorted last (as `top`/`child` did
before #435) can't tell sorted-path order and "active-last" apart — this
one can.

Returns the app and the temp dir (caller cleans up).

## Source
Lines 43–74 in `crates/oxide-app/src/app/view/dialogs/annotate_preview/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests.md) |
| calls | [sheet_with_net](/crates/oxide-app/src/app/handlers/menu/export/tests/sheet_with_net.md) |
| calls | [app_workspace](/crates/oxide-app/src/app/handlers/menu/export/tests/app_workspace.md) |
| calls | [open_with](/crates/oxide-app/src/app/handlers/menu/export/tests/open_with.md) |
| calls | [tab](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/tab.md) |
| called_by | [the_preview_omits_tabs_the_action_refuses_to_touch](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/the_preview_omits_tabs_the_action_refuses_to_touch.md) |
| called_by | [the_preview_promises_what_the_action_assigns](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/the_preview_promises_what_the_action_assigns.md) |

---
okf_version: "0.2"
type: Module
title: tests
description: The Annotate preview and the Annotate action must describe the same sheets
resource: crates/oxide-app/src/app/view/dialogs/annotate_preview/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate_preview/tests
language: rust
---

# tests

The Annotate preview and the Annotate action must describe the same sheets

## Docstring

The Annotate preview and the Annotate action must describe the same sheets
AND promise the same designator on each.

Round 5 repointed `handle_annotate` at the shared assembler and left the
preview on its own rule. The dialog the user approves and the operation that
runs then described *different sheet sets*, in both directions: the preview
hid the unlisted hierarchical children the action renumbers and saves to
disk, and listed loose tabs the action refuses to touch (#406).

Fixing the sheet *set* left the *order* still disagreeing: the preview
walked sorted-path order while the action walked cached tabs in tab order,
then unopened sheets sorted, then the active engine unconditionally last —
so a project whose active sheet doesn't happen to sort last got a preview
that promised one designator and an action that assigned another (#435).
[`crate::app::project_sheets::ordered_project_sheet_paths`] is now the one
walk both sides use.

These drive the real preview against the real handler over one fixture, so a
future change to either alone goes red.

## Relationships

| Type | Target |
|------|--------|
| related | [fixture](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/fixture.md) |
| related | [tab](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/tab.md) |
| related | [proposed_for](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/proposed_for.md) |
| related | [the_preview_promises_what_the_action_assigns](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/the_preview_promises_what_the_action_assigns.md) |
| related | [the_preview_omits_tabs_the_action_refuses_to_touch](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/the_preview_omits_tabs_the_action_refuses_to_touch.md) |

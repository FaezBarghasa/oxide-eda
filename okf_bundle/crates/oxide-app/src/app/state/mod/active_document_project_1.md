---
okf_version: "0.2"
type: Function
title: active_document_project
description: "Project that owns the *active document* — the scope for export, ERC,"
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/active_document_project_1
language: rust
---

# active_document_project

Project that owns the *active document* — the scope for export, ERC,

## Signature

```rust
pub fn active_document_project(&self) -> Option<&LoadedProject>
```

## Visibility

- `pub`

## Docstring

Project that owns the *active document* — the scope for export, ERC,
annotate and hierarchical child-sheet resolution.

Deliberately not `active_loaded_project()`: `active_project` is a
sticky workspace-wide pointer that survives focusing a tab with no
project of its own, so scoping off it makes an export of a loose
schematic emit another project's sheets (#406). Resolution rules and
the hierarchy walk live in [`scope::project_owning_sheet`].

- active schematic owned by a project → that project;
- active schematic owned by none (loose) → `None`, i.e. operate on
this one document;
- no active schematic at all (a PCB / symbol / footprint tab is
focused, or nothing is open) → the sticky pointer, which is what
"the workspace at large" means when there is no document to scope by.

## Source
Lines 674–679 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
| calls | [project_owning_sheet](/crates/oxide-app/src/app/state/scope/project_owning_sheet.md) |

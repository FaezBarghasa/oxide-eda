---
okf_version: "0.2"
type: Function
title: create_component_row
description: "Create a new component **row**:"
resource: crates/oxide-app/src/library/commands.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/commands/create_component_row
language: rust
---

# create_component_row

Create a new component **row**:

## Signature

```rust
pub fn create_component_row(
    state: &mut LibraryState,
    library_idx: usize,
    table: &str,
    internal_pn: &str,
    class: ComponentClass,
    symbol_ref: Option<PrimitiveRef>,
    footprint_ref: Option<PrimitiveRef>,
) -> Result<RowId, LibraryError>
```

## Visibility

- `pub`

## Docstring

Create a new component **row**:

1. Builds a [`ComponentRow`] with the user-supplied PN / class and
sentinel `Uuid::nil()` symbol/footprint refs (the primitive
binding is the user's explicit choice — picked from existing
`.snxsym` / `.snxfpt` files post-creation, never auto-minted).
2. Computes the canonical content hash via [`hash_row_content`].
3. Inserts the row into the chosen table via `adapter.insert_row`.

Returns the new row's `RowId` so the caller can open it as a
Component Preview tab via `LibraryMessage::OpenComponentRow`. The
preview's "Pick Symbol / Pick Footprint" affordance (Phase 2) lets
the user bind the primitives.

## Source
Lines 566–652 in `crates/oxide-app/src/library/commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [commands](/crates/oxide-app/src/library/commands.md) |
| calls | [hash_row_content](/crates/oxide-library/src/hash/hash_row_content.md) |
| called_by | [handle_browser_add_component](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_add_component.md) |
| called_by | [handle_new_component](/crates/oxide-app/src/app/dispatch/library/new_component/handle_new_component.md) |
| called_by | [handle_new_component_submit](/crates/oxide-app/src/app/dispatch/library/new_component/handle_new_component_submit.md) |

---
okf_version: "0.2"
type: Function
title: open_project_tree_document
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/open_document.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/open_document/open_project_tree_document_1
language: rust
---

# open_project_tree_document

## Signature

```rust
pub(super) fn open_project_tree_document(
        &mut self,
        tree_path: &[usize],
        filename: String,
    ) -> Result<Task<Message>>
```

## Visibility

- `pub(super)`

## Source
Lines 69–212 in `crates/oxide-app/src/app/handlers/dock/project_navigation/open_document.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [open_document](/crates/oxide-app/src/app/handlers/dock/project_navigation/open_document.md) |
| calls | [canonical_tree_label](/crates/oxide-app/src/app/handlers/dock/project_navigation/open_document/canonical_tree_label.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |

---
okf_version: "0.2"
type: Module
title: datasheet
description: Datasheet edits for a Component Preview row.
resource: crates/oxide-app/src/library/component_preview/updates/datasheet.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:28:52Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/datasheet
language: rust
---

# datasheet

Datasheet edits for a Component Preview row.

## Docstring

Datasheet edits for a Component Preview row.

Owns the Datasheet tab's three actions: switching between URL and
pinned-PDF modes, live URL entry, and applying an upload result by
content-hashing the picked bytes.

## Relationships

| Type | Target |
|------|--------|
| related | [set_mode](/crates/oxide-app/src/library/component_preview/updates/datasheet/set_mode.md) |
| related | [set_url](/crates/oxide-app/src/library/component_preview/updates/datasheet/set_url.md) |
| related | [apply_upload_result](/crates/oxide-app/src/library/component_preview/updates/datasheet/apply_upload_result.md) |
| related | [sha2](/_dependencies/cargo/sha2.md) |

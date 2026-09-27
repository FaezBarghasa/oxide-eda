---
okf_version: "0.2"
type: Function
title: ids_from_call
description: "Pull the first `\"...\"` string-literal argument that follows `call` in"
resource: crates/oxide-app/src/keymap/menu_command_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/keymap/menu_command_tests/ids_from_call
language: rust
---

# ids_from_call

Pull the first `"..."` string-literal argument that follows `call` in

## Signature

```rust
fn ids_from_call(src: &str, call: &str) -> Vec<String>
```

## Docstring

Pull the first `"..."` string-literal argument that follows `call` in
`src`, collecting one id per call site. Deliberately tiny (no regex
dependency): it walks to each occurrence of `call`, skips to the next
double-quote, and reads to the closing quote.

## Source
Lines 38–54 in `crates/oxide-app/src/keymap/menu_command_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_command_tests](/crates/oxide-app/src/keymap/menu_command_tests.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [every_menu_command_id_resolves_in_the_catalog](/crates/oxide-app/src/keymap/menu_command_tests/every_menu_command_id_resolves_in_the_catalog.md) |

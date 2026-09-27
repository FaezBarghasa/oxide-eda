---
okf_version: "0.2"
type: Function
title: append_library_symbols
resource: crates/oxide-app/src/app/handlers/dock/library_browser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/library_browser/append_library_symbols
language: rust
---

# append_library_symbols

## Signature

```rust
impl Oxide { fn append_library_symbols(
        library_path: &std::path::Path,
        library_name: &str,
        _library_entries: &mut Vec<crate::panels::LibrarySymbolEntry>,
        _loaded_symbols: &mut std::collections::HashMap<String, oxide_types::schematic::LibSymbol>,
    ) -> Result<()> }
```

## Source
Lines 109–125 in `crates/oxide-app/src/app/handlers/dock/library_browser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_browser](/crates/oxide-app/src/app/handlers/dock/library_browser.md) |
| calls | [log_warning](/crates/oxide-app/src/diagnostics/log_warning.md) |

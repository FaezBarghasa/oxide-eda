---
okf_version: "0.2"
type: Function
title: upload_texts
resource: crates/oxide-gfx/tests/regression_golden.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-gfx/tests/regression_golden/upload_texts_1
language: rust
---

# upload_texts

## Signature

```rust
fn upload_texts(
        &mut self,
        texts: &[TextItem],
        _params: TextUploadParams,
    ) -> Result<(), Self::TextError>
```

## Source
Lines 125–133 in `crates/oxide-gfx/tests/regression_golden.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [regression_golden](/crates/oxide-gfx/tests/regression_golden.md) |

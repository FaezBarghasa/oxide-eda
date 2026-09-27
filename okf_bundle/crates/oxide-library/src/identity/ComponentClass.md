---
okf_version: "0.2"
type: Class
title: ComponentClass
description: "Component class — picks the parameter template (\"resistor\", \"opamp\", …)."
resource: crates/oxide-library/src/identity.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/identity/ComponentClass
language: rust
---

# ComponentClass

Component class — picks the parameter template ("resistor", "opamp", …).

## Signature

```rust
pub struct ComponentClass
```

## Decorators

- `derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `serde(transparent)`

## Visibility

- `pub`

## Docstring

Component class — picks the parameter template ("resistor", "opamp", …).

Open string per `v0.9-refactor-2-plan.md` §4.1: users may add custom
classes; templates resolve dynamically through [`crate::TemplateRegistry`].
[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
[serde(transparent)]

## Source
Lines 115–115 in `crates/oxide-library/src/identity.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [identity](/crates/oxide-library/src/identity.md) |
| called_by | [component_row_json_roundtrip](/crates/oxide-library/src/component/component_row_json_roundtrip.md) |

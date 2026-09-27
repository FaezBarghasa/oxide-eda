---
okf_version: "0.2"
type: Function
title: build_ico
description: Bundle multiple native-rendered PNGs into a multi-size .ico.
resource: tools/build_icons.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:tools"
  - "domain:build_icons.py"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:00Z"
concept_id: tools/build_icons/build_ico
language: python
---

# build_ico

Bundle multiple native-rendered PNGs into a multi-size .ico.

## Signature

```python
def build_ico(pngs: dict[int, bytes], out: Path) -> None
```

## Docstring

Bundle multiple native-rendered PNGs into a multi-size .ico.

## Parameters

| Name | Type | Default |
|------|------|---------|
| `pngs` | `dict[int, bytes]` | `—` |

| `out` | `Path` | `—` |

## Returns
`None`

## Source
Lines 46–71 in `tools/build_icons.py`

## Relationships

| Type | Target |
|------|--------|
| related | [build_icons](/tools/build_icons.md) |
| calls | [pack](/crates/oxide-sketch/src/solver/state/pack.md) |
| called_by | [main](/tools/build_icons/main.md) |

# export

## Subdirectories

- [bom](bom/index.md)
- [mod](mod/index.md)
- [pdf_netlist](pdf_netlist/index.md)
- [print_preview](print_preview/index.md)
- [tests](tests/index.md)

## Modules

- [bom](bom.md) — BOM preview modal handlers. Split from `menu/export.rs`.
- [export](mod.md) — Export / print-preview menu handlers (PDF, netlist, BOM, print).
- [pdf_netlist](pdf_netlist.md) — Export handlers — PDF export dialog + netlist export. Split from `menu/export.rs`.
- [print_preview](print_preview.md) — Print-preview modal handlers. Split from `menu/export.rs`.
- [tests](tests.md) — Export scope regressions (#406) — asserted on the *emitted page set*.

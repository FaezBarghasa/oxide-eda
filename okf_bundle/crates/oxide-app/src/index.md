# src

## Subdirectories

- [active_bar](active_bar/index.md)
- [app](app/index.md)
- [canvas](canvas/index.md)
- [chrome](chrome/index.md)
- [config_root](config_root/index.md)
- [diagnostics](diagnostics/index.md)
- [dock](dock/index.md)
- [find_replace](find_replace/index.md)
- [first_run_tour](first_run_tour/index.md)
- [fonts](fonts/index.md)
- [icons](icons/index.md)
- [ignore](ignore/index.md)
- [keyboard_shortcuts_modal](keyboard_shortcuts_modal/index.md)
- [keymap](keymap/index.md)
- [library](library/index.md)
- [main](main/index.md)
- [menu_bar](menu_bar/index.md)
- [panels](panels/index.md)
- [passive_calculator_modal](passive_calculator_modal/index.md)
- [pcb_canvas](pcb_canvas/index.md)
- [preferences](preferences/index.md)
- [render_config](render_config/index.md)
- [renderer_scene_canvas](renderer_scene_canvas/index.md)
- [scene_shader](scene_shader/index.md)
- [schematic_runtime](schematic_runtime/index.md)
- [status_bar](status_bar/index.md)
- [styles](styles/index.md)
- [tab_bar](tab_bar/index.md)
- [test_support](test_support/index.md)
- [toolbar](toolbar/index.md)

## Modules

- [app](app.md) — Main Iced application — module root, update loop, view tree.
- [chrome](chrome.md) — OS-specific chrome polish for the borderless main window.
- [config_root](config_root.md) — Shared config-root resolver for oxide's on-disk preference files.
- [diagnostics](diagnostics.md)
- [feature_flags](feature_flags.md) — Compile-time constants for shipping incomplete subsystems dark.
- [find_replace](find_replace.md)
- [first_run_tour](first_run_tour.md) — First-run tour overlay — a single dismissible card pinned to the
- [icons](icons.md) — Central icon registry with runtime theme-aware tinting.
- [ignore](ignore.md) — One explicit form for "this `Result` is deliberately not actionable".
- [keyboard_shortcuts_modal](keyboard_shortcuts_modal.md) — Help ▸ Keyboard Shortcuts modal — single-page reference grouped by
- [lib](lib.md) — Oxide EDA — library face of the application binary.
- [passive_calculator_modal](passive_calculator_modal.md) — Tools ▸ Passive Network Calculator modal — the resistor / capacitor /
- [pcb_canvas](pcb_canvas.md)
- [render_config](render_config.md) — Render-configuration *types* for oxide-app — the enums the appearance
- [renderer_scene_canvas](renderer_scene_canvas.md)
- [scene_shader](scene_shader.md) — Generic GPU render path for any `oxide_gfx::scene::Scene`.
- [src](main.md) — Oxide EDA — AI-first electronics design automation.
- [status_bar](status_bar.md) — Bottom status bar — cursor position, grid, snap, layer, zoom, units.
- [styles](styles.md) — Custom Iced styles matching Altium Designer's dark theme chrome.
- [tab_bar](tab_bar.md) — Document tab bar — tabs for open schematic sheets and PCB.
- [test_support](test_support.md) — Test-only helpers for persistence tests (#416, #469, #482).
- [toolbar](toolbar.md) — Toolbar strip — tool buttons for schematic/PCB actions.

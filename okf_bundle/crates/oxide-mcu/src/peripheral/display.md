---
okf_version: "0.2"
type: Module
title: display
description: "Virtual Display & Touchscreen Simulation Subsystem."
resource: crates/oxide-mcu/src/peripheral/display.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:49:55Z"
concept_id: crates/oxide-mcu/src/peripheral/display
language: rust
---

# display

Virtual Display & Touchscreen Simulation Subsystem.

## Docstring

Virtual Display & Touchscreen Simulation Subsystem.

Simulates:
- **Character LCD** (HD44780 16x2 / 20x4 over Parallel or I2C PCF8574 Backpack)
- **Monochrome OLED / LCD** (SSD1306 128x64 / ST7565 / PCD8544 84x48 SPI/I2C)
- **Color TFT LCD & Display Controllers** (ILI9341 240x320, ST7789 240x240, GC9A01 Round Display, SSD1963 800x480 parallel RGB)
- **LED Dot Matrix / Seven Segment** (MAX7219 8x8, TM1637 4-digit 7-segment)
- **Touchscreen Controllers** (XPT2046 SPI Resistive Touch, FT6236 / GT911 I2C Capacitive Multi-touch)

## Relationships

| Type | Target |
|------|--------|
| related | [DisplayType](/crates/oxide-mcu/src/peripheral/display/DisplayType.md) |
| related | [TouchType](/crates/oxide-mcu/src/peripheral/display/TouchType.md) |
| related | [TouchEvent](/crates/oxide-mcu/src/peripheral/display/TouchEvent.md) |
| related | [DisplaySimulator](/crates/oxide-mcu/src/peripheral/display/DisplaySimulator.md) |
| related | [new_ssd1306_oled_128x64](/crates/oxide-mcu/src/peripheral/display/new_ssd1306_oled_128x64.md) |
| related | [new_ili9341_tft_touch_240x320](/crates/oxide-mcu/src/peripheral/display/new_ili9341_tft_touch_240x320.md) |
| related | [new_hd44780_lcd_16x2](/crates/oxide-mcu/src/peripheral/display/new_hd44780_lcd_16x2.md) |
| related | [set_pixel](/crates/oxide-mcu/src/peripheral/display/set_pixel.md) |
| related | [get_pixel](/crates/oxide-mcu/src/peripheral/display/get_pixel.md) |
| related | [inject_touch](/crates/oxide-mcu/src/peripheral/display/inject_touch.md) |
| related | [release_touch](/crates/oxide-mcu/src/peripheral/display/release_touch.md) |
| related | [clear](/crates/oxide-mcu/src/peripheral/display/clear.md) |
| related | [new_ssd1306_oled_128x64](/crates/oxide-mcu/src/peripheral/display/new_ssd1306_oled_128x64.md) |
| related | [new_ili9341_tft_touch_240x320](/crates/oxide-mcu/src/peripheral/display/new_ili9341_tft_touch_240x320.md) |
| related | [new_hd44780_lcd_16x2](/crates/oxide-mcu/src/peripheral/display/new_hd44780_lcd_16x2.md) |
| related | [set_pixel](/crates/oxide-mcu/src/peripheral/display/set_pixel.md) |
| related | [get_pixel](/crates/oxide-mcu/src/peripheral/display/get_pixel.md) |
| related | [inject_touch](/crates/oxide-mcu/src/peripheral/display/inject_touch.md) |
| related | [release_touch](/crates/oxide-mcu/src/peripheral/display/release_touch.md) |
| related | [clear](/crates/oxide-mcu/src/peripheral/display/clear.md) |
| related | [serde](/_dependencies/cargo/serde.md) |

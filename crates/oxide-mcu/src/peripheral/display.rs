//! Virtual Display & Touchscreen Simulation Subsystem.
//!
//! Simulates:
//! - **Character LCD** (HD44780 16x2 / 20x4 over Parallel or I2C PCF8574 Backpack)
//! - **Monochrome OLED / LCD** (SSD1306 128x64 / ST7565 / PCD8544 84x48 SPI/I2C)
//! - **Color TFT LCD & Display Controllers** (ILI9341 240x320, ST7789 240x240, GC9A01 Round Display, SSD1963 800x480 parallel RGB)
//! - **LED Dot Matrix / Seven Segment** (MAX7219 8x8, TM1637 4-digit 7-segment)
//! - **Touchscreen Controllers** (XPT2046 SPI Resistive Touch, FT6236 / GT911 I2C Capacitive Multi-touch)

use serde::{Deserialize, Serialize};

/// Display Architecture / Controller Type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplayType {
    /// HD44780 Character LCD (16x2 or 20x4)
    Hd44780Character { rows: u8, cols: u8 },
    /// SSD1306 Monochrome OLED (128x64 or 128x32)
    Ssd1306Oled { width: u32, height: u32 },
    /// ST7789 Color TFT (240x240 or 135x240)
    St7789Tft { width: u32, height: u32 },
    /// ILI9341 Color TFT (240x320)
    Ili9341Tft { width: u32, height: u32 },
    /// GC9A01 Round Color TFT (240x240)
    Gc9a01RoundTft { diameter: u32 },
    /// MAX7219 LED Matrix (8x8 cascades)
    Max7219LedMatrix { cascades: u8 },
    /// TM1637 7-Segment Display (4 digits)
    Tm1637SevenSegment { digits: u8 },
}

/// Touchscreen Input Type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TouchType {
    None,
    ResistiveXpt2046,
    CapacitiveFt6236,
    CapacitiveGt911,
}

/// Active Touch Event on Display surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TouchEvent {
    pub x: u16,
    pub y: u16,
    pub pressed: bool,
    pub finger_id: u8,
}

/// Universal Virtual Display Simulation Buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplaySimulator {
    pub name: String,
    pub display_type: DisplayType,
    pub touch_type: TouchType,
    pub width: u32,
    pub height: u32,
    /// Pixel Framebuffer stored as 32-bit RGBA8888 (Width * Height * 4 bytes).
    pub framebuffer_rgba: Vec<u8>,
    pub backlight_brightness: u8, // 0..255
    pub backlight_on: bool,
    /// Active touch input state.
    pub active_touch: Option<TouchEvent>,
    /// Character buffer for HD44780 / 7-Segment displays.
    pub text_buffer: String,
}

impl DisplaySimulator {
    pub fn new_ssd1306_oled_128x64() -> Self {
        let width = 128;
        let height = 64;
        Self {
            name: "SSD1306 128x64 OLED".to_string(),
            display_type: DisplayType::Ssd1306Oled { width, height },
            touch_type: TouchType::None,
            width,
            height,
            framebuffer_rgba: vec![0x00; (width * height * 4) as usize],
            backlight_brightness: 255,
            backlight_on: true,
            active_touch: None,
            text_buffer: String::new(),
        }
    }

    pub fn new_ili9341_tft_touch_240x320() -> Self {
        let width = 240;
        let height = 320;
        Self {
            name: "ILI9341 240x320 Color TFT (Touch)".to_string(),
            display_type: DisplayType::Ili9341Tft { width, height },
            touch_type: TouchType::ResistiveXpt2046,
            width,
            height,
            framebuffer_rgba: vec![0x00; (width * height * 4) as usize],
            backlight_brightness: 255,
            backlight_on: true,
            active_touch: None,
            text_buffer: String::new(),
        }
    }

    pub fn new_hd44780_lcd_16x2() -> Self {
        Self {
            name: "HD44780 16x2 Character LCD".to_string(),
            display_type: DisplayType::Hd44780Character { rows: 2, cols: 16 },
            touch_type: TouchType::None,
            width: 160,
            height: 32,
            framebuffer_rgba: vec![0x00; (160 * 32 * 4) as usize],
            backlight_brightness: 255,
            backlight_on: true,
            active_touch: None,
            text_buffer: "                \n                ".to_string(),
        }
    }

    /// Set pixel color at (x, y).
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if x < self.width && y < self.height {
            let offset = ((y * self.width + x) * 4) as usize;
            self.framebuffer_rgba[offset] = r;
            self.framebuffer_rgba[offset + 1] = g;
            self.framebuffer_rgba[offset + 2] = b;
            self.framebuffer_rgba[offset + 3] = a;
        }
    }

    /// Read pixel color at (x, y).
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<(u8, u8, u8, u8)> {
        if x < self.width && y < self.height {
            let offset = ((y * self.width + x) * 4) as usize;
            Some((
                self.framebuffer_rgba[offset],
                self.framebuffer_rgba[offset + 1],
                self.framebuffer_rgba[offset + 2],
                self.framebuffer_rgba[offset + 3],
            ))
        } else {
            None
        }
    }

    /// Injects touch screen interaction event.
    pub fn inject_touch(&mut self, x: u16, y: u16, pressed: bool) {
        if self.touch_type != TouchType::None {
            self.active_touch = Some(TouchEvent {
                x,
                y,
                pressed,
                finger_id: 0,
            });
        }
    }

    /// Clears active touch state.
    pub fn release_touch(&mut self) {
        self.active_touch = None;
    }

    /// Clears the framebuffer.
    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, r, g, b, 255);
            }
        }
    }
}

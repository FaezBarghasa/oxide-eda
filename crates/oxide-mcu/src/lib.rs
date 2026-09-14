//! `oxide-mcu` — Virtual Microcontroller (QEMU STM32/Cortex-M) Co-Simulation Engine.
//!
//! Provides:
//! - [`FirmwareImage`] & [`McuFamily`]: Firmware inspection, ELF header validation, and architecture mapping.
//! - [`QemuConfig`] & [`QemuInstance`]: Managed QEMU child process lifecycle, GDB stub bridge, and socket routing.
//! - [`PinBridge`], [`VirtualPinState`], & [`LogicLevel`]: Real-time synchronization between virtual GPIOs/ADCs/DACs and SPICE schematic nets.
//! - [`VirtualUart`]: Bi-directional UART serial terminal buffer.

pub mod firmware;
pub mod pin_bridge;
pub mod qemu;
pub mod uart;

pub use firmware::{FirmwareError, FirmwareImage, McuFamily, MemorySegment};
pub use pin_bridge::{LogicLevel, PinBridge, PinFunction, VirtualPinState};
pub use qemu::{QemuConfig, QemuError, QemuInstance};
pub use uart::VirtualUart;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_bridge_logic_and_adc() {
        let mut bridge = PinBridge::new(3.3);
        bridge.register_pin("PA0", PinFunction::AdcInput { channel: 0 }, Some("NET_SENSOR".to_string()));
        bridge.register_pin("PC13", PinFunction::GpioOutput, Some("NET_LED".to_string()));

        // Test ADC reading at 1.65V (half scale)
        bridge.update_from_spice_voltage("PA0", 1.65);
        let adc_val = bridge.read_adc_raw_12bit("PA0").unwrap();
        assert!((adc_val as i32 - 2048).abs() <= 2);

        // Test GPIO output driving voltage
        bridge.set_gpio_output("PC13", LogicLevel::High);
        let pin = bridge.pins.get("PC13").unwrap();
        assert_eq!(pin.analog_voltage, 3.3);
    }

    #[test]
    fn test_virtual_uart_loopback() {
        let mut uart = VirtualUart::new(115200);
        uart.write_from_terminal("HELLO\n");

        let mut received = Vec::new();
        while let Some(b) = uart.read_byte_for_mcu() {
            received.push(b);
            uart.push_from_mcu(b);
        }

        assert_eq!(&received, b"HELLO\n");
        assert_eq!(uart.history_lines, vec!["HELLO".to_string()]);
    }
}

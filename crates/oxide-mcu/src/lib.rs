//! `oxide-mcu` — Virtual ARM Microcontroller & Comprehensive Peripheral Co-Simulation Engine.
//!
//! Provides:
//! - [`FirmwareImage`], [`ArmArch`], [`ArmCore`], & [`McuFamily`]: Architecture profiling, core mapping, and SoC family presets.
//! - [`peripheral`]: Complete hardware peripheral emulation suite (NVIC, SysTick, GPIO, Timers/PWM, ADC, DAC, COMP, UART, SPI, I2C, DMA, CAN, Ethernet, USB, Watchdogs, RTC, CRC, RNG).
//! - [`PinBridge`], [`VirtualPinState`], & [`LogicLevel`]: Real-time synchronization between virtual MCU pins and SPICE schematic nets.
//! - [`QemuConfig`] & [`QemuInstance`]: Managed QEMU supervisor and GDB remote debugging stub.
//! - [`VirtualUart`]: Bi-directional UART serial terminal buffer.

pub mod firmware;
pub mod peripheral;
pub mod pin_bridge;
pub mod qemu;
pub mod uart;

pub use firmware::{ArmArch, ArmCore, FirmwareError, FirmwareImage, McuFamily, MemorySegment};
pub use pin_bridge::{LogicLevel, PinBridge, PinFunction, VirtualPinState};
pub use qemu::{QemuConfig, QemuError, QemuInstance};
pub use uart::VirtualUart;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arm_core_and_architecture_mapping() {
        // Test all Cortex-M series
        assert_eq!(ArmCore::CortexM0.arch(), ArmArch::ArmV6M);
        assert_eq!(ArmCore::CortexM0Plus.arch(), ArmArch::ArmV6M);
        assert_eq!(ArmCore::CortexM1.arch(), ArmArch::ArmV6M);
        assert_eq!(ArmCore::CortexM3.arch(), ArmArch::ArmV7M);
        assert_eq!(ArmCore::CortexM4.arch(), ArmArch::ArmV7EM);
        assert_eq!(ArmCore::CortexM4F.arch(), ArmArch::ArmV7EM);
        assert_eq!(ArmCore::CortexM7.arch(), ArmArch::ArmV7EM);
        assert_eq!(ArmCore::CortexM23.arch(), ArmArch::ArmV8MBaseline);
        assert_eq!(ArmCore::CortexM33.arch(), ArmArch::ArmV8MMainline);
        assert_eq!(ArmCore::CortexM55.arch(), ArmArch::ArmV81MMainline);
        assert_eq!(ArmCore::CortexM85.arch(), ArmArch::ArmV81MMainline);

        // Test Cortex-R and Cortex-A
        assert_eq!(ArmCore::CortexR5.arch(), ArmArch::ArmV7R);
        assert_eq!(ArmCore::CortexR52.arch(), ArmArch::ArmV8R);
        assert_eq!(ArmCore::CortexA53.arch(), ArmArch::ArmV8A);

        // Test QEMU CPU flags
        assert_eq!(ArmCore::CortexM0.qemu_cpu(), "cortex-m0");
        assert_eq!(ArmCore::CortexM0Plus.qemu_cpu(), "cortex-m0plus");
        assert_eq!(ArmCore::CortexM3.qemu_cpu(), "cortex-m3");
        assert_eq!(ArmCore::CortexM33.qemu_cpu(), "cortex-m33");
        assert_eq!(ArmCore::CortexM7.qemu_cpu(), "cortex-m7");
        assert_eq!(ArmCore::CortexM4F.qemu_cpu(), "cortex-m4");
    }

    #[test]
    fn test_soc_family_presets() {
        assert_eq!(McuFamily::Stm32F0.core(), ArmCore::CortexM0);
        assert_eq!(McuFamily::Rp2040.core(), ArmCore::CortexM0Plus);
        assert_eq!(McuFamily::Stm32F1.core(), ArmCore::CortexM3);
        assert_eq!(McuFamily::Stm32F4.core(), ArmCore::CortexM4F);
        assert_eq!(McuFamily::Stm32F7.core(), ArmCore::CortexM7);
        assert_eq!(McuFamily::Stm32U5.core(), ArmCore::CortexM33);
        assert_eq!(McuFamily::Rp2350.core(), ArmCore::CortexM33);
        assert_eq!(McuFamily::Nrf52.core(), ArmCore::CortexM4F);
    }

    #[test]
    fn test_peripheral_suite() {
        use peripheral::*;

        // 1. NVIC & SysTick
        let mut nvic = Nvic::new();
        nvic.enable_irq(15);
        nvic.set_pending(15);
        nvic.set_priority(15, 2);
        assert_eq!(nvic.get_highest_pending_irq(), Some(15));

        let mut systick = SysTick::new();
        systick.enabled = true;
        systick.tick_interrupt = true;
        systick.reload_value = 1000;
        systick.current_value = 500;
        assert!(systick.step_cycles(600));

        // 2. GPIO & BSRR
        let mut gpioa = GpioPort::new("GPIOA");
        gpioa.write_bsrr(0x0000_0005); // Set Pin 0 and Pin 2 High
        assert_eq!(gpioa.pins[0].output_data, LogicLevel::High);
        assert_eq!(gpioa.pins[1].output_data, LogicLevel::Low);
        assert_eq!(gpioa.pins[2].output_data, LogicLevel::High);

        // 3. Timers & PWM
        let mut tim1 = TimerPeripheral::new("TIM1");
        tim1.enabled = true;
        tim1.auto_reload = 1000;
        tim1.set_pwm_channel(0, 500); // 50% duty cycle
        assert_eq!(tim1.channels[0].duty_cycle, 0.5);

        // 4. ADC & DAC
        let mut adc1 = AdcPeripheral::new("ADC1", 3.3);
        adc1.set_channel_voltage(0, 1.65);
        let raw_adc = adc1.convert_channel(0);
        assert!((raw_adc as i32 - 2048).abs() <= 2);

        let mut dac1 = DacPeripheral::new("DAC1", 3.3);
        dac1.channels[0].enabled = true;
        dac1.write_channel_12bit(0, 2048);
        assert!((dac1.get_output_voltage(0) - 1.65).abs() < 0.01);

        // 5. UART & SPI & I2C
        let mut uart1 = UartPeripheral::new("USART1", 115200);
        uart1.write_byte(0x55);
        assert_eq!(uart1.pop_tx_byte(), Some(0x55));

        let mut spi1 = SpiPeripheral::new("SPI1");
        spi1.inject_rx(0xAA);
        assert_eq!(spi1.transfer_byte(0x12), 0xAA);

        let mut i2c1 = I2cPeripheral::new("I2C1", 0x50);
        assert!(i2c1.send_address(0x50, false));

        // 6. DMA & CAN & CRC & RNG
        let mut dma1 = DmaController::new("DMA1", 8);
        dma1.start_transfer(0, 0x2000_0000, 0x4000_0000, 100);
        dma1.step_transfers(60);
        assert!(dma1.channels[0].half_transfer_complete);

        let mut can1 = CanPeripheral::new("CAN1");
        let frame = CanFrame {
            id: 0x123,
            is_extended_id: false,
            is_fd: true,
            is_rtr: false,
            payload: vec![1, 2, 3, 4],
        };
        assert!(can1.transmit(frame));

        let mut crc = CrcPeripheral::new();
        let res = crc.feed_word(0x1234_5678);
        assert_ne!(res, 0xFFFF_FFFF);

        let mut rng = RngPeripheral::new(42);
        assert!(rng.next_u32().is_some());
    }

    #[test]
    fn test_pin_bridge_logic_and_adc() {
        let mut bridge = PinBridge::new(3.3);
        bridge.register_pin("PA0", PinFunction::AdcInput { channel: 0 }, Some("NET_SENSOR".to_string()));
        bridge.register_pin("PC13", PinFunction::GpioOutput, Some("NET_LED".to_string()));

        bridge.update_from_spice_voltage("PA0", 1.65);
        let adc_val = bridge.read_adc_raw_12bit("PA0").unwrap();
        assert!((adc_val as i32 - 2048).abs() <= 2);

        bridge.set_gpio_output("PC13", LogicLevel::High);
        let pin = bridge.pins.get("PC13").unwrap();
        assert_eq!(pin.analog_voltage, 3.3);
    }
}

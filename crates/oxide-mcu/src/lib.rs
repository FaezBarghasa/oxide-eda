//! `oxide-mcu` — Virtual ARM Microcontroller & Comprehensive Peripheral Co-Simulation Engine.
//!
//! Provides:
//! - [`FirmwareImage`], [`ArmArch`], [`ArmCore`], & [`McuFamily`]: Architecture profiling, core mapping, and SoC family presets.
//! - [`peripheral`]: Complete hardware peripheral emulation suite (NVIC, SysTick, GPIO, Timers/PWM, ADC, DAC, COMP, UART, SPI, I2C, DMA, CAN, Ethernet, USB, Watchdogs, RTC, CRC, RNG).
//! - [`PinBridge`], [`VirtualPinState`], & [`LogicLevel`]: Real-time synchronization between virtual MCU pins and SPICE schematic nets.
//! - [`QemuConfig`] & [`QemuInstance`]: Managed QEMU supervisor and GDB remote debugging stub.
//! - [`VirtualUart`]: Bi-directional UART serial terminal buffer.

pub mod arch;
pub mod firmware;
pub mod peripheral;
pub mod pin_bridge;
pub mod qemu;
pub mod storage;
pub mod uart;

pub use arch::{ArchClass, CoreProfile, McuVendor, MemoryModel};
pub use firmware::{ArmArch, ArmCore, FirmwareError, FirmwareImage, McuFamily, McuTarget, MemorySegment};
pub use pin_bridge::{LogicLevel, PinBridge, PinFunction, VirtualPinState};
pub use qemu::{QemuConfig, QemuError, QemuInstance};
pub use storage::{I2cEeprom, ParallelSram, SpiEeprom, SpiFlash, SpiRam};
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
    fn test_multi_vendor_mcu_targets() {
        // 1. Espressif ESP32 (Xtensa & RISC-V)
        let esp32_prof = McuTarget::Esp32.profile();
        assert_eq!(esp32_prof.arch, ArchClass::Xtensa);
        assert_eq!(esp32_prof.vendor, McuVendor::Espressif);
        assert_eq!(esp32_prof.num_cores, 2);
        assert_eq!(esp32_prof.qemu_executable, "qemu-system-xtensa");

        let esp32c3_prof = McuTarget::Esp32C3.profile();
        assert_eq!(esp32c3_prof.arch, ArchClass::RiscV);
        assert_eq!(esp32c3_prof.vendor, McuVendor::Espressif);
        assert_eq!(esp32c3_prof.qemu_executable, "qemu-system-riscv32");

        // 2. Microchip / Atmel AVR & PIC
        let uno_prof = McuTarget::ATmega328P.profile();
        assert_eq!(uno_prof.arch, ArchClass::Avr8);
        assert_eq!(uno_prof.vendor, McuVendor::MicrochipAtmel);
        assert_eq!(uno_prof.bit_width, 8);
        assert_eq!(uno_prof.qemu_executable, "qemu-system-avr");
        match uno_prof.memory_model {
            MemoryModel::HarvardSplit { flash_word_size_bytes, sram_size_bytes, eeprom_size_bytes } => {
                assert_eq!(flash_word_size_bytes, 32768);
                assert_eq!(sram_size_bytes, 2048);
                assert_eq!(eeprom_size_bytes, 1024);
            },
            _ => panic!("Expected HarvardSplit memory model for AVR"),
        }

        let pic_prof = McuTarget::Pic18F4550.profile();
        assert_eq!(pic_prof.arch, ArchClass::Pic8);
        assert_eq!(pic_prof.vendor, McuVendor::MicrochipAtmel);

        // 3. RISC-V (WCH & SiFive & RP2350 Hazard3)
        let ch32_prof = McuTarget::Ch32V003.profile();
        assert_eq!(ch32_prof.arch, ArchClass::RiscV);
        assert_eq!(ch32_prof.vendor, McuVendor::Wch);
        assert_eq!(ch32_prof.qemu_executable, "qemu-system-riscv32");

        let rp2350_rv = McuTarget::Rp2350RiscV.profile();
        assert_eq!(rp2350_rv.arch, ArchClass::RiscV);
        assert_eq!(rp2350_rv.vendor, McuVendor::RaspberryPi);

        // 4. STMicroelectronics STM32 (Full Spectrum)
        let stm32f4_prof = McuTarget::Stm32F4.profile();
        assert_eq!(stm32f4_prof.arch, ArchClass::Arm);
        assert_eq!(stm32f4_prof.vendor, McuVendor::StMicroelectronics);
        assert_eq!(stm32f4_prof.qemu_executable, "qemu-system-arm");

        // 5. NXP Semiconductors
        let s32k_prof = McuTarget::NxpS32K144.profile();
        assert_eq!(s32k_prof.arch, ArchClass::Arm);
        assert_eq!(s32k_prof.vendor, McuVendor::NxpSemiconductors);

        let imx_prof = McuTarget::NxpImxRt1060.profile();
        assert_eq!(imx_prof.arch, ArchClass::Arm);
        assert_eq!(imx_prof.max_frequency_hz, 600_000_000);

        // 6. Texas Instruments (MSP430 & C2000)
        let msp_prof = McuTarget::Msp430G2553.profile();
        assert_eq!(msp_prof.arch, ArchClass::Msp430);
        assert_eq!(msp_prof.vendor, McuVendor::TexasInstruments);
        assert_eq!(msp_prof.bit_width, 16);

        let c2000_prof = McuTarget::C2000Tms320F28379D.profile();
        assert_eq!(c2000_prof.arch, ArchClass::C2000);
        assert_eq!(c2000_prof.vendor, McuVendor::TexasInstruments);
        assert_eq!(c2000_prof.num_cores, 2);
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

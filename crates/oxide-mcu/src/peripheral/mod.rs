//! Peripheral Emulation Subsystem for Virtual ARM MCUs.
//!
//! Provides models for:
//! - [`nvic::Nvic`]: Nested Vectored Interrupt Controller (priority grouping & dispatch)
//! - [`systick::SysTick`]: 24-bit RTOS tick timer
//! - [`gpio::GpioPort`]: Multi-port GPIO (A..K with MODER, OTYPER, PUPDR, BSRR)
//! - [`timer::TimerPeripheral`]: Basic, General-Purpose, and Advanced Timers with PWM & dead-time
//! - [`adc::AdcPeripheral`]: Multi-channel 8..16-bit SAR ADC with SPICE net voltage sampling
//! - [`dac::DacPeripheral`]: Dual 12-bit DAC with waveform generators
//! - [`comparator::ComparatorPeripheral`]: Fast analog comparators with hysteresis
//! - [`uart::UartPeripheral`]: USART/UART/LPUART with fractional baud & FIFOs
//! - [`spi::SpiPeripheral`]: SPI master/slave & QSPI/OSPI flash controllers
//! - [`i2c::I2cPeripheral`]: I2C/SMBus master/slave with clock stretching
//! - [`dma::DmaController`]: Multi-stream DMA with circular buffer mode
//! - [`can::CanPeripheral`]: CAN 2.0A/B & CAN-FD controllers with mailboxes
//! - [`ethernet::EthernetMacPeripheral`]: 10/100M Ethernet MAC with DMA ring
//! - [`usb::UsbDevicePeripheral`]: USB Device controller with EP0..EP8
//! - [`watchdog::WatchdogPeripheral`]: Independent (IWDG) & Window (WWDG) watchdogs
//! - [`rtc::RtcPeripheral`]: Real-Time Clock with BCD calendar & alarms
//! - [`crc::CrcPeripheral`]: Hardware CRC-32/16/8 computation engine
//! - [`rng::RngPeripheral`]: Hardware True / Pseudo Random Number Generator

pub mod display;
pub mod nvic;
pub mod systick;
pub mod gpio;
pub mod timer;
pub mod adc;
pub mod dac;
pub mod comparator;
pub mod uart;
pub mod spi;
pub mod i2c;
pub mod dma;
pub mod can;
pub mod ethernet;
pub mod usb;
pub mod watchdog;
pub mod rtc;
pub mod crc;
pub mod rng;

pub use display::{DisplaySimulator, DisplayType, TouchEvent, TouchType};
pub use nvic::Nvic;
pub use systick::SysTick;
pub use gpio::{GpioMode, GpioPinState, GpioPort, GpioPull, GpioSpeed, GpioType};
pub use timer::{TimerChannelMode, TimerCounterMode, TimerPeripheral};
pub use adc::{AdcChannelConfig, AdcConversionMode, AdcPeripheral, AdcResolution};
pub use dac::{DacChannelMode, DacPeripheral, DacWaveform};
pub use comparator::{ComparatorHysteresis, ComparatorOutputPolarity, ComparatorPeripheral};
pub use uart::{Parity, StopBits, UartPeripheral, WordLength};
pub use spi::{SpiDataSize, SpiMode, SpiPeripheral, SpiRole};
pub use i2c::{I2cAddressingMode, I2cPeripheral, I2cSpeedMode};
pub use dma::{DmaChannel, DmaController, DmaDirection, DmaPriority};
pub use can::{CanFilterBank, CanFilterMode, CanFrame, CanPeripheral};
pub use ethernet::{EthernetFrame, EthernetMacPeripheral};
pub use usb::{UsbDevicePeripheral, UsbEndpoint, UsbEndpointType};
pub use watchdog::{Iwdg, WatchdogPeripheral, Wwdg};
pub use rtc::{RtcAlarm, RtcCalendarTime, RtcPeripheral};
pub use crc::{CrcAlgorithm, CrcPeripheral};
pub use rng::RngPeripheral;

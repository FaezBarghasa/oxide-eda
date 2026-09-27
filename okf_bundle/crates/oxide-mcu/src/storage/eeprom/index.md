# eeprom

## Classs

- [I2cEeprom](I2cEeprom.md) — I2C EEPROM Model (e.g. Microchip 24LC04, 24LC64, 24LC256, 24LC512, 24LC1025).
- [SpiEeprom](SpiEeprom.md) — SPI EEPROM Model (e.g. Microchip 25LC040, 25LC640, 25LC256, 25LC1024).
- [SpiEepromState](SpiEepromState.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]

## Functions

- [chip_deselect](chip_deselect.md) — Pull CS High: Ends transaction and commits page writes.
- [chip_deselect](chip_deselect_1.md) — Pull CS High: Ends transaction and commits page writes.
- [handle_i2c_read](handle_i2c_read.md) — Handles sequential I2C read from current internal address.
- [handle_i2c_read](handle_i2c_read_1.md) — Handles sequential I2C read from current internal address.
- [handle_i2c_write](handle_i2c_write.md) — Handles incoming I2C write transaction (address + data).
- [handle_i2c_write](handle_i2c_write_1.md) — Handles incoming I2C write transaction (address + data).
- [new_24lc04](new_24lc04.md)
- [new_24lc04](new_24lc04_1.md)
- [new_24lc256](new_24lc256.md)
- [new_24lc256](new_24lc256_1.md)
- [new_25lc256](new_25lc256.md)
- [new_25lc256](new_25lc256_1.md)
- [transfer_byte](transfer_byte.md) — Full-duplex SPI byte exchange.
- [transfer_byte](transfer_byte_1.md) — Full-duplex SPI byte exchange.

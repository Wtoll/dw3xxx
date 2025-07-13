//! Helper functions for performing SPI transactions
//! 

use embedded_hal::spi::{Operation, SpiDevice};

pub mod headers;
use headers::{short_addressed_header, full_addressed_header, fast_command_header, masked_write_header, AccessMode};

pub mod commands;
use commands::Command;

use crate::reg::bindings::{ReadableField, ReadableRegister, WritableField, WritableRegister};

/// Performs a fast command transaction
pub fn fast_command<SPI: SpiDevice>(spi: &mut SPI, command: Command) -> Result<(), SPI::Error> {
    spi.transaction(&mut [
        Operation::Write(&fast_command_header(command))
    ])
}

/// Performs a short addressed read transaction
pub fn short_addressed_read<SPI: SpiDevice>(spi: &mut SPI, base_address: u8, buf: &mut [u8]) -> Result<(), SPI::Error> {
    spi.transaction(&mut [
        Operation::Write(&short_addressed_header(base_address, AccessMode::Read)),
        Operation::Read(buf)
    ])
}

/// Performs a short addressed write transaction
pub fn short_addressed_write<SPI: SpiDevice>(spi: &mut SPI, base_address: u8, buf: &[u8]) -> Result<(), SPI::Error> {
    spi.transaction(&mut [
        Operation::Write(&short_addressed_header(base_address, AccessMode::Read)),
        Operation::Write(buf)
    ])
}

/// Performs a full addressed read transaction
pub fn full_addressed_read<SPI: SpiDevice>(spi: &mut SPI, base_address: u8, sub_address: u8, buf: &mut [u8]) -> Result<(), SPI::Error> {
    spi.transaction(&mut [
        Operation::Write(&full_addressed_header(base_address, sub_address, AccessMode::Read)),
        Operation::Read(buf)
    ])
}

/// Performs a full addressed write transaction
pub fn full_addressed_write<SPI: SpiDevice>(spi: &mut SPI, base_address: u8, sub_address: u8, buf: &[u8]) -> Result<(), SPI::Error> {
    spi.transaction(&mut [
        Operation::Write(&full_addressed_header(base_address, sub_address, AccessMode::Read)),
        Operation::Write(buf)
    ])
}

/// Performs an 8-bit masked write transaction
pub fn masked_write_8_bit<SPI: SpiDevice>(spi: &mut SPI, base_address: u8, sub_address: u8, and_mask: &[u8; 1], or_mask: &[u8; 1]) -> Result<(), SPI::Error> {
    spi.transaction(&mut [
        Operation::Write(&masked_write_header(base_address, sub_address, headers::MaskedWriteMode::EightBit)),
        Operation::Write(and_mask),
        Operation::Write(or_mask)
    ])
}

/// Performs a 16-bit masked write transaction
pub fn masked_write_16_bit<SPI: SpiDevice>(spi: &mut SPI, base_address: u8, sub_address: u8, and_mask: &[u8; 2], or_mask: &[u8; 2]) -> Result<(), SPI::Error> {
    spi.transaction(&mut [
        Operation::Write(&masked_write_header(base_address, sub_address, headers::MaskedWriteMode::SixteenBit)),
        Operation::Write(and_mask),
        Operation::Write(or_mask)
    ])
}

/// Performs a 32-bit masked write transaction
pub fn masked_write_32_bit<SPI: SpiDevice>(spi: &mut SPI, base_address: u8, sub_address: u8, and_mask: &[u8; 4], or_mask: &[u8; 4]) -> Result<(), SPI::Error> {
    spi.transaction(&mut [
        Operation::Write(&masked_write_header(base_address, sub_address, headers::MaskedWriteMode::ThirtyTwoBit)),
        Operation::Write(and_mask),
        Operation::Write(or_mask)
    ])
}

/// A thin wrapper over an spi device with helper functions for performing transactions
pub struct SpiWrapper<SPI: SpiDevice> {
    spi: SPI
}

impl<SPI: SpiDevice> SpiWrapper<SPI> {
    /// Creates a new instance of an [`SpiWrapper`]
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Decomposes an instance of an [`SpiWrapper`]
    pub fn decompose(self) -> SPI {
        self.spi
    }

    /// Provides mutable access to the underlying [`SpiDevice`]
    pub fn device(&mut self) -> &mut SPI {
        &mut self.spi
    }

    /// Performs a fast command transaction
    pub fn fast_command(&mut self, command: Command) -> Result<(), SPI::Error> {
        fast_command(&mut self.spi, command)
    }

    /// Performs a short read transaction
    pub fn short_addressed_read(&mut self, base_address: u8, buf: &mut [u8]) -> Result<(), SPI::Error> {
        short_addressed_read(&mut self.spi, base_address, buf)
    }

    /// Performs a short write transaction
    pub fn short_addressed_write(&mut self, base_address: u8, buf: &[u8]) -> Result<(), SPI::Error> {
        short_addressed_write(&mut self.spi, base_address, buf)
    }

    /// Performs a full addressed read transaction
    pub fn full_addressed_read(&mut self, base_address: u8, sub_address: u8, buf: &mut [u8]) -> Result<(), SPI::Error> {
        full_addressed_read(&mut self.spi, base_address, sub_address, buf)
    }

    /// Performs a full addressed write transaction
    pub fn full_addressed_write(&mut self, base_address: u8, sub_address: u8, buf: &[u8]) -> Result<(), SPI::Error> {
        full_addressed_write(&mut self.spi, base_address, sub_address, buf)
    }

    /// Performs an 8-bit masked write transaction
    pub fn masked_write_8_bit(&mut self, base_address: u8, sub_address: u8, and_mask: &[u8; 1], or_mask: &[u8; 1]) -> Result<(), SPI::Error> {
        masked_write_8_bit(&mut self.spi, base_address, sub_address, and_mask, or_mask)
    }

    /// Performs a 16-bit masked write transaction
    pub fn masked_write_16_bit(&mut self, base_address: u8, sub_address: u8, and_mask: &[u8; 2], or_mask: &[u8; 2]) -> Result<(), SPI::Error> {
        masked_write_16_bit(&mut self.spi, base_address, sub_address, and_mask, or_mask)
    }

    /// Performs a 32-bit masked write transaction
    pub fn masked_write_32_bit(&mut self, base_address: u8, sub_address: u8, and_mask: &[u8; 4], or_mask: &[u8; 4]) -> Result<(), SPI::Error> {
        masked_write_32_bit(&mut self.spi, base_address, sub_address, and_mask, or_mask)
    }
}
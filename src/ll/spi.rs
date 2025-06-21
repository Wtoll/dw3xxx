//! Helper functions for performing SPI transactions with the DW3XXX
//! 
//! # Transaction Helper Functions
//! 
//! This module includes a series of helper functions for performing SPI transactions with the device.
//! 
//! # Header Helper Functions
//! 
//! This module also includes a series of helper functions for generating SPI transaction headers for all of the device's functionality.
//! Each of the four main transaction types (fast command, short addressed, full addressed, and masked write) have their own functions
//! to generate an SPI header for that transaction type.
//! 
//! It should be noted that these functions only generate the headers for each transaction and do not in any way handle sending the headers
//! or the variable transaction data that must be read or written immediately following the headers.
//! 
//! ## Fast Command
//! To generate a header for a fast command transaction you may use the helper function [`fast_command`] as follows:
//! ```rust
//! # use dw3xxx::ll::{spi::fast_command_header, commands::Command};
//! let header: [u8; 1] = fast_command_header(Command::ClrIrqs);
//! ```
//! 
//! ## Short Addressed Transaction
//! To generate a header for a short addressed transaction you may use the helper function [`short_addressed`] as follows:
//! ```rust
//! # use dw3xxx::ll::spi::{short_addressed_header, AccessMode};
//! let header: [u8; 1] = short_addressed_header(0x12, AccessMode::Read);
//! ```
//! 
//! ## Full Addressed Transaction
//! To generate a header for a full addressed transaction you may use the helper function [`full_addressed`] as follows:
//! ```rust
//! # use dw3xxx::ll::spi::{full_addressed_header, AccessMode};
//! let header: [u8; 2] = full_addressed_header(0x05, 0x2C, AccessMode::Read);
//! ```
//! 
//! ## Masked Write Transaction
//! To generate a header for a masked write transaction you may use the helper function [`masked_write`] as follows:
//! ```rust
//! # use dw3xxx::ll::spi::{masked_write_header, MaskedWriteMode};
//! let header: [u8; 2] = masked_write_header(0x05, 0x10, MaskedWriteMode::EightBit);
//! ```
//! 
//! # Transaction Header Builder
//! 
//! For the vast majority of cases the helper functions should be sufficient for all of the device's functionality, however, this
//! module also contains a [`TransactionHeaderBuilder`] that can be used to dynamically create transaction headers if desired.
//! 
//! As a warning, the methods on the builder attempt to suggest best practices for creating transaction headers, but do not strictly
//! enforce them outside of setting the correct header length bit when [`TransactionHeaderBuilder::build`] is called. As such it is
//! possible to create transaction headers that may cause unintended functionality if the proper encoding is not used. For more
//! information on transaction header encoding please see the [DW3000 User Manual](https://www.qorvo.com/products/d/da008154)
//! section 2.3.1 "The SPI interface".
//! 
//! Additionally, many of the methods on [`TransactionHeaderBuilder`] have counterparts for clearing, writing, and setting the bit ranges
//! of each respective header. The "write" methods simply perform a bitwise or operation to set the relevant bits in each range, and as such
//! assume that the ranges begin completely zeroed out. If this is not known to be the case, the "clear" methods can be used to return each
//! respective bit range to 0 before writing a new value. Alternatively, the "set" methods may be used instead and are equivalent to first
//! clearing a bit range and then writing to that bit range immediately after.
//! 

use crate::ll::commands::Command;

/// Execute a fast command
pub fn fast_command() {
    todo!()
}

/// Perform a short addressed SPI transaction.
pub fn short_addressed() {
    todo!()
}

/// Perform a full addressed SPI transaction.
pub fn full_addressed() {
    todo!()
}

/// Perform a masked write SPI transaction.
pub fn masked_write() {
    todo!()
}

/// 
/// Generates a header for a fast command SPI transaction.
/// 
/// ```rust
/// # use dw3xxx::ll::{commands::Command, spi::fast_command_header};
/// assert_eq!(fast_command_header(Command::ClrIrqs), [0b10100101]);
/// ```
/// 
/// # Transaction Format
/// All fast command operations have a one octet header in the following form
/// 
/// | MSB |     |     |     |     |     |     | LSB |
/// |-----|-----|-----|-----|-----|-----|-----|-----|
/// |  0  |  1  |  2  |  3  |  4  |  5  |  6  |  7  |
/// |  1  |  0  |  X  |  X  |  X  |  X  |  X  |  1  |
/// 
/// Where X is the 5-bit fast command code.
/// 
pub fn fast_command_header(command: Command) -> [u8; 1] {
    let mut builder = TransactionHeaderBuilder::new_fast_command();
    
    builder.write_fast_command_type(command);

    builder.build()
}

/// 
/// Generates a header for a short addressed SPI transaction.
/// 
/// ```rust
/// # use dw3xxx::ll::spi::{short_addressed_header, AccessMode};
/// // 0x12 is the register file ID for RX_BUFFER_0
/// assert_eq!(short_addressed_header(0x12, AccessMode::Read), [0b00100100]);
/// // 0x14 is the register file ID for TX_BUFFER
/// assert_eq!(short_addressed_header(0x14, AccessMode::Write), [0b10101000]);
/// ```
/// 
/// # Transaction Format
/// All short addressed operations have a one octet header in the following form
/// 
/// | MSB |     |     |     |     |     |     | LSB |
/// |-----|-----|-----|-----|-----|-----|-----|-----|
/// |  0  |  1  |  2  |  3  |  4  |  5  |  6  |  7  |
/// |  A  |  0  |  B  |  B  |  B  |  B  |  B  |  0  |
/// 
/// Where A is the access mode (read (0) or write (1)) for the operation, and B is the 5-bit base address of the register file (register file ID).
/// 
pub fn short_addressed_header(base_address: u8, mode: AccessMode) -> [u8; 1] {
    let mut builder = TransactionHeaderBuilder::new_short();

    builder
        .write_access_mode(mode)
        .write_base_address(base_address);

    builder.build()
}

/// 
/// Generates a header for a full addressed SPI transaction.
/// 
/// ```rust
/// # use dw3xxx::ll::spi::{full_addressed_header, AccessMode};
/// // 0x05 is the register file ID for GPIO_CTRL (GPIO control register) and 0x2C is the sub-register for GPIO_RAW (GPIO raw state).
/// assert_eq!(full_addressed_header(0x05, 0x2C, AccessMode::Read), [0b01001010, 0b10110000]);
/// // 0x05 is the register file ID for GPIO_CTRL (GPIO control register) and 0x10 is the sub-register for GPIO_IRQE (GPIO interrupt enable).
/// assert_eq!(full_addressed_header(0x05, 0x10, AccessMode::Write), [0b11001010, 0b01000000]);
/// ```
/// 
/// # Transaction Format
/// All full addressed operations have a two octet header in the following form
/// 
/// | MSB | ___ | ___ | ___ | ___ | ___ | ___ | LSB | MSB | ___ | ___ | ___ | ___ | ___ | ___ | LSB |
/// |:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
/// |  0  |  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9  |  10 |  11 |  12 |  13 |  14 |  15 |
/// |  A  |  1  |  B  |  B  |  B  |  B  |  B  |  S  |  S  |  S  |  S  |  S  |  S  |  S  |  0  |  0  |
/// 
/// Where A is the access mode (read (0) or write (1)) for the operation, B is the 5-bit base address of the register file (register file ID),
/// and S is the 7-bit sub-address (octet offset index) for the field within the register file.
/// 
#[allow(unused_parens)]
pub fn full_addressed_header(base_address: u8, sub_address: u8, mode: AccessMode) -> [u8; 2] {
    let mut builder = TransactionHeaderBuilder::new_full();

    builder
        .write_base_address(base_address)
        .write_sub_address(sub_address)
        .write_access_mode(mode);

    builder.build()
}

/// 
/// Generates a header for a masked write SPI transaction.
/// 
/// ```rust
/// # use dw3xxx::ll::spi::{masked_write_header, full_addressed_header, MaskedWriteMode, AccessMode};
/// // 0x05 is the register file ID for GPIO_CTRL (GPIO control register) and 0x10 is the sub-register for GPIO_IRQE (GPIO interrupt enable).
/// assert_eq!(masked_write_header(0x05, 0x10, MaskedWriteMode::EightBit), [0b11001010, 0b01000001]);
/// assert_eq!(masked_write_header(0x05, 0x10, MaskedWriteMode::SixteenBit), [0b11001010, 0b01000010]);
/// assert_eq!(masked_write_header(0x05, 0x10, MaskedWriteMode::ThirtyTwoBit), [0b11001010, 0b01000011]);
/// 
/// // An unmasked "masked write" transaction is equivalent to a full addressed write transaction.
/// assert_eq!(masked_write_header(0x05, 0x10, MaskedWriteMode::Unmasked), full_addressed_header(0x05, 0x10, AccessMode::Write));
/// ```
/// 
/// # Transaction Format
/// All masked-write operations have a two octet header in the following form
/// 
/// | MSB | ___ | ___ | ___ | ___ | ___ | ___ | LSB | MSB | ___ | ___ | ___ | ___ | ___ | ___ | LSB |
/// |:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
/// |  0  |  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9  |  10 |  11 |  12 |  13 |  14 |  15 |
/// |  1  |  1  |  B  |  B  |  B  |  B  |  B  |  S  |  S  |  S  |  S  |  S  |  S  |  S  |  M  |  M  |
/// 
/// Where B is the 5-bit base address of the register file (register file ID), S is the 7-bit sub-address (octet offset index)
/// for the field within the register file, and M is the 2-bit length mode value (0b01 for 8-bit, 0b10 for 16-bit, and 0b11 for 32-bit).
/// 
#[allow(unused_parens)]
pub fn masked_write_header(base_address: u8, sub_address: u8, mode: MaskedWriteMode) -> [u8; 2] {
    let mut builder = TransactionHeaderBuilder::new_full();

    builder
        .write_base_address(base_address)
        .write_sub_address(sub_address)
        .write_access_mode(AccessMode::Write)
        .write_operation_mode(mode);

    builder.build()
}

/// The access mode of a short addressed or full addressed SPI transaction.
pub enum AccessMode {
    Read  = 0b0,
    Write = 0b1,
}

/// The write length mode for the masked write SPI transaction.
pub enum MaskedWriteMode {
    Unmasked     = 0b00,
    EightBit     = 0b01,
    SixteenBit   = 0b10,
    ThirtyTwoBit = 0b11
}

#[derive(Clone)]
pub struct TransactionHeaderBuilder<const N: usize> {
    inner: [u8; N]
}

impl<const N: usize> TransactionHeaderBuilder<N> {
    /// Clears the access mode bit of an SPI transaction header.
    pub const fn clear_access_mode(&mut self) -> &mut Self {
        self.inner[0] &= 0b01111111;

        self
    }

    /// Writes the access mode bit of an SPI transaction header.
    /// 
    /// It should be noted that this method does not clear the bit beforehand.
    /// Instead see [`set_access_mode`](TransactionHeaderBuilder::set_access_mode).
    #[allow(unused_parens)]
    pub const fn write_access_mode(&mut self, mode: AccessMode) -> &mut Self {
        self.inner[0] |= (((mode as u8) & 0b00000001) << 7);

        self
    }

    /// Sets the access mode bit of an SPI transaction header to the given access mode.
    pub const fn set_access_mode(&mut self, mode: AccessMode) -> &mut Self {
        self
            .clear_access_mode()
            .write_access_mode(mode)
    }

    /// Clears the type bit of an SPI transaction header.
    pub const fn clear_type_bit(&mut self) -> &mut Self {
        self.inner[0] &= 0b11111110;

        self
    }

    /// Writes the type bit of an SPI transaction header.
    /// 
    /// It should be noted that this method does not clear the bit beforehand.
    /// Instead see [`set_type_bit`](TransactionHeaderBuilder::set_type_bit).
    #[allow(unused_parens)]
    pub const fn write_type_bit(&mut self, bit: u8) -> &mut Self {
        self.inner[0] |= (bit & 0b00000001);

        self
    }

    /// Sets the type bit of an SPI transaction header.
    pub const fn set_type_bit(&mut self, bit: u8) -> &mut Self {
        self
            .clear_type_bit()
            .write_type_bit(bit)
    }

    /// Clears the middle five bits of an SPI transaction header.
    pub const fn clear_five_bits(&mut self) -> &mut Self {
        self.inner[0] &= 0b11000001;
        self
    }

    /// Writes the middle five bits of an SPI transaction header.
    /// 
    /// It should be noted that this method does not clear the bits beforehand.
    /// Instead see [`set_five_bits`](TransactionHeaderBuilder::set_five_bits).
    #[allow(unused_parens)]
    pub const fn write_five_bits(&mut self, bits: u8) -> &mut Self {
        self.inner[0] |= ((bits & 0b00011111) << 1);
        self
    }

    /// Sets the middle five bits of an SPI transaction header.
    pub const fn set_five_bits(&mut self, bits: u8) -> &mut Self {
        self
            .clear_five_bits()
            .write_five_bits(bits)
    }

    /// Clears the base address of an SPI transaction header.
    pub const fn clear_base_address(&mut self) -> &mut Self {
        self.clear_five_bits()
    }

    /// Writes the base address of an SPI transaction header.
    /// 
    /// It should be noted that this method does not clear the bits beforehand.
    /// Instead see [`set_base_address`](TransactionHeaderBuilder::set_base_address).
    #[allow(unused_parens)]
    pub const fn write_base_address(&mut self, address: u8) -> &mut Self {
        self.write_five_bits(address)
    }

    /// Sets the base address of an SPI transaction header.
    pub const fn set_base_address(&mut self, address: u8) -> &mut Self {
        self
            .clear_base_address()
            .write_base_address(address)
    }
}

impl TransactionHeaderBuilder<1> {
    /// An empty fast command type SPI transaction header cached at compile time
    pub const EMPTY_FAST_COMMAND: TransactionHeaderBuilder<1> = TransactionHeaderBuilder::empty_fast_command();

    /// Creates a new, empty, short SPI transaction header.
    pub const fn new_short() -> Self {
        Self { inner: [0b00000000] }
    }

    /// Creates a new, empty, fast command SPI transaction header.
    /// 
    /// This method is used to evaluate the constant [`Self::EMPTY_FAST_COMMAND`] at compile time.
    const fn empty_fast_command() -> Self {
        let mut header = Self::new_short();

        header
            .write_access_mode(AccessMode::Write)
            .write_type(true);

        header
    }

    /// Clones a new, empty, fast command SPI transaction header.
    /// 
    /// This method simply clones from [`Self::EMPTY_FAST_COMMAND`] for the sake of speed. 
    pub fn new_fast_command() -> Self {
        Self::EMPTY_FAST_COMMAND.clone()
    }

    /// Clears the type bit of a short SPI transaction header.
    pub const fn clear_type(&mut self) -> &mut Self {
        self.clear_type_bit()
    }

    /// Writes the type bit of a short SPI transaction header.
    /// 
    /// It should be noted that this method does not clear the bit beforehand.
    /// Instead see [`set_type`](TransactionHeaderBuilder::set_type).
    pub const fn write_type(&mut self, bit: bool) -> &mut Self {
        self.write_type_bit(bit as u8)
    }

    /// Sets the transaction type bit of a short SPI transaction header.
    pub const fn set_type(&mut self, bit: bool) -> &mut Self {
        self.set_type_bit(bit as u8)
    }

    /// Clears the fast command type of a short SPI transaction header.
    pub const fn clear_fast_command_type(&mut self) -> &mut Self {
        self.clear_five_bits()
    }

    /// Writes the fast command type of a short SPI transaction header.
    /// 
    /// It should be noted that this method does not clear the bits beforehand. Instead see
    /// [`set_fast_command_type`](TransactionHeaderBuilder::set_fast_command_type). Additionally, this method only writes the middle five
    /// bits of the short SPI transaction header to the provided command, and does not properly format the rest of the header with the
    /// correct access mode or transaction type. Instead see [`write_fast_command`](TransactionHeaderBuilder::write_fast_command).
    pub const fn write_fast_command_type(&mut self, command: Command) -> &mut Self {
        self.write_five_bits(command as u8)
    }

    /// Sets the fast command type of a short SPI transaction header.
    /// 
    /// It should be noted that this method only sets the middle five bits of the short SPI transaction header to the provided command,
    /// and does not properly format the rest of the header with the correct access mode or transaction type. Instead see
    /// [`set_fast_command`](TransactionHeaderBuilder::set_fast_command).
    pub const fn set_fast_command_type(&mut self, command: Command) -> &mut Self {
        self.set_five_bits(command as u8)
    }

    /// Clears the fast command of a short SPI transaction header. Additionally, for correctness, clears the access mode and the
    /// transaction type.
    pub const fn clear_fast_command(&mut self) -> &mut Self {
        self
            .clear_access_mode()
            .clear_fast_command_type()
            .clear_type()
    }

    /// Writes the fast command of a short SPI transaction header. Additionally, for correctness, writes the access mode to [`AccessMode::Write`],
    /// and the transaction type bit to 1 (for a fast command type transaction).
    /// 
    /// It should be noted that this method does not clear the bits beforehand. Instead see
    /// [`set_fast_command`](TransactionHeaderBuilder::set_fast_command).
    pub const fn write_fast_command(&mut self, command: Command) -> &mut Self {
        self
            .write_access_mode(AccessMode::Write)
            .write_fast_command_type(command)
            .write_type(true)
    }

    /// Sets the fast command of a short SPI transaction header. Additionally, for correctness, sets the access mode to [`AccessMode::Write`],
    /// and the transaction type bit to 1 (for a fast command type transaction).
    pub const fn set_fast_command(&mut self, command: Command) -> &mut Self {
        self
            .set_access_mode(AccessMode::Write)
            .set_fast_command_type(command)
            .set_type(true)
    }

    /// Builds the short SPI transaction header. Additionally, for correctness, sets the transaction header
    /// length bit to 0 (for a short SPI transaction header).
    pub const fn build(mut self) -> [u8; 1] {
        self.inner[0] &= 0b10111111;
        self.inner
    }
}

impl TransactionHeaderBuilder<2> {
    /// Creates a new, empty, full SPI transaction header.
    pub const fn new_full() -> Self {
        Self { inner: [0b01000000, 0b00000000] }
    }

    /// Clears the operation mode of a full SPI transaction header.
    pub const fn clear_operation_mode(&mut self) -> &mut Self {
        self.inner[1] &= 0b11111100;

        self
    }

    /// Writes the operation mode of a full SPI transaction header.
    /// 
    /// It should be noted that this method does not clear the bits beforehand.
    /// Instead see [`set_operation_mode`](TransactionHeaderBuilder::set_operation_mode).
    #[allow(unused_parens)]
    pub const fn write_operation_mode(&mut self, mode: MaskedWriteMode) -> &mut Self {
        self.inner[1] |= ((mode as u8) & 0b00000011);

        self
    }

    /// Sets the operation mode of a full SPI transaction header.
    pub const fn set_operation_mode(&mut self, mode: MaskedWriteMode) -> &mut Self {
        self
            .clear_operation_mode()
            .write_operation_mode(mode)
    }

    /// Clears the sub-address of a full SPI transaction header.
    pub const fn clear_sub_address(&mut self) -> &mut Self {
        self.clear_type_bit();
        self.inner[1] &= 0b00000011;

        self
    }

    /// Writes the sub-address of a full SPI transaction header.
    /// 
    /// It should be noted that this method does not clear the bits beforehand.
    /// Instead see [`set_sub_address`](TransactionHeaderBuilder::set_sub_address).
    #[allow(unused_parens)]
    pub const fn write_sub_address(&mut self, mut sub_address: u8) -> &mut Self {
        sub_address &= 0b01111111;
        self.write_type_bit(sub_address >> 6);
        self.inner[1] |= (sub_address << 2);

        self
    }

    /// Sets the sub-address of a full SPI transaction header.
    pub const fn set_sub_address(&mut self, sub_address: u8) -> &mut Self {
        self
            .clear_sub_address()
            .write_sub_address(sub_address)
    }

    /// Builds the full SPI transaction header. Additionally, for correctness, sets the transaction header
    /// length bit to 1 (for a full SPI transaction header).
    pub const fn build(mut self) -> [u8; 2] {
        self.inner[0] |= 0b01000000;
        self.inner
    }
}
use embedded_hal::spi::SpiDevice;

use crate::reg::bindings::ReadableRegister;

use super::spi::{SpiWrapper, commands::Command};

pub struct DW3XXX<SPI: SpiDevice> {
    spi: SpiWrapper<SPI>
}

impl<SPI: SpiDevice> DW3XXX<SPI> {
    /// Constructs a new instance of a DW3XXX driver.
    pub fn new(spi: SPI) -> Self {
        Self {
            spi: SpiWrapper::new(spi)
        }
    }

    /// Decomposes an instance of a DW3XXX driver into its component singletons.
    pub fn decompose(self) -> (SPI, ) {
        (self.spi.decompose(), )
    }

    /// Provides mutable access to the underlying [`SpiWrapper`]
    pub fn wrapper(&mut self) -> &mut SpiWrapper<SPI> {
        &mut self.spi
    }

    /// Clears all interrupts for the device
    pub async fn clear_interrupts(&mut self) -> Result<(), FastCommandError> {
        // Coverage for CLR_IRQS

        self.spi.fast_command(Command::ClrIrqs).expect("Encountered an SPI error");

        todo!()
    }

    /// Toggles the double buffer pointer.
    pub async fn toggle_buffer(&mut self) -> Result<(), FastCommandError> {
        // Coverage for DB_TOGGLE

        todo!()
    }

    /// Forces the device back into the idle state.
    pub async fn force_idle(&mut self) -> Result<(), FastCommandError> {
        // Coverage for TXRXOFF

        todo!()
    }

    /// Immediately receives.
    pub async fn receive(&mut self) -> Result<ReceiverFrame, ReceiveCommandError> {
        // Coverage for RX

        todo!()
    }

    /// Immediately transmits.
    pub async fn transmit(&mut self) -> Result<(), FastCommandError> {
        // Coverage for TX

        todo!()
    }

    /// Transmits and then receives.
    pub async fn transmit_receive(&mut self) -> Result<ReceiverFrame, TransmitReceiveCommandError> {
        // Coverage for TX_W4R

        todo!()
    }

    /// Listens for a preamble, and if one is not found, transmits.
    pub async fn listen_transmit(&mut self) -> Result<(), FastCommandError> {
        // Coverage for CCA_TX

        todo!()
    }

    /// Listens for a preamble, and if one is not found, transmits then receives.
    pub async fn listen_transmit_receive(&mut self) -> Result<ReceiverFrame, TransmitReceiveCommandError> {
        // Coverage for CCA_TX_W4R

        todo!()
    }

    /// Receives after a delay.
    pub async fn delayed_receive(&mut self, kind: TransceiverDelay) -> Result<ReceiverFrame, ReceiveCommandError> {
        // Coverage for:
        //  - DRX
        //  - DRX_TS
        //  - DRX_RS
        //  - DRX_REF

        todo!()
    }

    /// Transmits after a delay.
    pub async fn delayed_transmit(&mut self, kind: TransceiverDelay) -> Result<(), FastCommandError> {
        // Coverage for:
        //  - DTX
        //  - DTX_TS
        //  - DTX_RS
        //  - DTX_REF

        todo!()
    }

    /// Transmits after a delay and then receives.
    pub async fn delayed_transmit_receive(&mut self, kind: TransceiverDelay) -> Result<ReceiverFrame, TransmitReceiveCommandError> {
        // Coverage for:
        //  - DTX_W4R
        //  - DTX_TS_W4R
        //  - DTX_RS_W4R
        //  - DTX_REF_W4R

        todo!()
    }
}

/// The baseline from which a delayed transceiver operation is measured from.
pub enum TransceiverDelay {
    /// Calculates the delay as an absolute value.
    Absolute, // Regular variant
    /// Calculates the delay with reference to the last RX time.
    LastRx, // RS variant
    /// Calculates the delay with reference to the last TX time.
    LastTx, // TS variant
    /// Calculates the delay with reference to the value in the DREF_TIME register.
    Internal // REF variant
}

/// An error resulting from the [`transmit_receive`](DW3XXX::transmit_receive), [`listen_transmit_receive`](DW3XXX::listen_transmit_receive),
/// and [`delayed_transmit_receive`](DW3XXX::delayed_transmit_receive) methods.
pub enum TransmitReceiveCommandError {
    /// One of the receiver related errors.
    /// 
    /// See [`ReceiverError`].
    ReceiverError(ReceiverError),
    /// One of the fast command related errors.
    /// 
    /// See [`FastCommandError`].
    CommandError(FastCommandError)
}

/// An error resulting from the [`receive`](DW3XXX::receive) and [`delayed_receive`](DW3XXX::delayed_receive) methods.
pub enum ReceiveCommandError {
    /// One of the receiver related errors.
    /// 
    /// See [`ReceiverError`].
    ReceiverError(ReceiverError),
    /// One of the fast command related errors.
    /// 
    /// See [`FastCommandError`].
    CommandError(FastCommandError)
}

/// An error resulting from receiver operations
pub enum ReceiverError {
    PreambleTimeout,        // Coverage for RXPTO
    PreambleRejection,      // Coverage for RXPREJ
    SfdTimeout,             // Coverage for RXSTO
    FrameTimeout,           // Coverage for RXFTO
    PhrDecodeError,         // Coverage for RXPHE
    ReedSolomonDecodeError, // Coverage for RXFSL
    CiaTimeout,             // Coverage for CIAERR
    /// Receiver double buffer overrun.
    /// 
    /// This error will only ever occur when the optional double buffering functionality is enabled.
    DoubleBufferOverrun     // Coverage for RXOVRR
}

/// A data frame resulting from receiver operations
pub enum ReceiverFrame {
    /// A received frame that has passed the CRC check.
    Ok(),     // Coverage for RXFCG
    /// A received frame that has not passed the CRC check.
    Partial() // Coverage for RXFCE
}

/// An error resulting from fast command SPI transactions.
pub enum FastCommandError {
    /// One of the SPI related errors.
    /// 
    /// See [`SpiError`].
    SpiError(SpiError),
    /// An error executing a fast command.
    /// 
    /// Usually from attempting to execute fast commands in too quick of succession.
    FastCommandError // Coverage for CMD_ERR
}

/// An error resulting from SPI transactions.
pub enum SpiError {
    /// SPI transaction failed the CRC check.
    /// 
    /// This error will only ever occur when the optional SPI CRC functionality is enabled.
    CrcError,       // Coverage for SPICRCE
    /// SPI Overflow.
    OverflowError,  // Coverage for SPI_OVF
    /// SPI Underflow.
    UnderflowError, // Coverage for SPI_UNF
    /// SPI collision from internal contention with the device.
    CollisionError  // Coverage for SPIERR
}
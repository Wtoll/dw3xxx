//! Helper functions for executing fast commands
//! 

/// A fast command
pub enum Command {
    /// `CMD_TXRXOFF`
    /// Put the device into `IDLE` state and clear any events.
    TxRxOff   = 0x00,
    /// `CMD_TX`
    /// Immediately begin transmitting.
    Tx        = 0x01,
    /// `CMD_RX`
    /// Immediately being receiving.
    Rx        = 0x02,
    /// `CMD_DTX`
    /// Delayed transmission at `DX_TIME`.
    Dtx       = 0x03,
    /// `CMD_DRX`
    /// Delayed reception at `DX_TIME`.
    Drx       = 0x04,
    /// `CMD_DTX_TS`
    /// Delayed transmission at {last Tx timestamp} + `DX_TIME`.
    DtxTs     = 0x05,
    /// `CMD_DRX_TS`
    /// Delayed reception at {last Tx timestamp} + `DX_TIME`.
    DrxTs     = 0x06,
    /// `CMD_DTX_RS`
    /// Delayed transmission at {last Rx timestamp} + `DX_TIME`.
    DtxRs     = 0x07,
    /// `CMD_DRX_RS`
    /// Delayed reception at {last Rx timestamp} + `DX_TIME`.
    DrxRs     = 0x08,
    /// `CMD_DTX_REF`
    /// Delayed transmission at `DREF_TIME` + `DX_TIME`.
    DtxRef    = 0x09,
    /// `CMD_DRX_REF`
    /// Delayed reception at `DREF_TIME` + `DX_TIME`.
    DrxRef    = 0x0A,
    /// `CMD_CCA_TX`
    /// Transmit a packet if no preamble is detected.
    CcaTx     = 0x0B,
    /// `CMD_TX_W4R`
    /// Immediately begin transmitting, then wait to receive for a response.
    TxW4r     = 0x0C,
    /// `CMD_DTX_W4R`
    /// Delayed transmission at `DX_TIME`, then wait to receive for a response.
    DtxW4r    = 0x0D,
    /// `CMD_DTX_TS_W4R`
    /// Delayed transmission at {last Tx timestamp} + `DX_TIME`, then wait to receive for a response.
    DtxTsW4r  = 0x0E,
    /// `CMD_DTX_RS_W4R`
    /// Delayed transmission at {last Rx timestamp} + `DX_TIME`, then wait to receive for a response.
    DtxRsW4r  = 0x0F,
    /// `CMD_DTX_REF_W4R`
    /// Delayed transmission at `DREF_TIME` + `DX_TIME`, then wait to receive for a response.
    DtxRefW4r = 0x10,
    /// `CMD_CCA_TX_W4R`
    /// Transmit a packet if no preamble is detected, then wait to receive for a response.
    CcaTxW4r  = 0x11,
    /// `CMD_CLR_IRQS`
    /// Clear all interrupt events.
    ClrIrqs   = 0x12,
    /// `CMD_DB_TOGGLE`
    /// Toggle the double buffer pointer and notify the device that the host has finished processing the received buffer/data.
    DbToggle  = 0x13
}
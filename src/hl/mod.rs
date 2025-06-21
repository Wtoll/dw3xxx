/// Clears all interrupts
pub fn clear_interrupts() {
    // Coverage for CLR_IRQS
}

/// Toggles the double buffer pointer
pub fn toggle_buffer() {
    // Coverage for DB_TOGGLE
}

/// Forces the device back into the idle state
pub fn force_idle() {
    // Coverage for TXRXOFF
}

/// Receives
pub fn receive() {
    // Coverage for RX
}

/// Transmits
pub fn transmit() {
    // Coverage for TX
}

/// Transmits and then receives
pub fn transmit_receive() {
    // Coverage for TX_W4R
}

/// Listens for a preamble, and if one is not found, transmits.
pub fn listen_transmit() {
    // Coverage for CCA_TX
}

/// Listens for a preamble, and if one is not found, transmits then receives.
pub fn listen_transmit_receive() {
    // Coverage for CCA_TX_W4R
}

/// The baseline from which a delayed transceiver operation is measured from
pub enum TransceiverDelay {
    /// Calculate the delay as an absolute value
    Absolute, // Regular variant
    /// Calculate the delay with reference to the last RX time
    LastRx, // RS variant
    /// Calculate the delay with reference to the last TX time
    LastTx, // TS variant
    /// Calculate the delay with reference to the value in the DREF_TIME register
    Internal // REF variant
}

/// Receives after a delay
pub fn delayed_receive(kind: TransceiverDelay) {
    // Coverage for:
    //  - DRX
    //  - DRX_TS
    //  - DRX_RS
    //  - DRX_REF
}

/// Transmits after a delay
pub fn delayed_transmit(kind: TransceiverDelay) {
    // Coverage for:
    //  - DTX
    //  - DTX_TS
    //  - DTX_RS
    //  - DTX_REF
}

/// Transmits after a delay and then receives
pub fn delayed_transmit_receive(kind: TransceiverDelay) {
    // Coverage for:
    //  - DTX_W4R
    //  - DTX_TS_W4R
    //  - DTX_RS_W4R
    //  - DTX_REF_W4R
}
//! Static mappings for the registers of the DW3XXX.
//!
//! <div class="warning">
//! These mappings were not created entirely by hand, so there is a chance that they may not be correct. The register mappings that have
//! been verified by a human to be correct are labeled as such, but otherwise use at your own risk. If you would like to open a pull-request to
//! verify a specific register yourself and change the documentation accordingly that would be much appreciated.
//! </div>
//!
//! # Structure
//!
//! This module contains a series of types that implement the [`Register`] trait for each register of the device. Additionally, for each
//! register there is a submodule that contains a series of types for each individual field of that respective register that each implement
//! the [`Field`] trait. On their own these types do nothing, but their implementations of [`Register`] and [`Field`] contain the necessary
//! constants and types to statically reference that particular location within the device's register map. For more details see the
//! documentation for the [`Register`], [`Field`], [`Readable`], and [`Writable`] traits.
//!

/// A register of the DW3XXX.
pub trait Register {
    /// The base address of the register.
    const BASE_ADDRESS: u8;
    /// The sub-address of the register.
    const SUB_ADDRESS: u8;
    /// The length of the register.
    const LEN: usize;
    /// The type representing a view of the register.
    type RegisterView;
}

/// A field within a register of the DW3XXX.
pub trait Field {
    /// The parent register of the field.
    type Register: Register;
    /// The type level representation of the field.
    type Value;
    /// The index of the field's first bit within the register.
    const FIRST_BIT: usize;
    /// The size of the field in bits.
    const SIZE: u8;
}

/// A marker trait for a readable field.
pub trait Readable: Field {
    /// Read the field from the register.
    fn read(
        register: &<<Self as Field>::Register as Register>::RegisterView,
    ) -> <Self as Field>::Value {
        todo!()
    }
}

/// A marker trait for a writable field.
pub trait Writable: Field {
    /// Write the field to the register.
    fn write(
        register: &mut <<Self as Field>::Register as Register>::RegisterView,
        value: <Self as Field>::Value,
    ) {
        todo!()
    }
}

///
/// A helper macro to generate structured register and field structs organized into modules.
///
/// The declaration for each register is structured as follows:
/// ```markdown
/// /// Register documentation
/// [{BASE_ADDRESS}, {SUB_ADDRESS}, {LENGTH}, {RO, WO or RW}, NAME(name)] {
///     /// Field documentation
///     FIELD_NAME, [FIRST_BIT], [SIZE], [TYPE]
///     /// Field documentation
///     FIELD_NAME, [FIRST_BIT], [SIZE], [TYPE]
///
///     ...
/// }
/// ```
///
macro_rules! impl_registers {
    (
        $(
            $(
                #[$doc:meta]
            )*
            [$base_address:expr,
            $sub_address:expr,
            $reg_len:expr,
            $rw:tt,
            $reg_name:ident($reg_name_lower:ident)] {
            $(
                $(
                    #[$field_doc:meta]
                )*
                $field_name:ident,
                $field_start:expr,
                $field_size:expr,
                $field_ty:ty;
            )*
            }
        )*
    ) => {
        $(
            $(
                #[$doc]
            )*
            #[doc = ""]
            #[doc = " # Fields"]
            #[doc = ""]
            #[doc = concat!(" For access to the fields of this register see the [`", stringify!($reg_name_lower), "`] module.  ")]
            #[doc = ""]
            #[doc = " ## Quick Reference"]
            #[doc = ""]
            $(
                #[doc = concat!(" [`", stringify!($field_name), "`](", stringify!($reg_name_lower), "::", stringify!($field_name), ")  ")]
            )*
            #[doc = ""]
            #[allow(non_camel_case_types)]
            pub struct $reg_name;

            impl Register for $reg_name {
                const BASE_ADDRESS: u8    = $base_address;
                const SUB_ADDRESS:  u8    = $sub_address;
                const LEN:          usize = $reg_len;

                type RegisterView         = [u8; $reg_len];
            }

            #[doc = concat!(" Types for the fields within the register [`", stringify!($reg_name), "`].")]
            pub mod $reg_name_lower {
                $(
                    $(
                        #[$field_doc]
                    )*
                    #[doc = ""]
                    #[doc = " # Parent Register"]
                    #[doc = ""]
                    #[doc = concat!(" For access to the parent register of this field see [`", stringify!($reg_name), "`](super::", stringify!($reg_name), ").")]
                    #[doc = ""]
                    #[allow(non_camel_case_types)]
                    pub struct $field_name;

                    impl super::Field for $field_name {
                        type Register          = super::$reg_name;
                        type Value             = $field_ty;

                        const FIRST_BIT: usize = $field_start;
                        const SIZE: u8         = $field_size;
                    }

                    impl_rw!($rw, $field_name);
                )*
            }
        )*
    };
}

/// A helper macro for use in the [`impl_registers`] macro to generate implementations of the [`Readable`] and [`Writable`] traits for fields.
macro_rules! impl_rw {
    (RW, $field_name:ident) => {
        impl_rw!(RO, $field_name);
        impl_rw!(WO, $field_name);
    };
    (RO, $field_name:ident) => {
        impl super::Readable for $field_name {}
    };
    (WO, $field_name:ident) => {
        impl super::Writable for $field_name {}
    };
}

impl_registers! {
    /// Device identifier
    [0x00, 0x00, 4, RO, DEV_ID(dev_id)] {
        /// Revision
        REV, 0, 4,  u8;
        /// Version
        VER, 4, 4,  u8;
        /// Model
        MODEL, 8, 8,  u8;
        /// Register Identification Tag
        RIDTAG, 16, 16,  u16;
    }
    /// Extended Unique Identifier
    [0x00, 0x04, 8, RW, EUI(eui)] {
        /// Extended Unique Identifier
        VALUE, 0, 64,  u64;
    }
    /// PAN Identifier and Short Address
    [0x00, 0x0C, 4, RW, PANADR(panadr)] {
        /// Short Address
        SHORT_ADDR, 0, 16,  u16;
        /// PAN Identifier
        PAN_ID, 16, 16,  u16;
    }
    /// System Configuration
    [0x00, 0x10, 4, RW, SYS_CFG(sys_cfg)] {
        /// Frame Filtering Enable
        FFEN, 0, 1,  u8;
        /// disable auto-FCS Transmission
        DIS_FCS_TX, 1, 1,  u8;
        /// Disable frame check error handling
        DIS_FCE, 2, 1,  u8;
        /// Disable Double RX Buffer
        DIS_DRXB, 3, 1,  u8;
        /// PHR Mode
        PHR_MODE, 4, 1,  u8;
        /// Sets the PHR rate to match the data rate
        PHR_6M8, 5, 1,  u8;
        /// Enable SPI CRC functionnality
        SPI_CRCEN, 6, 1,  u8;
        /// Select CIA processing preamble CIR
        CIA_IPATOV, 7, 1,  u8;
        /// Select CIA processing STS CIR
        CIA_STS, 8, 1,  u8;
        /// Receive Wait Timeout Enable
        RXWTOE, 9, 1,  u8;
        /// Receiver Auto Re-enable
        RXAUTR, 10, 1,  u8;
        /// Automatic Acknowledge Enable
        AUTO_ACK, 11, 1,  u8;
        /// STS Packet Configuration
        CP_SPC, 12, 2,  u8;
        /// configures the SDC
        CP_SDC, 15, 1,  u8;
        /// configure PDoA
        PDOA_MODE, 16, 2,  u8;
        /// enable fast RX to TX turn around mode
        FAST_AAT, 18, 1,  u8;
    }
    ///
    /// Frame Filter Configuration Bit Map
    ///
    /// <div class="Warning">
    /// This register mapping and its contents have been hand verified to be correct by a real human.
    /// </div>
    ///
    /// IEEE802.15.4 standard frames begin with three bits, indicating the frame type, as follows:
    ///
    /// | Bits | Frame Type      |
    /// |-----:|:----------------|
    /// |  000 | Beacon          |
    /// |  001 | Data            |
    /// |  010 | Acknowledgement |
    /// |  011 | MAC             |
    /// |  100 | Reserved        |
    /// |  101 | Multipurpose    |
    /// |  110 | Fragmented      |
    /// |  111 | Extended        |
    ///
    /// This register contains a bit map that allows the receiver to filter which frames it will accept based on type and
    /// destination address. For any of these bits to apply, however, frame filtering must also be enabled using the
    /// [FFEN](sys_cfg::FFEN) bit in the [SYS_CFG] register.
    ///
    /// For more information on frame filtering please see the [DW3000 User Manual](https://www.qorvo.com/products/d/da008154)
    /// section 5.4 "Frame filtering".
    ///
    [0x00, 0x14, 2, RW, FF_CFG(ff_cfg)] {
        ///
        /// Frame Filtering Allow Beacon
        ///
        /// When this bit is 0 the frame filtering will ignore frames with the "Beacon" type (binary 000) in their header.
        /// For more information see [FF_CFG](super::FF_CFG).
        ///
        FFAB, 0, 1,  u8;
        ///
        /// Frame Filtering Allow Data
        ///
        /// When this bit is 0 the frame filtering will ignore frames with the "Data" type (binary 001) in their header.
        /// For more information see [FF_CFG](super::FF_CFG).
        ///
        FFAD, 1, 1,  u8;
        ///
        /// Frame Filtering Allow Acknowledgement
        ///
        /// When this bit is 0 the frame filtering will ignore frames with the "Acknowledgement" type (binary 010) in
        /// their header. For more information see [FF_CFG](super::FF_CFG).
        ///
        FFAA, 2, 1,  u8;
        ///
        /// Frame Filtering Allow MAC Command Frame
        ///
        /// When this bit is 0 the frame filtering will ignore frames with the "MAC" type (binary 011) in their header.
        /// For more information see [FF_CFG](super::FF_CFG).
        ///
        FFAM, 3, 1,  u8;
        ///
        /// Frame Filtering Allow Reserved
        ///
        /// When this bit is 0 the frame filtering will ignore frames with the "Reserved" type (binary 100) in their header.
        /// For more information see [FF_CFG](super::FF_CFG).
        ///
        FFAR, 4, 1,  u8;
        ///
        /// Frame Filtering Allow Multipurpose
        ///
        /// When this bit is 0 the frame filtering will ignore frames with the "Multipurpose" type (binary 101) in
        /// their header. For more information see [FF_CFG](super::FF_CFG).
        ///
        FFAMULTI, 5, 1,  u8;
        ///
        /// Frame Filtering Allow Fragmented
        ///
        /// When this bit is 0 the frame filtering will ignore frames with the "Fragmented" type (binary 110) in their header.
        /// For more information see [FF_CFG](super::FF_CFG).
        ///
        FFAF, 6, 1,  u8;
        ///
        /// Frame Filtering Allow Extended
        ///
        /// When this bit is 0 the frame filtering will ignore frames with the "Extended" type (binary 111) in their header.
        /// For more information see [FF_CFG](super::FF_CFG).
        ///
        FFAE, 7, 1,  u8;
        ///
        /// Frame Filtering Behave As Coordinator
        ///
        /// When this bit is enabled the device will operate as a PAN Coordinator meaning it will only accept frames that match
        /// the following criteria:
        /// * For "MAC" and "Data" type frames the source PAN ID must match that set in [PAN_ID](super::panadr::PAN_ID)
        /// * For "Multipurpose" type frames the destination PAN ID must match that set in [PAN_ID](super::panadr::PAN_ID)
        ///
        FFBC, 8, 1,  u8;
        ///
        /// Frame Filtering Allow MAC Implicit Broadcast
        ///
        /// When this bit is 0, then the destination addressing (PAN ID and destination address fields, as appropriate) of
        /// received frames must either be set to broadcast (0xFFFF) or a specific destination address, otherwise the frame will be
        /// rejected. When this bit is 1, then frames without a destination PAN ID and destination address are treated as
        /// though they are addressed to the broadcast PAN ID and broadcast short (16-bit) address.
        ///
        FFIB, 9, 1,  u8;
        ///
        /// Data pending for device at LE0 addr
        ///
        /// When this bit is 1 and the device receives a Data Request MAC Command frame whose source address matches the address
        /// set in [LE_ADDR0](super::le_pend_01::LE_ADDR0) of the [LE_PEND_01](super::LE_PEND_01) register then the automatically
        /// transmitted ACK frame will have the PEND bit set.
        ///
        /// Note: in order for this field to have an effect, automatic acknowledgement must be enabled by setting the
        /// [AUTO_ACK](super::sys_cfg::AUTO_ACK) bit of the [SYS_CFG](super::SYS_CFG) register.
        ///
        LE0_PEND, 10, 1,  u8;
        /// Data pending for device at LE1 addr
        ///
        /// When this bit is 1 and the device receives a Data Request MAC Command frame whose source address matches the address
        /// set in [LE_ADDR1](super::le_pend_01::LE_ADDR1) of the [LE_PEND_01](super::LE_PEND_01) register then the automatically
        /// transmitted ACK frame will have the PEND bit set.
        ///
        /// Note: in order for this field to have an effect, automatic acknowledgement must be enabled by setting the
        /// [AUTO_ACK](super::sys_cfg::AUTO_ACK) bit of the [SYS_CFG](super::SYS_CFG) register.
        ///
        LE1_PEND, 11, 1,  u8;
        ///
        /// Data pending for device at LE2 addr
        ///
        /// When this bit is 1 and the device receives a Data Request MAC Command frame whose source address matches the address
        /// set in [LE_ADDR2](super::le_pend_23::LE_ADDR2) of the [LE_PEND_23](super::LE_PEND_23) register then the automatically
        /// transmitted ACK frame will have the PEND bit set.
        ///
        /// Note: in order for this field to have an effect, automatic acknowledgement must be enabled by setting the
        /// [AUTO_ACK](super::sys_cfg::AUTO_ACK) bit of the [SYS_CFG](super::SYS_CFG) register.
        ///
        LE2_PEND, 12, 1,  u8;
        ///
        /// Data pending for device at LE3 addr
        ///
        /// When this bit is 1 and the device receives a Data Request MAC Command frame whose source address matches the address
        /// set in [LE_ADDR3](super::le_pend_23::LE_ADDR2) of the [LE_PEND_23](super::LE_PEND_23) register then the automatically
        /// transmitted ACK frame will have the PEND bit set.
        ///
        /// Note: in order for this field to have an effect, automatic acknowledgement must be enabled by setting the
        /// [AUTO_ACK](super::sys_cfg::AUTO_ACK) bit of the [SYS_CFG](super::SYS_CFG) register.
        ///
        LE3_PEND, 13, 1,  u8;
        ///
        /// Short Source Address Data Request Acknowledge With Pending
        ///
        /// When this bit is 0 and the device receives a Data Request MAC Command frame from any node with a short
        /// (16-bit) source address then the automatically transmitted ACK frame will follow the frame filtering rules
        /// defined by the [LE0_PEND], [LE1_PEND], [LE2_PEND], and [LE3_PEND] fields.
        ///
        /// When this bit is 1 and the device receives a Data Request MAC Command frame from any node with a short
        /// (16-bit) source address then the automatically transmitted ACK frame will have the PEND bit set
        ///
        /// Note: in order for this field to have an effect, automatic acknowledgement must be enabled by setting the
        /// [AUTO_ACK](super::sys_cfg::AUTO_ACK) bit of the [SYS_CFG](super::SYS_CFG) register.
        ///
        SSADRAPE, 14, 1,  u8;
        ///
        /// Long Source Address Data Request Acknowledge With Pending
        ///
        /// When the device receives a Data Request MAC Command frame from any node with a long (64-bit) source address
        /// then the automatically transmitted ACK frame will have the PEND bit set if this bit is 1 and will not have
        /// the PEND bit set if this bit is 0.
        ///
        /// Note: in order for this field to have an effect, automatic acknowledgement must be enabled by setting the
        /// [AUTO_ACK](super::sys_cfg::AUTO_ACK) bit of the [super::SYS_CFG] register.
        ///
        LSADRAPE, 15, 1,  u8;
    }
    /// SPI CRC read status
    [0x00, 0x18, 1, RO, SPI_RD_CRC(spi_rd_crc)] {
        /// SPI CRC read status
        VALUE, 0, 8,  u8;
    }
    ///  System Time Counter register
    [0x00, 0x1C, 4, RO, SYS_TIME(sys_time)] {
        /// System Time Counter register
        VALUE, 0, 32,  u32;
    }
    ///
    /// TX Frame Control
    ///
    /// <div class="Warning">
    /// This register mapping and its contents have been hand verified to be correct by a real human.
    /// </div>
    ///
    [0x00, 0x24, 6, RW, TX_FCTRL(tx_fctrl)] {
        /// TX Frame Length
        TXFLEN, 0, 10,  u16;
        /// Transmit Bit Rate
        TXBR, 10, 1,  u8;
        /// Transmit Ranging enable
        TR, 11, 1,  u8;
        /// Transmit Preamble Symbol Repetitions
        TXPSR, 12, 4,  u8;
        /// Transmit buffer index offset
        TXB_OFFSET, 16, 10,  u16;
        /// Fine PSR control
        FINE_PLEN, 40, 8,  u8;
    }
    ///
    /// Delayed Send or Receive Time
    ///
    /// <div class="warning">
    /// This register mapping and its contents have been hand verified to be correct by a real human.
    /// </div>
    ///
    [0x00, 0x2C, 4, RW, DX_TIME(dx_time)] {
        ///
        /// Delayed Send or Receive Time
        ///
        /// The value in this register is added to [DREF_TIME](super::DREF_TIME) before either the receiver or transmitter is
        /// turned on.
        ///
        /// The units are one half of the 499.2 MHz fundamental frequency (≈ 4 ns). The least significant bit of this register is
        /// ignored, so the smallest value that can be specified is 2 (≈ 8 ns).
        ///
        VALUE, 0, 32,  u32;
    }
    ///
    /// Delayed send or receive reference time
    ///
    /// <div class="warning">
    /// This register mapping and its contents have been hand verified to be correct by a real human.
    /// </div>
    ///
    [0x00, 0x30, 4, RW, DREF_TIME(dref_time)] {
        ///
        /// Delayed send or receive reference time
        ///
        /// Used to specify a time at which an an event happened (e.g. beacon was sent). Any value in [DX_TIME](super::DX_TIME)
        /// is added to this register before either the receiver or transmitter is turned on.
        ///
        /// The units are one half of the 499.2 MHz fundamental frequency (≈ 4 ns). The least significant bit
        /// of this register is ignored, so the smallest value that can be specified is 2 (≈ 8 ns).
        ///
        VALUE, 0, 32,  u32;
    }
    /// Receive frame wait timeout period
    [0x00, 0x34, 3, RW, RX_FWTO(rx_fwto)] {
        /// Receive frame wait timeout period
        VALUE, 0, 24,  u32;
    }
    /// System Control Register
    [0x00, 0x38, 1, RW, SYS_CTRL(sys_ctrl)] {
        /// System control
        VALUE, 0, 8,  u8;
    }
    ///
    /// System Event Mask Register
    ///
    /// <div class="warning">
    /// This register mapping and its contents have been hand verified to be correct by a real human.
    /// </div>
    ///
    [0x00, 0x3C, 6, RW, SYS_ENABLE(sys_enable)] {
        /// Mask clock PLL lock event
        CPLOCK_EN, 1, 1,  u8;
        /// Mask SPI CRC Error event
        SPICRCE_EN, 2, 1,  u8;
        /// Mask automatic acknowledge trigger event
        AAT_EN, 3, 1,  u8;
        /// Mask transmit frame begins event
        TXFRB_EN, 4, 1,  u8;
        /// Mask transmit preamble sent event
        TXPRS_EN, 5, 1,  u8;
        /// Mask transmit PHY Header Sent event
        TXPHS_EN, 6, 1,  u8;
        /// Mask transmit frame sent event
        TXFRS_EN, 7, 1,  u8;
        /// Mask receiver preamble detected event
        RXPRD_EN, 8, 1,  u8;
        /// Mask receiver SFD detected event
        RXSFDD_EN, 9, 1,  u8;
        /// Mask CIA processing done event
        CIADONE_EN, 10, 1,  u8;
        /// Mask receiver PHY header detect event
        RXPHD_EN, 11, 1,  u8;
        /// Mask receiver PHY header error event
        RXPHE_EN, 12, 1,  u8;
        /// Mask receiver data frame ready event
        RXFR_EN, 13, 1,  u8;
        /// Mask receiver FCS good event
        RXFCG_EN, 14, 1,  u8;
        /// Mask receiver FCS error event
        RXFCE_EN, 15, 1,  u8;
        /// Mask receiver Reed Solomon Frame Sync Loss event
        RXRFSL_EN, 16, 1,  u8;
        /// Mask Receive Frame Wait Timeout event
        RXFTO_EN, 17, 1,  u8;
        /// Mask leading edge detection processing error event
        CIAERR_EN, 18, 1,  u8;
        /// Mask Voltage warning event
        VWARN_EN, 19, 1,  u8;
        /// Receiver overrun
        RXOVRR_EN, 20, 1,  u8;
        /// Mask Preamble detection timeout event
        RXPTO_EN, 21, 1,  u8;
        /// Mask SPI ready event
        SPIRDY_EN, 23, 1,  u8;
        /// Mask IDLE RC event
        RCINIT_EN, 24, 1,  u8;
        /// Mask PLL Losing Lock warning event
        PLL_HILO_EN, 25, 1,  u8;
        /// Mask Receive SFD timeout event
        RXSTO_EN, 26, 1,  u8;
        /// Mask Half Period Delay Warning event
        HPDWARN_EN, 27, 1,  u8;
        /// Mask Scramble Timestamp Sequence (STS) error event
        CPERR_EN, 28, 1,  u8;
        /// Mask Automatic Frame Filtering rejection event
        ARFE_EN, 29, 1,  u8;
        /// Mask Receiver Preamble Rejection event
        RXPREJ_EN, 33, 1,  u8;
        /// Mask Voltage/Temperature variation dtection interrupt event
        VT_DET_EN, 36, 1,  u8;
        /// Mask GPIO interrupt event
        GPIOIRQ_EN, 37, 1,  u8;
        /// Mask AES done interrupt event
        AES_DONE_EN, 38, 1,  u8;
        /// Mask AES error interrupt event
        AES_ERR_EN, 39, 1,  u8;
        /// Mask CMD error interrupt event
        CDM_ERR_EN, 40, 1,  u8;
        /// Mask SPI overflow interrupt event
        SPI_OVF_EN, 41, 1,  u8;
        /// Mask SPI underflow interrupt event
        SPI_UNF_EN, 42, 1,  u8;
        /// Mask SPI error interrupt event
        SPI_ERR_EN, 43, 1,  u8;
        /// Mask CCA fail interrupt event
        CCA_FAIL_EN, 44, 1,  u8;
    }
    ///
    /// System Event Status Register
    ///
    /// <div class="warning">
    /// This register mapping and its contents have been hand verified to be correct by a real human.
    /// </div>
    ///
    [0x00, 0x44, 6, RW, SYS_STATUS(sys_status)] {
        /// Interrupt Request Status
        IRQS, 0, 1,  u8;
        /// Clock PLL Lock
        CPLOCK, 1, 1,  u8;
        /// External Sync Clock Reset
        SPICRCE, 2, 1,  u8;
        /// Automatic Acknowledge Trigger
        AAT, 3, 1,  u8;
        /// TX Frame Begins
        TXFRB, 4, 1,  u8;
        /// TX Preamble Sent
        TXPRS, 5, 1,  u8;
        /// TX PHY Header Sent
        TXPHS, 6, 1,  u8;
        /// TX Frame Sent
        TXFRS, 7, 1,  u8;
        /// RX Preamble Detected
        RXPRD, 8, 1,  u8;
        /// RX SFD Detected
        RXSFDD, 9, 1,  u8;
        /// LDE Processing Done
        CIADONE, 10, 1,  u8;
        /// RX PHY Header Detect
        RXPHD, 11, 1,  u8;
        /// RX PHY Header Error
        RXPHE, 12, 1,  u8;
        /// RX Data Frame Ready
        RXFR, 13, 1,  u8;
        /// RX FCS Good
        RXFCG, 14, 1,  u8;
        /// RX FCS Error
        RXFCE, 15, 1,  u8;
        /// RX Reed-Solomon Frame Sync Loss
        RXFSL, 16, 1,  u8;
        /// RX Frame Wait Timeout
        RXFTO, 17, 1,  u8;
        /// Leading Edge Detection Error
        CIAERR, 18, 1,  u8;
        /// Low voltage warning
        VWARN, 19, 1,  u8;
        /// RX Overrun
        RXOVRR, 20, 1,  u8;
        /// Preamble detection timeout
        RXPTO, 21, 1,  u8;
        /// SPI ready for host access
        SPIRDY, 23, 1,  u8;
        /// RC INIT
        RCINIT, 24, 1,  u8;
        /// lock PLL Losing Lock
        PLL_HILO, 25, 1,  u8;
        /// Receive SFD timeout
        RXSTO, 26, 1,  u8;
        /// Half Period Delay Warning
        HPDWARN, 27, 1,  u8;
        /// Scramble Timestamp Sequence (STS) error
        CPERR, 28, 1,  u8;
        /// Automatic Frame Filtering rejection
        ARFE, 29, 1,  u8;
        /// Receiver Preamble Rejection
        RXPREJ, 29, 1,  u8;
        /// Voltage or temperature variation detected
        VT_DET, 33, 1,  u8;
        /// GPIO interrupt
        GPIOIRQ, 36, 1,  u8;
        /// AES-DMA operation complete
        AES_DONE, 37, 1,  u8;
        /// AES-DMA error
        AES_ERR, 38, 1,  u8;
        /// Command error
        CMD_ERR, 39, 1,  u8;
        /// SPI overflow error
        SPI_OVF, 40, 1,  u8;
        /// SPI underflow error
        SPI_UNF, 41, 1,  u8;
        /// SPI collision error
        SPIERR, 42, 1,  u8;
        /// This event will be set as a result of failure of CMD_CCA_TX to transmit a packet
        CCA_FAIL, 43, 1,  u8;
    }
    /// RX Frame Information
    [0x00, 0x4C, 4, RO, RX_FINFO(rx_finfo)] {
        /// Receive Frame Length
        RXFLEN, 0, 10,  u16;
        /// Receive Non-Standard Preamble Length
        RXNSPL, 11, 2,  u8;
        /// Receive Bit Rate Report
        RXBR, 13, 1,  u8;
        /// Receiver Ranging
        RNG, 15, 1,  u8;
        /// RX Pulse Repetition Rate Report
        RXPRF, 16, 2,  u8;
        /// RX Preamble Repetition
        RXPSR, 18, 2,  u8;
        /// Preamble Accumulation Count
        RXPACC, 20, 12,  u16;
    }
    ///
    /// Receive Timestamp
    ///
    /// <div class="warning">
    /// This register mapping and its contents have been hand verified to be correct by a real human.
    /// </div>
    ///
    [0x00, 0x64, 16, RO, RX_TIME(rx_time)] {
        ///
        /// Fully adjusted timestamp of reception
        ///
        /// The least significant bit is roughly equivalent to 15.65ps (<math xmlns="http://www.w3.org/1998/Math/MathML"><mfrac><mn>1</mn><mrow><mn>128</mn><mo>(</mo><mn>499.2</mn><mo>&#xd7;</mo><msup><mn>10</mn><mrow><mo>-</mo><mn>6</mn></mrow></msup><mo>)</mo></mrow></mfrac></math>).
        ///
        /// The value is available here when the leading edge determination and timestamp adjustments
        /// are completed (when the [CIADONE](super::sys_status::CIADONE) event status flag bit is set).
        ///
        RX_STAMP, 0, 40,  u64;
        ///
        /// Raw timestamp of reception
        ///
        /// the value of the system clock captured at start of the PHR. The precision is approximately 125 MHz (8 ns),
        /// i.e. the least significant bit is zero.
        ///
        RX_RAWST, 64, 32, u32;
    }
    ///
    /// Transmit Timestamp
    ///
    /// <div class="warning">
    /// This register mapping and its contents have been hand verified to be correct by a real human.
    /// </div>
    ///
    [0x00, 0x74, 5, RO, TX_TIME(tx_time)] {
        ///
        /// Fully adjusted time of transmission
        ///
        /// The least significant bit is roughly equivalent to 15.65ps (<math xmlns="http://www.w3.org/1998/Math/MathML"><mfrac><mn>1</mn><mrow><mn>128</mn><mo>(</mo><mn>499.2</mn><mo>&#xd7;</mo><msup><mn>10</mn><mrow><mo>-</mo><mn>6</mn></mrow></msup><mo>)</mo></mrow></mfrac></math>).
        /// The value becomes available here after the PHR transmission has completed.
        ///
        TX_STAMP, 0, 40,  u64;
    }
    ///
    /// Transmit raw timestamp
    ///
    /// <div class="warning">
    /// This register mapping and its contents have been hand verified to be correct by a real human.
    /// </div>
    ///
    [0x01, 0x00, 4, RO, TX_RAWST(tx_rawst)] {
        ///
        /// Raw timestamp of transmission
        ///
        VALUE, 0, 32,  u32;
    }
    ///
    /// Transmitter antenna delay
    ///
    /// <div class="warning">
    /// This register mapping and its contents have been hand verified to be correct by a real human.
    /// </div>
    ///
    [0x01, 0x04, 2, RW, TX_ANTD(tx_antd)] {
        ///
        /// Transmitter antenna delay
        ///
        /// Accounts for the delay between the internal digital timestamp of the RMARKER and the time the RMARKER is at the antenna.
        /// The value is automatically added to the raw timestamp ([TX_RAWST](super::tx_rawst::VALUE)) to get the fully adjusted
        /// timestamp ([TX_STAMP](super::tx_stamp::TX_STAMP)).
        ///
        /// The least significant bit is roughly equivalent to 15.65ps (<math xmlns="http://www.w3.org/1998/Math/MathML"><mfrac><mn>1</mn><mrow><mn>128</mn><mo>(</mo><mn>499.2</mn><mo>&#xd7;</mo><msup><mn>10</mn><mrow><mo>-</mo><mn>6</mn></mrow></msup><mo>)</mo></mrow></mfrac></math>).
        /// The default antenna delay is `0x4015`, which is approximately 256.74 ns.
        ///
        VALUE, 0, 16,  u16;
    }
    ///
    /// Acknowledgement delay time and response time
    ///
    /// <div class="warning">
    /// This register mapping and its contents have been hand verified to be correct by a real human.
    /// </div>
    ///
    [0x01, 0x08, 4, RW, ACK_RESP(ack_resp)] {
        ///
        /// Wait-for-Response turn-around time
        ///
        /// Configures the turn-around time between TX complete and RX enable when one of the wait for response functions
        /// are being used. The time specified is in units of approximately 1 µs, or 128 system clock cycles.
        ///
        /// This may be used to save power by delaying the turn-on of the receiver, to align with the response time of the
        /// remote system, rather than turning on the receiver immediately after transmission completes.
        ///
        W4R_TIM, 0, 20,  u32;
        ///
        /// Auto-Acknowledgement turn-around time
        ///
        /// Configures the turn-around time between the correct receipt of a data frame (or MAC command frame) and the
        /// automatic transmission of the acknowledgement frame. The time specified is in units of preamble symbols.
        ///
        /// To ensure that the receiver is ready for the first preamble symbol, and assuming that the remote transmitter
        /// has its [W4R_TIM] parameter set to 0, the recommended minimum settings are 2 and 3 for data rates of 850 kb/s
        /// and 6.8 Mb/s respectively. This is most important at the 6.8 Mb/s data rate, where preamble sequences are
        /// generally short, and losing even a few preamble symbols could potentially compromise ACK reception.
        ///
        /// Where the [W4R_TIM] parameter of the remote transmitter is larger than zero, this setting should be increased
        /// to ensure that none of the packet is sent before the remote receiver is listening.
        ///
        /// Note: in order for this bit to have an effect, automatic acknowledgement must be enabled by setting the
        /// [AUTO_ACK](super::sys_cfg::AUTO_ACK) bit of the [SYS_CFG](super::SYS_CFG) register.
        ///
        ACK_TIM, 24, 8,  u8;
    }
    /// TX Power Control
    [0x01, 0x0C, 4, RW, TX_POWER(tx_power)] {
        /// TX Power Control value
        VALUE, 0, 32,  u32;
    }
    ///
    /// Channel Control Register
    ///
    /// <div class="warning">
    /// This register mapping and its contents have been hand verified to be correct by a real human.
    /// </div>
    ///
    [0x01, 0x14, 2, RW, CHAN_CTRL(chan_ctrl)] {
        /// Selects the receive channel.
        RF_CHAN, 0, 1,  u8;
        /// Enables the non-standard Decawave proprietary SFD sequence.
        SFD_TYPE, 1, 2,  u8;
        /// Selects the preamble code used in the transmitter.
        TX_PCODE, 3, 5,  u8;
        /// Selects the preamble code used in the receiver.
        RX_PCODE, 8, 5,  u8;
    }
    /// Low Energy device address 0 and 1
    [0x01, 0x18, 4, RW, LE_PEND_01(le_pend_01)] {
        /// Low Energy device 16-bit address
        LE_ADDR0, 0, 16,  u16;
        /// Low Energy device 16-bit address
        LE_ADDR1, 16, 16,  u16;
    }
    /// Low Energy device address 2 and 3
    [0x01, 0x1C, 4, RW, LE_PEND_23(le_pend_23)] {
        /// Low Energy device 16-bit address
        LE_ADDR2, 0, 16,  u16;
        /// Low Energy device 16-bit address
        LE_ADDR3, 16, 16,  u16;
    }
    /// SPI collision status
    [0x01, 0x20, 1, RW, SPI_COLLISION(spi_collision)] {
        /// SPI collision status
        VALUE, 0, 8,  u8;
    }
    /// RX double buffer status
    [0x01, 0x24, 1, RW, RDB_STATUS(rdb_status)] {
        /// Receiver FCS Good
        RXFCG0, 0, 1,  u8;
        /// Receiver Data Frame Ready
        RXFR0, 1, 1,  u8;
        /// CIA processing done on the CIR relating to a message in RX_BUFFER_0 when operating in double buffer mode
        CIADONE0, 2, 1,  u8;
        /// Scramble Timestamp Sequence (STS) error
        CP_ERR0, 3, 1,  u8;
        /// Receiver FCS Good
        RXFCG1, 4, 1,  u8;
        /// Receiver Data Frame Ready
        RXFR1, 5, 1,  u8;
        /// CIA processing done on the CIR relating to a message in RX_BUFFER_1 when operating in double buffer mode
        CIADONE1, 6, 1,  u8;
        /// Scramble Timestamp Sequence (STS) error
        CP_ERR1, 7, 1,  u8;
    }
    /// RX double buffer diagnostic configuration
    [0x01, 0x28, 1, RW, RDB_DIAG(rdb_diag)] {
        /// RX double buffer diagnostic mode
        RDB_DMODE, 0, 3,  u8;
    }
    /// AES configuration
    [0x01, 0x30, 2, RW, AES_CFG(aes_cfg)] {
        /// Mode of operation of AES core
        MODE, 0, 1,  u8;
        /// AES Key Size
        KEY_SIZE, 1, 2,  u8;
        /// Address offset of AES KEY
        KEY_ADDR, 3, 3,  u8;
        /// Load the AES KEY from AES KEY source
        KEY_LOAD, 6, 1,  u8;
        /// AES key source
        KEY_SRC, 7, 1,  u8;
        /// Size of AES tag field
        TAG_SIZE, 8, 3,  u8;
        /// AES Core select
        CORE_SEL, 11, 1,  u8;
        /// AES key Memory source
        KEY_OTP, 12, 1,  u8;
    }
    /// AES GCM core mode
    [0x01, 0x34, 4, RW, AES_IV0(aes_iv0)] {
        /// AES GCM core mode
        VALUE, 0, 32,  u32;
    }
    /// AES GCM core mode
    [0x01, 0x38, 4, RW, AES_IV1(aes_iv1)] {
        /// AES GCM core mode
        VALUE, 0, 32,  u32;
    }
    /// AES GCM core mode
    [0x01, 0x3C, 4, RW, AES_IV2(aes_iv2)] {
        /// AES GCM core mode
        VALUE, 0, 32,  u32;
    }
    /// AES GCM core mode
    [0x01, 0x40, 2, RW, AES_IV3(aes_iv3)] {
        /// AES GCM core mode
        VALUE, 0, 16,  u16;
    }
    /// AES GCM core mode
    [0x01, 0x42, 2, RW, AES_IV4(aes_iv4)] {
        /// AES GCM core mode
        VALUE, 0, 16,  u16;
    }
    /// DMA configuration register
    [0x01, 0x44, 8, RW, DMA_CFG(dma_cfg)] {
        /// Source memory port for DMA transfer
        SRC_PORT, 0, 3,  u8;
        /// Address offset within source memory for DMA transfer
        SRC_ADDR, 3, 10,  u16;
        /// Destination memory port for DMA transfer
        DST_PORT, 13, 3,  u8;
        /// Address offset within destination memory for DMA transfer
        DST_ADDR, 16, 10,  u16;
        /// Select the endianess of the CP seed port
        CP_END_SEL, 26, 1,  u8;
        /// Size of header field in the packet to be transferred via the DMA
        HDR_SIZE, 32, 7,  u8;
        /// Size of payload field in the packet to be transferred via the DMA
        PYLD_SIZE, 39, 10,  u8;
    }
    /// Start AES operation
    [0x01, 0x4C, 1, RW, AES_START(aes_start)] {
        /// Start AES operation
        VALUE, 0, 1,  u8;
    }
    /// The AES Status
    [0x01, 0x50, 4, RW, AES_STS(aes_sts)] {
        /// AES operation complete. Write 1 to clear
        AES_DONE, 0, 1,  u8;
        /// AES authentication error. Write 1 to clear.
        AUTH_ERR, 1, 1,  u8;
        /// Indicates error with DMA transfer to memory. Write 1 to clear
        TRANS_ERR, 2, 1,  u8;
        /// Indicates access conflict between multiple masters (SPI host, CIA engine and AES-DMA engine) trying to access same memory
        MEM_CONF, 3, 1,  u8;
        /// Indicates AES scratch RAM is empty
        RAM_EMPTY, 4, 1,  u8;
        /// Indicates AES scratch RAM is full
        RAM_FULL, 5, 1,  u8;
    }
    /// The 128-bit KEY for the AES GCM/CCM* core
    [0x01, 0x54, 16, RW, AES_KEY(aes_key)] {
        /// value
        VALUE, 0, 128,  u128;
    }
    /// STS configuration
    [0x02, 0x00, 2, RW, STS_CFG(sts_cfg)] {
        /// STS length
        CPS_LEN, 0, 8,  u8;
    }
    /// STS control
    [0x02, 0x04, 1, RW, STS_CTRL(sts_ctrl)] {
        /// Load STS_IV bit into the AES-128 block for the generation of STS
        LOAD_IV, 0, 1,  u8;
        /// Start from last, when it is set to 1 the STS generation starts from the last count that was used by the AES-128 block for the generation of the previous STS.
        RST_LAST, 1, 1,  u8;
    }
    /// STS status
    [0x02, 0x08, 2, RW, STS_STS(sts_sts)] {
        /// STS accumulation quality
        ACC_QUAL, 0, 12,  u16;
    }
    /// STS 128-bit KEY
    [0x02, 0x0C, 16, RW, STS_KEY(sts_key)] {
        /// value
        VALUE, 0, 128,  u128;
    }
    /// STS 128-bit IV
    [0x02, 0x1C, 16, RW, STS_IV(sts_iv)] {
        /// value
        VALUE, 0, 128,  u128;
    }
    /// RX tuning configuration register
    [0x03, 0x18, 2, RW, DGC_CFG(dgc_cfg)] {
        /// RX tuning enable bit
        RX_TUNE_EN, 0, 1,  u8;
        /// RX tuning threshold configuration for 64 MHz PRF
        THR_64, 9, 6,  u8;
    }
    /// DGC_CFG0
    [0x03, 0x1C, 4, RW, DGC_CFG0(dgc_cfg0)] {
        /// Value
        VALUE, 0, 32,  u32;
    }
    /// DGC_CFG1
    [0x03, 0x20, 4, RW, DGC_CFG1(dgc_cfg1)] {
        /// Value
        VALUE, 0, 32,  u32;
    }
    /// DGC_LUT_0
    [0x03, 0x38, 4, RW, DGC_LUT_0(dgc_lut_0)] {
        /// Value
        VALUE, 0, 32,  u32;
    }
    /// DGC_LUT_1
    [0x03, 0x3C, 4, RW, DGC_LUT_1(dgc_lut_1)] {
        /// Value
        VALUE, 0, 32,  u32;
    }
    /// DGC_LUT_2
    [0x03, 0x40, 4, RW, DGC_LUT_2(dgc_lut_2)] {
        /// Value
        VALUE, 0, 32,  u32;
    }
    /// DGC_LUT_3
    [0x03, 0x44, 4, RW, DGC_LUT_3(dgc_lut_3)] {
        /// Value
        VALUE, 0, 32,  u32;
    }
    /// DGC_LUT_4
    [0x03, 0x48, 4, RW, DGC_LUT_4(dgc_lut_4)] {
        /// Value
        VALUE, 0, 32,  u32;
    }
    /// DGC_LUT_5
    [0x03, 0x4C, 4, RW, DGC_LUT_5(dgc_lut_5)] {
        /// Value
        VALUE, 0, 32,  u32;
    }
    /// DGC_LUT_6
    [0x03, 0x50, 4, RW, DGC_LUT_6(dgc_lut_6)] {
        /// Value
        VALUE, 0, 32,  u32;
    }
    /// Reports DGC information
    [0x03, 0x60, 4, RW, DGC_DBG(dgc_dbg)] {
        /// DGC decision index.
        DGC_DECISION, 28, 3,  u8;
    }
    /// External clock synchronisation counter configuration
    [0x04, 0x00, 4, RW, EC_CTRL(ec_ctrl)] {
        /// Wait counter used for external timebase reset
        OSTS_WAIT, 3, 8,  u8;
        /// External timebase reset mode enable bit
        OSTR_MODE, 11, 1,  u8;
    }
    /// RX calibration block configuration
    [0x04, 0x0C, 4, RW, RX_CAL(rx_cal)] {
        /// RX calibration mode
        CAL_MODE, 0, 2,  u8;
        /// RX calibration enable
        CAL_EN, 4, 4,  u8;
        /// RX calibration tuning value
        COMP_DLY, 16, 4,  u8;
    }
    /// RX calibration block result
    [0x04, 0x14, 4, RW, RX_CAL_RESI(rx_cal_resi)] {
        /// reports the result once the RX calibration is complete
        VALUE, 0, 29,  u32;
    }
    /// RX calibration block result
    [0x04, 0x1C, 4, RW, RX_CAL_RESQ(rx_cal_resq)] {
        /// reports the result once the RX calibration is complete
        VALUE, 0, 29,  u32;
    }
    /// RX calibration block status
    [0x04, 0x20, 1, RW, RX_CAL_STS(rx_cal_sts)] {
        ///  reports the status once the RX calibration is complete
        VALUE, 0, 1,  u8;
    }
    /// GPIO Mode Control Register
    [0x05, 0x00, 4, RW, GPIO_MODE(gpio_mode)] {
        ///  Mode Selection for GPIO0/RXOKLED
        MSGP0, 0, 3,  u8;
        ///  Mode Selection for GPIO1/SFDLED
        MSGP1, 3, 3,  u8;
        ///  Mode Selection for GPIO2/RXLED
        MSGP2, 6, 3,  u8;
        ///  Mode Selection for GPIO3/TXLED
        MSGP3, 9, 3,  u8;
        ///  Mode Selection for GPIO4/EXTPA
        MSGP4, 12, 3,  u8;
        ///  Mode Selection for GPIO5/EXTTXE
        MSGP5, 15, 3,  u8;
        ///  Mode Selection for GPIO6/EXTRXE
        MSGP6, 18, 3,  u8;
        ///  Mode Selection for GPIO7
        MSGP7, 21, 3,  u8;
        ///  Mode Selection for GPIO8
        MSGP8, 24, 3,  u8;
    }
    /// GPIO Drive Strength and Pull Control
    [0x05, 0x04, 2, RW, GPIO_PULL_EN(gpio_pull_en)] {
        ///  Setting to 0 will lower the drive strength
        MGPEN0, 0, 1,  u8;
        ///  Setting to 0 will lower the drive strength
        MGPEN1, 1, 1,  u8;
        ///  Setting to 0 will lower the drive strength
        MGPEN2, 2, 1,  u8;
        ///  Setting to 0 will lower the drive strength
        MGPEN3, 3, 1,  u8;
        ///  Setting to 0 will lower the drive strength
        MGPEN4, 4, 1,  u8;
        ///  Setting to 0 will lower the drive strength
        MGPEN5, 5, 1,  u8;
        ///  Setting to 0 will lower the drive strength
        MGPEN6, 6, 1,  u8;
        ///  Setting to 0 will lower the drive strength
        MGPEN7, 7, 1,  u8;
        ///  Setting to 0 will lower the drive strength
        MGPEN8, 8, 1,  u8;
    }
    /// GPIO Direction Control Register
    [0x05, 0x08, 2, RW, GPIO_DIR(gpio_dir)] {
        ///   value of 0 means the pin is an output
        GPD0, 0, 1,  u8;
        ///   value of 0 means the pin is an output
        GPD1, 1, 1,  u8;
        ///   value of 0 means the pin is an output
        GPD2, 2, 1,  u8;
        ///   value of 0 means the pin is an output
        GPD3, 3, 1,  u8;
        ///   value of 0 means the pin is an output
        GPD4, 4, 1,  u8;
        ///   value of 0 means the pin is an output
        GPD5, 5, 1,  u8;
        ///   value of 0 means the pin is an output
        GPD6, 6, 1,  u8;
        ///   value of 0 means the pin is an output
        GPD7, 7, 1,  u8;
        ///   value of 0 means the pin is an output
        GPD8, 8, 1,  u8;
    }
    /// GPIO Data Output Register
    [0x05, 0x0C, 2, RW, GPIO_OUT(gpio_out)] {
        ///   show the current output setting
        GOP0, 0, 1,  u8;
        ///   show the current output setting
        GOP1, 1, 1,  u8;
        ///   show the current output setting
        GOP2, 2, 1,  u8;
        ///   show the current output setting
        GOP3, 3, 1,  u8;
        ///   show the current output setting
        GOP4, 4, 1,  u8;
        ///   show the current output setting
        GOP5, 5, 1,  u8;
        ///   show the current output setting
        GOP6, 6, 1,  u8;
        ///   show the current output setting
        GOP7, 7, 1,  u8;
        ///   show the current output setting
        GOP8, 8, 1,  u8;
    }
    /// GPIO Interrupt Enable
    [0x05, 0x10, 2, RW, GPIO_IRQE(gpio_irqe)] {
        ///   selected as interrupt source
        GIRQE0, 0, 1,  u8;
        ///   selected as interrupt source
        GIRQE1, 1, 1,  u8;
        ///   selected as interrupt source
        GIRQE2, 2, 1,  u8;
        ///   selected as interrupt source
        GIRQE3, 3, 1,  u8;
        ///   selected as interrupt source
        GIRQE4, 4, 1,  u8;
        ///   selected as interrupt source
        GIRQE5, 5, 1,  u8;
        ///   selected as interrupt source
        GIRQE6, 6, 1,  u8;
        ///   selected as interrupt source
        GIRQE7, 7, 1,  u8;
        ///   selected as interrupt source
        GIRQE8, 8, 1,  u8;
    }
    /// GPIO Interrupt Status
    [0x05, 0x14, 2, RW, GPIO_ISTS(gpio_ists)] {
        ///   Value 1 means GPIO gave rise to the GPIOIRQ SYS_STATUS event
        GISTS0, 0, 1,  u8;
        ///   Value 1 means GPIO gave rise to the GPIOIRQ SYS_STATUS event
        GISTS1, 1, 1,  u8;
        ///   Value 1 means GPIO gave rise to the GPIOIRQ SYS_STATUS event
        GISTS2, 2, 1,  u8;
        ///   Value 1 means GPIO gave rise to the GPIOIRQ SYS_STATUS event
        GISTS3, 3, 1,  u8;
        ///   Value 1 means GPIO gave rise to the GPIOIRQ SYS_STATUS event
        GISTS4, 4, 1,  u8;
        ///   Value 1 means GPIO gave rise to the GPIOIRQ SYS_STATUS event
        GISTS5, 5, 1,  u8;
        ///   Value 1 means GPIO gave rise to the GPIOIRQ SYS_STATUS event
        GISTS6, 6, 1,  u8;
        ///   Value 1 means GPIO gave rise to the GPIOIRQ SYS_STATUS event
        GISTS7, 7, 1,  u8;
        ///   Value 1 means GPIO gave rise to the GPIOIRQ SYS_STATUS event
        GISTS8, 8, 1,  u8;
    }
    /// GPIO Interrupt Sense Selection
    [0x05, 0x18, 2, RW, GPIO_ISEN(gpio_isen)] {
        ///   GPIO IRQ Sense selection GPIO input
        GISEN0, 0, 1,  u8;
        ///   GPIO IRQ Sense selection GPIO input
        GISEN1, 1, 1,  u8;
        ///   GPIO IRQ Sense selection GPIO input
        GISEN2, 2, 1,  u8;
        ///   GPIO IRQ Sense selection GPIO input
        GISEN3, 3, 1,  u8;
        ///   GPIO IRQ Sense selection GPIO input
        GISEN4, 4, 1,  u8;
        ///   GPIO IRQ Sense selection GPIO input
        GISEN5, 5, 1,  u8;
        ///   GPIO IRQ Sense selection GPIO input
        GISEN6, 6, 1,  u8;
        ///   GPIO IRQ Sense selection GPIO input
        GISEN7, 7, 1,  u8;
        ///   GPIO IRQ Sense selection GPIO input
        GISEN8, 8, 1,  u8;
    }
    /// GPIO Interrupt Mode (Level / Edge)
    [0x05, 0x1C, 2, RW, GPIO_IMODE(gpio_imode)] {
        ///   GPIO IRQ Mode selection for GPIO input
        GIMOD0, 0, 1,  u8;
        ///   GPIO IRQ Mode selection for GPIO input
        GIMOD1, 1, 1,  u8;
        ///   GPIO IRQ Mode selection for GPIO input
        GIMOD2, 2, 1,  u8;
        ///   GPIO IRQ Mode selection for GPIO input
        GIMOD3, 3, 1,  u8;
        ///   GPIO IRQ Mode selection for GPIO input
        GIMOD4, 4, 1,  u8;
        ///   GPIO IRQ Mode selection for GPIO input
        GIMOD5, 5, 1,  u8;
        ///   GPIO IRQ Mode selection for GPIO input
        GIMOD6, 6, 1,  u8;
        ///   GPIO IRQ Mode selection for GPIO input
        GIMOD7, 7, 1,  u8;
        ///   GPIO IRQ Mode selection for GPIO input
        GIMOD8, 8, 1,  u8;
    }
    /// GPIO Interrupt “Both Edge” Select
    [0x05, 0x20, 2, RW, GPIO_IBES(gpio_ibes)] {
        ///   GPIO IRQ “Both Edge” selection for GPIO input
        GIBES0, 0, 1,  u8;
        ///   GPIO IRQ “Both Edge” selection for GPIO input
        GIBES1, 1, 1,  u8;
        ///   GPIO IRQ “Both Edge” selection for GPIO input
        GIBES2, 2, 1,  u8;
        ///   GPIO IRQ “Both Edge” selection for GPIO input
        GIBES3, 3, 1,  u8;
        ///   GPIO IRQ “Both Edge” selection for GPIO input
        GIBES4, 4, 1,  u8;
        ///   GPIO IRQ “Both Edge” selection for GPIO input
        GIBES5, 5, 1,  u8;
        ///   GPIO IRQ “Both Edge” selection for GPIO input
        GIBES6, 6, 1,  u8;
        ///   GPIO IRQ “Both Edge” selection for GPIO input
        GIBES7, 7, 1,  u8;
        ///   GPIO IRQ “Both Edge” selection for GPIO input
        GIBES8, 8, 1,  u8;
    }
    /// GPIO Interrupt Latch Clear
    [0x05, 0x24, 4, RW, GPIO_ICLR(gpio_iclr)] {
        ///   GPIO IRQ latch clear for GPIO input
        GICLR0, 0, 1,  u8;
        ///   GPIO IRQ latch clear for GPIO input
        GICLR1, 1, 1,  u8;
        ///   GPIO IRQ latch clear for GPIO input
        GICLR2, 2, 1,  u8;
        ///   GPIO IRQ latch clear for GPIO input
        GICLR3, 3, 1,  u8;
        ///   GPIO IRQ latch clear for GPIO input
        GICLR4, 4, 1,  u8;
        ///   GPIO IRQ latch clear for GPIO input
        GICLR5, 5, 1,  u8;
        ///   GPIO IRQ latch clear for GPIO input
        GICLR6, 6, 1,  u8;
        ///   GPIO IRQ latch clear for GPIO input
        GICLR7, 7, 1,  u8;
        ///   GPIO IRQ latch clear for GPIO input
        GICLR8, 8, 1,  u8;
    }
    /// GPIO Interrupt De-bounce Enable
    [0x05, 0x28, 4, RW, GPIO_IDBE(gpio_idbe)] {
        ///   GPIO IRQ de-bounce enable for GPIO
        GIDBE0, 0, 1,  u8;
        ///   GPIO IRQ de-bounce enable for GPIO
        GIDBE1, 1, 1,  u8;
        ///   GPIO IRQ de-bounce enable for GPIO
        GIDBE2, 2, 1,  u8;
        ///   GPIO IRQ de-bounce enable for GPIO
        GIDBE3, 3, 1,  u8;
        ///   GPIO IRQ de-bounce enable for GPIO
        GIDBE4, 4, 1,  u8;
        ///   GPIO IRQ de-bounce enable for GPIO
        GIDBE5, 5, 1,  u8;
        ///   GPIO IRQ de-bounce enable for GPIO
        GIDBE6, 6, 1,  u8;
        ///   GPIO IRQ de-bounce enable for GPIO
        GIDBE7, 7, 1,  u8;
        ///   GPIO IRQ de-bounce enable for GPIO
        GIDBE8, 8, 1,  u8;
    }
    /// GPIO Raw State
    [0x05, 0x2C, 2, RO, GPIO_RAW(gpio_raw)] {
        ///   GPIO port raw state
        GRAWP0, 0, 1,  u8;
        ///   GPIO port raw state
        GRAWP1, 1, 1,  u8;
        ///   GPIO port raw state
        GRAWP2, 2, 1,  u8;
        ///   GPIO port raw state
        GRAWP3, 3, 1,  u8;
        ///   GPIO port raw state
        GRAWP4, 4, 1,  u8;
        ///   GPIO port raw state
        GRAWP5, 5, 1,  u8;
        ///   GPIO port raw state
        GRAWP6, 6, 1,  u8;
        ///   GPIO port raw state
        GRAWP7, 7, 1,  u8;
        ///   GPIO port raw state
        GRAWP8, 8, 1,  u8;
    }
    /// PAC configuration
    [0x06, 0x00, 2, RW, DTUNE0(dtune0)] {
        ///   Preamble Acquisition Chunk size
        PAC, 0, 2,  u8;
        ///   Tuning bit 4 of digital tuning reg0
        DT0B4, 4, 1,  u8;
    }
    /// SFD timeout
    [0x06, 0x02, 2, RW, RX_SFD_TOC(rx_sfd_toc)] {
        /// don't set to 0
        VALUE, 0, 16,  u16;
    }
    /// Preamble detection timeout
    [0x06, 0x04, 2, RW, PRE_TOC(pre_toc)] {
        /// digital receiver configuration
        VALUE, 0, 16,  u16;
    }
    /// Receiver tuning register
    [0x06, 0x0C, 4, RW, DTUNE3(dtune3)] {
        /// value
        VALUE, 0, 32,  u32;
    }
    /// Digital Tuning Reserved register
    [0x06, 0x10, 4, RW, DTUNE4(dtune4)] {
        /// value
        VALUE, 24, 8,  u32;
    }
    /// Digital Tuning Reserved register
    [0x06, 0x14, 4, RO, DTUNE5(dtune5)] {
        /// value
        VALUE, 0, 32,  u32;
    }
    /// Carrier recovery integrator register
    [0x06, 0x29, 3, RO, DRX_CAR_INT(drx_car_int)] {
        /// value
        VALUE, 0, 24,  u32;
    }
    /// RF control enable
    [0x07, 0x00, 4, RW, RF_ENABLE(rf_enable)] {
        /// value
        VALUE, 0, 32,  u32;
    }
    /// RF enable mask
    [0x07, 0x04, 4, RW, RF_CTRL_MASK(rf_ctrl_mask)] {
        /// value
        VALUE, 0, 32,  u32;
    }
    /// RF switch configuration
    [0x07, 0x14, 4, RW, RF_SWITCH(rf_switch)] {
        /// When set to 1, the automatic toggling of the antenna switch is disabled when the device is operating in PDoA modes
        ANTSWNOTOGGLE, 0, 1,  u8;
        /// Specifies the starting port for reception when the device is operating in PDoA modes
        ANTSWPDOAPORT, 1, 1,  u8;
        /// Setting this to 1 will enable manual control of the antenna switch
        ANTSWEN, 8, 1,  u8;
        /// Manual control of antenna switch when ANTSWEN is set
        ANTSWCTRL, 12, 3,  u8;
        /// Setting this to 1 will enable manual control of the TX RX switch
        TRXSWEN, 16, 1,  u8;
        /// TX/RX switch control when TRXSWEN bit is set
        TRXSWCTRL, 24, 6,  u8;
    }
    /// RF transmitter configuration
    [0x07, 0x1A, 1, RW, RF_TX_CTRL_1(rf_tx_ctrl_1)] {
        /// value
        VALUE, 0, 8,  u8;
    }
    /// RF transmitter configuration
    [0x07, 0x1C, 4, RW, RF_TX_CTRL_2(rf_tx_ctrl_2)] {
        /// Pulse Generator Delay value
        VALUE, 0, 32,  u32;
    }
    /// Transmitter test configuration
    [0x07, 0x28, 1, RW, TX_TEST(tx_test)] {
        /// Transmitter test enable
        TX_ENTEST, 0, 4,  u8;
    }
    /// Transmitter Calibration – SAR temperaturesensor read enable
    [0x07, 0x34, 1, RW, SAR_TEST(rsar_test)] {
        /// Writing 1 enables the SAR temperature sensor reading
        SAR_RDEN, 2, 1,  u8;
    }
    /// Internal LDO voltage tuning parameter
    [0x07, 0x40, 8, RW, LDO_TUNE(ldo_tune)] {
        ///  used to control the output voltage levels of the on chip LDOs
        VALUE, 0, 61,  u128;
    }
    /// LDO control
    [0x07, 0x48, 4, RW, LDO_CTRL(ldo_ctrl)] {
        ///  LDO control
        LOW, 0, 16,  u16;
        ///  LDO control
        HIGH, 16, 16,  u16;
    }
    /// LDO tuning register
    [0x07, 0x51, 1, RW, LDO_RLOAD(ldo_rload)] {
        ///  LDO tuning register
        VALUE, 0, 8,  u8;
    }
    /// Transmitter Calibration – SAR control
    [0x08, 0x00, 1, RW, SAR_CTRL(sar_ctrl)] {
        /// Writing 1 sets SAR enable and writing 0 clears the enable.
        SAR_START, 0, 1,  u8;
    }
    /// Transmitter Calibration – SAR  status
    [0x08, 0x04, 1, RW, SAR_STATUS(sar_status)] {
        /// Set to 1 when the data is ready to be read.
        SAR_DONE, 0, 1,  u8;
    }
    /// Transmitter Calibration –Latest SAR readings
    [0x08, 0x08, 3, RO, SAR_READING(sar_reading)] {
        /// Latest SAR reading for Voltage level.
        SAR_LVBAT, 0, 8,  u8;
        /// Latest SAR reading for Temperature level.
        SAR_LTEMP, 8, 8,  u8;
    }
    /// Transmitter Calibration – SAR readings at last wake-up
    [0x08, 0x0C, 2, RO, SAR_WAKE_RD(sar_wake_rd)] {
        /// SAR reading of Voltage level taken at last wake up event.
        SAR_WVBAT, 0, 8,  u8;
        /// To read the temp, use SAR_READING instead.
        SAR_WTEMP, 8, 8,  u8;
    }
    /// Transmitter Calibration – Pulse Generator control
    [0x08, 0x10, 2, RW, PGC_CTRL(pgc_ctrl)] {
        /// Start the pulse generator calibration.
        PG_START, 0, 1,  u8;
        /// Start the pulse generator auto-calibration.
        PGC_AUTO_CAL, 1, 1,  u8;
        /// Number of clock cycles over which to run the pulse generator calibration counter.
        PGC_TMEAS, 2, 4,  u8;
    }
    /// Transmitter Calibration – Pulse Generator status
    [0x08, 0x14, 2, RO, PGC_STATUS(pgc_status)] {
        /// Pulse generator count value
        PG_DELAY_CNT, 0, 12,  u16;
        /// Auto-calibration of the PG_DELAY  has completed.
        AUTOCAL_DONE, 12, 1,  u8;
    }
    /// Transmitter Calibration – Pulse Generator test
    [0x08, 0x18, 2, RW, PG_TEST(pg_test)] {
        /// Pulse Generator test
        VALUE, 0, 16,  u16;
    }
    /// Transmitter Calibration – Pulse Generator count target value
    [0x08, 0x1C, 2, RO, PG_CAL_TARGET(pg_cal_target)] {
        /// Pulse generator target value of PG_COUNT at which point PG auto cal will complete.
        VALUE, 0, 12,  u16;
    }
    /// PLL configuration
    [0x09, 0x00, 2, RW, PLL_CFG(pll_cfg)] {
        /// PLL configuration
        VALUE, 0, 16,  u16;
    }
    /// PLL coarse code – starting code for calibration procedure
    [0x09, 0x04, 4, RW, PLL_CC(pll_cc)] {
        /// PLL calibration coarse code for channel 9.
        CH9_CODE, 0, 8,  u8;
        /// PLL calibration coarse code for channel 5.
        CH5_CODE, 8, 14,  u16;
        /// PLL calibration coarse code.
        VALUE, 0, 32,  u32;
    }
    /// PLL calibration configuration
    [0x09, 0x08, 2, RW, PLL_CAL(pll_cal)] {
        /// Use the coarse code value as set in PLL_CC register as starting point for PLL calibration.
        USE_OLD, 1, 1,  u8;
        /// PLL calibration configuration value.
        PLL_CFG_LD, 4, 4,  u8;
        /// PLL  calibration  enable  bit.
        CAL_EN, 8, 1,  u8;
    }
    /// Frequency synthesiser – Crystal trim
    [0x09, 0x14, 1, RW, XTAL(xtal)] {
        /// Crystal Trim.
        VALUE, 0, 8,  u8;
    }
    /// AON wake up configuration register
    [0x0A, 0x00, 3, RW, AON_DIG_CFG(aon_dig_cfg)] {
        /// On Wake-up download the AON array.
        ONW_AON_DLD, 0, 1,  u8;
        /// On Wake-up Run the (temperature and voltage) Analog-to-Digital Convertors.
        ONW_RUN_SAR, 1, 1,  u8;
        /// On Wake-up go to IDLE_PLL state.
        ONW_GO2IDLE, 8, 1,  u8;
        /// On Wake-up go to RX.
        ONW_GO2RX, 9, 1,  u8;
        /// On Wake-up perform RX calibration
        ONW_PGFCAL, 11, 1,  u8;
    }
    /// AON control register
    [0x0A, 0x04, 1, RW, AON_CTRL(aon_ctrl)] {
        /// Copy the user configurations from the AON memory to the host interface register set.
        RESTORE, 0, 1,  u8;
        /// Copy the user configurations from the host interface register  set  into  the  AON  memory.
        SAVE, 1, 1,  u8;
        /// Upload the AON block configurations to the AON.
        CFG_UPLOAD, 2, 1,  u8;
        /// Direct AON memory access read.
        DCA_READ, 3, 1,  u8;
        /// Direct AON memory write access
        DCA_WRITE, 4, 1,  u8;
        /// Direct AON memory write access. Needs to be set when using address > 0xFF
        DCA_WRITE_HI, 5, 1,  u8;
        /// Direct AON memory access enable bit.
        DCA_ENAB, 7, 1,  u8;
    }
    /// AON direct access read data result
    [0x0A, 0x08, 1, RW, AON_RDATA(aon_rdata)] {
        /// AON direct access read data result
        VALUE, 0, 8,  u8;
    }
    /// AON direct access address
    [0x0A, 0x0C, 2, RW, AON_ADDR(aon_addr)] {
        /// AON direct access address
        VALUE, 0, 16,  u16;
    }
    /// AON direct access write data
    [0x0A, 0x10, 1, RW, AON_WDATA(aon_wdata)] {
        /// AON direct access write data
        VALUE, 0, 8,  u8;
    }
    /// AON configuration register
    [0x0A, 0x14, 1, RW, AON_CFG(aon_cfg)] {
        /// Sleep enable configuration bit.
        SLEEP_EN, 0, 1,  u8;
        /// Wake when sleep counter elapses.
        WAKE_CNT, 1, 1,  u8;
        /// Enable the BROWNOUT detector during SLEEP or DEEPSLEEP.
        BROUT_EN, 2, 1,  u8;
        /// Wake using SPI access.
        WAKE_CSN, 3, 1,  u8;
        /// Wake using WAKEUP pin.
        WAKE_WUP, 4, 1,  u8;
        /// Preserve Sleep.
        PRES_SLEEP, 5, 1,  u8;
    }
    /// OTP data to program to a particular address
    [0x0B, 0x00, 4, RW, OTP_WDATA(otp_wdata)] {
        /// OTP data to program to a particular address
        VALUE, 0, 32,  u32;
    }
    /// OTP address to which to program the data
    [0x0B, 0x04, 4, RW, OTP_ADDR(otp_addr)] {
        /// Address within OTP memory that will be accessed read or written.
        VALUE, 0, 11,  u16;
    }
    /// OTP configuration register
    [0x0B, 0x08, 2, RW, OTP_CFG(otp_cfg)] {
        /// Enable manual control over OTP interface.
        OTP_MAN, 0, 1,  u8;
        /// OTP read enable.
        OTP_READ, 1, 1,  u8;
        /// OTP write enable.
        OTP_WRITE, 2, 1,  u8;
        /// OTP write mode.
        OTP_WRITE_MR, 3, 1,  u8;
        /// Loading of the RX_TUNE_CAL parameter
        DGC_KICK, 6, 1,  u8;
        /// Loading of the LDOTUNE_CAL parameter
        LDO_KICK, 7, 1,  u8;
        /// Loading of the BIASTUNE_CAL parameter
        BIAS_KICK, 8, 1,  u8;
        /// Loading of the operating parameter set selected by the OPS_SEL configuration
        OPS_KICK, 10, 1,  u8;
        /// Operating parameter set selection.
        OPS_SEL, 11, 2,  u8;
        /// RX_TUNE parameter set selection.
        DGC_SEL, 13, 1,  u8;
    }
    /// OTP memory programming status register
    [0x0B, 0x0C, 1, RW, OTP_STAT(otp_stat)] {
        /// OTP Programming Done
        OTP_PROG_DONE, 0, 1,  u8;
        /// OTP Programming Voltage OK.
        OTP_VPP_OK, 1, 1,  u8;
    }
    /// OTP data read from given address
    [0x0B, 0x10, 4, RO, OTP_RDATA(otp_rdata)] {
        /// OTP data read from given address
        VALUE, 0, 32,  u32;
    }
    /// OTP Special Register (SR) read data
    [0x0B, 0x14, 4, RW, OTP_SRDATA(otp_srdata)] {
        /// OTP Special Register (SR) read data
        VALUE, 0, 32,  u32;
    }
    /// Preamble sequence receive time stamp and status
    [0x0C, 0x00, 8, RO, IP_TS(ip_ts)] {
        /// Preamble sequence Time of Arrival estimate.
        IP_TOA, 0, 40,  u64;
        /// Phase of arrival as computed from the preamble CIR.
        IP_POA, 40, 14,  u16;
        /// Preamble sequence Time of Arrival status indicator.
        IP_TOAST, 56, 8,  u8;
    }
    /// STS receive time stamp and status
    [0x0C, 0x08, 8, RO, STS_TS(sts_ts)] {
        /// STS Time of Arrival estimate.
        STS_TOA, 0, 40,  u64;
        /// Phase of arrival as computed from the STS CIR.
        STS_POA, 40, 14,  u16;
        /// STS sequence Time of Arrival status indicator.
        STS_TOAST, 55, 9,  u16;
    }
    /// 2nd STS receive time stamp and status
    [0x0C, 0x10, 8, RO, STS1_TS(sts1_ts)] {
        /// STS second Time of Arrival estimate.
        STS1_TOA, 0, 40,  u64;
        /// Phase of arrival as computed from the STS based CIR estimate.
        STS1_POA, 40, 14,  u16;
        /// STS second Time of Arrival status indicator.
        STS1_TOAST, 55, 9,  u16;
    }
    /// The TDoA between the two CIRs
    [0x0C, 0x18, 6, RO, TDOA(tdoa)] {
        /// The TDoA between the two CIRs
        VALUE, 0, 48,  u64;
    }
    /// The PDoA between the two CIRs
    [0x0C, 0x1E, 2, RO, PDOA(pdoa)] {
        /// Phase difference result.
        VALUE, 0, 14,  u16;
        /// First path threshold test mode.
        FP_TH_MD, 14, 1,  u8;
    }
    /// CIA Diagnostic 0
    [0x0C, 0x20, 4, RO, CIA_DIAG_0(cia_diag_0)] {
        /// Clock offset estimate.
        COE_PPM, 0, 13,  u16;
    }
    /// Reserved diagnostic data
    [0x0C, 0x24, 4, RO, CIA_DIAG_1(cia_diag_1)] {
    }
    /// Preamble Diagnostic 0 – peak
    [0x0C, 0x28, 4, RO, IP_DIAG_0(ip_diag_0)] {
        /// Amplitude of the sample accumulated using the preamble sequence.
        IP_PEAKA, 0, 21,  u32;
        /// Index of the sample accumulated using the preamble sequence.
        IP_PEAKI, 21, 10,  u16;
    }
    /// Preamble Diagnostic 1 – power indication
    [0x0C, 0x2C, 4, RO, IP_DIAG_1(ip_diag_1)] {
        /// Channel area accumulated using the preamble sequence.
        IP_CAREA, 0, 17,  u32;
    }
    /// Preamble Diagnostic 2 – magnitude @ FP + 1
    [0x0C, 0x30, 4, RO, IP_DIAG_2(ip_diag_2)] {
        /// Magnitude of the sample at the first index immediately after the estimated first path position accumulated using the preamble sequence.
        IP_FP1M, 0, 22,  u32;
    }
    /// Preamble Diagnostic 3 – magnitude @ FP + 2
    [0x0C, 0x34, 4, RO, IP_DIAG_3(ip_diag_3)] {
        /// Magnitude of the sample at the second index immediately after the estimated first path position accumulated using the preamble sequence.
        IP_FP2M, 0, 22,  u32;
    }
    /// Preamble Diagnostic 4 – magnitude @ FP + 3
    [0x0C, 0x38, 4, RO, IP_DIAG_4(ip_diag_4)] {
        /// Magnitude of the sample at the third index immediately after the estimated first path position accumulated using the preamble sequence.
        IP_FP3M, 0, 22,  u32;
    }
    /// Reserved diagnostic data
    [0x0C, 0x3C, 12, RO, IP_DIAG_RES1(ip_diag_res1)] {
    }
    /// Preamble Diagnostic 8 – first path
    [0x0C, 0x48, 4, RO, IP_DIAG_8(ip_diag_8)] {
        /// Estimated first path location accumulated using the preamble sequence.
        IP_FP, 0, 16,  u16;
    }
    /// Reserved diagnostic data
    [0x0C, 0x4C, 12, RO, IP_DIAG_RES2(ip_diag_res2)] {
    }
    /// Preamble Diagnostic 12 – symbols accumulated
    [0x0C, 0x58, 4, RO, IP_DIAG_12(ip_diag_12)] {
        /// Number of preamble sequence symbols that were accumulated to form the preamble CIR.
        IP_NACC, 0, 12,  u16;
    }
    /// STS 0 Diagnostic 0 – STS CIA peak amplitude
    [0x0C, 0x5C, 4, RO, STS_DIAG_0(sts_diag_0)] {
        /// Amplitude of the sample accumulated using the STS
        CP0_PEAKA, 0, 21,  u32;
        /// Index of the sample accumulated using the STS
        CP0_PEAKI, 21, 9,  u16;
    }
    /// STS 0 Diagnostic 1 – STS power indication
    [0x0C, 0x60, 4, RO, STS_DIAG_1(sts_diag_1)] {
        /// Channel area accumulated using the the STS
        CP0_CAREA, 0, 16,  u16;
    }
    /// STS 0 Diagnostic 2 – STS magnitude @ FP + 1
    [0x0C, 0x64, 4, RO, STS_DIAG_2(sts_diag_2)] {
        /// Magnitude of the sample at the first index immediately after the estimated first path position accumulated using the STS
        CP0_FP1M, 0, 22,  u32;
    }
    /// STS 0 Diagnostic 3 – STS magnitude @ FP + 2
    [0x0C, 0x68, 4, RO, STS_DIAG_3(sts_diag_3)] {
        /// Magnitude of the sample at the second index immediately after the estimated first path position accumulated using the STS
        CP0_FP2M, 0, 22,  u32;
    }
    /// STS 0 Diagnostic 4 – STS magnitude @ FP + 3
    [0x0D, 0x00, 4, RO, STS_DIAG_4(sts_diag_4)] {
        /// Magnitude of the sample at the third index immediately after the estimated first path position accumulated using the STS
        CP0_FP3M, 0, 22,  u32;
    }
    /// Reserved diagnostic data
    [0x0D, 0x04, 12, RO, STS0_DIAG_RES1(sts0_diag_res1)] {
    }
    /// STS 0 Diagnostic 8 – STS first path
    [0x0D, 0x10, 4, RO, STS_DIAG_8(sts_diag_8)] {
        /// Estimated first path location accumulated using the STS
        CP0_FP, 0, 15,  u16;
    }
    /// Reserved diagnostic data
    [0x0D, 0x14, 12, RO, STS0_DIAG_RES2(sts0_diag_res2)] {
    }
    /// STS 0 diagnostic 12 – accumulated STS length
    [0x0D, 0x20, 4, RO, STS_DIAG_12(sts_diag_12)] {
        /// Number of preamble sequence symbols that were accumulated to form the preamble CIR.
        CP0_NACC, 0, 11,  u16;
    }
    /// Reserved diagnostic data
    [0x0D, 0x24, 20, RO, STS0_DIAG_RES3(sts0_diag_res3)] {
    }
    /// STS 1 Diagnostic 0 – STS CIA peak amplitude
    [0x0D, 0x38, 4, RO, STS1_DIAG_0(sts1_diag_0)] {
        /// Amplitude of the sample accumulated using the STS
        CP1_PEAKA, 0, 21,  u32;
        /// Index of the sample accumulated using the STS
        CP1_PEAKI, 21, 9,  u16;
    }
    /// STS 1 Diagnostic 1 – STS power indication
    [0x0D, 0x3C, 4, RO, STS1_DIAG_1(sts1_diag_1)] {
        /// Channel area accumulated using the the STS
        CP1_CAREA, 0, 16,  u16;
    }
    /// STS 1 Diagnostic 2 – STS magnitude @ FP + 1
    [0x0D, 0x40, 4, RO, STS1_DIAG_2(sts1_diag_2)] {
        /// Magnitude of the sample at the first index immediately after the estimated first path position accumulated using the STS
        CP1_FP1M, 0, 22,  u32;
    }
    /// STS 1 Diagnostic 3 – STS magnitude @ FP + 2
    [0x0D, 0x44, 4, RO, STS1_DIAG_3(sts1_diag_3)] {
        /// Magnitude of the sample at the second index immediately after the estimated first path position accumulated using the STS
        CP1_FP2M, 0, 22,  u32;
    }
    /// STS 1 Diagnostic 4 – STS magnitude @ FP + 3
    [0x0D, 0x48, 4, RO, STS1_DIAG_4(sts1_diag_4)] {
        /// Magnitude of the sample at the third index immediately after the estimated first path position accumulated using the STS
        CP1_FP3M, 0, 22,  u32;
    }
    /// Reserved diagnostic data
    [0x0D, 0x4C, 12, RO, STS1_DIAG_RES1(sts1_diag_res1)] {
    }
    /// STS 1 Diagnostic 8 – STS first path
    [0x0D, 0x58, 4, RO, STS1_DIAG_8(sts1_diag_8)] {
        /// Estimated first path location accumulated using the STS
        CP1_FP, 0, 15,  u16;
    }
    /// Reserved diagnostic data
    [0x0D, 0x5C, 12, RO, STS1_DIAG_RES2(sts1_diag_res2)] {
    }
    /// STS 1 Diagnostic 12 – STS accumulated STS length
    [0x0D, 0x68, 4, RO, STS1_DIAG_12(sts1_diag_12)] {
        /// Number of preamble sequence symbols that were accumulated to form the preamble CIR.
        CP1_NACC, 0, 11,  u16;
    }
    /// CIA general configuration
    [0x0E, 0x00, 4, RW, CIA_CONF(cia_conf)] {
        /// Configures the receive antenna delay.
        RXANTD, 0, 16,  u16;
        ///  Minimum Diagnostics.
        MINDIAG, 20, 1,  u8;
    }
    /// First path temp adjustment and thresholds
    [0x0E, 0x04, 4, RW, FP_CONF(fp_conf)] {
        /// The threshold to use when performing the FP_AGREE test.
        FP_AGREED_TH, 8, 3,  u8;
        /// Temperature at which the device was calibrated.
        CAL_TEMP, 11, 8,  u8;
        /// Temperature compensation for RX antenna delay.
        TC_RXDLY_EN, 20, 1,  u8;
    }
    /// Preamble Config – CIA preamble configuration
    [0x0E, 0x0C, 4, RW, IP_CONF_LO(ip_conf_lo)] {
        /// Preamble Noise Threshold Multiplier.
        IP_NTM, 0, 5,   u8;
        /// Preamble Peak Multiplier.
        IP_PMULT, 5, 2,   u8;
        /// Undocumented bitfield for SCP mode.
        IP_SCP, 8, 2,   u8;
        /// Preamble replica threshold multiplier
        IP_RTM, 16, 5,  u8;
    }
    /// Preamble Config – CIA preamble configuration
    [0x0E, 0x0E, 4, RW, IP_CONF_HI(ip_conf_hi)] {
        /// Undocumented IP_CONF_HI register
        VALUE, 0, 32,  u32;
    }
    /// STS Config 0 – CIA STS configuration
    [0x0E, 0x12, 4, RW, STS_CONF_0(sts_conf_0)] {
        /// STS Noise Threshold Multiplier.
        STS_NTM, 0, 5,  u8;
        /// STS Peak Multiplier.
        STS_PMULT, 5, 2,  u8;
        /// Undocumented bitfield for SCP mode.
        STS_SCP, 8, 8,   u8;
        /// STS replica threshold multiplier
        STS_RTM, 16, 7,  u8;
    }
    /// STS Config 1 – CIA STS configuration
    [0x0E, 0x16, 4, RW, STS_CONF_1(sts_conf_1)] {
        /// Tuning value
        RES_B0, 0, 8,  u8;
        /// Checks to see if the two ToA estimates are within allowed tolerances.
        FP_AGREED_EN, 28, 1,  u8;
        /// Checks how consistent the impulse response stays during the accumulation of the STS.
        STS_CQ_EN, 29, 1,  u8;
        /// Compare the sampling statistics of the STS reception to those of the earlier reception of the preamble sequence.
        STS_SS_EN, 30, 1,  u8;
        /// Test the growth rate of the STS based CIR to the earlier growth rate of the preamble based CIR.
        STS_PGR_EN, 31, 1,  u8;
    }
    /// User adjustment to the PDoA
    [0x0E, 0x1A, 2, RW, CIA_ADJUST(cia_adjust)] {
        /// Adjustment value to account for non-balanced antenna circuits.
        VALUE, 0, 14,  u8;
    }
    /// Event counter control
    [0x0F, 0x00, 1, RW, EVC_CTRL(evc_ctrl)] {
        /// Event Counters Enable.
        EVC_EN, 0, 1,  u8;
        /// Event Counters Clear.
        EVC_CLR, 1, 1,  u8;
    }
    /// PHR error counter
    [0x0F, 0x04, 2, RO, EVC_PHE(evc_phe)] {
        /// PHR Error Event Counter.
        VALUE, 0, 12,  u16;
    }
    /// RSD error counter
    [0x0F, 0x06, 2, RO, EVC_RSE(evc_rse)] {
        /// Reed Solomon decoder (Sync Loss) Error Event Counter.
        VALUE, 0, 12,  u16;
    }
    /// Frame check sequence good counter
    [0x0F, 0x08, 2, RO, EVC_FCG(evc_fcg)] {
        /// Frame Check Sequence Good Event Counter.
        VALUE, 0, 12,  u16;
    }
    /// Frame Check Sequence error counter
    [0x0F, 0x0A, 2, RO, EVC_FCE(evc_fce)] {
        /// Frame Check Sequence Error Event Counter.
        VALUE, 0, 12,  u16;
    }
    /// Frame filter rejection counter
    [0x0F, 0x0C, 1, RO, EVC_FFR(evc_ffr)] {
        /// Frame Filter Rejection Event Counter.
        VALUE, 0, 8,  u8;
    }
    /// RX overrun error counter
    [0x0F, 0x0E, 1, RO, EVC_OVR (evc_ovr)] {
        /// RX Overrun Error Event Counter.
        VALUE, 0, 8,  u8;
    }
    /// SFD timeout counter
    [0x0F, 0x10, 2, RO, EVC_STO(evc_sto)] {
        /// SFD timeout errors Event Counter.
        VALUE, 0, 12,  u16;
    }
    /// Preamble timeout counter
    [0x0F, 0x12, 2, RO, EVC_PTO(evc_pto)] {
        /// Preamble  Detection  Timeout  Event  Counter.
        VALUE, 0, 12,  u16;
    }
    /// RX frame wait timeout counter
    [0x0F, 0x14, 1, RO, EVC_FWTO(evc_fwto)] {
        /// RX  Frame  Wait  Timeout  Event  Counter.
        VALUE, 0, 8,  u8;
    }
    /// TX frame sent counter
    [0x0F, 0x16, 2, RO, EVC_TXFS(evc_txfs)] {
        /// TX Frame Sent Event Counter.
        VALUE, 0, 12,  u16;
    }
    /// Half period warning counter
    [0x0F, 0x18, 1, RO, EVC_HPW(evc_hpw)] {
        /// Half Period Warning Event Counter.
        VALUE, 0, 8,  u8;
    }
    /// SPI write CRC error counter
    [0x0F, 0x1A, 1, RO, EVC_SWCE(evc_swce)] {
        /// SPI write CRC error counter.
        VALUE, 0, 8,  u8;
    }
    /// Digital diagnostics reserved area 1
    [0x0F, 0x1C, 8, RO, EVC_RES1(evc_res1)] {
        /// Digital diagnostics reserved area 1
        VALUE, 0, 64,  u64;
    }
    /// Test mode control register
    [0x0F, 0x24, 4, RW, DIAG_TMC(diag_tmc)] {
        /// Transmit Power Spectrum Test Mode.
        TX_PSTM, 4, 1,  u8;
        /// Host interrupt polarity.
        HIRQ_POL, 21, 1,  u8;
        /// Enable the CIA watchdog.
        CIA_WDEN, 24, 1,  u8;
        /// Run the CIA manually.
        CIA_RUN, 26, 1,  u8;
    }
    /// STS quality error counter
    [0x0F, 0x28, 1, RO, EVC_CPQE(evc_cpqe)] {
        /// STS quality error counter
        VALUE, 0, 8,  u8;
    }
    /// Low voltage warning error counter
    [0x0F, 0x2A, 1, RO, EVC_VWARN(evc_vwarn)] {
        /// Low voltage warning error counter
        VALUE, 0, 8,  u8;
    }
    /// SPI mode
    [0x0F, 0x2C, 1, RO, SPI_MODE(spi_mode)] {
        /// SPI mode
        VALUE, 0, 2,  u8;
    }
    /// System states *
    [0x0F, 0x30, 4, RO, SYS_STATE(sys_state)] {
        /// Current Transmit State Machine value
        TX_STATE, 0, 4,  u8;
        /// Current Receive State Machine value
        RX_STATE, 8, 4,  u8;
        /// Current PMSC State Machine value
        PMSC_STATE, 16, 8,  u8;
    }
    /// Fast command status
    [0x0F, 0x3C, 1, RO, FCMD_STAT(fcmd_stat)] {
        /// Fast command status.
        VALUE, 0, 5,  u8;
    }
    /// Current value of  the low 32-bits of the STS IV
    [0x0F, 0x48, 4, RO, CTR_DBG(ctr_dbg)] {
        /// Current value of  the low 32-bits of the STS IV
        VALUE, 0, 32,  u32;
    }
    /// SPI CRC LFSR initialisation code
    [0x0F, 0x4C, 1, RO, SPICRCINIT(spicrcinit)] {
        /// SPI CRC LFSR initialisation code for the SPI CRC function.
        VALUE, 0, 8,  u8;
    }
    /// Soft reset of the device blocks
    [0x11, 0x00, 2, RW, SOFT_RST(soft_rst)] {
        /// Soft ARM reset
        ARM_RST, 0, 1,  u8;
        /// Soft PRGN reset
        PRGN_RST, 1, 1,  u8;
        /// Soft CIA reset
        CIA_RST, 2, 1,  u8;
        /// Soft BIST reset
        BIST_RST, 3, 1,  u8;
        /// Soft RX reset
        RX_RST, 4, 1,  u8;
        /// Soft TX reset
        TX_RST, 5, 1,  u8;
        /// Soft HIF reset
        HIF_RST, 6, 1,  u8;
        /// Soft PMSC reset
        PMSC_RST, 7, 1,  u8;
        /// Soft GPIO reset
        GPIO_RST, 8, 1,  u8;
    }
    /// PMSC clock control register
    [0x11, 0x04, 4, RW, CLK_CTRL(clk_ctrl)] {
        /// System Clock Selection field.
        SYS_CLK, 0, 2,  u8;
        /// Receiver Clock Selection
        RX_CLK, 2, 2,  u8;
        /// Transmitter Clock Selection.
        TX_CLK, 4, 2,  u8;
        /// Force Accumulator Clock Enable
        ACC_CLK_EN, 6, 1,  u8;
        /// Force CIA Clock Enable
        CIA_CLK_EN, 8, 1,  u8;
        /// Analog-to-Digital Convertor Clock Enable.
        SAR_CLK_EN, 10, 1,  u8;
        /// Accumulator Memory Clock Enable.
        ACC_MCLK_EN, 15, 1,  u8;
        /// GPIO clock Enable
        GPIO_CLK_EN, 16, 1,  u8;
        /// GPIO De-bounce Clock Enable.
        GPIO_DCLK_EN, 18, 1,  u8;
        /// GPIO de-bounce reset (NOT), active low.
        GPIO_DRST_N, 19, 1,  u8;
        /// Kilohertz clock Enable.
        LP_CLK_EN, 23, 1,  u8;
    }
    /// PMSC sequencing control register
    [0x11, 0x08, 4, RW, SEQ_CTRL(seq_ctrl)] {
        /// Automatic  IDLE_RC  to  IDLE_PLL.
        AINIT2IDLE, 8, 1,  u8;
        /// After TX automatically Sleep.
        ATX2SLP, 11, 1,  u8;
        /// After RX automatically Sleep.
        ARX2SLP, 12, 1,  u8;
        /// This enables a 1 GHz clock used for some external SYNC modes.
        PLL_SYNC, 15, 1,  u8;
        /// CIA run enable.
        CIARUNE, 17, 1,  u8;
        /// Force to IDLE_RC state.
        FORCE2INIT, 23, 1,  u8;
        /// Kilohertz clock divisor.
        LP_CLK_DIV, 26, 6,  u8;
    }
    /// PMSC fine grain TX sequencing control
    [0x11, 0x12, 4, RW, TXFSEQ(txfseq)] {
        /// PMSC fine grain TX sequencing control
        VALUE, 0, 32,  u32;
    }
    /// PMSC fine grain TX sequencing control
    [0x11, 0x16, 4, RW, LED_CTRL(led_ctrl)] {
        /// Blink time count value.
        BLINK_TIM, 0, 8,  u8;
        /// Blink Enable.
        BLINK_EN, 8, 1,  u8;
        /// Manually triggers an LED blink.
        FORCE_TRIG, 16, 4,  u8;
    }
    /// Receiver SNIFF mode configuration
    [0x11, 0x1A, 4, RW, RX_SNIFF(rx_sniff)] {
        /// SNIFF Mode ON time.
        SNIFF_ON, 0, 4,  u8;
        /// SNIFF Mode OFF time specified in μs.
        SNIFF_OFF, 8, 8,  u8;
    }
    /// Analog blocks’ calibration values
    [0x11, 0x1F, 2, RW, BIAS_CTRL(bias_ctrl)] {
        /// Analog blocks’ calibration values
        VALUE, 0, 14,  u16;
    }
    ///
    /// Transmit Data Buffer
    ///
    /// <div class="warning">
    /// This register mapping and its contents have been hand verified to be correct by a real human.
    /// </div>
    ///
    [0x14, 0x00, 1024, WO, TX_BUFFER(tx_buffer)] {
    }
    /// Read access to accumulator data memory
    [0x15, 0x00, 12288, RO, ACC_MEM(acc_mem)] {
    }
    /// Scratch RAM memory buffer
    [0x16, 0x00, 127, RW, SCRATCH_RAM(scratch_ram)] {
    }
    /// storage for up to 8 x 128 bit AES KEYs
    [0x17, 0x00, 128, RW, AES_KEY_RAM(aes_key_ram)] {
        /// 1st AES key
        AES_KEY1, 0, 128,  u128;
        /// 2nd AES key
        AES_KEY2, 128, 128,  u128;
        /// 3rd AES key
        AES_KEY3, 256, 128,  u128;
        /// 4th AES key
        AES_KEY4, 384, 128,  u128;
        /// 5th AES key
        AES_KEY5, 512, 128,  u128;
        /// 6th AES key
        AES_KEY6, 640, 128,  u128;
        /// 7th AES key
        AES_KEY7, 768, 128,  u128;
        /// 8th AES key
        AES_KEY8, 896, 128,  u128;
    }
    /// Double buffer diagnostic register set
    [0x18, 0x00, 464, RO, DB_DIAG(db_diag)] {
    }
    /// Double buffer diagnostic register set 1
    [0x18, 0x00, 232, RO, DB_DIAG_SET1(db_diag_set1)] {
    }
    /// Double buffer diagnostic register set 2
    [0x18, 0xE8, 232, RO, DB_DIAG_SET2(db_diag_set2)] {
    }
    /// Indirect pointer A
    [0x1D, 0x00, 1, RW, INDIRECT_PTR_A(indirect_ptr_a)] {
        /// Indirect pointer A
        VALUE, 0, 8,  u8;
    }
    /// Indirect pointer B
    [0x1E, 0x00, 1, RW, INDIRECT_PTR_B(indirect_ptr_b)] {
        /// Indirect pointer B
        VALUE, 0, 8,  u8;
    }
    /// Fast System Event Status Register
    [0x1F, 0x00, 1, RO, FINT_STAT(fint_stat)] {
        /// TXFRB or TXPRS or TXPHS or TXFRS.
        TXOK, 0, 1,   u8;
        /// AAT or CCA_FAIL.
        CCA_FAIL, 1, 1,   u8;
        /// CIAERR
        RXTSERR, 2, 1,   u8;
        /// RXFR and CIADONE or RXFCG.
        RXOK, 3, 1,   u8;
        /// RXFCE or RXFSL or  RXPHE or  ARFE or  RXSTO or RXOVRR.
        RXERR, 4, 1,   u8;
        /// RXFTO  or  RXPTO.
        RXTO, 5, 1,   u8;
        /// VT_DET or GPIOIRQ or RCINIT or SPIRDY.
        SYS_EVENT, 6, 1,   u8;
        /// AES_ERR or CMD_ERR or SPI_UNF or SPI_OVF or SPIERR or PLL_HILO or VWARN.
        SYS_PANIC, 7, 1,   u8;
    }
    /// Base address of the register to be accessed through indirect pointer A
    [0x1F, 0x04, 1, RW, PTR_ADDR_A(ptr_addr_a)] {
        /// Base address of the register to be accessed through indirect pointer A
        PTRA_BASE, 0, 5,   u8;
    }
    /// Offset address of the register to be accessed through indirect pointer A
    [0x1F, 0x08, 2, RW, PTR_OFFSET_A(ptr_offset_a)] {
        /// Offset address of the register to be accessed through indirect pointer A
        PTRA_OFS, 0, 15,   u16;
    }
    /// Base address of the register to be accessed through indirect pointer B
    [0x1F, 0x0C, 1, RW, PTR_ADDR_B(ptr_addr_b)] {
        /// Base address of the register to be accessed through indirect pointer B
        PTRB_BASE, 0, 5,   u8;
    }
    /// Offset address of the register to be accessed through indirect pointer B
    [0x1F, 0x10, 2, RW, PTR_OFFSET_B(ptr_offset_b)] {
        /// Offset address of the register to be accessed through indirect pointer B
        PTRB_OFS, 0, 15,   u16;
    }
}

impl TX_BUFFER {
    pub fn write(&mut self) {
        todo!()
    }
}

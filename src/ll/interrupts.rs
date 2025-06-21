//! Interrupts for the DW3XXX.
//! 

/// 
/// An interrupt from the DW3XXX
/// 
/// The enum representation values are equal to the offest of each interrupt bit within the interrupt registers. It should be noted, however,
/// that the interrupt bits are not continuous within the register, so there are index values for which there is no corresponding interrupt.
/// Moreover, the interrupt registers span two 32-bit sub-registers, so the indices range from 0 to 64.
/// 
pub enum Interrupt {
    Cplock  = 1,
    Spicrce = 2,
    Aat     = 3,
    Txfrb   = 4,
    Txprs   = 5,
    Txphs   = 6,
    Txfrs   = 7,
    Rxprd   = 8,
    Rxsfdd  = 9,
    Ciadone = 10,
    Rxphd   = 11,
    Rxphe   = 12,
    Rxfr    = 13,
    Rxfcg   = 14,
    Rxfce   = 15,
    Rxfsl   = 16,
    Rxfto   = 17,
    Ciaerr  = 18,
    Vwarn   = 19,
    Rxovrr  = 20,
    Rxpto   = 21,
    Spirdy  = 23,
    Rcinit  = 24,
    PllHilo = 25,
    Rxsto   = 26,
    Hpdwarn = 27,
    Cperr   = 28,
    Arfe    = 29,
    Rxprej  = 33,
    VtDet   = 36,
    Gpioirq = 37,
    AesDone = 38,
    AesErr  = 39,
    CmdErr  = 40,
    SpiOvf  = 41,
    SpiUnf  = 42,
    Spierr  = 43,
    CcaFail = 44
}
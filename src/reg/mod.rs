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

use core::ops::{BitOrAssign, Index, IndexMut, Shl};

pub mod bindings;

///
/// Generates a register "or" mask for a field of a given start bit and length in a register of `N` bytes.
/// ```rust
/// # use dw3xxx::ll::reg::register_or_mask;
/// //                                         [   01   ]  [   02   ]  [   03   ]
/// //                                         [87654321]  [87654321]  [87654321]
/// assert_eq!(register_or_mask::<3>(5, 12), [0b11100000, 0b11111111, 0b00000001])
/// ```
///
pub fn register_or_mask<const N: usize>(start: usize, len: usize) -> [u8; N] {
    let mut res = [0b00000000; N];

    for i in start..(start + len) {
        let byte_index = i / 8;
        let bit_index = i % 8;

        res[byte_index] |= 1 << bit_index;
    }

    res
}

///
/// Generates a register "and" mask for field of a given start bit and length in a register of `N` bytes.
///
/// ```rust
/// # use dw3xxx::ll::reg::register_and_mask;
/// //                                          [   01   ]  [   02   ]  [   03   ]
/// //                                          [87654321]  [87654321]  [87654321]
/// assert_eq!(register_and_mask::<3>(5, 12), [0b00011111, 0b00000000, 0b11111110])
/// ```
///
pub fn register_and_mask<const N: usize>(start: usize, len: usize) -> [u8; N] {
    let mut res = [0b11111111; N];

    for i in start..(start + len) {
        let byte_index = i / 8;
        let bit_index = i % 8;

        res[byte_index] ^= 1 << bit_index;
    }

    res
}
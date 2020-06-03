#![no_std]
#![allow(non_camel_case_types)]

#[cfg(not(feature = "device-selected"))]
compile_error!(
        "This crate requires one of the following device features enabled:
        same70q21b
"
);

pub use embedded_hal as hal;

pub use nb;
pub use nb::block;

#[cfg(feature = "same70q21b")]
pub use atsame7xx_pac::atsame70q21b as pac;

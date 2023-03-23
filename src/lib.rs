#![feature(portable_simd)]

#[cfg(feature = "dev")]
pub mod keccak;
#[cfg(not(feature = "dev"))]
mod keccak;

#[cfg(feature = "dev")]
pub mod sponge;
#[cfg(not(feature = "dev"))]
mod sponge;

mod tests;
mod turboshake128;
mod turboshake256;

pub use turboshake128::TurboShake128;
pub use turboshake256::TurboShake256;

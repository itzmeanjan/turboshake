//! # TurboShake
//!
//! TurboSHAKE is a family of extendable output functions (xof) powered by round-reduced ( i.e. 12 -rounds ) Keccak-p[1600, 12] permutation, proposed in https://ia.cr/2023/342.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! turboshake = "=0.4.1"
//! ```
//!
//! Then, use it in your code:
//!
//! ```rust
//! use turboshake::{TurboShake128, TurboShake256};
//!
//! let mut md = [0u8; 32];
//!
//! let mut ts128 = TurboShake128::default();
//! ts128.absorb(b"hello");
//! ts128.finalize::<{TurboShake128::DEFAULT_DOMAIN_SEPARATOR}>();
//! ts128.squeeze(&mut md);
//!
//! println!("TurboSHAKE128: {:x?}", md);
//!
//! let mut ts256 = TurboShake256::default();
//! ts256.absorb(b"hello");
//! ts256.finalize::<{TurboShake256::DEFAULT_DOMAIN_SEPARATOR}>();
//! ts256.squeeze(&mut md);
//!
//! println!("TurboSHAKE256: {:x?}", md);
//! ```
//!
//! See project [README](https://github.com/itzmeanjan/turboshake) for more details.

#[cfg(feature = "dev")]
pub mod keccak;
#[cfg(not(feature = "dev"))]
mod keccak;

#[cfg(feature = "dev")]
pub mod sponge;
#[cfg(not(feature = "dev"))]
mod sponge;

mod branch_opt_util;
mod tests;
mod turboshake128;
mod turboshake256;

pub use turboshake128::TurboShake128;
pub use turboshake256::TurboShake256;

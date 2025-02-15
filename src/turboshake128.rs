use crate::{keccak, sponge};

/// TurboSHAKE128 Extendable Output Function (XOF), which can produce an arbitrary amount of output, given an arbitrary length input.
///
/// See section 1 of TurboSHAKE specification https://ia.cr/2023/342.
#[derive(Copy, Clone)]
pub struct TurboShake128 {
    state: [u64; 25],
    offset: usize,
    is_ready: usize,
    squeezable: usize,
}

impl Default for TurboShake128 {
    /// Create a default instance of TurboSHAKE128 Extendable Output Function (XOF).
    ///
    /// # Inputs
    ///
    /// None
    ///
    /// # Outputs
    ///
    /// A default `TurboShake128` object.
    ///
    /// # Example
    ///
    /// ```
    /// use turboshake::{TurboShake128};
    /// let mut shake = TurboShake128::default();
    /// ```
    fn default() -> Self {
        Self {
            state: [0u64; 25],
            offset: 0,
            is_ready: usize::MIN,
            squeezable: 0,
        }
    }
}

impl TurboShake128 {
    /// If you don't need multiple instances of TurboSHAKE128, feel free to pass this as domain seperator constant, during finalization.
    pub const DEFAULT_DOMAIN_SEPARATOR: u8 = 0x1f;

    const BIT_LENGTH_OF_KECCAK_PERMUTATION_STATE: usize = keccak::W * keccak::LANE_CNT;
    const TARGET_BIT_SECURITY_LEVEL: usize = 128;
    const CAPACITY_BITS: usize = 2 * Self::TARGET_BIT_SECURITY_LEVEL;
    const RATE_BITS: usize = Self::BIT_LENGTH_OF_KECCAK_PERMUTATION_STATE - Self::CAPACITY_BITS;
    const RATE_BYTES: usize = Self::RATE_BITS / u8::BITS as usize;

    /// Absorbs arbitrary many input bytes into the TurboSHAKE128 sponge state.
    ///
    /// # Inputs
    ///
    /// * `msg`: A slice of bytes to be absorbed.
    ///
    /// # Outputs
    ///
    /// * `bool`: True if the absorption was successful, false otherwise.  Returns false if the instance has already been finalized.
    ///
    /// # Example
    ///
    /// ```
    /// use turboshake::{TurboShake128};
    /// let mut shake = TurboShake128::default();
    /// let message = b"This is a test message";
    /// shake.absorb(message);
    /// ```
    pub fn absorb(&mut self, msg: &[u8]) -> bool {
        if self.is_ready == usize::MAX {
            return false;
        }

        sponge::absorb::<{ Self::RATE_BYTES }>(&mut self.state, &mut self.offset, msg);
        true
    }

    /// Finalizes the TurboSHAKE128 sponge state.
    ///
    /// # Inputs
    ///
    /// * `D`: A domain separator byte.  Consider using `TurboShake128::DEFAULT_DOMAIN_SEPARATOR` if you don't need multiple instances of TurboSHAKE128.
    ///
    /// # Outputs
    ///
    /// * `bool`: True if the finalization was successful, false otherwise. Returns false if the instance has already been finalized.
    ///
    /// # Example
    ///
    /// ```
    /// use turboshake::{TurboShake128};
    /// let mut shake = TurboShake128::default();
    /// let message = b"This is a test message";
    /// shake.absorb(message);
    /// shake.finalize::<{TurboShake128::DEFAULT_DOMAIN_SEPARATOR}>();
    /// ```
    pub fn finalize<const D: u8>(&mut self) -> bool {
        // See top of page 2 of https://ia.cr/2023/342
        const { assert!(D >= 0x01 && D <= 0x7f) };

        if self.is_ready == usize::MAX {
            return false;
        }

        sponge::finalize::<{ Self::RATE_BYTES }, { D }>(&mut self.state, &mut self.offset);

        self.is_ready = usize::MAX;
        self.squeezable = Self::RATE_BYTES;
        true
    }

    /// Squeezes arbitrary many output bytes from the TurboSHAKE128 sponge state.
    ///
    /// # Inputs
    ///
    /// * `out`: A mutable slice of bytes to be filled with squeezed output.
    ///
    /// # Outputs
    ///
    /// * `bool`: True if the squeezing was successful, false otherwise. Returns false if the instance has not been finalized.
    ///
    /// # Example
    ///
    /// ```
    /// use turboshake::{TurboShake128};
    /// let mut shake = TurboShake128::default();
    /// let message = b"This is a test message";
    /// shake.absorb(message);
    /// shake.finalize::<{TurboShake128::DEFAULT_DOMAIN_SEPARATOR}>();
    /// let mut output = [0u8; 32];
    /// shake.squeeze(&mut output);
    /// ```
    pub fn squeeze(&mut self, out: &mut [u8]) -> bool {
        if self.is_ready != usize::MAX {
            return false;
        }

        sponge::squeeze::<{ Self::RATE_BYTES }>(&mut self.state, &mut self.squeezable, out);
        true
    }
}

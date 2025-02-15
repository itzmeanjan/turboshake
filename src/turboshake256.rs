use crate::{keccak, sponge};

/// TurboSHAKE256 Extendable Output Function (XOF), which can produce an arbitrary amount of output, given an arbitrary length input.
///
/// See section 1 of TurboSHAKE specification https://ia.cr/2023/342.
#[derive(Copy, Clone)]
pub struct TurboShake256 {
    state: [u64; 25],
    offset: usize,
    is_ready: usize,
    squeezable: usize,
}

impl Default for TurboShake256 {
    /// Create a default instance of TurboSHAKE256 Extendable Output Function (XOF).
    ///
    /// # Inputs
    ///
    /// None
    ///
    /// # Outputs
    ///
    /// A default `TurboShake256` object.
    ///
    /// # Example
    ///
    /// ```
    /// use turboshake::{TurboShake256};
    /// let mut shake = TurboShake256::default();
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

impl TurboShake256 {
    /// If you don't need multiple instances of TurboSHAKE256, feel free to pass this as domain seperator constant, during finalization.
    pub const DEFAULT_DOMAIN_SEPARATOR: u8 = 0x1f;

    const BIT_LENGTH_OF_KECCAK_PERMUTATION_STATE: usize = keccak::W * keccak::LANE_CNT;
    const TARGET_BIT_SECURITY_LEVEL: usize = 256;
    const CAPACITY_BITS: usize = 2 * Self::TARGET_BIT_SECURITY_LEVEL;
    const RATE_BITS: usize = Self::BIT_LENGTH_OF_KECCAK_PERMUTATION_STATE - Self::CAPACITY_BITS;
    const RATE_BYTES: usize = Self::RATE_BITS / u8::BITS as usize;

    /// Absorbs arbitrary many input bytes into the TurboSHAKE256 sponge state.
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
    /// use turboshake::{TurboShake256};
    /// let mut shake = TurboShake256::default();
    /// let message = b"This is a test message";
    /// assert!(shake.absorb(message));
    /// ```
    pub fn absorb(&mut self, msg: &[u8]) -> bool {
        if self.is_ready == usize::MAX {
            return false;
        }

        sponge::absorb::<{ Self::RATE_BYTES }>(&mut self.state, &mut self.offset, msg);
        true
    }

    /// Finalizes the TurboSHAKE256 sponge state.
    ///
    /// # Inputs
    ///
    /// * `D`: A domain separator byte.  Consider using `TurboShake256::DEFAULT_DOMAIN_SEPARATOR` if you don't need multiple instances of TurboSHAKE256.
    ///
    /// # Outputs
    ///
    /// * `bool`: True if the finalization was successful, false otherwise. Returns false if the instance has already been finalized.
    ///
    /// # Example
    ///
    /// ```
    /// use turboshake::{TurboShake256};
    /// let mut shake = TurboShake256::default();
    /// let message = b"This is a test message";
    /// assert!(shake.absorb(message));
    /// assert!(shake.finalize::<{TurboShake256::DEFAULT_DOMAIN_SEPARATOR}>());
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

    /// Squeezes arbitrary many output bytes from the TurboSHAKE256 sponge state.
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
    /// use turboshake::{TurboShake256};
    /// let mut shake = TurboShake256::default();
    /// let message = b"This is a test message";
    /// assert!(shake.absorb(message));
    /// assert!(shake.finalize::<{TurboShake256::DEFAULT_DOMAIN_SEPARATOR}>());
    /// let mut output = [0u8; 32];
    /// assert!(shake.squeeze(&mut output));
    /// ```
    pub fn squeeze(&mut self, out: &mut [u8]) -> bool {
        if self.is_ready != usize::MAX {
            return false;
        }

        sponge::squeeze::<{ Self::RATE_BYTES }>(&mut self.state, &mut self.squeezable, out);
        true
    }
}

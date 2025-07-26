use crate::{branch_opt_util, error::TurboShakeError, keccak, sponge};

/// TurboSHAKE256 Extendable Output Function (XOF)
///
/// Given any arbitrary length input, in incremental form or in one-shot form,
/// it can produce an arbitrary long pseudo-random, deterministic output, offering
/// at max 256-bits of security.
///
/// See section 1 of TurboSHAKE specification https://ia.cr/2023/342.
#[derive(Clone)]
pub struct TurboShake256 {
    state: [u64; keccak::LANE_CNT],
    offset: usize,
    is_ready_to_squeeze: usize,
    squeezable: usize,
}

impl Default for TurboShake256 {
    /// Create a default instance of TurboSHAKE256 Extendable Output Function (XOF).
    ///
    /// # Inputs
    ///
    /// None
    ///
    /// # Returns
    ///
    /// A default `TurboShake256` object.
    ///
    /// # Example
    ///
    /// ```
    /// use turboshake::TurboShake256;
    ///
    /// let mut ts = TurboShake256::default();
    /// ```
    fn default() -> Self {
        Self {
            state: [0u64; keccak::LANE_CNT],
            offset: 0,
            is_ready_to_squeeze: usize::MIN,
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
    /// It can be called as many times needed, as long as `finalize` has not been called.
    ///
    /// # Inputs
    ///
    /// * `msg`: An arbitrary length (including empty) slice of bytes to be absorbed.
    ///
    /// # Returns
    ///
    /// * `Result<(), TurboShakeError>`: `Ok(())` if the absorption was successful.
    /// Returns `Err(TurboShakeError::DataAbsorptionPhaseAlreadyFinalized)` if the instance has already been finalized.
    ///
    /// # Example
    ///
    /// ```
    /// use turboshake::TurboShake256;
    ///
    /// let mut ts = TurboShake256::default();
    /// let message = b"This is a test message";
    /// assert_eq!(ts.absorb(message), Ok(()));
    /// ```
    pub fn absorb(&mut self, msg: &[u8]) -> Result<(), TurboShakeError> {
        if branch_opt_util::unlikely(self.is_ready_to_squeeze == usize::MAX) {
            return Err(TurboShakeError::DataAbsorptionPhaseAlreadyFinalized);
        }

        sponge::absorb::<{ Self::RATE_BYTES }>(&mut self.state, &mut self.offset, msg);
        Ok(())
    }

    /// Finalizes the TurboSHAKE256 sponge state. After all input bytes are absorbed,
    /// the sponge can be finalized, then it can only be used for squeezing output.
    ///
    /// # Inputs
    ///
    /// * `D`: A domain separator byte.  Consider using `TurboShake256::DEFAULT_DOMAIN_SEPARATOR` if you don't need multiple instances of TurboSHAKE256.
    ///
    /// # Returns
    ///
    /// * `Result<(), TurboShakeError>`: `Ok(())` if the finalization was successful.
    /// Returns `Err(TurboShakeError::DataAbsorptionPhaseAlreadyFinalized)` if the instance has already been finalized.
    ///
    /// # Example
    ///
    /// ```
    /// use turboshake::TurboShake256;
    ///
    /// let mut ts = TurboShake256::default();
    /// let message = b"This is a test message";
    ///
    /// assert_eq!(ts.absorb(message), Ok(()));
    /// assert_eq!(ts.finalize::<{TurboShake256::DEFAULT_DOMAIN_SEPARATOR}>(), Ok(()));
    /// ```
    pub fn finalize<const D: u8>(&mut self) -> Result<(), TurboShakeError> {
        // See top of page 2 of https://ia.cr/2023/342
        const { assert!(D >= 0x01 && D <= 0x7f) };

        if branch_opt_util::unlikely(self.is_ready_to_squeeze == usize::MAX) {
            return Err(TurboShakeError::DataAbsorptionPhaseAlreadyFinalized);
        }

        sponge::finalize::<{ Self::RATE_BYTES }, { D }>(&mut self.state, &mut self.offset);

        self.is_ready_to_squeeze = usize::MAX;
        self.squeezable = Self::RATE_BYTES;
        Ok(())
    }

    /// Squeezes arbitrary many output bytes from the TurboSHAKE256 sponge state.
    /// Only after the sponge state is finalized, it can be squeezed from.
    ///
    /// # Inputs
    ///
    /// * `out`: An arbitrary length (including empty) mutable slice of bytes to be filled with squeezed output.
    ///
    /// # Outputs
    ///
    /// * `Result<(), TurboShakeError>`: `Ok(())` if the squeezing was successful.
    /// Returns `Err(TurboShakeError::StillInDataAbsorptionPhase)` if the instance has not yet been finalized.
    ///
    /// # Example
    ///
    /// ```
    /// use turboshake::TurboShake256;
    ///
    /// let mut ts = TurboShake256::default();
    /// let message = b"This is a test message";
    ///
    /// assert_eq!(ts.absorb(message), Ok(()));
    /// assert_eq!(ts.finalize::<{TurboShake256::DEFAULT_DOMAIN_SEPARATOR}>(), Ok(()));
    ///
    /// let mut output = [0u8; 32];
    /// assert_eq!(ts.squeeze(&mut output), Ok(()));
    /// ```
    pub fn squeeze(&mut self, out: &mut [u8]) -> Result<(), TurboShakeError> {
        if branch_opt_util::unlikely(self.is_ready_to_squeeze != usize::MAX) {
            return Err(TurboShakeError::StillInDataAbsorptionPhase);
        }

        sponge::squeeze::<{ Self::RATE_BYTES }>(&mut self.state, &mut self.squeezable, out);
        Ok(())
    }
}

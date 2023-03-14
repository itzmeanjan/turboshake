use crate::sponge;

/// TurboSHAKE128 Extendable Output Function (XOF)
///
/// See section 1 of TurboSHAKE specification https://ia.cr/2023/342
#[derive(Copy, Clone)]
pub struct TurboShake128 {
    state: [u64; 25],
    offset: usize,
    is_ready: usize,
    squeezable: usize,
}

impl TurboShake128 {
    /// If you don't need multiple instances of TurboSHAKE128, feel free to pass
    /// this as domain seperator constant, during finalization.
    pub const DEFAULT_DOMAIN_SEPARATOR: u8 = 0x1f;

    const CAPACITY_BITS: usize = 256;
    const RATE_BITS: usize = 1600 - Self::CAPACITY_BITS;
    const RATE_BYTES: usize = Self::RATE_BITS / 8;
    const RATE_WORDS: usize = Self::RATE_BYTES / 8;

    /// Create a new instance of TurboSHAKE128 Extendable Output Function (XOF), into
    /// which arbitrary number of message bytes can be absorbed and arbitrary many bytes
    /// can be squeezed out.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            state: [0u64; 25],
            offset: 0,
            is_ready: usize::MIN,
            squeezable: 0,
        }
    }

    /// Given N -bytes input message, this routine consumes those into Keccak\[256\] sponge state
    ///
    /// Note, this routine can be called arbitrary number of times, each time with arbitrary
    /// bytes of input message, until keccak\[256\] state is finalized ( by calling routine with
    /// similar name ). Once finalized, calling this routine again doesn't do anything.
    ///
    /// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/shake128.hpp#L43-L130
    #[inline(always)]
    pub fn absorb(&mut self, msg: &[u8]) {
        if self.is_ready == usize::MAX {
            return;
        }

        sponge::absorb::<{ Self::RATE_BYTES }, { Self::RATE_WORDS }>(
            &mut self.state,
            &mut self.offset,
            msg,
        );
    }

    /// After consuming N -bytes ( by invoking absorb routine arbitrary many times,
    /// each time with arbitrary input bytes ), this routine is invoked when no more
    /// input bytes remaining to be consumed into Keccak\[256\] sponge state.
    ///
    /// Note, once this routine is called, calling absorb() or finalize() again, on same
    /// TurboSHAKE128 object doesn't do anything. After finalization, one might wish to
    /// read arbitrary many bytes by squeezing sponge, which is done by calling squeeze()
    /// function, as many times required.
    ///
    /// Consider using D = 0x1f, if you don't need multiple instances of TurboSHAKE128 XOF.
    ///
    /// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/shake128.hpp#L132-L192
    #[inline(always)]
    pub fn finalize<const D: u8>(&mut self) {
        // See top of page 2 of https://ia.cr/2023/342
        debug_assert!(D >= 0x01 && D <= 0x7f);

        if self.is_ready == usize::MAX {
            return;
        }

        sponge::finalize::<{ Self::RATE_BYTES }, { Self::RATE_WORDS }, { D }>(
            &mut self.state,
            &mut self.offset,
        );

        self.is_ready = usize::MAX;
        self.squeezable = Self::RATE_BYTES;
    }

    /// Given that N -bytes input message is already absorbed into sponge state, this
    /// routine is used for squeezing M -bytes out of consumable part of sponge state
    /// ( i.e. rate portion of the state )
    ///
    /// Note, this routine can be called arbitrary number of times, for squeezing arbitrary
    /// number of bytes from sponge Keccak\[256\].
    ///
    /// Make sure you absorb message bytes first, then only call this function, otherwise
    /// it can't squeeze anything out.
    ///
    /// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/shake128.hpp#L194-L238
    #[inline(always)]
    pub fn squeeze(&mut self, out: &mut [u8]) {
        if self.is_ready != usize::MAX {
            return;
        }

        sponge::squeeze::<{ Self::RATE_BYTES }, { Self::RATE_WORDS }>(
            &mut self.state,
            &mut self.squeezable,
            out,
        );
    }
}

use crate::keccak;
use std::cmp;

/// TurboSHAKE256 Extendable Output Function (XOF)
///
/// See section 1 of TurboSHAKE specification https://ia.cr/2023/342
#[derive(Copy, Clone)]
pub struct TurboShake256 {
    state: [u64; 25],
    offset: usize,
    is_ready: usize,
    squeezable: usize,
}

impl TurboShake256 {
    /// If you don't need multiple instances of TurboSHAKE256, feel free to pass
    /// this as domain seperator constant, during finalization.
    pub const DEFAULT_DOMAIN_SEPARATOR: u8 = 0x1f;

    const CAPACITY_BITS: usize = 512;
    const RATE_BITS: usize = 1600 - Self::CAPACITY_BITS;
    const RATE_BYTES: usize = Self::RATE_BITS / 8;
    const RATE_WORDS: usize = Self::RATE_BYTES / 8;

    /// Create a new instance of TurboSHAKE256 Extendable Output Function (XOF), into
    /// which arbitrary number of message bytes can be absorbed and arbitrary many bytes
    /// can be squeezed out.
    pub fn new() -> Self {
        Self {
            state: [0u64; 25],
            offset: 0,
            is_ready: usize::MIN,
            squeezable: 0,
        }
    }

    /// Given N -bytes input message, this routine consumes those into Keccak\[512\] sponge state
    ///
    /// Note, this routine can be called arbitrary number of times, each time with arbitrary
    /// bytes of input message, until Keccak\[512\] state is finalized ( by calling routine with
    /// similar name ). Once finalized, calling this routine again doesn't do anything.
    ///
    /// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/shake256.hpp#L43-L130
    pub fn absorb(&mut self, msg: &[u8]) {
        if self.is_ready == usize::MAX {
            return;
        }

        let mlen = msg.len();

        let mut blk_bytes = [0u8; Self::RATE_BYTES];

        let blk_cnt = (self.offset + mlen) / Self::RATE_BYTES;
        let till = blk_cnt * Self::RATE_BYTES;
        let mut moff = 0;

        while moff < till {
            let byte_cnt = Self::RATE_BYTES - self.offset;

            blk_bytes.fill(0u8);
            blk_bytes[self.offset..].copy_from_slice(&msg[moff..(moff + byte_cnt)]);

            for i in 0..Self::RATE_WORDS {
                let word = u64::from_le_bytes(blk_bytes[i * 8..(i + 1) * 8].try_into().unwrap());
                self.state[i] ^= word;
            }

            moff += Self::RATE_BYTES - self.offset;
            self.offset += Self::RATE_BYTES - self.offset;

            keccak::permute(&mut self.state);
            self.offset = 0;
        }

        let rm_bytes = mlen - moff;

        let src_frm = moff;
        let src_to = src_frm + rm_bytes;
        let dst_frm = self.offset;
        let dst_to = dst_frm + rm_bytes;

        blk_bytes.fill(0u8);
        blk_bytes[dst_frm..dst_to].copy_from_slice(&msg[src_frm..src_to]);

        for i in 0..Self::RATE_WORDS {
            let word = u64::from_le_bytes(blk_bytes[i * 8..(i + 1) * 8].try_into().unwrap());
            self.state[i] ^= word;
        }

        self.offset += rm_bytes;

        if self.offset == Self::RATE_BYTES {
            keccak::permute(&mut self.state);
            self.offset = 0;
        }
    }

    /// After consuming N -bytes ( by invoking absorb routine arbitrary many times,
    /// each time with arbitrary input bytes ), this routine is invoked when no more
    /// input bytes remaining to be consumed into Keccak\[512\] sponge state.
    ///
    /// Note, once this routine is called, calling absorb() or finalize() again, on same
    /// TurboSHAKE256 object doesn't do anything. After finalization, one might wish to
    /// read arbitrary many bytes by squeezing sponge, which is done by calling squeeze()
    /// function, as many times required.
    ///
    /// Consider using D = 0x1f, if you don't need multiple instances of TurboSHAKE256 XOF.
    ///
    /// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/shake256.hpp#L132-L192
    pub fn finalize<const D: u8>(&mut self) {
        if self.is_ready == usize::MAX {
            return;
        }

        // See top of page 2 of https://ia.cr/2023/342
        debug_assert!(D >= 0x01 && D <= 0x7f);

        let mut blk_bytes = [0u8; Self::RATE_BYTES];
        blk_bytes[self.offset] = D;
        blk_bytes[Self::RATE_BYTES - 1] ^= 0x80;

        for i in 0..Self::RATE_WORDS {
            let word = u64::from_le_bytes(blk_bytes[i * 8..(i + 1) * 8].try_into().unwrap());
            self.state[i] ^= word;
        }

        keccak::permute(&mut self.state);

        self.offset = 0;
        self.is_ready = usize::MAX;
        self.squeezable = Self::RATE_BYTES;
    }

    /// Given that N -bytes input message is already absorbed into sponge state, this
    /// routine is used for squeezing M -bytes out of consumable part of sponge state
    /// ( i.e. rate portion of the state ).
    ///
    /// Note, this routine can be called arbitrary number of times, for squeezing arbitrary
    /// number of bytes from sponge Keccak\[512\].
    ///
    /// Make sure you absorb message bytes first, then only call this function, otherwise
    /// it can't squeeze anything out.
    ///
    /// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/shake256.hpp#L194-L237
    pub fn squeeze(&mut self, out: &mut [u8]) {
        if self.is_ready != usize::MAX {
            return;
        }

        let olen = out.len();
        let mut rate = [0u8; Self::RATE_BYTES];
        let mut off = 0;

        while off < olen {
            let read = cmp::min(self.squeezable, olen - off);
            let soff = Self::RATE_BYTES - self.squeezable;

            for i in 0..Self::RATE_WORDS {
                rate[i * 8..(i + 1) * 8].copy_from_slice(&self.state[i].to_le_bytes());
            }

            out[off..(off + read)].copy_from_slice(&rate[soff..(soff + read)]);

            self.squeezable -= read;
            off += read;

            if self.squeezable == 0 {
                keccak::permute(&mut self.state);
                self.squeezable = Self::RATE_BYTES;
            }
        }
    }
}

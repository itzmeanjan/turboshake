use crate::keccak;

/// TurboSHAKE128 Extendable Output Function (XOF)
///
/// See section 1 of TurboSHAKE specification https://ia.cr/2023/342
#[derive(Copy, Clone)]
pub struct TurboShake128 {
    state: [u64; 25],
    offset: usize,
    absorbed: usize,
    ready: usize,
    squeezable: usize,
}

impl TurboShake128 {
    const CAPACITY_BITS: usize = 256;
    const RATE_BITS: usize = 1600 - Self::CAPACITY_BITS;
    const RATE_BYTES: usize = Self::RATE_BITS / 8;
    const RATE_WORDS: usize = Self::RATE_BYTES / 8;

    /// Create a new instance of TurboSHAKE128 Extendable Output Function (XOF), into
    /// which arbitrary number of message bytes can be absorbed and arbitrary many bytes
    /// can be squeezed out.
    pub fn new() -> Self {
        Self {
            state: [0u64; 25],
            offset: 0,
            absorbed: 0,
            ready: 0,
            squeezable: 0,
        }
    }

    /// Given N -bytes input message, this routine consumes those into Keccak[256] sponge state
    ///
    /// Note, this routine can be called arbitrary number of times, each time with arbitrary
    /// bytes of input message, until keccak[256] state is finalized ( by calling routine with
    /// similar name ).
    ///
    /// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/shake128.hpp#L43-L130
    pub fn absorb(&mut self, msg: &[u8]) {
        if self.ready == usize::MAX {
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

        self.absorbed += mlen;
    }
}

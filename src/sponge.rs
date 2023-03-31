use crate::keccak;
use std::cmp;

/// Given N -bytes message, this routine consumes it into Keccak\[c\] permutation state s.t.
/// `offset` ( second parameter ) denotes how many bytes are already consumed into rate portion
/// of the state.
///
/// - c i.e. capacity can be either of 256 or 512 -bits.
/// - Rate portion will have bitwidth of 1600 - c.
/// - `offset` will live in 0 <= offset < RATE_BYTES.
#[inline(always)]
pub fn absorb<const RATE_BYTES: usize, const RATE_WORDS: usize>(
    state: &mut [u64; 25],
    offset: &mut usize,
    msg: &[u8],
) {
    let mut blk_bytes = [0u8; RATE_BYTES];

    let blk_cnt = (*offset + msg.len()) / RATE_BYTES;
    let mut moff = 0;

    for _ in 0..blk_cnt {
        let byte_cnt = RATE_BYTES - *offset;

        blk_bytes.fill(0u8);
        blk_bytes[*offset..].copy_from_slice(&msg[moff..(moff + byte_cnt)]);

        for i in 0..RATE_WORDS {
            let word = u64::from_le_bytes(blk_bytes[i * 8..(i + 1) * 8].try_into().unwrap());
            state[i] ^= word;
        }

        moff += byte_cnt;
        *offset += byte_cnt;

        keccak::permute(state);
        *offset = 0;
    }

    let rm_bytes = msg.len() - moff;

    let src_frm = moff;
    let src_to = src_frm + rm_bytes;
    let dst_frm = *offset;
    let dst_to = dst_frm + rm_bytes;

    blk_bytes.fill(0u8);
    blk_bytes[dst_frm..dst_to].copy_from_slice(&msg[src_frm..src_to]);

    for i in 0..RATE_WORDS {
        let word = u64::from_le_bytes(blk_bytes[i * 8..(i + 1) * 8].try_into().unwrap());
        state[i] ^= word;
    }

    *offset += rm_bytes;
}

/// Given that N message bytes are already consumed into Keccak\[c\] permutation state, this routine
/// finalizes sponge state and makes it ready for squeezing, by appending padding bytes to input
/// message s.t. total absorbed message byte length becomes multiple of RATE_BYTES.
///
/// - c i.e. capacity can be either of 256 or 512 -bits.
/// - Rate portion will have bitwidth of 1600 - c.
/// - `offset` will live in 0 <= offset < RATE_BYTES.
#[inline(always)]
pub fn finalize<const RATE_BYTES: usize, const RATE_WORDS: usize, const D: u8>(
    state: &mut [u64; 25],
    offset: &mut usize,
) {
    let mut blk_bytes = [0u8; RATE_BYTES];
    blk_bytes[*offset] = D;
    blk_bytes[RATE_BYTES - 1] ^= 0x80;

    for i in 0..RATE_WORDS {
        let word = u64::from_le_bytes(blk_bytes[i * 8..(i + 1) * 8].try_into().unwrap());
        state[i] ^= word;
    }

    keccak::permute(state);
    *offset = 0;
}

/// Given that Keccak\[c\] permutation state is finalized, this routine can be invoked
/// for squeezing N -bytes out of rate portion of the state.
///
/// - c i.e. capacity can be either of 256 or 512 -bits.
/// - Rate portion will have bitwidth of 1600 - c.
/// - `readable` denotes how many bytes can be squeezed without permutating the sponge state.
/// - When `readable` becomes 0, state needs to be permutated again, after which RATE_BYTES can be squeezed.
#[inline(always)]
pub fn squeeze<const RATE_BYTES: usize, const RATE_WORDS: usize>(
    state: &mut [u64; 25],
    readable: &mut usize,
    out: &mut [u8],
) {
    let olen = out.len();
    let mut rate = [0u8; RATE_BYTES];
    let mut off = 0;

    while off < olen {
        let read = cmp::min(*readable, olen - off);
        let soff = RATE_BYTES - *readable;

        for i in 0..RATE_WORDS {
            rate[i * 8..(i + 1) * 8].copy_from_slice(&state[i].to_le_bytes());
        }

        out[off..(off + read)].copy_from_slice(&rate[soff..(soff + read)]);

        *readable -= read;
        off += read;

        if *readable == 0 {
            keccak::permute(state);
            *readable = RATE_BYTES;
        }
    }
}

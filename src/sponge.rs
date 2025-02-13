use crate::keccak;
use std::cmp::{self, min};

const KECCAK_WORD_BYTE_LEN: usize = keccak::W / 8;

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
    const { assert!(RATE_BYTES % KECCAK_WORD_BYTE_LEN == 0) }

    let mut block = [0u8; RATE_BYTES];
    let mut msg_offset = 0;

    while msg_offset < msg.len() {
        let remaining_num_bytes = msg.len() - msg_offset;
        let absorbable_num_bytes = min(remaining_num_bytes, RATE_BYTES - *offset);
        let effective_block_byte_len = *offset + absorbable_num_bytes;
        let padded_efffective_block_len = (effective_block_byte_len + (KECCAK_WORD_BYTE_LEN - 1))
            & KECCAK_WORD_BYTE_LEN.wrapping_neg();

        block[..padded_efffective_block_len].fill(0);
        block[*offset..(*offset + absorbable_num_bytes)]
            .copy_from_slice(&msg[msg_offset..(msg_offset + absorbable_num_bytes)]);

        let mut state_word_index = 0;
        block[..padded_efffective_block_len]
            .chunks_exact(KECCAK_WORD_BYTE_LEN)
            .for_each(|chunk_bytes| {
                let chunk_as_word = u64::from_le_bytes(chunk_bytes.try_into().unwrap());

                state[state_word_index] ^= chunk_as_word;
                state_word_index += 1;
            });

        *offset += absorbable_num_bytes;
        msg_offset += absorbable_num_bytes;

        if *offset == RATE_BYTES {
            keccak::permute(state);
            *offset = 0;
        }
    }
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
    let state_word_index = *offset / KECCAK_WORD_BYTE_LEN;
    let byte_index_in_state_word = *offset % KECCAK_WORD_BYTE_LEN;
    let shl_bit_offset = byte_index_in_state_word * 8;

    state[state_word_index] ^= (D as u64) << shl_bit_offset;
    state[RATE_WORDS - 1] ^= 0x80u64 << 56;

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

/// Converts a slice of bytes into a u64 in little-endian byte order.
///
/// Reads at most 8 bytes from the input slice. If the slice is shorter than 8 bytes, it reads only the available bytes,
/// while setting other bytes to 0. The function handles cases where the input slice is empty.
///
/// # Arguments
///
/// * `bytes` - The slice of bytes to convert.
///
/// # Returns
///
/// A u64 representing the bytes in little-endian byte order.
#[inline(always)]
pub fn u64_from_le_bytes(bytes: &[u8]) -> u64 {
    let mut word = 0;
    let readable_num_bytes = min(bytes.len(), std::mem::size_of::<u64>());

    for (idx, &byte) in bytes.iter().enumerate().take(readable_num_bytes) {
        word |= (byte as u64) << (idx * 8);
    }

    word
}

/// Converts a u64 into a slice of bytes in little-endian byte order.
///
/// Writes at most 8 bytes to the output slice. If the slice is shorter than 8 bytes, it writes only the those many bytes.
/// The function handles cases where the output slice is empty.
///
/// # Arguments
///
/// * `word` - The u64 to convert.
/// * `bytes` - The mutable slice of bytes to write to.
#[inline(always)]
pub fn u64_to_le_bytes(word: u64, bytes: &mut [u8]) {
    let writable_num_bytes = min(bytes.len(), std::mem::size_of::<u64>());

    for (idx, byte) in bytes.iter_mut().enumerate().take(writable_num_bytes) {
        *byte = (word >> (idx * 8)) as u8;
    }
}

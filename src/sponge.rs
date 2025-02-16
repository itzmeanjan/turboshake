use crate::keccak;
use std::cmp::min;

const KECCAK_WORD_BYTE_LEN: usize = keccak::W / 8;

/// Given N -bytes message, this routine consumes it into Keccak\[c\] permutation state s.t.
/// `offset` ( second parameter ) denotes how many bytes are already consumed into rate portion
/// of the state.
///
/// - c i.e. capacity can be either of 256 or 512 -bits.
/// - Rate portion will have bitwidth of 1600 - c.
/// - `offset` will live in 0 <= offset < RATE_BYTES.
#[inline(always)]
pub fn absorb<const NUM_BYTES_IN_RATE: usize>(state: &mut [u64; 25], offset: &mut usize, msg: &[u8]) {
    const { assert!(NUM_BYTES_IN_RATE % KECCAK_WORD_BYTE_LEN == 0) }

    let mut block = [0u8; NUM_BYTES_IN_RATE];
    let mut msg_offset = 0;

    while msg_offset < msg.len() {
        let remaining_num_bytes = msg.len() - msg_offset;
        let absorbable_num_bytes = min(remaining_num_bytes, NUM_BYTES_IN_RATE - *offset);
        let effective_block_byte_len = *offset + absorbable_num_bytes;
        let padded_effective_block_byte_len = (effective_block_byte_len + (KECCAK_WORD_BYTE_LEN - 1)) & KECCAK_WORD_BYTE_LEN.wrapping_neg();
        let padded_effective_block_begins_at = *offset & KECCAK_WORD_BYTE_LEN.wrapping_neg();

        block[padded_effective_block_begins_at..padded_effective_block_byte_len].fill(0);
        block[*offset..(*offset + absorbable_num_bytes)].copy_from_slice(&msg[msg_offset..(msg_offset + absorbable_num_bytes)]);

        let mut state_word_index = padded_effective_block_begins_at / KECCAK_WORD_BYTE_LEN;
        block[padded_effective_block_begins_at..padded_effective_block_byte_len]
            .chunks_exact(KECCAK_WORD_BYTE_LEN)
            .for_each(|chunk_bytes| {
                let chunk_as_word = u64::from_le_bytes(chunk_bytes.try_into().unwrap());

                state[state_word_index] ^= chunk_as_word;
                state_word_index += 1;
            });

        *offset += absorbable_num_bytes;
        msg_offset += absorbable_num_bytes;

        if *offset == NUM_BYTES_IN_RATE {
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
pub fn finalize<const NUM_BYTES_IN_RATE: usize, const D: u8>(state: &mut [u64; 25], offset: &mut usize) {
    let num_words_in_rate = const { NUM_BYTES_IN_RATE / 8 };
    let state_word_index = *offset / KECCAK_WORD_BYTE_LEN;
    let byte_index_in_state_word = *offset % KECCAK_WORD_BYTE_LEN;
    let shl_bit_offset = byte_index_in_state_word * 8;

    state[state_word_index] ^= (D as u64) << shl_bit_offset;
    state[num_words_in_rate - 1] ^= 0x80u64 << 56;

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
pub fn squeeze<const NUM_BYTES_IN_RATE: usize>(state: &mut [u64; 25], readable: &mut usize, out: &mut [u8]) {
    const { assert!(NUM_BYTES_IN_RATE % KECCAK_WORD_BYTE_LEN == 0) }

    let mut block = [0u8; NUM_BYTES_IN_RATE];
    let mut out_offset = 0;

    while out_offset < out.len() {
        let state_byte_offset = NUM_BYTES_IN_RATE - *readable;
        let remaining_num_bytes = out.len() - out_offset;
        let squeezable_num_bytes = min(remaining_num_bytes, *readable);
        let effective_block_byte_len = state_byte_offset + squeezable_num_bytes;
        let padded_efffective_block_byte_len = (effective_block_byte_len + (KECCAK_WORD_BYTE_LEN - 1)) & KECCAK_WORD_BYTE_LEN.wrapping_neg();
        let padded_effective_block_begins_at = state_byte_offset & KECCAK_WORD_BYTE_LEN.wrapping_neg();

        let mut state_word_index = padded_effective_block_begins_at / KECCAK_WORD_BYTE_LEN;
        block[padded_effective_block_begins_at..padded_efffective_block_byte_len]
            .chunks_exact_mut(KECCAK_WORD_BYTE_LEN)
            .for_each(|chunk_bytes| {
                chunk_bytes.copy_from_slice(&state[state_word_index].to_le_bytes());
                state_word_index += 1;
            });

        out[out_offset..(out_offset + squeezable_num_bytes)].copy_from_slice(&block[state_byte_offset..(state_byte_offset + squeezable_num_bytes)]);

        *readable -= squeezable_num_bytes;
        out_offset += squeezable_num_bytes;

        if *readable == 0 {
            keccak::permute(state);
            *readable = NUM_BYTES_IN_RATE;
        }
    }
}

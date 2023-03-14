use crate::keccak;

/// Given N -bytes message, this routine consumes it into Keccak\[c\] permutation state s.t.
/// offset ( second parameter ) denotes how many bytes are already consumed into rate portion
/// of the state.
///
/// - c i.e. capacity can be either of 256 or 512 -bits.
/// - Rate portion will have bitwidth of 1600 - c.
/// - offset will live in 0 <= offset < RATE_BYTES.
pub fn absorb<const RATE_BYTES: usize, const RATE_WORDS: usize>(
    state: &mut [u64; 25],
    offset: &mut usize,
    msg: &[u8],
) {
    let mlen = msg.len();

    let mut blk_bytes = [0u8; RATE_BYTES];

    let blk_cnt = (*offset + mlen) / RATE_BYTES;
    let till = blk_cnt * RATE_BYTES;
    let mut moff = 0;

    while moff < till {
        let byte_cnt = RATE_BYTES - *offset;

        blk_bytes.fill(0u8);
        blk_bytes[*offset..].copy_from_slice(&msg[moff..(moff + byte_cnt)]);

        for i in 0..RATE_WORDS {
            let word = u64::from_le_bytes(blk_bytes[i * 8..(i + 1) * 8].try_into().unwrap());
            state[i] ^= word;
        }

        moff += RATE_BYTES - *offset;
        *offset += RATE_BYTES - *offset;

        keccak::permute(state);
        *offset = 0;
    }

    let rm_bytes = mlen - moff;

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

    if *offset == RATE_BYTES {
        keccak::permute(state);
        *offset = 0;
    }
}

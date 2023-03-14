/// Logarithm base 2 of bit width of lane of Keccak-p\[1600, 12\] permutation
const L: u32 = 6;

/// Bit width of each lane of Keccak-p\[1600, 12\] permutation
const W: u32 = 1 << L;

/// \# -of rounds of Keccak permutation is applied per iteration
const ROUNDS: u32 = 12;

/// Starting index of Keccak permutation round
const SIDX: u32 = 12 + 2 * L - ROUNDS;

/// End index of Keccak permutation round
const EIDX: u32 = 12 + 2 * L - 1;

/// Keccak-p\[1600, 12\] step mapping function θ, see section 3.2.1 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L145-L175
fn theta(state: &mut [u64; 25]) {
    let mut c = [0u64; 5];

    for i in (0..25).step_by(5) {
        c[0] ^= state[i + 0];
        c[1] ^= state[i + 1];
        c[2] ^= state[i + 2];
        c[3] ^= state[i + 3];
        c[4] ^= state[i + 4];
    }

    let mut d = [0u64; 5];

    for i in 0..5 {
        let pidx = (i + 4) % 5;
        let nidx = (i + 1) % 5;

        d[i] = c[pidx] ^ c[nidx].rotate_left(1);
    }

    for i in (0..25).step_by(5) {
        state[i + 0] ^= d[0];
        state[i + 1] ^= d[1];
        state[i + 2] ^= d[2];
        state[i + 3] ^= d[3];
        state[i + 4] ^= d[4];
    }
}

/// Keccak-p\[1600, 12\] step mapping function ρ, see section 3.2.2 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L177-L190
///
/// Lane rotation factor table taken from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L25-L35
fn rho(state: &mut [u64; 25]) {
    const ROT: [u32; 25] = [
        0 % W,
        1 % W,
        190 % W,
        28 % W,
        91 % W,
        36 % W,
        300 % W,
        6 % W,
        55 % W,
        276 % W,
        3 % W,
        10 % W,
        171 % W,
        153 % W,
        231 % W,
        105 % W,
        45 % W,
        15 % W,
        21 % W,
        136 % W,
        210 % W,
        66 % W,
        253 % W,
        120 % W,
        78 % W,
    ];

    for i in 0..25 {
        state[i] = state[i].rotate_left(ROT[i]);
    }
}

/// Keccak-p\[1600, 12\] step mapping function π, see section 3.2.3 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L192-L207
///
/// Permutation table taken from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L37-L48
fn pi(state: &[u64; 25]) -> [u64; 25] {
    const PERM: [usize; 25] = [
        0, 6, 12, 18, 24, 3, 9, 10, 16, 22, 1, 7, 13, 19, 20, 4, 5, 11, 17, 23, 2, 8, 14, 15, 21,
    ];

    (0..25)
        .map(|idx| state[PERM[idx]])
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap()
}

/// Keccak-p\[1600, 12\] step mapping function χ, see section 3.2.4 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L209-L227
fn chi(state: &[u64; 25]) -> [u64; 25] {
    let mut _state = [0u64; 25];

    for y in 0..5 {
        let off = y * 5;
        for x in 0..5 {
            let x1 = (x + 1) % 5;
            let x2 = (x + 2) % 5;

            _state[off + x] = state[off + x] ^ (!state[off + x1] & state[off + x2]);
        }
    }

    _state
}

/// Keccak-p\[1600, 12\] step mapping function ι, see section 3.2.5 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L229-L235
///
/// Round constants taken from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L134-L141
fn iota(state: &mut [u64; 25], ridx: u32) {
    debug_assert!(ridx >= SIDX && ridx <= EIDX);

    const RC: [u64; 24] = [
        1,
        32898,
        9223372036854808714,
        9223372039002292224,
        32907,
        2147483649,
        9223372039002292353,
        9223372036854808585,
        138,
        136,
        2147516425,
        2147483658,
        2147516555,
        9223372036854775947,
        9223372036854808713,
        9223372036854808579,
        9223372036854808578,
        9223372036854775936,
        32778,
        9223372039002259466,
        9223372039002292353,
        9223372036854808704,
        2147483649,
        9223372039002292232,
    ];
    state[0] ^= RC[ridx as usize];
}

/// Keccak-p\[1600, 12\] round function, which applies all five
/// step mapping functions in order, mutating state array, following
/// section 3.3 of https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L237-L251
fn round(state: &mut [u64; 25], ridx: u32) {
    theta(state);
    rho(state);
    *state = pi(state);
    *state = chi(state);
    iota(state, ridx);
}

/// Keccak-p\[1600, 12\] permutation, applying 12 rounds of permutation
/// on state of dimension 5 x 5 x 64 ( = 1600 -bits ), following algorithm 7 defined
/// in section 3.3 of SHA3 specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L253-L493
pub fn permute(state: &mut [u64; 25]) {
    (SIDX..(EIDX + 1)).for_each(|ridx| round(state, ridx));
}

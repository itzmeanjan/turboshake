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
    const LANE_SIZE: u32 = 64;
    const ROT: [u32; 25] = [
        0 % LANE_SIZE,
        1 % LANE_SIZE,
        190 % LANE_SIZE,
        28 % LANE_SIZE,
        91 % LANE_SIZE,
        36 % LANE_SIZE,
        300 % LANE_SIZE,
        6 % LANE_SIZE,
        55 % LANE_SIZE,
        276 % LANE_SIZE,
        3 % LANE_SIZE,
        10 % LANE_SIZE,
        171 % LANE_SIZE,
        153 % LANE_SIZE,
        231 % LANE_SIZE,
        105 % LANE_SIZE,
        45 % LANE_SIZE,
        15 % LANE_SIZE,
        21 % LANE_SIZE,
        136 % LANE_SIZE,
        210 % LANE_SIZE,
        66 % LANE_SIZE,
        253 % LANE_SIZE,
        120 % LANE_SIZE,
        78 % LANE_SIZE,
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
fn pi(state: [u64; 25]) -> [u64; 25] {
    const PERM: [usize; 25] = [
        0, 6, 12, 18, 24, 3, 9, 10, 16, 22, 1, 7, 13, 19, 20, 4, 5, 11, 17, 23, 2, 8, 14, 15, 21,
    ];

    (0..25)
        .map(|idx| state[PERM[idx]])
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap()
}

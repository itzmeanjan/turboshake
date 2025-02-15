/// Logarithm base 2 of bit width of lane of Keccak-p\[1600, 12\] permutation.
const L: usize = 6;

/// Bit width of each lane of Keccak-p\[1600, 12\] permutation.
pub const W: usize = 1 << L;

/// \# -of lanes in keccak permutation state s.t. each lane is of 64 -bit width.
pub const LANE_CNT: usize = 25;

/// \# -of rounds of Keccak permutation is applied per iteration i.e. it's Keccak-p\[1600, 12\].
const ROUNDS: usize = 12;

/// Maximum number of rounds that can be supported by Keccak-f\[1600\] permutation.
const MAX_ROUNDS: usize = 12 + 2 * L;

/// Compile-time computed lane rotation factor table used when applying ρ step mapping function.
const ROT: [usize; LANE_CNT] = compute_rotation_factors_table();

/// Compile-time computed permutation table used when applying π step mapping function.
const PERM: [usize; LANE_CNT] = compute_permutation_table();

/// Compile-time computed round constants table used when applying ι step mapping function.
const RC: [u64; ROUNDS] = compute_round_constants_table();

/// Compile-time evaluable function for generating leftwards circular rotation offset
/// for lanes of the keccak state array, computed following step 3(a), 3(b) of algorithm 2
/// in section 3.2.2 of https://dx.doi.org/10.6028/NIST.FIPS.202.
const fn compute_rotation_factors_table() -> [usize; LANE_CNT] {
    let mut table = [0usize; LANE_CNT];

    let mut x = 1;
    let mut y = 0;
    let mut t = 0;
    while t <= 23 {
        table[y * 5 + x] = ((t + 1) * (t + 2) / 2) % W;

        let y_prime = (2 * x + 3 * y) % 5;
        x = y;
        y = y_prime;

        t += 1;
    }

    table
}

/// Compile-time evaluable function for generating table used during application
/// of π step mapping function on keccak-p\[1600, 12\] permutation state. See section
/// 3.2.3 of the specification https://dx.doi.org/10.6028/NIST.FIPS.202.
const fn compute_permutation_table() -> [usize; LANE_CNT] {
    let mut table = [0usize; LANE_CNT];

    let mut y = 0;
    while y < 5 {
        let mut x = 0;
        while x < 5 {
            table[y * 5 + x] = x * 5 + (x + 3 * y) % 5;
            x += 1;
        }
        y += 1;
    }

    table
}

/// Compile-time evaluable computation of single bit of Keccak-p\[1600, 12\] round constant,
/// using binary LFSR, defined by primitive polynomial x^8 + x^6 + x^5 + x^4 + 1.
///
/// See algorithm 5 in section 3.2.5 of http://dx.doi.org/10.6028/NIST.FIPS.202.
/// Taken from https://github.com/itzmeanjan/sha3/blob/faef1bd6f/include/keccak.hpp#L53-L91.
const fn rc(t: usize) -> bool {
    // step 1 of algorithm 5
    if t % 255 == 0 {
        return true;
    }

    // step 2 of algorithm 5
    //
    // note, step 3.a of algorithm 5 is also being
    // executed in this statement ( for first iteration, with i = 1 ) !
    let mut r = 0b10000000u16;

    // step 3 of algorithm 5
    let mut i = 1;
    while i <= t % 255 {
        let b0 = r & 1;

        r = (r & 0b011111111) ^ ((((r >> 8) & 1) ^ b0) << 8);
        r = (r & 0b111101111) ^ ((((r >> 4) & 1) ^ b0) << 4);
        r = (r & 0b111110111) ^ ((((r >> 3) & 1) ^ b0) << 3);
        r = (r & 0b111111011) ^ ((((r >> 2) & 1) ^ b0) << 2);

        // step 3.f of algorithm 5
        //
        // note, this statement also executes step 3.a for upcoming
        // iterations ( i.e. when i > 1 )
        r >>= 1;

        i += 1;
    }

    ((r >> 7) & 1) == 1
}

/// Compile-time evaluable computation of a 64 -bit round constant, which is XOR-ed into
/// the very first lane ( = lane(0, 0) ) of Keccak-p\[1600, 12\] permutation state.
///
/// Taken from https://github.com/itzmeanjan/sha3/blob/faef1bd6f/include/keccak.hpp#L93C1-L109C2
const fn compute_round_constant(r_idx: usize) -> u64 {
    let mut rc_word = 0;

    let mut j = 0;
    while j < (L + 1) {
        let boff = (1usize << j) - 1;
        rc_word |= (rc(j + 7 * r_idx) as u64) << boff;

        j += 1;
    }

    rc_word
}

/// Compile-time evaluable computation of all round constants of Keccak-p\[1600, 12\] permutation.
const fn compute_round_constants_table() -> [u64; ROUNDS] {
    let mut table = [0u64; ROUNDS];

    let mut r_idx = MAX_ROUNDS - ROUNDS;
    while r_idx < MAX_ROUNDS {
        table[r_idx - ROUNDS] = compute_round_constant(r_idx);
        r_idx += 1;
    }

    table
}

/// Keccak-p\[1600, 12\] step mapping function θ, see section 3.2.1 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L145-L175
#[inline(always)]
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

    d[0] = c[4] ^ c[1].rotate_left(1);
    d[1] = c[0] ^ c[2].rotate_left(1);
    d[2] = c[1] ^ c[3].rotate_left(1);
    d[3] = c[2] ^ c[4].rotate_left(1);
    d[4] = c[3] ^ c[0].rotate_left(1);

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
#[inline(always)]
fn rho(state: &mut [u64; 25]) {
    state.iter_mut().enumerate().for_each(|(i, v)| *v = v.rotate_left(ROT[i] as u32));
}

/// Keccak-p\[1600, 12\] step mapping function π, see section 3.2.3 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L192-L207
#[inline(always)]
fn pi<T>(istate: &[T; 25], ostate: &mut [T; 25])
where
    T: Copy,
{
    ostate.iter_mut().enumerate().for_each(|(i, v)| *v = istate[PERM[i]]);
}

/// Keccak-p\[1600, 12\] step mapping function χ, see section 3.2.4 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L209-L227
#[inline(always)]
fn chi(istate: &[u64; 25], ostate: &mut [u64; 25]) {
    for y in 0..5 {
        let off = y * 5;

        ostate[off + 0] = istate[off + 0] ^ (!istate[off + 1] & istate[off + 2]);
        ostate[off + 1] = istate[off + 1] ^ (!istate[off + 2] & istate[off + 3]);
        ostate[off + 2] = istate[off + 2] ^ (!istate[off + 3] & istate[off + 4]);
        ostate[off + 3] = istate[off + 3] ^ (!istate[off + 4] & istate[off + 0]);
        ostate[off + 4] = istate[off + 4] ^ (!istate[off + 0] & istate[off + 1]);
    }
}

/// Keccak-p\[1600, 12\] step mapping function ι, see section 3.2.5 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L229-L235
#[inline(always)]
fn iota(lane: u64, ridx: usize) -> u64 {
    lane ^ RC[ridx]
}

/// Keccak-p\[1600, 12\] round function, which applies all five
/// step mapping functions in order, mutating state array, following
/// section 3.3 of https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L237-L251
#[inline(always)]
fn round(state: &mut [u64; 25], ridx: usize) {
    let mut _state = [0u64; 25];

    theta(state);
    rho(state);
    pi(state, &mut _state);
    chi(&_state, state);
    state[0] = iota(state[0], ridx);
}

/// Keccak-p\[1600, 12\] permutation, applying 12 rounds of permutation
/// on state of dimension 5 x 5 x 64 ( = 1600 -bits ), following algorithm 7 defined
/// in section 3.3 of SHA3 specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L253-L493
#[inline(always)]
pub fn permute(state: &mut [u64; 25]) {
    const { assert!(ROUNDS % 2 == 0) }

    for i in (0..ROUNDS).step_by(2) {
        round(state, i);
        round(state, i + 1);
    }
}

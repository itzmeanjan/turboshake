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

/// Keccak-p\[1600, 12\] round function, which applies all five step mapping functions in order, for four consecutive rounds
/// starting from round index `ridx`, mutating state array, following section 3.3 of https://dx.doi.org/10.6028/NIST.FIPS.202.
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b6ce9069/include/sha3/internals/keccak.hpp#L140-L583
#[inline(always)]
fn roundx4(state: &mut [u64; LANE_CNT], ridx: usize) {
    let mut c = [0u64; 5];
    let mut d = [0u64; 5];
    let mut t;

    // Round ridx + 0
    for i in (0..LANE_CNT).step_by(5) {
        c[0] ^= state[i + 0];
        c[1] ^= state[i + 1];
        c[2] ^= state[i + 2];
        c[3] ^= state[i + 3];
        c[4] ^= state[i + 4];
    }

    d[0] = c[4] ^ c[1].rotate_left(1);
    d[1] = c[0] ^ c[2].rotate_left(1);
    d[2] = c[1] ^ c[3].rotate_left(1);
    d[3] = c[2] ^ c[4].rotate_left(1);
    d[4] = c[3] ^ c[0].rotate_left(1);

    c[0] = state[0] ^ d[0];
    t = state[6] ^ d[1];
    c[1] = t.rotate_left(ROT[6] as u32);
    t = state[12] ^ d[2];
    c[2] = t.rotate_left(ROT[12] as u32);
    t = state[18] ^ d[3];
    c[3] = t.rotate_left(ROT[18] as u32);
    t = state[24] ^ d[4];
    c[4] = t.rotate_left(ROT[24] as u32);

    state[0] = c[0] ^ (c[2] & !c[1]) ^ RC[ridx];
    state[6] = c[1] ^ (c[3] & !c[2]);
    state[12] = c[2] ^ (c[4] & !c[3]);
    state[18] = c[3] ^ (c[0] & !c[4]);
    state[24] = c[4] ^ (c[1] & !c[0]);

    t = state[10] ^ d[0];
    c[2] = t.rotate_left(ROT[10] as u32);
    t = state[16] ^ d[1];
    c[3] = t.rotate_left(ROT[16] as u32);
    t = state[22] ^ d[2];
    c[4] = t.rotate_left(ROT[22] as u32);
    t = state[3] ^ d[3];
    c[0] = t.rotate_left(ROT[3] as u32);
    t = state[9] ^ d[4];
    c[1] = t.rotate_left(ROT[9] as u32);

    state[10] = c[0] ^ (c[2] & !c[1]);
    state[16] = c[1] ^ (c[3] & !c[2]);
    state[22] = c[2] ^ (c[4] & !c[3]);
    state[3] = c[3] ^ (c[0] & !c[4]);
    state[9] = c[4] ^ (c[1] & !c[0]);

    t = state[20] ^ d[0];
    c[4] = t.rotate_left(ROT[20] as u32);
    t = state[1] ^ d[1];
    c[0] = t.rotate_left(ROT[1] as u32);
    t = state[7] ^ d[2];
    c[1] = t.rotate_left(ROT[7] as u32);
    t = state[13] ^ d[3];
    c[2] = t.rotate_left(ROT[13] as u32);
    t = state[19] ^ d[4];
    c[3] = t.rotate_left(ROT[19] as u32);

    state[20] = c[0] ^ (c[2] & !c[1]);
    state[1] = c[1] ^ (c[3] & !c[2]);
    state[7] = c[2] ^ (c[4] & !c[3]);
    state[13] = c[3] ^ (c[0] & !c[4]);
    state[19] = c[4] ^ (c[1] & !c[0]);

    t = state[5] ^ d[0];
    c[1] = t.rotate_left(ROT[5] as u32);
    t = state[11] ^ d[1];
    c[2] = t.rotate_left(ROT[11] as u32);
    t = state[17] ^ d[2];
    c[3] = t.rotate_left(ROT[17] as u32);
    t = state[23] ^ d[3];
    c[4] = t.rotate_left(ROT[23] as u32);
    t = state[4] ^ d[4];
    c[0] = t.rotate_left(ROT[4] as u32);

    state[5] = c[0] ^ (c[2] & !c[1]);
    state[11] = c[1] ^ (c[3] & !c[2]);
    state[17] = c[2] ^ (c[4] & !c[3]);
    state[23] = c[3] ^ (c[0] & !c[4]);
    state[4] = c[4] ^ (c[1] & !c[0]);

    t = state[15] ^ d[0];
    c[3] = t.rotate_left(ROT[15] as u32);
    t = state[21] ^ d[1];
    c[4] = t.rotate_left(ROT[21] as u32);
    t = state[2] ^ d[2];
    c[0] = t.rotate_left(ROT[2] as u32);
    t = state[8] ^ d[3];
    c[1] = t.rotate_left(ROT[8] as u32);
    t = state[14] ^ d[4];
    c[2] = t.rotate_left(ROT[14] as u32);

    state[15] = c[0] ^ (c[2] & !c[1]);
    state[21] = c[1] ^ (c[3] & !c[2]);
    state[2] = c[2] ^ (c[4] & !c[3]);
    state[8] = c[3] ^ (c[0] & !c[4]);
    state[14] = c[4] ^ (c[1] & !c[0]);

    // Round ridx + 1
    c.fill(0);

    for i in (0..LANE_CNT).step_by(5) {
        c[0] ^= state[i + 0];
        c[1] ^= state[i + 1];
        c[2] ^= state[i + 2];
        c[3] ^= state[i + 3];
        c[4] ^= state[i + 4];
    }

    d[0] = c[4] ^ c[1].rotate_left(1);
    d[1] = c[0] ^ c[2].rotate_left(1);
    d[2] = c[1] ^ c[3].rotate_left(1);
    d[3] = c[2] ^ c[4].rotate_left(1);
    d[4] = c[3] ^ c[0].rotate_left(1);

    c[0] = state[0] ^ d[0];
    t = state[16] ^ d[1];
    c[1] = t.rotate_left(ROT[6] as u32);
    t = state[7] ^ d[2];
    c[2] = t.rotate_left(ROT[12] as u32);
    t = state[23] ^ d[3];
    c[3] = t.rotate_left(ROT[18] as u32);
    t = state[14] ^ d[4];
    c[4] = t.rotate_left(ROT[24] as u32);

    state[0] = c[0] ^ (c[2] & !c[1]) ^ RC[ridx + 1];
    state[16] = c[1] ^ (c[3] & !c[2]);
    state[7] = c[2] ^ (c[4] & !c[3]);
    state[23] = c[3] ^ (c[0] & !c[4]);
    state[14] = c[4] ^ (c[1] & !c[0]);

    t = state[20] ^ d[0];
    c[2] = t.rotate_left(ROT[10] as u32);
    t = state[11] ^ d[1];
    c[3] = t.rotate_left(ROT[16] as u32);
    t = state[2] ^ d[2];
    c[4] = t.rotate_left(ROT[22] as u32);
    t = state[18] ^ d[3];
    c[0] = t.rotate_left(ROT[3] as u32);
    t = state[9] ^ d[4];
    c[1] = t.rotate_left(ROT[9] as u32);

    state[20] = c[0] ^ (c[2] & !c[1]);
    state[11] = c[1] ^ (c[3] & !c[2]);
    state[2] = c[2] ^ (c[4] & !c[3]);
    state[18] = c[3] ^ (c[0] & !c[4]);
    state[9] = c[4] ^ (c[1] & !c[0]);

    t = state[15] ^ d[0];
    c[4] = t.rotate_left(ROT[20] as u32);
    t = state[6] ^ d[1];
    c[0] = t.rotate_left(ROT[1] as u32);
    t = state[22] ^ d[2];
    c[1] = t.rotate_left(ROT[7] as u32);
    t = state[13] ^ d[3];
    c[2] = t.rotate_left(ROT[13] as u32);
    t = state[4] ^ d[4];
    c[3] = t.rotate_left(ROT[19] as u32);

    state[15] = c[0] ^ (c[2] & !c[1]);
    state[6] = c[1] ^ (c[3] & !c[2]);
    state[22] = c[2] ^ (c[4] & !c[3]);
    state[13] = c[3] ^ (c[0] & !c[4]);
    state[4] = c[4] ^ (c[1] & !c[0]);

    t = state[10] ^ d[0];
    c[1] = t.rotate_left(ROT[5] as u32);
    t = state[1] ^ d[1];
    c[2] = t.rotate_left(ROT[11] as u32);
    t = state[17] ^ d[2];
    c[3] = t.rotate_left(ROT[17] as u32);
    t = state[8] ^ d[3];
    c[4] = t.rotate_left(ROT[23] as u32);
    t = state[24] ^ d[4];
    c[0] = t.rotate_left(ROT[4] as u32);

    state[10] = c[0] ^ (c[2] & !c[1]);
    state[1] = c[1] ^ (c[3] & !c[2]);
    state[17] = c[2] ^ (c[4] & !c[3]);
    state[8] = c[3] ^ (c[0] & !c[4]);
    state[24] = c[4] ^ (c[1] & !c[0]);

    t = state[5] ^ d[0];
    c[3] = t.rotate_left(ROT[15] as u32);
    t = state[21] ^ d[1];
    c[4] = t.rotate_left(ROT[21] as u32);
    t = state[12] ^ d[2];
    c[0] = t.rotate_left(ROT[2] as u32);
    t = state[3] ^ d[3];
    c[1] = t.rotate_left(ROT[8] as u32);
    t = state[19] ^ d[4];
    c[2] = t.rotate_left(ROT[14] as u32);

    state[5] = c[0] ^ (c[2] & !c[1]);
    state[21] = c[1] ^ (c[3] & !c[2]);
    state[12] = c[2] ^ (c[4] & !c[3]);
    state[3] = c[3] ^ (c[0] & !c[4]);
    state[19] = c[4] ^ (c[1] & !c[0]);

    // Round ridx + 2
    c.fill(0);

    for i in (0..LANE_CNT).step_by(5) {
        c[0] ^= state[i + 0];
        c[1] ^= state[i + 1];
        c[2] ^= state[i + 2];
        c[3] ^= state[i + 3];
        c[4] ^= state[i + 4];
    }

    d[0] = c[4] ^ c[1].rotate_left(1);
    d[1] = c[0] ^ c[2].rotate_left(1);
    d[2] = c[1] ^ c[3].rotate_left(1);
    d[3] = c[2] ^ c[4].rotate_left(1);
    d[4] = c[3] ^ c[0].rotate_left(1);

    c[0] = state[0] ^ d[0];
    t = state[11] ^ d[1];
    c[1] = t.rotate_left(ROT[6] as u32);
    t = state[22] ^ d[2];
    c[2] = t.rotate_left(ROT[12] as u32);
    t = state[8] ^ d[3];
    c[3] = t.rotate_left(ROT[18] as u32);
    t = state[19] ^ d[4];
    c[4] = t.rotate_left(ROT[24] as u32);

    state[0] = c[0] ^ (c[2] & !c[1]) ^ RC[ridx + 2];
    state[11] = c[1] ^ (c[3] & !c[2]);
    state[22] = c[2] ^ (c[4] & !c[3]);
    state[8] = c[3] ^ (c[0] & !c[4]);
    state[19] = c[4] ^ (c[1] & !c[0]);

    t = state[15] ^ d[0];
    c[2] = t.rotate_left(ROT[10] as u32);
    t = state[1] ^ d[1];
    c[3] = t.rotate_left(ROT[16] as u32);
    t = state[12] ^ d[2];
    c[4] = t.rotate_left(ROT[22] as u32);
    t = state[23] ^ d[3];
    c[0] = t.rotate_left(ROT[3] as u32);
    t = state[9] ^ d[4];
    c[1] = t.rotate_left(ROT[9] as u32);

    state[15] = c[0] ^ (c[2] & !c[1]);
    state[1] = c[1] ^ (c[3] & !c[2]);
    state[12] = c[2] ^ (c[4] & !c[3]);
    state[23] = c[3] ^ (c[0] & !c[4]);
    state[9] = c[4] ^ (c[1] & !c[0]);

    t = state[5] ^ d[0];
    c[4] = t.rotate_left(ROT[20] as u32);
    t = state[16] ^ d[1];
    c[0] = t.rotate_left(ROT[1] as u32);
    t = state[2] ^ d[2];
    c[1] = t.rotate_left(ROT[7] as u32);
    t = state[13] ^ d[3];
    c[2] = t.rotate_left(ROT[13] as u32);
    t = state[24] ^ d[4];
    c[3] = t.rotate_left(ROT[19] as u32);

    state[5] = c[0] ^ (c[2] & !c[1]);
    state[16] = c[1] ^ (c[3] & !c[2]);
    state[2] = c[2] ^ (c[4] & !c[3]);
    state[13] = c[3] ^ (c[0] & !c[4]);
    state[24] = c[4] ^ (c[1] & !c[0]);

    t = state[20] ^ d[0];
    c[1] = t.rotate_left(ROT[5] as u32);
    t = state[6] ^ d[1];
    c[2] = t.rotate_left(ROT[11] as u32);
    t = state[17] ^ d[2];
    c[3] = t.rotate_left(ROT[17] as u32);
    t = state[3] ^ d[3];
    c[4] = t.rotate_left(ROT[23] as u32);
    t = state[14] ^ d[4];
    c[0] = t.rotate_left(ROT[4] as u32);

    state[20] = c[0] ^ (c[2] & !c[1]);
    state[6] = c[1] ^ (c[3] & !c[2]);
    state[17] = c[2] ^ (c[4] & !c[3]);
    state[3] = c[3] ^ (c[0] & !c[4]);
    state[14] = c[4] ^ (c[1] & !c[0]);

    t = state[10] ^ d[0];
    c[3] = t.rotate_left(ROT[15] as u32);
    t = state[21] ^ d[1];
    c[4] = t.rotate_left(ROT[21] as u32);
    t = state[7] ^ d[2];
    c[0] = t.rotate_left(ROT[2] as u32);
    t = state[18] ^ d[3];
    c[1] = t.rotate_left(ROT[8] as u32);
    t = state[4] ^ d[4];
    c[2] = t.rotate_left(ROT[14] as u32);

    state[10] = c[0] ^ (c[2] & !c[1]);
    state[21] = c[1] ^ (c[3] & !c[2]);
    state[7] = c[2] ^ (c[4] & !c[3]);
    state[18] = c[3] ^ (c[0] & !c[4]);
    state[4] = c[4] ^ (c[1] & !c[0]);

    // Round ridx + 3
    c.fill(0);

    for i in (0..LANE_CNT).step_by(5) {
        c[0] ^= state[i + 0];
        c[1] ^= state[i + 1];
        c[2] ^= state[i + 2];
        c[3] ^= state[i + 3];
        c[4] ^= state[i + 4];
    }

    d[0] = c[4] ^ c[1].rotate_left(1);
    d[1] = c[0] ^ c[2].rotate_left(1);
    d[2] = c[1] ^ c[3].rotate_left(1);
    d[3] = c[2] ^ c[4].rotate_left(1);
    d[4] = c[3] ^ c[0].rotate_left(1);

    c[0] = state[0] ^ d[0];
    t = state[1] ^ d[1];
    c[1] = t.rotate_left(ROT[6] as u32);
    t = state[2] ^ d[2];
    c[2] = t.rotate_left(ROT[12] as u32);
    t = state[3] ^ d[3];
    c[3] = t.rotate_left(ROT[18] as u32);
    t = state[4] ^ d[4];
    c[4] = t.rotate_left(ROT[24] as u32);

    state[0] = c[0] ^ (c[2] & !c[1]) ^ RC[ridx + 3];
    state[1] = c[1] ^ (c[3] & !c[2]);
    state[2] = c[2] ^ (c[4] & !c[3]);
    state[3] = c[3] ^ (c[0] & !c[4]);
    state[4] = c[4] ^ (c[1] & !c[0]);

    t = state[5] ^ d[0];
    c[2] = t.rotate_left(ROT[10] as u32);
    t = state[6] ^ d[1];
    c[3] = t.rotate_left(ROT[16] as u32);
    t = state[7] ^ d[2];
    c[4] = t.rotate_left(ROT[22] as u32);
    t = state[8] ^ d[3];
    c[0] = t.rotate_left(ROT[3] as u32);
    t = state[9] ^ d[4];
    c[1] = t.rotate_left(ROT[9] as u32);

    state[5] = c[0] ^ (c[2] & !c[1]);
    state[6] = c[1] ^ (c[3] & !c[2]);
    state[7] = c[2] ^ (c[4] & !c[3]);
    state[8] = c[3] ^ (c[0] & !c[4]);
    state[9] = c[4] ^ (c[1] & !c[0]);

    t = state[10] ^ d[0];
    c[4] = t.rotate_left(ROT[20] as u32);
    t = state[11] ^ d[1];
    c[0] = t.rotate_left(ROT[1] as u32);
    t = state[12] ^ d[2];
    c[1] = t.rotate_left(ROT[7] as u32);
    t = state[13] ^ d[3];
    c[2] = t.rotate_left(ROT[13] as u32);
    t = state[14] ^ d[4];
    c[3] = t.rotate_left(ROT[19] as u32);

    state[10] = c[0] ^ (c[2] & !c[1]);
    state[11] = c[1] ^ (c[3] & !c[2]);
    state[12] = c[2] ^ (c[4] & !c[3]);
    state[13] = c[3] ^ (c[0] & !c[4]);
    state[14] = c[4] ^ (c[1] & !c[0]);

    t = state[15] ^ d[0];
    c[1] = t.rotate_left(ROT[5] as u32);
    t = state[16] ^ d[1];
    c[2] = t.rotate_left(ROT[11] as u32);
    t = state[17] ^ d[2];
    c[3] = t.rotate_left(ROT[17] as u32);
    t = state[18] ^ d[3];
    c[4] = t.rotate_left(ROT[23] as u32);
    t = state[19] ^ d[4];
    c[0] = t.rotate_left(ROT[4] as u32);

    state[15] = c[0] ^ (c[2] & !c[1]);
    state[16] = c[1] ^ (c[3] & !c[2]);
    state[17] = c[2] ^ (c[4] & !c[3]);
    state[18] = c[3] ^ (c[0] & !c[4]);
    state[19] = c[4] ^ (c[1] & !c[0]);

    t = state[20] ^ d[0];
    c[3] = t.rotate_left(ROT[15] as u32);
    t = state[21] ^ d[1];
    c[4] = t.rotate_left(ROT[21] as u32);
    t = state[22] ^ d[2];
    c[0] = t.rotate_left(ROT[2] as u32);
    t = state[23] ^ d[3];
    c[1] = t.rotate_left(ROT[8] as u32);
    t = state[24] ^ d[4];
    c[2] = t.rotate_left(ROT[14] as u32);

    state[20] = c[0] ^ (c[2] & !c[1]);
    state[21] = c[1] ^ (c[3] & !c[2]);
    state[22] = c[2] ^ (c[4] & !c[3]);
    state[23] = c[3] ^ (c[0] & !c[4]);
    state[24] = c[4] ^ (c[1] & !c[0]);
}

/// Keccak-p\[1600, 12\] permutation, applying 12 rounds of permutation
/// on state of dimension 5 x 5 x 64 ( = 1600 -bits ), following algorithm 7 defined
/// in section 3.3 of SHA3 specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L253-L493
#[inline(always)]
pub fn permute(state: &mut [u64; LANE_CNT]) {
    const STEP_BY: usize = 4;
    const { assert!(ROUNDS % STEP_BY == 0) }

    roundx4(state, 0);
    roundx4(state, 4);
    roundx4(state, 8);
}

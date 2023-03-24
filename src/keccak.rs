use crunchy::unroll;

#[cfg(feature = "simdx2")]
use std::simd::u64x2;
#[cfg(feature = "simdx4")]
use std::simd::u64x4;

/// Logarithm base 2 of bit width of lane of Keccak-p\[1600, 12\] permutation
const L: usize = 6;

/// Bit width of each lane of Keccak-p\[1600, 12\] permutation
const W: usize = 1 << L;

/// \# -of rounds of Keccak permutation is applied per iteration i.e. it's Keccak-p\[1600, 12\]
const ROUNDS: usize = 12;

/// Lane rotation factor table taken from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L25-L35
const ROT: [usize; 25] = [
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

/// Permutation table taken from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L37-L48
const PERM: [usize; 25] = [
    0, 6, 12, 18, 24, 3, 9, 10, 16, 22, 1, 7, 13, 19, 20, 4, 5, 11, 17, 23, 2, 8, 14, 15, 21,
];

/// Round constants taken from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L134-L141
const RC: [u64; ROUNDS] = [
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

/// Keccak-p\[1600, 12\] step mapping function θ, see section 3.2.1 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L145-L175
#[inline(always)]
fn theta(state: &mut [u64; 25]) {
    let mut c = [0u64; 5];

    unroll! {
        for i in (0..25).step_by(5) {
            c[0] ^= state[i + 0];
            c[1] ^= state[i + 1];
            c[2] ^= state[i + 2];
            c[3] ^= state[i + 3];
            c[4] ^= state[i + 4];
        }
    }

    let mut d = [0u64; 5];

    d[0] = c[4] ^ c[1].rotate_left(1);
    d[1] = c[0] ^ c[2].rotate_left(1);
    d[2] = c[1] ^ c[3].rotate_left(1);
    d[3] = c[2] ^ c[4].rotate_left(1);
    d[4] = c[3] ^ c[0].rotate_left(1);

    unroll! {
        for i in (0..25).step_by(5) {
            state[i + 0] ^= d[0];
            state[i + 1] ^= d[1];
            state[i + 2] ^= d[2];
            state[i + 3] ^= d[3];
            state[i + 4] ^= d[4];
        }
    }
}

/// Keccak-p\[1600, 12\] step mapping function θ, parallelly applied on two Keccak-p\[1600\]
/// states, represented using 128 -bit vectors, following algorithm described on section 3.2.1
/// of SHA3 specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L145-L175
#[cfg(feature = "simdx2")]
#[inline(always)]
fn thetax2(state: &mut [u64x2; 25]) {
    let zeros = u64x2::splat(0u64);
    let mut c = [zeros; 5];

    unroll! {
        for i in (0..25).step_by(5) {
            c[0] ^= state[i + 0];
            c[1] ^= state[i + 1];
            c[2] ^= state[i + 2];
            c[3] ^= state[i + 3];
            c[4] ^= state[i + 4];
        }
    }

    let mut d = [zeros; 5];
    let ones = u64x2::splat(1u64);
    let sixtythrees = u64x2::splat(63u64);

    d[0] = c[4] ^ ((c[1] << ones) | (c[1] >> sixtythrees));
    d[1] = c[0] ^ ((c[2] << ones) | (c[2] >> sixtythrees));
    d[2] = c[1] ^ ((c[3] << ones) | (c[3] >> sixtythrees));
    d[3] = c[2] ^ ((c[4] << ones) | (c[4] >> sixtythrees));
    d[4] = c[3] ^ ((c[0] << ones) | (c[0] >> sixtythrees));

    unroll! {
        for i in (0..25).step_by(5) {
            state[i+0] ^= d[0];
            state[i+1] ^= d[1];
            state[i+2] ^= d[2];
            state[i+3] ^= d[3];
            state[i+4] ^= d[4];
        }
    }
}

/// Keccak-p\[1600, 12\] step mapping function θ, parallelly applied on four Keccak-p\[1600\]
/// states, represented using 256 -bit SIMD registers, following algorithm described on section
/// 3.2.1 of SHA3 specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L145-L175
#[cfg(feature = "simdx4")]
#[inline(always)]
fn thetax4(state: &mut [u64x4; 25]) {
    let zeros = u64x4::splat(0u64);
    let mut c = [zeros; 5];

    unroll! {
        for i in (0..25).step_by(5) {
            c[0] ^= state[i + 0];
            c[1] ^= state[i + 1];
            c[2] ^= state[i + 2];
            c[3] ^= state[i + 3];
            c[4] ^= state[i + 4];
        }
    }

    let mut d = [zeros; 5];
    let ones = u64x4::splat(1u64);
    let sixtythrees = u64x4::splat(63u64);

    d[0] = c[4] ^ ((c[1] << ones) | (c[1] >> sixtythrees));
    d[1] = c[0] ^ ((c[2] << ones) | (c[2] >> sixtythrees));
    d[2] = c[1] ^ ((c[3] << ones) | (c[3] >> sixtythrees));
    d[3] = c[2] ^ ((c[4] << ones) | (c[4] >> sixtythrees));
    d[4] = c[3] ^ ((c[0] << ones) | (c[0] >> sixtythrees));

    unroll! {
        for i in (0..25).step_by(5) {
            state[i + 0] ^= d[0];
            state[i + 1] ^= d[1];
            state[i + 2] ^= d[2];
            state[i + 3] ^= d[3];
            state[i + 4] ^= d[4];
        }
    }
}

/// Keccak-p\[1600, 12\] step mapping function ρ, see section 3.2.2 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L177-L190
#[inline(always)]
fn rho(state: &mut [u64; 25]) {
    unroll! {
        for i in 0..25 {
            state[i] = state[i].rotate_left(ROT[i] as u32);
        }
    }
}

/// Keccak-p\[1600, 12\] step mapping function ρ, parallelly applied on two Keccak-p\[1600\]
/// states, represented using 128 -bit vectors, following algorithm described on section 3.2.2 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L177-L190
#[cfg(feature = "simdx2")]
#[inline(always)]
fn rhox2(state: &mut [u64x2; 25]) {
    unroll! {
        for i in 0..25 {
            let shl = u64x2::splat(ROT[i] as u64);
            let shr = u64x2::splat((64 - ROT[i]) as u64);

            state[i] = (state[i] << shl) | (state[i] >> shr);
        }
    }
}

/// Keccak-p\[1600, 12\] step mapping function ρ, parallelly applied on four Keccak-p\[1600\]
/// states, represented using 256 -bit SIMD registers, following algorithm described on section
/// 3.2.2 of SHA3 specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L177-L190
#[cfg(feature = "simdx4")]
#[inline(always)]
fn rhox4(state: &mut [u64x4; 25]) {
    unroll! {
        for i in 0..25 {
            let shl = u64x4::splat(ROT[i] as u64);
            let shr = u64x4::splat((64 - ROT[i]) as u64);

            state[i] = (state[i] << shl) | (state[i] >> shr);
        }
    }
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
    unroll! {
        for i in 0..25 {
            ostate[i] = istate[PERM[i]];
        }
    }
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

/// Keccak-p\[1600, 12\] step mapping function χ, parallelly applied on two Keccak-p\[1600\]
/// states, represented using 128 -bit vectors, following algorithm described on section 3.2.4 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L209-L227
#[cfg(feature = "simdx2")]
#[inline(always)]
fn chix2(istate: &[u64x2; 25], ostate: &mut [u64x2; 25]) {
    for y in 0..5 {
        let off = y * 5;

        ostate[off + 0] = istate[off + 0] ^ (!istate[off + 1] & istate[off + 2]);
        ostate[off + 1] = istate[off + 1] ^ (!istate[off + 2] & istate[off + 3]);
        ostate[off + 2] = istate[off + 2] ^ (!istate[off + 3] & istate[off + 4]);
        ostate[off + 3] = istate[off + 3] ^ (!istate[off + 4] & istate[off + 0]);
        ostate[off + 4] = istate[off + 4] ^ (!istate[off + 0] & istate[off + 1]);
    }
}

/// Keccak-p\[1600, 12\] step mapping function χ, parallelly applied on four Keccak-p\[1600\]
/// states, represented using 256 -bit SIMD registers, following algorithm described on section
/// 3.2.4 of SHA3 specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L209-L227
#[cfg(feature = "simdx4")]
#[inline(always)]
fn chix4(istate: &[u64x4; 25], ostate: &mut [u64x4; 25]) {
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

/// Keccak-p\[1600, 12\] step mapping function ι, parallelly applied on two Keccak-p\[1600\]
/// states, represented using 128 -bit vectors, following algorithm described on section 3.2.5 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L229-L235
#[cfg(feature = "simdx2")]
#[inline(always)]
fn iotax2(lane: u64x2, ridx: usize) -> u64x2 {
    lane ^ u64x2::splat(RC[ridx])
}

/// Keccak-p\[1600, 12\] step mapping function ι, parallelly applied on four Keccak-p\[1600\]
/// states, represented using 256 -bit SIMD registers, following algorithm described on section
/// 3.2.5 of SHA3 specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L229-L235
#[cfg(feature = "simdx4")]
#[inline(always)]
fn iotax4(lane: u64x4, ridx: usize) -> u64x4 {
    lane ^ u64x4::splat(RC[ridx])
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

/// Keccak-p\[1600, 12\] round function, parallelly applied on two Keccak-p\[1600\]
/// states, represented using 128 -bit vectors, applying all five step mapping functions
/// in order, mutating state array, following algorithm described on section 3.3
/// of https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L237-L251
#[cfg(feature = "simdx2")]
#[inline(always)]
fn roundx2(state: &mut [u64x2; 25], ridx: usize) {
    thetax2(state);
    rhox2(state);

    let zeros = u64x2::splat(0u64);
    let mut _state = [zeros; 25];

    pi(state, &mut _state);
    chix2(&_state, state);
    state[0] = iotax2(state[0], ridx);
}

/// Keccak-p\[1600, 12\] round function, parallelly applied on four Keccak-p\[1600\]
/// states, represented using 256 -bit SIMD registers, applying all five step mapping
/// functions in order, mutating state array, following algorithm described on section
/// 3.3 of https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L237-L251
#[cfg(feature = "simdx4")]
#[inline(always)]
fn roundx4(state: &mut [u64x4; 25], ridx: usize) {
    thetax4(state);
    rhox4(state);

    let zeros = u64x4::splat(0u64);
    let mut _state = [zeros; 25];

    pi(state, &mut _state);
    chix4(&_state, state);
    state[0] = iotax4(state[0], ridx);
}

/// Keccak-p\[1600, 12\] permutation, applying 12 rounds of permutation
/// on state of dimension 5 x 5 x 64 ( = 1600 -bits ), following algorithm 7 defined
/// in section 3.3 of SHA3 specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L253-L493
#[inline(always)]
pub fn permute(state: &mut [u64; 25]) {
    for i in 0..ROUNDS {
        round(state, i);
    }
}

/// Keccak-p\[1600, 12\] permutation, applying 12 rounds of permutation, parallelly on
/// two Keccak-p\[1600\] states, following algorithm 7 defined in section 3.3 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Every lane is 128 -bit wide ( instead of usual 64 -bits ), holding two different
/// Keccak-p\[1600\] lanes, each of width 64 -bits. Each lane is laid out on a 128 -bit
/// register as shown below.
///
/// \[127, 126, 125, ..., 65, 64 || 63, 62, ..., 3, 2, 1, 0\]
///
/// \[<--------state\[1\]--------> || <-------state\[0\]------->\]
///
/// \[<-----------u64----------> || <-----------u64-------->\]
///
/// \[<-------------------------u64x2---------------------->\]
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L253-L493
#[cfg(feature = "simdx2")]
#[inline(always)]
pub fn permutex2(state0: &mut [u64; 25], state1: &mut [u64; 25]) {
    let zeros = u64x2::splat(0u64);
    let mut state = [zeros; 25];

    for i in 0..25 {
        let arr = [state0[i], state1[i]];
        state[i] = u64x2::from_array(arr);
    }

    for i in 0..ROUNDS {
        roundx2(&mut state, i);
    }

    for i in 0..25 {
        let arr = state[i].to_array();
        state0[i] = arr[0];
        state1[i] = arr[1];
    }
}

/// Keccak-p\[1600, 12\] permutation, applying 12 rounds of permutation, parallelly on
/// four Keccak-p\[1600\] states, following algorithm 7 defined in section 3.3 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Every lane is 256 -bit wide ( instead of usual 64 -bits ), holding four different
/// Keccak-p\[1600\] lanes, each of width 64 -bits. Each lane is laid out on a 256 -bit
/// SIMD register as shown below.
///
/// \[255, ..., 192, || 191, ..., 128, || 127, ..., 64, || 63, ..., 0\]
///
/// \[<-state\[3\]-> || <-state\[2\]-> || <-state\[1\]-> || <-state\[0\]->\]
///
/// \[<-----u64----> || <-----u64----> || <-----u64----> || <-----u64---->\]
///
/// \[<--------------------------------u64x4------------------------------>\]
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed/include/keccak.hpp#L253-L493
#[cfg(feature = "simdx4")]
#[inline(always)]
pub fn permutex4(
    state0: &mut [u64; 25],
    state1: &mut [u64; 25],
    state2: &mut [u64; 25],
    state3: &mut [u64; 25],
) {
    let zeros = u64x4::splat(0u64);
    let mut state = [zeros; 25];

    for i in 0..25 {
        let arr = [state0[i], state1[i], state2[i], state3[i]];
        state[i] = u64x4::from_array(arr);
    }

    for i in 0..ROUNDS {
        roundx4(&mut state, i);
    }

    for i in 0..25 {
        let arr = state[i].to_array();
        state0[i] = arr[0];
        state1[i] = arr[1];
        state2[i] = arr[2];
        state3[i] = arr[3];
    }
}

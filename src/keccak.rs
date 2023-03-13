/// Keccak-p[1600, 12] step mapping function θ, see section 3.2.1 of SHA3
/// specification https://dx.doi.org/10.6028/NIST.FIPS.202
///
/// Adapted from https://github.com/itzmeanjan/sha3/blob/b5e897ed8002c94569a5d7433f65ba606880ac12/include/keccak.hpp#L145-L175
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

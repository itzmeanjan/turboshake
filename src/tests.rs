#![cfg(test)]

use crate::{TurboShake128, TurboShake256};
use rand::{thread_rng, RngCore};
use std::cmp;
use test_case::test_case;

/// Ensure functional correctness of Keccak-p\[1600, 12\] permutation's 2x
/// SIMD parallel implementation.
#[cfg(feature = "simdx2")]
#[test]
fn test_keccakx2() {
    use rand::Rng;

    use crate::keccak;

    let mut rng = thread_rng();

    let mut state0 = [0u64; 25];
    let mut state1 = [0u64; 25];

    rng.fill(&mut state0);
    rng.fill(&mut state1);

    let mut state0_copy = state0;
    let mut state1_copy = state1;

    keccak::permute(&mut state0);
    keccak::permute(&mut state1);
    keccak::permutex2(&mut state0_copy, &mut state1_copy);

    assert_eq!(state0, state0_copy);
    assert_eq!(state1, state1_copy);
}

/// Ensure functional correctness of Keccak-p\[1600, 12\] permutation's 4x
/// SIMD parallel implementation.
#[cfg(feature = "simdx4")]
#[test]
fn test_keccakx4() {
    use rand::Rng;

    use crate::keccak;

    let mut rng = thread_rng();

    let mut state0 = [0u64; 25];
    let mut state1 = [0u64; 25];
    let mut state2 = [0u64; 25];
    let mut state3 = [0u64; 25];

    rng.fill(&mut state0);
    rng.fill(&mut state1);
    rng.fill(&mut state2);
    rng.fill(&mut state3);

    let mut state0_copy = state0;
    let mut state1_copy = state1;
    let mut state2_copy = state2;
    let mut state3_copy = state3;

    keccak::permute(&mut state0);
    keccak::permute(&mut state1);
    keccak::permute(&mut state2);
    keccak::permute(&mut state3);
    keccak::permutex4(
        &mut state0_copy,
        &mut state1_copy,
        &mut state2_copy,
        &mut state3_copy,
    );

    assert_eq!(state0, state0_copy);
    assert_eq!(state1, state1_copy);
    assert_eq!(state2, state2_copy);
    assert_eq!(state3, state3_copy);
}

/// Generates static byte pattern of length 251, following
/// https://www.ietf.org/archive/id/draft-irtf-cfrg-kangarootwelve-09.html#name-test-vectors
#[allow(dead_code)]
fn pattern() -> [u8; 251] {
    (0..251).map(|i| i).collect::<Vec<u8>>().try_into().unwrap()
}

/// Generates bytearray of length n by repeating static byte pattern returned by `pattern()`,
/// following https://www.ietf.org/archive/id/draft-irtf-cfrg-kangarootwelve-09.html#name-test-vectors
#[allow(dead_code)]
fn ptn(n: usize) -> Vec<u8> {
    let mut res = vec![0; n];

    let mut off = 0;
    while off < n {
        let read = cmp::min(n - off, 251);
        res[off..(off + read)].copy_from_slice(&pattern()[..read]);
        off += read;
    }

    res
}

/// Given a message M of length n -bytes, absorbs it into TurboSHAKE128 object, while
/// finalizing it by using domain seperator constant D ( generic constant parameter )
/// and returning TurboSHAKE128 object, ready to be squeezed.
#[allow(dead_code)]
fn turboshake128<const D: u8>(msg: &[u8]) -> TurboShake128 {
    let mut hasher = TurboShake128::new();
    hasher.absorb(msg);
    hasher.finalize::<D>();
    hasher
}

/// Given a message M of length n -bytes, absorbs it into TurboSHAKE256 object, while
/// finalizing it by using domain seperator constant D ( generic constant parameter )
/// and returning TurboSHAKE256 object, ready to be squeezed.
#[allow(dead_code)]
fn turboshake256<const D: u8>(msg: &[u8]) -> TurboShake256 {
    let mut hasher = TurboShake256::new();
    hasher.absorb(msg);
    hasher.finalize::<D>();
    hasher
}

/// TurboSHAKE128 test vectors are collected from https://www.ietf.org/archive/id/draft-irtf-cfrg-kangarootwelve-09.html#name-test-vectors,
/// based on reference implementation https://www.ietf.org/archive/id/draft-irtf-cfrg-kangarootwelve-09.html#name-pseudocode
#[test]
fn test_turboshake128() {
    let mut out = [0u8; 32];
    turboshake128::<0x01>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "868cbd53b078205abb85815d941f7d0376bff5b8888a6a2d03483afbaf83967f"
    );

    let mut out = [0u8; 64];
    turboshake128::<0x01>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "868cbd53b078205abb85815d941f7d0376bff5b8888a6a2d03483afbaf83967f226e2cad5e7b1ec4ca72236f076462199fea48c93438ad4c49c767f9417be7c5"
    );

    let mut out = [0u8; 10032];
    turboshake128::<0x01>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out[10000..]),
        "fa09df77a17a33fe098328ba02786ac770301386f77d0731f2b866bd0140b412"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x01>(&ptn(17usize.pow(0))).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "0fc5bb1616bfd8121beb8cd6cde167ffbe4b11e51d9bc9a6a92c34ed3e46f4e1"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x01>(&ptn(17usize.pow(1))).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "6f0f5f330a7114ed345b97d012f8a8bac5ba32f1c0aafab22ef880737bf0c103"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x01>(&ptn(17usize.pow(2))).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "6232caa37353b5adb0e16e5beb97928110c5b837531339a2c9eb08014faa8ef6"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x01>(&ptn(17usize.pow(3))).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "668105870786e2aa80718487563aa06824eabc1d3a8e8b642f6d9996244fe8cf"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x01>(&ptn(17usize.pow(4))).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "795de7dd0ec596c20145d1784ac2acd625b4f62653872a06d8a8b9a0543aa863"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x01>(&ptn(17usize.pow(5))).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "4185e05262bcbcf7f74f50f08a710791ea0a12fba13c3a23ff07c33c0110bd20"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x02>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "8bcf8b0266eb3ef49e2b1df2eb627021d86281801116761f44efc976444f021b"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x03>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "a0347b35a7fa3d2f8561b3a4648de357be6762a6b76d5b2c1119cda104688192"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x0c>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "2c6462e826d1d5fa989b91ae4d8b3a3b63df64141e0ac0f9a1fbdf653b4ccf13"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x23>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "82d2b02713285b0dc2e8d1f2b40848ee62589b5b11262867e610e15ee62e1835"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x3a>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "55c63f13a040da7034f67d7b7b9a173426970419232209c01ca176e08b5acf5c"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x51>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "4e2695cf70d7c6c87e80a9f383b7aa6f0f8a4b0727f5cd2951c6947dffab6425"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x68>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "2e1c136a8af2e8b4c4cf9a7bca593d798f61bd1f153cd08483447a5de4369b1e"
    );

    let mut out = [0u8; 32];
    turboshake128::<0x7f>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "e4e1fd449c36ef25256c896e1907af3f458253d4a0bd820a6fef83377ae031f9"
    );
}

/// TurboSHAKE256 test vectors are collected from https://www.ietf.org/archive/id/draft-irtf-cfrg-kangarootwelve-09.html#name-test-vectors,
/// based on reference implementation https://www.ietf.org/archive/id/draft-irtf-cfrg-kangarootwelve-09.html#name-pseudocode
#[test]
fn test_turboshake256() {
    let mut out = [0u8; 32];
    turboshake256::<0x01>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "e3dd2df0943bde6d82e39ec36059f35cd76720e2df38cc6b10b69fddfcaa3a4a"
    );

    let mut out = [0u8; 64];
    turboshake256::<0x01>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "e3dd2df0943bde6d82e39ec36059f35cd76720e2df38cc6b10b69fddfcaa3a4a72fbbbe42c00ced7aa88e26d4675dd6e2c43c4413c4ea4d44bb170f03a981cab"
    );

    let mut out = [0u8; 10032];
    turboshake256::<0x01>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out[10000..]),
        "b021b244dcd9599966d7742225fc7372639233f0ff0863fa79683ebf1f57114f"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x01>(&ptn(17usize.pow(0))).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "73ebf1d543d855a3c5e4be6322f75604c254f70394b396884b6010fcca694722"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x01>(&ptn(17usize.pow(1))).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "1da47d188755b75307a242a8f2675bbd76aebf8a13b1d40f587a0732cbb3dc3d"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x01>(&ptn(17usize.pow(2))).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "a48c938770f916b09d764e29e2279b90d5fa3dd0e006ee8d6c2eb0db8893525e"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x01>(&ptn(17usize.pow(3))).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "75e8668d3a46baa7c75c3ac7d33fc2c218df38cdf0f8d70352a495bd9d5d6dfa"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x01>(&ptn(17usize.pow(4))).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "ffa49653e40c7ba33f11c278d99be3010f65446a7bf8a69d70b07feb54e7107c"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x01>(&ptn(17usize.pow(5))).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "2ad2b3beb8671840fa9d5e8f7faf2d1139d99483f3c4e56a6a25553f83c25931"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x02>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "cfdbc69ec2652711dc3013cee68def374948ef09e62d82f2749e3dbc71f04dce"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x03>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "1402a1d6bebcf52cdbc7074c3d7b1adc545646458400a63980ebb3dd0ab04c68"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x0c>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "3c78a84557f19506a6151985664cf6163c4d4033d6bc310f8e8dde56e232abf4"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x23>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "bd8f3f5eae3fb4ba604ad2d9d9431867532ab1e2f773819620b79281e3258bbc"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x3a>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "cfa491078479604fd78e967071a081cf357a1244d2999c929c318782a24d7c21"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x51>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "b92a11dd21017255a8285bbdf413269dcfae55f79d188a55cc2e04ea667bc047"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x68>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "229acb8530b6e700bebb304655a5dfad00f7ac4ab7f582ee909c11b96f6d5fb3"
    );

    let mut out = [0u8; 32];
    turboshake256::<0x7f>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "49b38a11204328440c4c40fdaee305629379936d7a31f9474c4f0fb062a2a427"
    );
}

/// Test if both oneshot and incremental hashing API of TurboSHAKE128 produces same result for same input message.
///
/// Adapated from https://github.com/itzmeanjan/ascon/blob/f9ce50dd23b89e073e1f8fe94318d694e9b6770e/include/test/test_incremental_hashing.hpp#L11-L56
#[test_case(32, 32; "message length = 32B, digest length = 32B")]
#[test_case(64, 64; "message length = 64B, digest length = 64B")]
#[test_case(128, 128; "message length = 128B, digest length = 128B")]
#[test_case(256, 256; "message length = 256B, digest length = 256B")]
#[test_case(512, 512; "message length = 512B, digest length = 512B")]
#[test_case(1024, 1024; "message length = 1024B, digest length = 1024B")]
fn test_incremental_ts128_hashing(mlen: usize, dlen: usize) {
    // generate random input bytes ( of length mlen )
    let mut rng = thread_rng();
    let mut msg = vec![0u8; mlen];
    rng.fill_bytes(&mut msg);

    // digest computed by oneshot hasher
    let mut dig0 = vec![0u8; dlen];
    // digest computed by incremental hasher
    let mut dig1 = vec![0u8; dlen];

    // oneshot hashing
    let mut hasher0 = TurboShake128::new();
    hasher0.absorb(&msg);
    hasher0.finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
    hasher0.squeeze(&mut dig0);

    // incremental hashing
    let mut hasher1 = TurboShake128::new();

    let mut off = 0;
    while off < mlen {
        // because we don't want to be stuck in an infinite loop if msg[off] = 0 !
        let elen = cmp::min(cmp::max(msg[off] as usize, 1), mlen - off);

        hasher1.absorb(&msg[off..(off + elen)]);
        off += elen;
    }

    hasher1.finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
    hasher1.squeeze(&mut dig1);

    // finally compare if both of them arrive at same digest or not !
    assert_eq!(hex::encode(dig0), hex::encode(dig1));
}

/// Test if both oneshot and incremental hashing API of TurboSHAKE256 produces same result for same input message.
///
/// Adapated from https://github.com/itzmeanjan/ascon/blob/f9ce50dd23b89e073e1f8fe94318d694e9b6770e/include/test/test_incremental_hashing.hpp#L11-L56
#[test_case(32, 32; "message length = 32B, digest length = 32B")]
#[test_case(64, 64; "message length = 64B, digest length = 64B")]
#[test_case(128, 128; "message length = 128B, digest length = 128B")]
#[test_case(256, 256; "message length = 256B, digest length = 256B")]
#[test_case(512, 512; "message length = 512B, digest length = 512B")]
#[test_case(1024, 1024; "message length = 1024B, digest length = 1024B")]
fn test_incremental_ts256_hashing(mlen: usize, dlen: usize) {
    // generate random input bytes ( of length mlen )
    let mut rng = thread_rng();
    let mut msg = vec![0u8; mlen];
    rng.fill_bytes(&mut msg);

    // digest computed by oneshot hasher
    let mut dig0 = vec![0u8; dlen];
    // digest computed by incremental hasher
    let mut dig1 = vec![0u8; dlen];

    // oneshot hashing
    let mut hasher0 = TurboShake256::new();
    hasher0.absorb(&msg);
    hasher0.finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
    hasher0.squeeze(&mut dig0);

    // incremental hashing
    let mut hasher1 = TurboShake256::new();

    let mut off = 0;
    while off < mlen {
        // because we don't want to be stuck in an infinite loop if msg[off] = 0 !
        let elen = cmp::min(cmp::max(msg[off] as usize, 1), mlen - off);

        hasher1.absorb(&msg[off..(off + elen)]);
        off += elen;
    }

    hasher1.finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
    hasher1.squeeze(&mut dig1);

    // finally compare if both of them arrive at same digest or not !
    assert_eq!(hex::encode(dig0), hex::encode(dig1));
}

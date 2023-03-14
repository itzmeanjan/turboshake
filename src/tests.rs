use crate::TurboShake128;
use std::cmp;

/// Generates static byte pattern of length 251, following
/// https://www.ietf.org/archive/id/draft-irtf-cfrg-kangarootwelve-09.html#name-test-vectors
fn pattern() -> [u8; 251] {
    (0..251).map(|i| i).collect::<Vec<u8>>().try_into().unwrap()
}

/// Generates bytearray of length n by repeating static byte pattern returned by `pattern()`,
/// following https://www.ietf.org/archive/id/draft-irtf-cfrg-kangarootwelve-09.html#name-test-vectors
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
fn turboshake128<const D: u8>(msg: &[u8]) -> TurboShake128 {
    let mut hasher = TurboShake128::new();
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

    let mut out = [0u8; 32];
    turboshake128::<0x7f>(&[0u8; 0]).squeeze(&mut out);
    assert_eq!(
        hex::encode(&out),
        "e4e1fd449c36ef25256c896e1907af3f458253d4a0bd820a6fef83377ae031f9"
    );
}

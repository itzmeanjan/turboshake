use rand::prelude::*;
use turboshake::TurboShake128;

fn main() {
    let mut rng = rand::rng();

    let mlen = 64;
    let mut msg = vec![0u8; mlen];
    rng.fill_bytes(&mut msg);

    let dlen = 32;
    let mut dig = vec![0u8; dlen];

    let mut hasher = TurboShake128::default();

    hasher.absorb(&msg[..mlen / 2]).expect("data absorption must not fail");
    hasher.absorb(&msg[mlen / 2..]).expect("data absorption must not fail");

    hasher
        .finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>()
        .expect("finalization must not fail");

    hasher.squeeze(&mut dig[..dlen / 2]).expect("data squeezing must not fail");
    hasher.squeeze(&mut dig[dlen / 2..]).expect("data squeezing must not fail");

    println!("Message: {}", const_hex::encode(&msg));
    println!("Digest: {}", const_hex::encode(&dig));
}

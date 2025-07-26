use rand::prelude::*;
use turboshake::TurboShake256;

fn main() {
    let mut rng = rand::rng();

    let mlen = 64;
    let mut msg = vec![0u8; mlen];
    rng.fill_bytes(&mut msg);

    let dlen = 32;
    let mut dig = vec![0u8; dlen];

    let mut hasher = TurboShake256::default();
    hasher.absorb(&msg[..mlen / 2]);
    hasher.absorb(&msg[mlen / 2..]);
    hasher.finalize::<{ TurboShake256::DEFAULT_DOMAIN_SEPARATOR }>();
    hasher.squeeze(&mut dig[..dlen / 2]);
    hasher.squeeze(&mut dig[dlen / 2..]);

    println!("Message: {}", hex::encode(&msg));
    println!("Digest: {}", hex::encode(&dig));
}

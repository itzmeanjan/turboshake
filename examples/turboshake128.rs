use rand::{thread_rng, RngCore};
use turboshake::TurboShake128;

fn main() {
    let mut rng = thread_rng();

    let mlen = 64;
    let mut msg = vec![0u8; mlen];
    rng.fill_bytes(&mut msg);

    let dlen = 32;
    let mut dig = vec![0u8; dlen];

    let mut hasher = TurboShake128::new();
    hasher.absorb(&msg[..mlen / 2]);
    hasher.absorb(&msg[mlen / 2..]);
    hasher.finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
    hasher.squeeze(&mut dig[..dlen / 2]);
    hasher.squeeze(&mut dig[dlen / 2..]);
    hasher.reset();

    // You may begin the absorb->finalize->squeeze cycle again !

    println!("Message: {}", hex::encode(&msg));
    println!("Digest: {}", hex::encode(&dig));
}

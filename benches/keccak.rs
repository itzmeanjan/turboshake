use divan;
use divan::counter::{BytesCount, BytesFormat, ItemsCount};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use turboshake::keccak;

fn main() {
    divan::Divan::default().bytes_format(BytesFormat::Binary).run_benches();
}

#[divan::bench]
fn permute_12_rounds(bencher: divan::Bencher) {
    let mut rng = ChaCha8Rng::from_os_rng();

    let mut state = [0u64; 25];
    let state_byte_len = state.len() * 8;
    rng.fill(&mut state);

    bencher
        .counter(BytesCount::new(state_byte_len))
        .counter(ItemsCount::new(1usize))
        .with_inputs(|| (state.clone()))
        .bench_values(|mut state| keccak::permute(divan::black_box(&mut state)));
}

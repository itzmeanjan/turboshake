use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::{thread_rng, Rng, RngCore};
use turboshake::{keccak, TurboShake128, TurboShake256};

fn keccak(c: &mut Criterion) {
    let mut rng = thread_rng();

    c.bench_function("keccak-p[1600, 12] (cached)", |bench| {
        let mut state = [0u64; 25];
        rng.fill(&mut state);

        bench.iter(|| keccak::permute(black_box(&mut state)))
    });

    c.bench_function("keccak-p[1600, 12] (random)", |bench| {
        bench.iter_batched(
            || {
                (0..25)
                    .map(|_| rng.gen::<u64>())
                    .collect::<Vec<u64>>()
                    .try_into()
                    .unwrap()
            },
            |mut state| keccak::permute(black_box(&mut state)),
            BatchSize::SmallInput,
        )
    });
}

fn turboshake128<const MLEN: usize, const DLEN: usize>(c: &mut Criterion) {
    let mut rng = thread_rng();

    c.bench_function(
        &format!("turboshake128/{}/{} (cached)", MLEN, DLEN),
        |bench| {
            let mut msg = vec![0u8; MLEN];
            let mut dig = vec![0u8; DLEN];
            rng.fill_bytes(&mut msg);

            bench.iter(|| {
                let mut hasher = TurboShake128::new();
                hasher.absorb(black_box(&msg));
                hasher.finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
                hasher.squeeze(black_box(&mut dig));
            });
        },
    );

    c.bench_function(
        &format!("turboshake128/{}/{} (random)", MLEN, DLEN),
        |bench| {
            let mut msg = vec![0u8; MLEN];
            let mut dig = vec![0u8; DLEN];
            rng.fill_bytes(&mut msg);

            bench.iter_batched(
                || msg.clone(),
                |msg| {
                    let mut hasher = TurboShake128::new();
                    hasher.absorb(black_box(&msg));
                    hasher.finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
                    hasher.squeeze(black_box(&mut dig));
                },
                BatchSize::SmallInput,
            );
        },
    );
}

fn turboshake256<const MLEN: usize, const DLEN: usize>(c: &mut Criterion) {
    let mut rng = thread_rng();

    c.bench_function(
        &format!("turboshake256/{}/{} (cached)", MLEN, DLEN),
        |bench| {
            let mut msg = vec![0u8; MLEN];
            let mut dig = vec![0u8; DLEN];
            rng.fill_bytes(&mut msg);

            bench.iter(|| {
                let mut hasher = TurboShake256::new();
                hasher.absorb(black_box(&msg));
                hasher.finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
                hasher.squeeze(black_box(&mut dig));
            });
        },
    );

    c.bench_function(
        &format!("turboshake256/{}/{} (random)", MLEN, DLEN),
        |bench| {
            let mut msg = vec![0u8; MLEN];
            let mut dig = vec![0u8; DLEN];
            rng.fill_bytes(&mut msg);

            bench.iter_batched(
                || msg.clone(),
                |msg| {
                    let mut hasher = TurboShake256::new();
                    hasher.absorb(black_box(&msg));
                    hasher.finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
                    hasher.squeeze(black_box(&mut dig));
                },
                BatchSize::SmallInput,
            );
        },
    );
}

criterion_group!(permutation, keccak);
criterion_group!(hashing, turboshake128<32, 32>, turboshake128<64, 32>, turboshake128<128, 32>, turboshake128<256, 32>, turboshake128<512, 32>, turboshake128<1024, 32>, turboshake128<2048, 32>, turboshake128<4096, 32>, turboshake256<32, 32>, turboshake256<64, 32>, turboshake256<128, 32>, turboshake256<256, 32>, turboshake256<512, 32>, turboshake256<1024, 32>, turboshake256<2048, 32>, turboshake256<4096, 32>);
criterion_main!(permutation, hashing);

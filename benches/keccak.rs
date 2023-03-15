use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::{thread_rng, Rng};
use turboshake::keccak;

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

criterion_group!(permutation, keccak);
criterion_main!(permutation);

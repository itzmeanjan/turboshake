use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use rand::{thread_rng, Rng};
use turboshake::keccak;

#[cfg(not(feature = "simdx2"))]
fn keccak(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("keccak");
    group.throughput(Throughput::Bytes(200u64)); // size of keccak-p[1600] permutation state

    group.bench_function("keccak-p[1600, 12] (cached)", |bench| {
        let mut state = [0u64; 25];
        rng.fill(&mut state);

        bench.iter(|| keccak::permute(black_box(&mut state)))
    });
    group.bench_function("keccak-p[1600, 12] (random)", |bench| {
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

    group.finish();
}

#[cfg(feature = "simdx2")]
fn keccak(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("keccak");
    group.throughput(Throughput::Bytes(400u64));

    group.bench_function("keccak-p[1600, 12] x2 (cached)", |bench| {
        let mut state0 = [0u64; 25];
        let mut state1 = [0u64; 25];
        rng.fill(&mut state0);
        rng.fill(&mut state1);

        bench.iter(|| keccak::permutex2(black_box(&mut state0), black_box(&mut state1)));
    });
    group.bench_function("keccak-p[1600, 12] x2 (random)", |bench| {
        let mut state0 = [0u64; 25];
        let mut state1 = [0u64; 25];
        rng.fill(&mut state0);
        rng.fill(&mut state1);

        bench.iter_batched(
            || (state0.clone(), state1.clone()),
            |(mut state0, mut state1)| {
                keccak::permutex2(black_box(&mut state0), black_box(&mut state1))
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(permutation, keccak);
criterion_main!(permutation);

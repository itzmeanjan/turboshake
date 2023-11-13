use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use rand::{thread_rng, Rng};
use turboshake::keccak;

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "aarch64",
    target_arch = "loongarch64"
))]
use criterion_cycles_per_byte::CyclesPerByte;

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "aarch64",
    target_arch = "loongarch64"
))]
type CriterionHandler = Criterion<CyclesPerByte>;

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "aarch64",
    target_arch = "loongarch64"
)))]
type CriterionHandler = Criterion;

#[cfg(not(any(feature = "simdx2", feature = "simdx4")))]
fn keccak(c: &mut CriterionHandler) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("keccak");
    group.throughput(Throughput::Bytes(200u64)); // size of keccak-p[1600] permutation state

    group.bench_function("keccak-p[1600, 12] (cached)", |bench| {
        let mut state = [0u64; 25];
        rng.fill(&mut state);

        bench.iter(|| keccak::permute(black_box(&mut state)))
    });
    group.bench_function("keccak-p[1600, 12] (random)", |bench| {
        let mut state = [0u64; 25];
        rng.fill(&mut state);

        bench.iter_batched(
            || state.clone(),
            |mut state| keccak::permute(black_box(&mut state)),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

#[cfg(any(feature = "simdx2", feature = "simdx4"))]
fn keccak(c: &mut CriterionHandler) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("keccak");

    #[cfg(feature = "simdx2")]
    {
        group.throughput(Throughput::Bytes(200u64 * 2));

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
    }

    #[cfg(feature = "simdx4")]
    {
        group.throughput(Throughput::Bytes(200u64 * 4));

        group.bench_function("keccak-p[1600, 12] x4 (cached)", |bench| {
            let mut state0 = [0u64; 25];
            let mut state1 = [0u64; 25];
            let mut state2 = [0u64; 25];
            let mut state3 = [0u64; 25];
            rng.fill(&mut state0);
            rng.fill(&mut state1);
            rng.fill(&mut state2);
            rng.fill(&mut state3);

            bench.iter(|| {
                keccak::permutex4(
                    black_box(&mut state0),
                    black_box(&mut state1),
                    black_box(&mut state2),
                    black_box(&mut state3),
                )
            });
        });
        group.bench_function("keccak-p[1600, 12] x4 (random)", |bench| {
            let mut state0 = [0u64; 25];
            let mut state1 = [0u64; 25];
            let mut state2 = [0u64; 25];
            let mut state3 = [0u64; 25];
            rng.fill(&mut state0);
            rng.fill(&mut state1);
            rng.fill(&mut state2);
            rng.fill(&mut state3);

            bench.iter_batched(
                || {
                    (
                        state0.clone(),
                        state1.clone(),
                        state2.clone(),
                        state3.clone(),
                    )
                },
                |(mut state0, mut state1, mut state2, mut state3)| {
                    keccak::permutex4(
                        black_box(&mut state0),
                        black_box(&mut state1),
                        black_box(&mut state2),
                        black_box(&mut state3),
                    )
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "aarch64",
    target_arch = "loongarch64"
))]
criterion_group!(name = permutation; config = Criterion::default().with_measurement(CyclesPerByte); targets = keccak);

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "aarch64",
    target_arch = "loongarch64"
)))]
criterion_group!(permutation, keccak);

criterion_main!(permutation);

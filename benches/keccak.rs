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

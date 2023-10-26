use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use rand::{thread_rng, RngCore};
use turboshake::{TurboShake128, TurboShake256};

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
))]
use criterion_cycles_per_byte::CyclesPerByte;

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
))]
type CriterionHandler = Criterion<CyclesPerByte>;

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
)))]
type CriterionHandler = Criterion;

fn turboshake128(c: &mut CriterionHandler) {
    const DIGEST_LEN: usize = 32;
    const MIN_MSG_LEN: usize = 32;
    const MAX_MSG_LEN: usize = 4096;

    let mut rng = thread_rng();

    let mut mlen = MIN_MSG_LEN;
    while mlen <= MAX_MSG_LEN {
        let mut group = c.benchmark_group("turboshake128");
        group.throughput(Throughput::Bytes(mlen as u64));

        group.bench_function(&format!("{}/{} (cached)", mlen, DIGEST_LEN), |bench| {
            let mut msg = vec![0u8; mlen];
            let mut dig = vec![0u8; DIGEST_LEN];
            rng.fill_bytes(&mut msg);

            bench.iter(|| {
                let mut hasher = TurboShake128::new();
                hasher.absorb(black_box(&msg));
                hasher.finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
                hasher.squeeze(black_box(&mut dig));
            });
        });
        group.bench_function(&format!("{}/{} (random)", mlen, DIGEST_LEN), |bench| {
            let mut msg = vec![0u8; mlen];
            let mut dig = vec![0u8; DIGEST_LEN];
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
        });

        group.finish();
        mlen = 2 * mlen;
    }
}

fn turboshake256(c: &mut CriterionHandler) {
    const DIGEST_LEN: usize = 32;
    const MIN_MSG_LEN: usize = 32;
    const MAX_MSG_LEN: usize = 4096;

    let mut rng = thread_rng();

    let mut mlen = MIN_MSG_LEN;
    while mlen <= MAX_MSG_LEN {
        let mut group = c.benchmark_group("turboshake256");
        group.throughput(Throughput::Bytes(mlen as u64));

        group.bench_function(&format!("{}/{} (cached)", mlen, DIGEST_LEN), |bench| {
            let mut msg = vec![0u8; mlen];
            let mut dig = vec![0u8; DIGEST_LEN];
            rng.fill_bytes(&mut msg);

            bench.iter(|| {
                let mut hasher = TurboShake256::new();
                hasher.absorb(black_box(&msg));
                hasher.finalize::<{ TurboShake256::DEFAULT_DOMAIN_SEPARATOR }>();
                hasher.squeeze(black_box(&mut dig));
            });
        });
        group.bench_function(&format!("{}/{} (random)", mlen, DIGEST_LEN), |bench| {
            let mut msg = vec![0u8; mlen];
            let mut dig = vec![0u8; DIGEST_LEN];
            rng.fill_bytes(&mut msg);

            bench.iter_batched(
                || msg.clone(),
                |msg| {
                    let mut hasher = TurboShake256::new();
                    hasher.absorb(black_box(&msg));
                    hasher.finalize::<{ TurboShake256::DEFAULT_DOMAIN_SEPARATOR }>();
                    hasher.squeeze(black_box(&mut dig));
                },
                BatchSize::SmallInput,
            );
        });

        group.finish();
        mlen = 2 * mlen;
    }
}

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
))]
criterion_group!(name = hashing; config = Criterion::default().with_measurement(CyclesPerByte); targets = turboshake128, turboshake256);

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
)))]
criterion_group!(hashing, turboshake128, turboshake256);

criterion_main!(hashing);

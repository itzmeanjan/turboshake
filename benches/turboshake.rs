use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use rand::{thread_rng, RngCore};
use turboshake::{TurboShake128, TurboShake256};

fn turboshake128<const MLEN: usize, const DLEN: usize>(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("turboshake128");
    group.throughput(Throughput::Bytes(MLEN as u64));

    group.bench_function(&format!("{}/{} (cached)", MLEN, DLEN), |bench| {
        let mut msg = vec![0u8; MLEN];
        let mut dig = vec![0u8; DLEN];
        rng.fill_bytes(&mut msg);

        bench.iter(|| {
            let mut hasher = TurboShake128::new();
            hasher.absorb(black_box(&msg));
            hasher.finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
            hasher.squeeze(black_box(&mut dig));
        });
    });
    group.bench_function(&format!("{}/{} (random)", MLEN, DLEN), |bench| {
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
    });

    group.finish();
}

fn turboshake256<const MLEN: usize, const DLEN: usize>(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("turboshake256");
    group.throughput(Throughput::Bytes(MLEN as u64));

    group.bench_function(&format!("{}/{} (cached)", MLEN, DLEN), |bench| {
        let mut msg = vec![0u8; MLEN];
        let mut dig = vec![0u8; DLEN];
        rng.fill_bytes(&mut msg);

        bench.iter(|| {
            let mut hasher = TurboShake256::new();
            hasher.absorb(black_box(&msg));
            hasher.finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
            hasher.squeeze(black_box(&mut dig));
        });
    });
    group.bench_function(&format!("{}/{} (random)", MLEN, DLEN), |bench| {
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
    });

    group.finish();
}

criterion_group!(hashing, turboshake128<32, 32>, turboshake128<64, 32>, turboshake128<128, 32>, turboshake128<256, 32>, turboshake128<512, 32>, turboshake128<1024, 32>, turboshake128<2048, 32>, turboshake128<4096, 32>, turboshake256<32, 32>, turboshake256<64, 32>, turboshake256<128, 32>, turboshake256<256, 32>, turboshake256<512, 32>, turboshake256<1024, 32>, turboshake256<2048, 32>, turboshake256<4096, 32>);
criterion_main!(hashing);

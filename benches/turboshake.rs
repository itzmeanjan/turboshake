use divan;
use divan::counter::{BytesCount, BytesFormat, ItemsCount};
use rand::prelude::*;
use std::fmt::Display;
use turboshake::{TurboShake128, TurboShake256};

fn main() {
    divan::Divan::default().bytes_format(BytesFormat::Binary).run_benches();
}

struct InputOutputSize {
    msg_byte_len: usize,
    md_byte_len: usize,
}

fn format_bytes(bytes: usize) -> String {
    let suffixes = ["B", "KB", "MB", "GB"];
    let mut index = 0;
    let mut size = bytes as f64;

    while size >= 1024.0 && index < suffixes.len() - 1 {
        size /= 1024.0;
        index += 1;
    }

    format!("{:.1}{}", size, suffixes[index])
}

impl Display for InputOutputSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "msg = {}, md = {}", format_bytes(self.msg_byte_len), format_bytes(self.md_byte_len))
    }
}

const ARGS: &[InputOutputSize] = &[
    InputOutputSize {
        msg_byte_len: 32,
        md_byte_len: 64,
    },
    InputOutputSize {
        msg_byte_len: 128,
        md_byte_len: 64,
    },
    InputOutputSize {
        msg_byte_len: 512,
        md_byte_len: 64,
    },
    InputOutputSize {
        msg_byte_len: 2048,
        md_byte_len: 64,
    },
    InputOutputSize {
        msg_byte_len: 8192,
        md_byte_len: 64,
    },
];

#[divan::bench(args = ARGS)]
fn turboshake128(bencher: divan::Bencher, io_size: &InputOutputSize) {
    let mut rng = rand::rng();

    let mut msg = vec![0u8; io_size.msg_byte_len];
    let mut md = vec![0u8; io_size.md_byte_len];

    rng.fill_bytes(&mut msg);
    rng.fill_bytes(&mut md);

    bencher
        .counter(BytesCount::new(io_size.msg_byte_len + io_size.md_byte_len))
        .counter(ItemsCount::new(1usize))
        .with_inputs(|| (msg.clone(), md.clone()))
        .bench_values(|(msg, mut md)| {
            let mut hasher = TurboShake128::default();

            hasher.absorb(divan::black_box(&msg));
            hasher.finalize::<{ TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
            hasher.squeeze(divan::black_box(&mut md));
        });
}

#[divan::bench(args = ARGS)]
fn turboshake256(bencher: divan::Bencher, io_size: &InputOutputSize) {
    let mut rng = rand::rng();

    let mut msg = vec![0u8; io_size.msg_byte_len];
    let mut md = vec![0u8; io_size.md_byte_len];

    rng.fill_bytes(&mut msg);
    rng.fill_bytes(&mut md);

    bencher
        .counter(BytesCount::new(io_size.msg_byte_len + io_size.md_byte_len))
        .counter(ItemsCount::new(1usize))
        .with_inputs(|| (msg.clone(), md.clone()))
        .bench_values(|(msg, mut md)| {
            let mut hasher = TurboShake256::default();

            hasher.absorb(divan::black_box(&msg));
            hasher.finalize::<{ TurboShake256::DEFAULT_DOMAIN_SEPARATOR }>();
            hasher.squeeze(divan::black_box(&mut md));
        });
}

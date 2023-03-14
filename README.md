# turboshake
TurboSHAKE: A Family of e**X**tendable **O**utput **F**unctions based on round reduced Keccak[1600] Permutation

## Overview

TurboSHAKE is a family of extendable output functions (XOFs) powered by round-reduced ( i.e. 12 -rounds ) Keccak-p[1600, 12] permutation. Keccak-p[1600, 12] has previously been used in fast hashing algorithm KangarooTwelve ( more @ https://keccak.team/kangarootwelve.html ). Recently a formal specification, describing TurboSHAKE was released ( more @ https://ia.cr/2023/342 ) which generally exposes the underlying primitive of KangarooTwelve ( also known as **K12**, see https://blake12.org ) so that post-quantum public key cryptosystems ( such as Kyber, Dilithium etc. - being standardized by NIST ) benefit from it ( more @ https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/5HveEPBsbxY ).

Here I'm maintaining a Rust library which implements TurboSHAKE{128, 256} XOF s.t. one can absorb arbitrary many bytes into sponge state, finalize sponge and squeeze arbitrary many bytes out of sponge.

## Prerequisites

- Rust stable toolchain; see https://rustup.rs for installation guide.

```bash
# When developing this library, I was using
$ rustc --version
rustc 1.68.0 (2c8cc3432 2023-03-06)
```

## Testing

> **Warning** This library implementation doesn't *yet* claim its functional correctness because I've not found any **K**nown **A**nswer **T**ests for TurboSHAKE.

## Benchmarking

Issue following command for benchmarking round-reduced Keccak-p[1600, 12] permutation and TurboSHAKE{128, 256} XOF ( for various input sizes ). Note, squeezed output size is kept constant at 32 -bytes.

```bash
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench
```

### On **Intel(R) Core(TM) i5-8279U CPU @ 2.40GHz**

```bash
keccak-p[1600, 12] (cached)
                        time:   [427.94 ns 430.33 ns 434.01 ns]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

keccak-p[1600, 12] (random)
                        time:   [433.15 ns 434.92 ns 436.68 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

turboshake128/32/32 (cached)
                        time:   [441.30 ns 442.46 ns 443.61 ns]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

turboshake128/32/32 (random)
                        time:   [475.92 ns 477.18 ns 478.58 ns]
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  4 (4.00%) high severe

turboshake128/64/32 (cached)
                        time:   [441.37 ns 443.11 ns 445.58 ns]
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) high mild
  11 (11.00%) high severe

turboshake128/64/32 (random)
                        time:   [482.14 ns 483.80 ns 485.64 ns]
Found 11 outliers among 100 measurements (11.00%)
  8 (8.00%) high mild
  3 (3.00%) high severe

turboshake128/128/32 (cached)
                        time:   [441.00 ns 442.05 ns 443.19 ns]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

turboshake128/128/32 (random)
                        time:   [497.72 ns 499.75 ns 501.94 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

turboshake128/256/32 (cached)
                        time:   [872.20 ns 874.26 ns 876.51 ns]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

turboshake128/256/32 (random)
                        time:   [947.51 ns 951.61 ns 956.12 ns]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

turboshake128/512/32 (cached)
                        time:   [1.7421 µs 1.7485 µs 1.7572 µs]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

turboshake128/512/32 (random)
                        time:   [1.9043 µs 1.9136 µs 1.9225 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

turboshake128/1024/32 (cached)
                        time:   [3.0337 µs 3.0450 µs 3.0601 µs]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

turboshake128/1024/32 (random)
                        time:   [3.1385 µs 3.1516 µs 3.1665 µs]
Found 17 outliers among 100 measurements (17.00%)
  10 (10.00%) high mild
  7 (7.00%) high severe

turboshake128/2048/32 (cached)
                        time:   [5.6126 µs 5.6298 µs 5.6498 µs]
Found 12 outliers among 100 measurements (12.00%)
  8 (8.00%) high mild
  4 (4.00%) high severe

turboshake128/2048/32 (random)
                        time:   [5.7591 µs 5.7773 µs 5.7966 µs]
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) high mild
  2 (2.00%) high severe

turboshake128/4096/32 (cached)
                        time:   [10.812 µs 10.860 µs 10.916 µs]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

turboshake128/4096/32 (random)
                        time:   [10.999 µs 11.034 µs 11.072 µs]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

turboshake256/32/32 (cached)
                        time:   [439.24 ns 440.66 ns 442.35 ns]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

turboshake256/32/32 (random)
                        time:   [477.80 ns 479.40 ns 481.34 ns]
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  6 (6.00%) high mild
  3 (3.00%) high severe

turboshake256/64/32 (cached)
                        time:   [438.45 ns 439.41 ns 440.40 ns]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

turboshake256/64/32 (random)
                        time:   [486.78 ns 489.88 ns 492.96 ns]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

turboshake256/128/32 (cached)
                        time:   [440.13 ns 441.52 ns 443.06 ns]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

turboshake256/128/32 (random)
                        time:   [502.20 ns 506.00 ns 510.37 ns]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

turboshake256/256/32 (cached)
                        time:   [870.85 ns 872.62 ns 874.63 ns]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

turboshake256/256/32 (random)
                        time:   [960.29 ns 964.64 ns 969.34 ns]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

turboshake256/512/32 (cached)
                        time:   [1.7372 µs 1.7424 µs 1.7483 µs]
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) low mild
  8 (8.00%) high mild
  6 (6.00%) high severe

turboshake256/512/32 (random)
                        time:   [2.5907 µs 2.8738 µs 3.1747 µs]
Found 14 outliers among 100 measurements (14.00%)
  8 (8.00%) high mild
  6 (6.00%) high severe

turboshake256/1024/32 (cached)
                        time:   [3.6393 µs 3.6474 µs 3.6562 µs]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe

turboshake256/1024/32 (random)
                        time:   [3.7930 µs 3.8016 µs 3.8115 µs]
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) low severe
  5 (5.00%) low mild
  4 (4.00%) high mild
  6 (6.00%) high severe

turboshake256/2048/32 (cached)
                        time:   [8.2066 µs 8.7717 µs 9.5178 µs]
Found 13 outliers among 100 measurements (13.00%)
  13 (13.00%) high mild

turboshake256/2048/32 (random)
                        time:   [7.0309 µs 7.0552 µs 7.0829 µs]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

turboshake256/4096/32 (cached)
                        time:   [13.300 µs 13.332 µs 13.369 µs]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

turboshake256/4096/32 (random)
                        time:   [13.503 µs 13.567 µs 13.646 µs]
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) high mild
  7 (7.00%) high severe
```

## Usage

Using TurboSHAKE{128, 256} XOF API is fairly easy

1) Add `turboshake` to Cargo.toml, as your project dependency

```toml
[dependencies]
turboshake = { git = "https://github.com/itzmeanjan/turboshake" }
```

2) Create a TurboSHAKE{128, 256} XOF object.

```rust
use turboshake;

fn main() {
    let msg = [1u8; 8];      // message to be absorbed
    let mut dig = [0u8; 32]; // digest to be computed

    let mut hasher = turboshake::TurboShake128::new();
    // ...
}
```

3) Absorb N -bytes message into sponge state by invoking `absorb()` M -many times.

```rust
hasher.absorb(&msg[..2]);
hasher.absorb(&msg[2..4]);
hasher.absorb(&msg[4..]);
```

4) When all message bytes are consumed, finalize sponge state by calling `finalize()`.

```rust
// Note, one needs to pass a domain seperator constant byte in finalization step.
// You can use 0x1f ( i.e. default domain seperator value ) if you're not using
// multiple instances of TurboSHAKE. Consider reading section 1 ( top of page 2 )
// of TurboSHAKE specification https://eprint.iacr.org/2023/342.pdf.
hasher.finalize::<{ turboshake::TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>();
```

5) Now sponge is ready to be squeezed i.e. read arbitrary many bytes by invoking `squeeze()` arbitrary many times.

```rust
hasher.squeeze(&mut dig[..16]);
hasher.squeeze(&mut dig[16..]);
```

I maintain two examples demonstrating use of TurboSHAKE{128, 256} XOF API.

- [turboSHAKE128](./examples/turboshake128.rs)
- [turboSHAKE256](./examples/turboshake256.rs)

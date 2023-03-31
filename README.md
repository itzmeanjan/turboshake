# turboshake
TurboSHAKE: A Family of e**X**tendable **O**utput **F**unctions based on round reduced ( 12 rounds ) Keccak[1600] Permutation

## Overview

TurboSHAKE is a family of extendable output functions (XOFs) powered by round-reduced ( i.e. 12 -rounds ) Keccak-p[1600, 12] permutation. Keccak-p[1600, 12] has previously been used in fast parallel hashing algorithm KangarooTwelve ( more @ https://keccak.team/kangarootwelve.html ). Recently a formal specification, describing TurboSHAKE was released ( more @ https://ia.cr/2023/342 ) which generally exposes the underlying primitive of KangarooTwelve ( also known as **K12**, see https://blake12.org ) so that post-quantum public key cryptosystems ( such as Kyber, Dilithium etc. - being standardized by NIST ) benefit from it ( more @ https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/5HveEPBsbxY ).

Here I'm maintaining a Rust library which implements TurboSHAKE{128, 256} XOF s.t. one can absorb arbitrary many bytes into sponge state, finalize sponge and squeeze arbitrary many bytes out of sponge. It also exposes ( not by default, controlled by Rust feature gate `dev` ) raw API for keccak-p[1600, 12] permutation and sponge operations i.e. absorption, finalization and squeezing. Other features ( such as `simdx2` or `simdx4` ) expose advanced Keccak-p[1600, 12] permutation implementation s.t. using {128, 256} -bit SIMD registers for parallelly applying 2 or 4 keccak permutations. See [usage](#usage) section below for more info on how to use these.

## Prerequisites

Rust nightly toolchain; see https://rustup.rs for installation guide. 

> **Note** Nightly toolchain is required because I use `portable_simd` feature ( more @ https://doc.rust-lang.org/std/simd/struct.Simd.html ) for SIMD implementation of Keccak-p[1600, 12] permutation. See [rust-toolchain](./rust-toolchain.toml) file for understanding how toolchain version is overridden in this crate.

```bash
# When developing this library, I was using
$ rustc --version
rustc 1.70.0-nightly (a266f1199 2023-03-22)
```

## Testing

For ensuring functional correctness of TurboSHAKE{128, 256} implementation, I use test vectors from section 4 ( on page 9 ) and Appendix A ( on page 17 ) of https://datatracker.ietf.org/doc/draft-irtf-cfrg-kangarootwelve. Issue following command to run test cases

```bash
cargo test --lib
```

To ensure that {2, 4}x SIMD parallel Keccak-p[1600, 12] permutation is correctly implemented, I've added some test cases. Issue following command

```bash
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo test --lib keccak --features="simdx2 simdx4"
```

## Benchmarking

Issue following command for benchmarking round-reduced Keccak-p[1600, 12] permutation and TurboSHAKE{128, 256} XOF, for varying input sizes and constant ( = 32 -bytes ) squeezed output size.

```bash
# When interested in TurboSHAKE{128, 256} XOF
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench turboshake

# When interested in scalar Keccak-p[1600, 12] permutation
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench keccak --features="dev"

# When interested in 2x SIMD parallel Keccak-p[1600, 12] permutation
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench keccak --features="dev simdx2"

# When interested in 4x SIMD parallel Keccak-p[1600, 12] permutation
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench keccak --features="dev simdx4"
```

### On **Intel(R) Core(TM) i5-8279U CPU @ 2.40GHz**

#### TurboSHAKE{128, 256} XOF

```bash
turboshake128/32/32 (cached)
                        time:   [196.27 ns 196.96 ns 197.68 ns]
                        thrpt:  [154.38 MiB/s 154.95 MiB/s 155.49 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
turboshake128/32/32 (random)
                        time:   [224.49 ns 226.95 ns 230.58 ns]
                        thrpt:  [132.35 MiB/s 134.47 MiB/s 135.94 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe

turboshake128/64/32 (cached)
                        time:   [194.85 ns 196.51 ns 198.65 ns]
                        thrpt:  [307.25 MiB/s 310.59 MiB/s 313.25 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
turboshake128/64/32 (random)
                        time:   [229.64 ns 230.75 ns 231.90 ns]
                        thrpt:  [263.20 MiB/s 264.50 MiB/s 265.78 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe

turboshake128/128/32 (cached)
                        time:   [193.21 ns 193.82 ns 194.47 ns]
                        thrpt:  [627.72 MiB/s 629.81 MiB/s 631.80 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
turboshake128/128/32 (random)
                        time:   [239.42 ns 240.62 ns 241.83 ns]
                        thrpt:  [504.78 MiB/s 507.31 MiB/s 509.87 MiB/s]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

turboshake128/256/32 (cached)
                        time:   [357.85 ns 359.04 ns 360.64 ns]
                        thrpt:  [676.97 MiB/s 679.97 MiB/s 682.25 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe
turboshake128/256/32 (random)
                        time:   [453.21 ns 455.88 ns 458.74 ns]
                        thrpt:  [532.19 MiB/s 535.53 MiB/s 538.70 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

turboshake128/512/32 (cached)
                        time:   [687.83 ns 689.47 ns 691.34 ns]
                        thrpt:  [706.28 MiB/s 708.20 MiB/s 709.89 MiB/s]
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe
turboshake128/512/32 (random)
                        time:   [861.01 ns 871.25 ns 881.36 ns]
                        thrpt:  [554.01 MiB/s 560.44 MiB/s 567.10 MiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

turboshake128/1024/32 (cached)
                        time:   [1.1853 µs 1.1886 µs 1.1922 µs]
                        thrpt:  [819.13 MiB/s 821.59 MiB/s 823.88 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe
turboshake128/1024/32 (random)
                        time:   [1.3049 µs 1.3127 µs 1.3203 µs]
                        thrpt:  [739.65 MiB/s 743.95 MiB/s 748.39 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe

turboshake128/2048/32 (cached)
                        time:   [2.1791 µs 2.1978 µs 2.2206 µs]
                        thrpt:  [879.55 MiB/s 888.67 MiB/s 896.29 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
turboshake128/2048/32 (random)
                        time:   [2.3710 µs 2.3837 µs 2.3981 µs]
                        thrpt:  [814.43 MiB/s 819.37 MiB/s 823.77 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

turboshake128/4096/32 (cached)
                        time:   [4.1462 µs 4.1579 µs 4.1706 µs]
                        thrpt:  [936.61 MiB/s 939.47 MiB/s 942.12 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
turboshake128/4096/32 (random)
                        time:   [4.4655 µs 4.4910 µs 4.5200 µs]
                        thrpt:  [864.21 MiB/s 869.80 MiB/s 874.76 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

turboshake256/32/32 (cached)
                        time:   [185.13 ns 185.68 ns 186.31 ns]
                        thrpt:  [163.80 MiB/s 164.35 MiB/s 164.84 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe
turboshake256/32/32 (random)
                        time:   [225.98 ns 226.75 ns 227.59 ns]
                        thrpt:  [134.09 MiB/s 134.59 MiB/s 135.04 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

turboshake256/64/32 (cached)
                        time:   [185.37 ns 185.99 ns 186.82 ns]
                        thrpt:  [326.70 MiB/s 328.16 MiB/s 329.26 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
turboshake256/64/32 (random)
                        time:   [233.69 ns 234.75 ns 235.97 ns]
                        thrpt:  [258.66 MiB/s 260.00 MiB/s 261.19 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe

turboshake256/128/32 (cached)
                        time:   [188.13 ns 188.91 ns 189.67 ns]
                        thrpt:  [643.59 MiB/s 646.17 MiB/s 648.86 MiB/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
turboshake256/128/32 (random)
                        time:   [245.24 ns 246.77 ns 248.37 ns]
                        thrpt:  [491.48 MiB/s 494.66 MiB/s 497.75 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

turboshake256/256/32 (cached)
                        time:   [350.73 ns 351.57 ns 352.51 ns]
                        thrpt:  [692.59 MiB/s 694.42 MiB/s 696.10 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
turboshake256/256/32 (random)
                        time:   [441.48 ns 442.86 ns 444.30 ns]
                        thrpt:  [549.50 MiB/s 551.29 MiB/s 553.00 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

turboshake256/512/32 (cached)
                        time:   [679.69 ns 681.65 ns 683.80 ns]
                        thrpt:  [714.07 MiB/s 716.32 MiB/s 718.39 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
turboshake256/512/32 (random)
                        time:   [858.58 ns 868.71 ns 878.49 ns]
                        thrpt:  [555.82 MiB/s 562.08 MiB/s 568.71 MiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

turboshake256/1024/32 (cached)
                        time:   [1.3423 µs 1.3458 µs 1.3496 µs]
                        thrpt:  [723.60 MiB/s 725.65 MiB/s 727.54 MiB/s]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
turboshake256/1024/32 (random)
                        time:   [1.4489 µs 1.4552 µs 1.4616 µs]
                        thrpt:  [668.16 MiB/s 671.06 MiB/s 673.99 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

turboshake256/2048/32 (cached)
                        time:   [2.6615 µs 2.6675 µs 2.6743 µs]
                        thrpt:  [730.34 MiB/s 732.19 MiB/s 733.85 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe
turboshake256/2048/32 (random)
                        time:   [2.8298 µs 2.8415 µs 2.8548 µs]
                        thrpt:  [684.15 MiB/s 687.35 MiB/s 690.21 MiB/s]
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) high mild
  6 (6.00%) high severe

turboshake256/4096/32 (cached)
                        time:   [5.1313 µs 5.1456 µs 5.1615 µs]
                        thrpt:  [756.80 MiB/s 759.14 MiB/s 761.26 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
turboshake256/4096/32 (random)
                        time:   [5.3920 µs 5.4127 µs 5.4342 µs]
                        thrpt:  [718.82 MiB/s 721.68 MiB/s 724.45 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
```

#### Scalar Keccak-p[1600, 12] Permutation

```bash
keccak/keccak-p[1600, 12] (cached)
                        time:   [172.29 ns 173.40 ns 174.85 ns]
                        thrpt:  [1.0653 GiB/s 1.0742 GiB/s 1.0811 GiB/s]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe
keccak/keccak-p[1600, 12] (random)
                        time:   [187.82 ns 189.48 ns 191.23 ns]
                        thrpt:  [997.43 MiB/s 1006.6 MiB/s 1015.5 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
```


#### 2x SIMD parallel Keccak-p[1600, 12] Permutation

```bash
keccak/keccak-p[1600, 12] x2 (cached)
                        time:   [225.73 ns 226.31 ns 226.93 ns]
                        thrpt:  [1.6416 GiB/s 1.6461 GiB/s 1.6503 GiB/s]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
keccak/keccak-p[1600, 12] x2 (random)
                        time:   [271.94 ns 274.44 ns 277.29 ns]
                        thrpt:  [1.3435 GiB/s 1.3574 GiB/s 1.3699 GiB/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe
```

#### 4x SIMD parallel Keccak-p[1600, 12] Permutation

```bash
keccak/keccak-p[1600, 12] x4 (cached)
                        time:   [265.62 ns 266.36 ns 267.11 ns]
                        thrpt:  [2.7894 GiB/s 2.7972 GiB/s 2.8050 GiB/s]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
keccak/keccak-p[1600, 12] x4 (random)
                        time:   [401.66 ns 406.54 ns 410.82 ns]
                        thrpt:  [1.8136 GiB/s 1.8327 GiB/s 1.8550 GiB/s]
```

## Usage

Using TurboSHAKE{128, 256} XOF API is fairly easy

1) Add `turboshake` to Cargo.toml, with proper ( or may be none if you're only using it for TurboSHAKE XOF ) feature flags ( based on your intended use case ), as your project dependency

```toml
[dependencies]
# If only interested in using TurboSHAKE{128, 256} XOF API, do
# either
turboshake = { git = "https://github.com/itzmeanjan/turboshake" }
# or
turboshake = "0.1.5"

# If interested in using underlying keccak-p[1600, 12] permutation and sponge (developer) API
turboshake = { version = "0.1.5", features = "dev" }
# or if interested in using underlying 2x SIMD parallel keccak-p[1600, 12] permutation API
turboshake = { version = "0.1.5", features = ["dev", "simdx2"] }
# or if interested in using underlying 4x SIMD parallel keccak-p[1600, 12] permutation API
turboshake = { version = "0.1.5", features = ["dev", "simdx4"] }
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

3) Absorb N(>=0) -bytes message into sponge state by invoking `absorb()` M(>1) -many times.

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

You should be able to run those examples with following commands

```bash
cargo run --example turboshake128

Message: 44abe2f57f3186dd40e761d955fbda1b0dd21a86ed17fdd0d389e1b578857b09a0ef1236ef02cefd6f7d7e7a23e1d200066361de50315655b614ef5f7f72f1e6
Digest: 721f1b1c6afa722e10001f2e2058844756cdf51c4ca00179073665a34720b317

# or
cargo run --example turboshake256

Message: 2f9f1b0bcf2b22a641ac3db02308c3bdf19acea8d271bd4d72d107c53b19e145fa520ffe15cdba0236131071b0d4f84cb57b2842220f5d13ff0393cb1c37d679
Digest: 9e5310a6f2965899ebcdea891b01d08431957ad0dd12bee163c55c8e38b2cf4c
```

I also maintain examples showing usage of keccak-p[1600, 12] permutation, hidden behind `dev` feature-gate, in [keccak.rs](./examples/keccak.rs). Run that example by issuing

```bash
cargo run --example keccak --features="dev"
```

In case you're planning to use {2, 4}x SIMD parallel Keccak-p[1600, 12] permutation, which is hidden behind `dev` and `simdx{2,4}` feature-gates, consider looking at [simd_keccak.rs](./examples/simd_keccak.rs). You can run that example by issuing

```bash
cargo run --example simd_keccak --features="dev simdx4"
```

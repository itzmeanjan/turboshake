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
                        time:   [192.12 ns 192.64 ns 193.21 ns]
                        thrpt:  [157.95 MiB/s 158.42 MiB/s 158.84 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
turboshake128/32/32 (random)
                        time:   [227.07 ns 227.96 ns 228.95 ns]
                        thrpt:  [133.29 MiB/s 133.87 MiB/s 134.39 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

turboshake128/64/32 (cached)
                        time:   [189.56 ns 190.81 ns 192.51 ns]
                        thrpt:  [317.06 MiB/s 319.87 MiB/s 321.98 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
turboshake128/64/32 (random)
                        time:   [234.05 ns 234.74 ns 235.48 ns]
                        thrpt:  [259.19 MiB/s 260.01 MiB/s 260.78 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

turboshake128/128/32 (cached)
                        time:   [190.15 ns 190.85 ns 191.67 ns]
                        thrpt:  [636.89 MiB/s 639.62 MiB/s 641.97 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
turboshake128/128/32 (random)
                        time:   [246.11 ns 247.20 ns 248.34 ns]
                        thrpt:  [491.54 MiB/s 493.81 MiB/s 495.99 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

turboshake128/256/32 (cached)
                        time:   [362.72 ns 364.08 ns 365.60 ns]
                        thrpt:  [667.79 MiB/s 670.58 MiB/s 673.09 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
turboshake128/256/32 (random)
                        time:   [453.01 ns 455.44 ns 458.20 ns]
                        thrpt:  [532.82 MiB/s 536.06 MiB/s 538.93 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

turboshake128/512/32 (cached)
                        time:   [706.11 ns 708.11 ns 710.38 ns]
                        thrpt:  [687.36 MiB/s 689.56 MiB/s 691.51 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
turboshake128/512/32 (random)
                        time:   [881.90 ns 892.06 ns 901.74 ns]
                        thrpt:  [541.49 MiB/s 547.36 MiB/s 553.67 MiB/s]

turboshake128/1024/32 (cached)
                        time:   [1.2173 µs 1.2208 µs 1.2247 µs]
                        thrpt:  [797.42 MiB/s 799.92 MiB/s 802.26 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
turboshake128/1024/32 (random)
                        time:   [1.3102 µs 1.3156 µs 1.3217 µs]
                        thrpt:  [738.85 MiB/s 742.30 MiB/s 745.37 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe

turboshake128/2048/32 (cached)
                        time:   [2.2567 µs 2.2672 µs 2.2816 µs]
                        thrpt:  [856.05 MiB/s 861.48 MiB/s 865.50 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
turboshake128/2048/32 (random)
                        time:   [2.3796 µs 2.3922 µs 2.4080 µs]
                        thrpt:  [811.09 MiB/s 816.47 MiB/s 820.79 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

turboshake128/4096/32 (cached)
                        time:   [4.3059 µs 4.3170 µs 4.3297 µs]
                        thrpt:  [902.19 MiB/s 904.86 MiB/s 907.19 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) high mild
  2 (2.00%) high severe
turboshake128/4096/32 (random)
                        time:   [4.4958 µs 4.5177 µs 4.5417 µs]
                        thrpt:  [860.08 MiB/s 864.65 MiB/s 868.87 MiB/s]
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) high mild
  6 (6.00%) high severe

turboshake256/32/32 (cached)
                        time:   [187.46 ns 187.87 ns 188.31 ns]
                        thrpt:  [162.06 MiB/s 162.44 MiB/s 162.80 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
turboshake256/32/32 (random)
                        time:   [228.14 ns 230.18 ns 232.87 ns]
                        thrpt:  [131.05 MiB/s 132.58 MiB/s 133.77 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

turboshake256/64/32 (cached)
                        time:   [187.78 ns 188.37 ns 189.05 ns]
                        thrpt:  [322.86 MiB/s 324.01 MiB/s 325.03 MiB/s]
Found 12 outliers among 100 measurements (12.00%)
  8 (8.00%) high mild
  4 (4.00%) high severe
turboshake256/64/32 (random)
                        time:   [233.33 ns 234.31 ns 235.39 ns]
                        thrpt:  [259.30 MiB/s 260.49 MiB/s 261.58 MiB/s]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

turboshake256/128/32 (cached)
                        time:   [188.01 ns 188.58 ns 189.24 ns]
                        thrpt:  [645.06 MiB/s 647.31 MiB/s 649.27 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high severe
turboshake256/128/32 (random)
                        time:   [243.56 ns 245.10 ns 246.75 ns]
                        thrpt:  [494.71 MiB/s 498.04 MiB/s 501.19 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

turboshake256/256/32 (cached)
                        time:   [364.29 ns 365.10 ns 366.00 ns]
                        thrpt:  [667.05 MiB/s 668.69 MiB/s 670.18 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
turboshake256/256/32 (random)
                        time:   [461.17 ns 463.19 ns 465.35 ns]
                        thrpt:  [524.64 MiB/s 527.08 MiB/s 529.40 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

turboshake256/512/32 (cached)
                        time:   [714.28 ns 716.45 ns 718.91 ns]
                        thrpt:  [679.20 MiB/s 681.53 MiB/s 683.60 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe
turboshake256/512/32 (random)
                        time:   [910.24 ns 920.89 ns 931.37 ns]
                        thrpt:  [524.26 MiB/s 530.23 MiB/s 536.43 MiB/s]

turboshake256/1024/32 (cached)
                        time:   [1.4109 µs 1.4161 µs 1.4216 µs]
                        thrpt:  [686.93 MiB/s 689.59 MiB/s 692.16 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
turboshake256/1024/32 (random)
                        time:   [1.5463 µs 1.5528 µs 1.5600 µs]
                        thrpt:  [626.02 MiB/s 628.89 MiB/s 631.57 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

turboshake256/2048/32 (cached)
                        time:   [2.7985 µs 2.8059 µs 2.8143 µs]
                        thrpt:  [694.01 MiB/s 696.07 MiB/s 697.92 MiB/s]
Found 13 outliers among 100 measurements (13.00%)
  8 (8.00%) high mild
  5 (5.00%) high severe
turboshake256/2048/32 (random)
                        time:   [3.0009 µs 3.0131 µs 3.0270 µs]
                        thrpt:  [645.23 MiB/s 648.21 MiB/s 650.85 MiB/s]
Found 13 outliers among 100 measurements (13.00%)
  8 (8.00%) high mild
  5 (5.00%) high severe

turboshake256/4096/32 (cached)
                        time:   [5.3983 µs 5.4146 µs 5.4330 µs]
                        thrpt:  [718.98 MiB/s 721.43 MiB/s 723.61 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe
turboshake256/4096/32 (random)
                        time:   [5.7319 µs 5.7521 µs 5.7741 µs]
                        thrpt:  [676.51 MiB/s 679.10 MiB/s 681.49 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
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
turboshake = "0.1.4"

# If interested in using underlying keccak-p[1600, 12] permutation and sponge (developer) API
turboshake = { version = "0.1.4", features = "dev" }
# or if interested in using underlying 2x SIMD parallel keccak-p[1600, 12] permutation API
turboshake = { version = "0.1.4", features = ["dev", "simdx2"] }
# or if interested in using underlying 4x SIMD parallel keccak-p[1600, 12] permutation API
turboshake = { version = "0.1.4", features = ["dev", "simdx4"] }
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

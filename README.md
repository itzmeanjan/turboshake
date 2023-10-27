# turboshake
TurboSHAKE: A Family of e**X**tendable **O**utput **F**unctions based on round reduced ( 12 rounds ) Keccak[1600] Permutation

## Overview

TurboSHAKE is a family of extendable output functions (XOFs) powered by round-reduced ( i.e. 12 -rounds ) Keccak-p[1600, 12] permutation. Keccak-p[1600, 12] has previously been used in fast parallel hashing algorithm KangarooTwelve ( more @ https://keccak.team/kangarootwelve.html ). Recently a formal specification, describing TurboSHAKE was released ( more @ https://ia.cr/2023/342 ) which generally exposes the underlying primitive of KangarooTwelve ( also known as **K12**, see https://blake12.org ) so that post-quantum public key cryptosystems ( such as Kyber, Dilithium etc. - being standardized by NIST ) benefit from it ( more @ https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/5HveEPBsbxY ).

Here I'm maintaining a Rust library which implements TurboSHAKE{128, 256} XOF s.t. one can absorb arbitrary many bytes into sponge state, finalize sponge and squeeze arbitrary many bytes out of sponge. It also exposes ( not by default, controlled by Rust feature gate `"dev"` ) raw API for keccak-p[1600, 12] permutation and sponge operations i.e. absorption, finalization and squeezing. Other features ( such as `"simdx2"` or `"simdx4"` ) expose advanced Keccak-p[1600, 12] permutation implementation s.t. using {128, 256} -bit SIMD registers for parallelly applying 2 or 4 keccak permutations. See [usage](#usage) section below for more info on how to use these.

## Prerequisites

Rust nightly toolchain; see https://rustup.rs for installation guide. 

> **Note** Nightly toolchain is required because I use `portable_simd` feature ( more @ https://doc.rust-lang.org/std/simd/struct.Simd.html ) for SIMD implementation of Keccak-p[1600, 12] permutation. See [rust-toolchain](./rust-toolchain.toml) file for understanding how toolchain version is overridden in this crate.

```bash
# When developing this library, I was using
$ rustc --version
rustc 1.75.0-nightly (df871fbf0 2023-10-24)
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

### On **12th Gen Intel(R) Core(TM) i7-1260P**

#### TurboSHAKE{128, 256} XOF

```bash
turboshake128/32/64 (cached)
                        time:   [300.2013 cycles 300.6455 cycles 301.1383 cycles]
                        thrpt:  [3.1369 cpb 3.1317 cpb 3.1271 cpb]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
turboshake128/32/64 (random)
                        time:   [332.1907 cycles 332.7514 cycles 333.3522 cycles]
                        thrpt:  [3.4724 cpb 3.4662 cpb 3.4603 cpb]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild

turboshake128/64/64 (cached)
                        time:   [280.0360 cycles 280.3097 cycles 280.6003 cycles]
                        thrpt:  [2.1922 cpb 2.1899 cpb 2.1878 cpb]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
turboshake128/64/64 (random)
                        time:   [335.8502 cycles 336.4064 cycles 336.9526 cycles]
                        thrpt:  [2.6324 cpb 2.6282 cpb 2.6238 cpb]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) low severe
  5 (5.00%) low mild

turboshake128/128/64 (cached)
                        time:   [296.6593 cycles 297.1498 cycles 297.6553 cycles]
                        thrpt:  [1.5503 cpb 1.5477 cpb 1.5451 cpb]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
turboshake128/128/64 (random)
                        time:   [347.2465 cycles 347.4888 cycles 347.7725 cycles]
                        thrpt:  [1.8113 cpb 1.8098 cpb 1.8086 cpb]
Found 20 outliers among 100 measurements (20.00%)
  15 (15.00%) low severe
  3 (3.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe

turboshake128/256/64 (cached)
                        time:   [513.7296 cycles 514.3083 cycles 514.9579 cycles]
                        thrpt:  [1.6092 cpb 1.6072 cpb 1.6054 cpb]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  7 (7.00%) high mild
turboshake128/256/64 (random)
                        time:   [577.1995 cycles 577.6613 cycles 578.1892 cycles]
                        thrpt:  [1.8068 cpb 1.8052 cpb 1.8037 cpb]
Found 18 outliers among 100 measurements (18.00%)
  13 (13.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe

turboshake128/512/64 (cached)
                        time:   [1005.1465 cycles 1007.1958 cycles 1009.3091 cycles]
                        thrpt:  [1.7523 cpb 1.7486 cpb 1.7450 cpb]
turboshake128/512/64 (random)
                        time:   [1060.4501 cycles 1061.7896 cycles 1063.1359 cycles]
                        thrpt:  [1.8457 cpb 1.8434 cpb 1.8411 cpb]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

turboshake128/1024/64 (cached)
                        time:   [1852.6330 cycles 1856.6869 cycles 1861.1529 cycles]
                        thrpt:  [1.7106 cpb 1.7065 cpb 1.7028 cpb]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
turboshake128/1024/64 (random)
                        time:   [1870.4664 cycles 1876.7310 cycles 1882.9403 cycles]
                        thrpt:  [1.7306 cpb 1.7249 cpb 1.7192 cpb]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) low mild
  2 (2.00%) high mild

turboshake128/2048/64 (cached)
                        time:   [3209.0614 cycles 3218.2832 cycles 3228.0305 cycles]
                        thrpt:  [1.5284 cpb 1.5238 cpb 1.5194 cpb]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
turboshake128/2048/64 (random)
                        time:   [3411.0388 cycles 3422.9302 cycles 3435.0158 cycles]
                        thrpt:  [1.6264 cpb 1.6207 cpb 1.6151 cpb]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

turboshake128/4096/64 (cached)
                        time:   [6427.1110 cycles 6442.7551 cycles 6458.1191 cycles]
                        thrpt:  [1.5524 cpb 1.5487 cpb 1.5450 cpb]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
turboshake128/4096/64 (random)
                        time:   [6652.8576 cycles 6666.2079 cycles 6679.8884 cycles]
                        thrpt:  [1.6057 cpb 1.6025 cpb 1.5992 cpb]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) low severe
  2 (2.00%) low mild

turboshake256/32/64 (cached)
                        time:   [290.2433 cycles 290.6056 cycles 290.9691 cycles]
                        thrpt:  [3.0309 cpb 3.0271 cpb 3.0234 cpb]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
turboshake256/32/64 (random)
                        time:   [317.6748 cycles 318.4999 cycles 319.3679 cycles]
                        thrpt:  [3.3267 cpb 3.3177 cpb 3.3091 cpb]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low mild

turboshake256/64/64 (cached)
                        time:   [271.3381 cycles 271.5099 cycles 271.6935 cycles]
                        thrpt:  [2.1226 cpb 2.1212 cpb 2.1198 cpb]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe
turboshake256/64/64 (random)
                        time:   [317.8984 cycles 318.0956 cycles 318.3030 cycles]
                        thrpt:  [2.4867 cpb 2.4851 cpb 2.4836 cpb]
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

turboshake256/128/64 (cached)
                        time:   [271.1864 cycles 271.3779 cycles 271.5804 cycles]
                        thrpt:  [1.4145 cpb 1.4134 cpb 1.4124 cpb]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
turboshake256/128/64 (random)
                        time:   [335.0108 cycles 335.3328 cycles 335.6551 cycles]
                        thrpt:  [1.7482 cpb 1.7465 cpb 1.7448 cpb]
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) low severe
  5 (5.00%) low mild
  1 (1.00%) high mild

turboshake256/256/64 (cached)
                        time:   [512.2497 cycles 513.5156 cycles 514.8756 cycles]
                        thrpt:  [1.6090 cpb 1.6047 cpb 1.6008 cpb]
turboshake256/256/64 (random)
                        time:   [566.7176 cycles 567.1456 cycles 567.5909 cycles]
                        thrpt:  [1.7737 cpb 1.7723 cpb 1.7710 cpb]
Found 14 outliers among 100 measurements (14.00%)
  9 (9.00%) low severe
  4 (4.00%) low mild
  1 (1.00%) high mild

turboshake256/512/64 (cached)
                        time:   [1074.7070 cycles 1076.0370 cycles 1077.4826 cycles]
                        thrpt:  [1.8706 cpb 1.8681 cpb 1.8658 cpb]
Found 16 outliers among 100 measurements (16.00%)
  10 (10.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe
turboshake256/512/64 (random)
                        time:   [1117.6844 cycles 1119.7195 cycles 1121.6716 cycles]
                        thrpt:  [1.9473 cpb 1.9440 cpb 1.9404 cpb]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) low severe
  1 (1.00%) low mild

turboshake256/1024/64 (cached)
                        time:   [2141.1026 cycles 2143.8033 cycles 2146.6343 cycles]
                        thrpt:  [1.9730 cpb 1.9704 cpb 1.9679 cpb]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
turboshake256/1024/64 (random)
                        time:   [2184.5754 cycles 2188.1021 cycles 2191.6081 cycles]
                        thrpt:  [2.0143 cpb 2.0111 cpb 2.0079 cpb]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) low severe
  2 (2.00%) high mild

turboshake256/2048/64 (cached)
                        time:   [4255.8212 cycles 4261.1077 cycles 4266.6846 cycles]
                        thrpt:  [2.0202 cpb 2.0176 cpb 2.0151 cpb]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
turboshake256/2048/64 (random)
                        time:   [4315.7201 cycles 4322.8028 cycles 4329.9128 cycles]
                        thrpt:  [2.0501 cpb 2.0468 cpb 2.0434 cpb]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) low severe
  2 (2.00%) low mild
  3 (3.00%) high mild

turboshake256/4096/64 (cached)
                        time:   [7739.5467 cycles 7765.2710 cycles 7791.6487 cycles]
                        thrpt:  [1.8730 cpb 1.8667 cpb 1.8605 cpb]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
turboshake256/4096/64 (random)
                        time:   [8195.0008 cycles 8209.3241 cycles 8224.9569 cycles]
                        thrpt:  [1.9772 cpb 1.9734 cpb 1.9700 cpb]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
```

#### Scalar Keccak-p[1600, 12] Permutation

```bash
keccak/keccak-p[1600, 12] (cached)
                        time:   [241.4698 cycles 241.6765 cycles 241.9185 cycles]
                        thrpt:  [1.2096 cpb 1.2084 cpb 1.2073 cpb]
Found 11 outliers among 100 measurements (11.00%)
  9 (9.00%) high mild
  2 (2.00%) high severe
keccak/keccak-p[1600, 12] (random)
                        time:   [263.9347 cycles 264.9104 cycles 265.9320 cycles]
                        thrpt:  [1.3297 cpb 1.3246 cpb 1.3197 cpb]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```


#### 2x SIMD parallel Keccak-p[1600, 12] Permutation

```bash
keccak/keccak-p[1600, 12] x2 (cached)
                        time:   [453.2136 cycles 453.4695 cycles 453.7510 cycles]
                        thrpt:  [1.1344 cpb 1.1337 cpb 1.1330 cpb]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
keccak/keccak-p[1600, 12] x2 (random)
                        time:   [484.9887 cycles 485.6587 cycles 486.3218 cycles]
                        thrpt:  [1.2158 cpb 1.2141 cpb 1.2125 cpb]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
```

#### 4x SIMD parallel Keccak-p[1600, 12] Permutation

```bash
keccak/keccak-p[1600, 12] x4 (cached)
                        time:   [713.5586 cycles 713.8267 cycles 714.1180 cycles]
                        thrpt:  [0.8926 cpb 0.8923 cpb 0.8919 cpb]
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low severe
  10 (10.00%) high mild
  1 (1.00%) high severe
keccak/keccak-p[1600, 12] x4 (random)
                        time:   [842.6883 cycles 844.9042 cycles 846.9812 cycles]
                        thrpt:  [1.0587 cpb 1.0561 cpb 1.0534 cpb]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild
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
turboshake = "0.1.7"

# If interested in using underlying keccak-p[1600, 12] permutation and sponge (developer) API
turboshake = { version = "0.1.7", features = "dev" }
# or if interested in using underlying 2x SIMD parallel keccak-p[1600, 12] permutation API
turboshake = { version = "0.1.7", features = ["dev", "simdx2"] }
# or if interested in using underlying 4x SIMD parallel keccak-p[1600, 12] permutation API
turboshake = { version = "0.1.7", features = ["dev", "simdx4"] }
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

6) Finally you can reset the state of the sponge and restart the whole `absorb->finalize->squeeze` cycle.

```rust
hasher.reset();
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

I also maintain examples showing usage of keccak-p[1600, 12] permutation, hidden behind `"dev"` feature-gate, in [keccak.rs](./examples/keccak.rs). Run that example by issuing

```bash
cargo run --example keccak --features="dev"
```

In case you're planning to use {2, 4}x SIMD parallel Keccak-p[1600, 12] permutation, which is hidden behind `dev` and `simdx{2,4}` feature-gates, consider looking at [simd_keccak.rs](./examples/simd_keccak.rs). You can run that example by issuing

```bash
cargo run --example simd_keccak --features="dev simdx4"
```

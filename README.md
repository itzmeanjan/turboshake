# turboshake
TurboSHAKE: A Family of e**X**tendable **O**utput **F**unctions (XOFs) based on round reduced (12 -rounds) Keccak[1600] permutation.

## Overview
TurboSHAKE is a family of extendable output functions (XOFs) powered by round-reduced (i.e. 12 -rounds) Keccak-p[1600, 12] permutation. Keccak-p[1600, 12] has previously been used in fast parallel hashing algorithm KangarooTwelve (more @ https://keccak.team/kangarootwelve.html). Recently a formal specification, describing TurboSHAKE was released (more @ https://ia.cr/2023/342) which generally exposes the underlying primitive of KangarooTwelve (also known as **K12**, see https://blake12.org) so that post-quantum public key cryptosystems (such as ML-KEM, ML-DSA etc. - standardized by NIST) might benefit from it (more @ https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/5HveEPBsbxY).

Here I'm maintaining a zero-dependency Rust library which implements TurboSHAKE{128, 256} Xof s.t. one can absorb arbitrary many bytes into sponge state, by calling `absorb` function as many times needed; finalize sponge and then start squeezing arbitrary many bytes out of sponge, by caling `squeeze` as many times needed. See [usage](#usage) section below for more info.

## Prerequisites
Rust stable toolchain; see https://rustup.rs for installation guide. MSRV for this library crate is **1.85.0**.

```bash
# When developing this library, I was using
$ rustc --version
rustc 1.88.0 (6b00bc388 2025-06-23)
```

## Testing
For ensuring functional correctness of TurboSHAKE{128, 256} implementation, I use test vectors from section 4 (on page 9) and Appendix A (on page 17) of https://datatracker.ietf.org/doc/draft-irtf-cfrg-kangarootwelve. Run following command(s) to run all test cases.

```bash
# Running tests on host.
make test

# Testing on web assembly target, using `wasmtime`.
rustup target add wasm32-wasip1
cargo install wasmtime-cli --locked
make test-wasm
```

```bash
running 16 tests
test tests::state_transition_should_work_in_ts256 ... ok
test tests::test_incremental_ts128_hashing::message_length_128b_digest_length_128b ... ok
test tests::state_transition_should_work_in_ts128 ... ok
test tests::test_incremental_ts128_hashing::message_length_32b_digest_length_32b ... ok
test tests::test_incremental_ts256_hashing::message_length_128b_digest_length_128b ... ok
test tests::test_incremental_ts256_hashing::message_length_32b_digest_length_32b ... ok
test tests::test_incremental_ts128_hashing::message_length_2kb_digest_length_2kb ... ok
test tests::test_incremental_ts128_hashing::message_length_512b_digest_length_512b ... ok
test tests::test_incremental_ts256_hashing::message_length_512b_digest_length_512b ... ok
test tests::test_incremental_ts256_hashing::message_length_2kb_digest_length_2kb ... ok
test tests::test_incremental_ts128_hashing::message_length_8kb_digest_length_8kb ... ok
test tests::test_incremental_ts256_hashing::message_length_8kb_digest_length_8kb ... ok
test tests::test_incremental_ts128_hashing::message_length_32kb_digest_length_32kb ... ok
test tests::test_incremental_ts256_hashing::message_length_32kb_digest_length_32kb ... ok
test tests::test_turboshake128 ... ok
test tests::test_turboshake256 ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

   Doc-tests turboshake

running 9 tests
test src/turboshake256.rs - turboshake256::TurboShake256::absorb (line 70) ... ok
test src/turboshake128.rs - turboshake128::TurboShake128::squeeze (line 138) ... ok
test src/turboshake256.rs - turboshake256::TurboShake256::squeeze (line 138) ... ok
test src/lib.rs - (line 16) ... ok
test src/turboshake128.rs - turboshake128::TurboShake128::default (line 31) ... ok
test src/turboshake128.rs - turboshake128::TurboShake128::absorb (line 70) ... ok
test src/turboshake128.rs - turboshake128::TurboShake128::finalize (line 100) ... ok
test src/turboshake256.rs - turboshake256::TurboShake256::default (line 31) ... ok
test src/turboshake256.rs - turboshake256::TurboShake256::finalize (line 100) ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Code Coverage
To generate a detailed code coverage report in HTML format, use [cargo-tarpaulin](https://github.com/xd009642/tarpaulin):

```bash
# Install cargo-tarpaulin if not already installed
cargo install cargo-tarpaulin
make coverage
```

```bash
Coverage Results:
|| Tested/Total Lines:
|| src/branch_opt_util.rs: 4/9
|| src/error.rs: 0/6
|| src/keccak.rs: 346/393
|| src/sponge.rs: 52/56
|| src/turboshake128.rs: 19/20
|| src/turboshake256.rs: 19/20
|| 
87.30% coverage, 440/504 lines covered
```

This will create a HTML coverage report at `tarpaulin-report.html` that you can open in your web browser to view detailed line-by-line coverage information for all source files.

> [!NOTE]
> There is a help menu, which introduces you to all available commands; just run `$ make` from the root directory of this crate.

## Benchmarking
Run following command for benchmarking round-reduced Keccak-p[1600, 12] permutation and TurboSHAKE{128, 256} Xof, for variable input and output sizes.

> [!WARNING]
> When benchmarking make sure you've disabled CPU frequency scaling, otherwise numbers you see can be misleading. I found https://github.com/google/benchmark/blob/b40db869/docs/reducing_variance.md helpful.

```bash
make bench
```

### On 12th Gen Intel(R) Core(TM) i7-1260P
Running kernel `Linux 6.11.0-14-generic x86_64`, with Rust compiler `1.84.1 (e71f9a9a9 2025-01-27)`, compiled in `optimized` mode.

```bash
Timer precision: 20 ns
keccak                fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ permute_12_rounds  82.32 ns      │ 226.3 ns      │ 83.1 ns       │ 88.24 ns      │ 100     │ 3200
                      2.262 GiB/s   │ 842.7 MiB/s   │ 2.241 GiB/s   │ 2.11 GiB/s    │         │
                      12.14 Mitem/s │ 4.418 Mitem/s │ 12.03 Mitem/s │ 11.33 Mitem/s │         │

Timer precision: 19 ns
turboshake                      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ turboshake128                              │               │               │               │         │
│  ├─ msg = 2.0KB, md = 64.0B   1.25 µs       │ 9.534 µs      │ 1.475 µs      │ 1.547 µs      │ 100     │ 100
│  │                            1.572 GiB/s   │ 211.2 MiB/s   │ 1.332 GiB/s   │ 1.27 GiB/s    │         │
│  │                            799.6 Kitem/s │ 104.8 Kitem/s │ 677.6 Kitem/s │ 646.1 Kitem/s │         │
│  ├─ msg = 8.0KB, md = 64.0B   4.592 µs      │ 6.655 µs      │ 4.623 µs      │ 4.644 µs      │ 100     │ 100
│  │                            1.674 GiB/s   │ 1.155 GiB/s   │ 1.663 GiB/s   │ 1.655 GiB/s   │         │
│  │                            217.7 Kitem/s │ 150.2 Kitem/s │ 216.3 Kitem/s │ 215.3 Kitem/s │         │
│  ├─ msg = 32.0B, md = 64.0B   115.1 ns      │ 121.6 ns      │ 116 ns        │ 116.1 ns      │ 100     │ 1600
│  │                            795.3 MiB/s   │ 752.8 MiB/s   │ 788.9 MiB/s   │ 788.4 MiB/s   │         │
│  │                            8.687 Mitem/s │ 8.223 Mitem/s │ 8.617 Mitem/s │ 8.612 Mitem/s │         │
│  ├─ msg = 128.0B, md = 64.0B  119.7 ns      │ 249.9 ns      │ 124 ns        │ 131.8 ns      │ 100     │ 1600
│  │                            1.493 GiB/s   │ 732.4 MiB/s   │ 1.441 GiB/s   │ 1.356 GiB/s   │         │
│  │                            8.352 Mitem/s │ 4 Mitem/s     │ 8.061 Mitem/s │ 7.584 Mitem/s │         │
│  ╰─ msg = 512.0B, md = 64.0B  400.6 ns      │ 725.6 ns      │ 407.1 ns      │ 423.9 ns      │ 100     │ 400
│                               1.339 GiB/s   │ 757 MiB/s     │ 1.317 GiB/s   │ 1.265 GiB/s   │         │
│                               2.496 Mitem/s │ 1.378 Mitem/s │ 2.456 Mitem/s │ 2.358 Mitem/s │         │
╰─ turboshake256                              │               │               │               │         │
   ├─ msg = 2.0KB, md = 64.0B   1.526 µs      │ 4.404 µs      │ 1.544 µs      │ 1.574 µs      │ 100     │ 100
   │                            1.288 GiB/s   │ 457.2 MiB/s   │ 1.273 GiB/s   │ 1.248 GiB/s   │         │
   │                            655 Kitem/s   │ 227 Kitem/s   │ 647.6 Kitem/s │ 634.9 Kitem/s │         │
   ├─ msg = 8.0KB, md = 64.0B   5.711 µs      │ 8.574 µs      │ 5.747 µs      │ 5.922 µs      │ 100     │ 100
   │                            1.346 GiB/s   │ 918.2 MiB/s   │ 1.337 GiB/s   │ 1.298 GiB/s   │         │
   │                            175 Kitem/s   │ 116.6 Kitem/s │ 173.9 Kitem/s │ 168.8 Kitem/s │         │
   ├─ msg = 32.0B, md = 64.0B   114.2 ns      │ 201.4 ns      │ 116.7 ns      │ 125.1 ns      │ 100     │ 1600
   │                            801.4 MiB/s   │ 454.3 MiB/s   │ 784.3 MiB/s   │ 731.5 MiB/s   │         │
   │                            8.754 Mitem/s │ 4.963 Mitem/s │ 8.566 Mitem/s │ 7.99 Mitem/s  │         │
   ├─ msg = 128.0B, md = 64.0B  119.9 ns      │ 141.9 ns      │ 121.9 ns      │ 122.3 ns      │ 100     │ 1600
   │                            1.49 GiB/s    │ 1.259 GiB/s   │ 1.465 GiB/s   │ 1.461 GiB/s   │         │
   │                            8.334 Mitem/s │ 7.046 Mitem/s │ 8.197 Mitem/s │ 8.172 Mitem/s │         │
   ╰─ msg = 512.0B, md = 64.0B  400.2 ns      │ 427.2 ns      │ 408.2 ns      │ 407.4 ns      │ 100     │ 800
                                1.34 GiB/s    │ 1.255 GiB/s   │ 1.314 GiB/s   │ 1.316 GiB/s   │         │
                                2.498 Mitem/s │ 2.34 Mitem/s  │ 2.449 Mitem/s │ 2.454 Mitem/s │         │
```

## Usage
Using TurboSHAKE{128, 256} Xof API is fairly easy.

1) Add `turboshake` to your project's Cargo.toml.

```toml
[dependencies]
turboshake = "0.5.0"
```

2) Create a TurboSHAKE{128, 256} Xof object.

```rust
use turboshake;

fn main() {
    let msg = [1u8; 8];      // message to be absorbed
    let mut dig = [0u8; 32]; // digest to be computed

    let mut hasher = turboshake::TurboShake128::default();
    // ...
}
```

3) Absorb N(>=0) -bytes message into sponge state by invoking `absorb()` M(>1) -many times.

```rust
hasher.absorb(&msg[..2]).expect("data absorption must not fail");
hasher.absorb(&msg[2..4]).expect("data absorption must not fail");
hasher.absorb(&msg[4..]).expect("data absorption must not fail");
```

4) When all message bytes are consumed, finalize sponge state by calling `finalize()`.

```rust
// Note, one needs to pass a domain seperator constant byte in finalization step.
// You can use 0x1f ( i.e. default domain seperator value ) if you're not using
// multiple instances of TurboSHAKE. Consider reading section 1 ( top of page 2 )
// of TurboSHAKE specification https://eprint.iacr.org/2023/342.pdf.
hasher.finalize::<{ turboshake::TurboShake128::DEFAULT_DOMAIN_SEPARATOR }>().expect("finalization must not fail");
```

5) Now sponge is ready to be squeezed i.e. read arbitrary many bytes by invoking `squeeze()` arbitrary many times.

```rust
hasher.squeeze(&mut dig[..16]).expect("data squeezing must not fail");
hasher.squeeze(&mut dig[16..]).expect("data squeezing must not fail");
```

I maintain two examples demonstrating use of TurboSHAKE{128, 256} Xof API.

- [TurboSHAKE128](./examples/turboshake128.rs)
- [TurboSHAKE256](./examples/turboshake256.rs)

You should be able to run those examples with `$ make example`

```bash
     Running `target/debug/examples/turboshake128`
Message: e159c50514082cd410872dfd4c2b4930f3938820f98b0caae7ac6447919b05ec72848349e59a005f31ad4566fb6ed016c8b6495005a6b2782a6a2af478eda484
Digest: d815057ae50debf6c2e70de88bb773ecbdd7674e7ac1277729d7d56132689262

     Running `target/debug/examples/turboshake256`
Message: 7ce2691f95c35b9ed5b37e2971fd045a4d7a3660ef5ea0aa9baff27d8f04fd1e756bd7f4c746a41a7a32d7cb130a53290827188dcf037d8702bcc15147aee34c
Digest: f205817dc34eff3c77a901aefbbc8130e0fd92899f6be2cd319b44c501b51865
```

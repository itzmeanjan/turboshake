# turboshake
TurboSHAKE: A Family of e**X**tendable **O**utput **F**unctions based on round reduced ( 12 rounds ) Keccak[1600] Permutation.

## Overview
TurboSHAKE is a family of extendable output functions (xof) powered by round-reduced ( i.e. 12 -rounds ) Keccak-p[1600, 12] permutation. Keccak-p[1600, 12] has previously been used in fast parallel hashing algorithm KangarooTwelve ( more @ https://keccak.team/kangarootwelve.html ). Recently a formal specification, describing TurboSHAKE was released ( more @ https://ia.cr/2023/342 ) which generally exposes the underlying primitive of KangarooTwelve ( also known as **K12**, see https://blake12.org ) so that post-quantum public key cryptosystems ( such as ML-KEM, ML-DSA etc. - standardized by NIST ) might benefit from it ( more @ https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/5HveEPBsbxY ).

Here I'm maintaining a Rust library which implements TurboSHAKE{128, 256} xof s.t. one can absorb arbitrary many bytes into sponge state, finalize sponge and squeeze arbitrary many bytes out of sponge. See [usage](#usage) section below for more info.

## Prerequisites
Rust stable toolchain; see https://rustup.rs for installation guide.

```bash
# When developing this library, I was using
$ rustc --version
rustc 1.84.1 (e71f9a9a9 2025-01-27)
```

## Testing
For ensuring functional correctness of TurboSHAKE{128, 256} implementation, I use test vectors from section 4 ( on page 9 ) and Appendix A ( on page 17 ) of https://datatracker.ietf.org/doc/draft-irtf-cfrg-kangarootwelve. Issue following command to run all test cases

```bash
cargo test
```

## Benchmarking
Issue following command for benchmarking round-reduced Keccak-p[1600, 12] permutation and TurboSHAKE{128, 256} Xof, for variable input and output sizes.

> [!WARNING]
> When benchmarking make sure you've disabled CPU frequency scaling, otherwise numbers you see can be misleading. I found https://github.com/google/benchmark/blob/b40db869/docs/reducing_variance.md helpful.

```bash
cargo bench --bench keccak --features=dev --profile optimized # Only keccak permutation
cargo bench --bench turboshake --profile optimized            # Only TurboSHAKE{128, 256} Xof
cargo bench --all-features --profile optimized                # Both of above
```

### On *12th Gen Intel(R) Core(TM) i7-1260P*
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
turboshake = "0.4.1"
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

I maintain two examples demonstrating use of TurboSHAKE{128, 256} Xof API.

- [TurboSHAKE128](./examples/turboshake128.rs)
- [TurboSHAKE256](./examples/turboshake256.rs)

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

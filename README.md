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
╰─ permute_12_rounds  92.07 ns      │ 97.82 ns      │ 92.66 ns      │ 92.81 ns      │ 100     │ 3200
                      2.022 GiB/s   │ 1.904 GiB/s   │ 2.01 GiB/s    │ 2.006 GiB/s   │         │
                      10.86 Mitem/s │ 10.22 Mitem/s │ 10.79 Mitem/s │ 10.77 Mitem/s │         │

Timer precision: 19 ns
turboshake                      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ turboshake128                              │               │               │               │         │
│  ├─ msg = 2.0KB, md = 64.0B   1.618 µs      │ 5.18 µs       │ 1.652 µs      │ 1.701 µs      │ 100     │ 100
│  │                            1.215 GiB/s   │ 388.7 MiB/s   │ 1.19 GiB/s    │ 1.155 GiB/s   │         │
│  │                            617.8 Kitem/s │ 193 Kitem/s   │ 605.1 Kitem/s │ 587.5 Kitem/s │         │
│  ├─ msg = 8.0KB, md = 64.0B   5.084 µs      │ 12.19 µs      │ 5.137 µs      │ 5.559 µs      │ 100     │ 100
│  │                            1.512 GiB/s   │ 645.6 MiB/s   │ 1.496 GiB/s   │ 1.383 GiB/s   │         │
│  │                            196.6 Kitem/s │ 82 Kitem/s    │ 194.6 Kitem/s │ 179.8 Kitem/s │         │
│  ├─ msg = 32.0B, md = 64.0B   122.7 ns      │ 136.9 ns      │ 123.9 ns      │ 124.5 ns      │ 100     │ 1600
│  │                            745.9 MiB/s   │ 668.6 MiB/s   │ 738.8 MiB/s   │ 735 MiB/s     │         │
│  │                            8.147 Mitem/s │ 7.303 Mitem/s │ 8.069 Mitem/s │ 8.028 Mitem/s │         │
│  ├─ msg = 128.0B, md = 64.0B  125.1 ns      │ 257.3 ns      │ 127.4 ns      │ 130.6 ns      │ 100     │ 800
│  │                            1.429 GiB/s   │ 711.4 MiB/s   │ 1.403 GiB/s   │ 1.368 GiB/s   │         │
│  │                            7.993 Mitem/s │ 3.885 Mitem/s │ 7.848 Mitem/s │ 7.654 Mitem/s │         │
│  ╰─ msg = 512.0B, md = 64.0B  438.2 ns      │ 2.916 µs      │ 486.2 ns      │ 525 ns        │ 100     │ 800
│                               1.224 GiB/s   │ 188.3 MiB/s   │ 1.103 GiB/s   │ 1.021 GiB/s   │         │
│                               2.281 Mitem/s │ 342.8 Kitem/s │ 2.056 Mitem/s │ 1.904 Mitem/s │         │
╰─ turboshake256                              │               │               │               │         │
   ├─ msg = 2.0KB, md = 64.0B   1.637 µs      │ 5.427 µs      │ 1.665 µs      │ 1.736 µs      │ 100     │ 100
   │                            1.201 GiB/s   │ 371 MiB/s     │ 1.18 GiB/s    │ 1.132 GiB/s   │         │
   │                            610.6 Kitem/s │ 184.2 Kitem/s │ 600.3 Kitem/s │ 575.7 Kitem/s │         │
   ├─ msg = 8.0KB, md = 64.0B   6.103 µs      │ 10.97 µs      │ 6.164 µs      │ 6.529 µs      │ 100     │ 100
   │                            1.259 GiB/s   │ 717.2 MiB/s   │ 1.247 GiB/s   │ 1.177 GiB/s   │         │
   │                            163.8 Kitem/s │ 91.09 Kitem/s │ 162.2 Kitem/s │ 153.1 Kitem/s │         │
   ├─ msg = 32.0B, md = 64.0B   117.2 ns      │ 131.2 ns      │ 118.9 ns      │ 119.2 ns      │ 100     │ 1600
   │                            780.9 MiB/s   │ 697.6 MiB/s   │ 769.4 MiB/s   │ 767.8 MiB/s   │         │
   │                            8.53 Mitem/s  │ 7.62 Mitem/s  │ 8.404 Mitem/s │ 8.386 Mitem/s │         │
   ├─ msg = 128.0B, md = 64.0B  124 ns        │ 145.7 ns      │ 126.4 ns      │ 127.7 ns      │ 100     │ 1600
   │                            1.441 GiB/s   │ 1.227 GiB/s   │ 1.413 GiB/s   │ 1.399 GiB/s   │         │
   │                            8.061 Mitem/s │ 6.861 Mitem/s │ 7.906 Mitem/s │ 7.827 Mitem/s │         │
   ╰─ msg = 512.0B, md = 64.0B  432.1 ns      │ 497.8 ns      │ 449.4 ns      │ 450.2 ns      │ 100     │ 400
                                1.241 GiB/s   │ 1.077 GiB/s   │ 1.193 GiB/s   │ 1.191 GiB/s   │         │
                                2.314 Mitem/s │ 2.008 Mitem/s │ 2.224 Mitem/s │ 2.22 Mitem/s  │         │
```

## Usage
Using TurboSHAKE{128, 256} Xof API is fairly easy.

1) Add `turboshake` to your project's Cargo.toml.

```toml
[dependencies]
turboshake = "0.2.0"
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

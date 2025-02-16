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
Timer precision: 19 ns
keccak                fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ permute_12_rounds  92.88 ns      │ 100.1 ns      │ 97.21 ns      │ 96.48 ns      │ 100     │ 3200
                      2.005 GiB/s   │ 1.86 GiB/s    │ 1.916 GiB/s   │ 1.93 GiB/s    │         │
                      10.76 Mitem/s │ 9.989 Mitem/s │ 10.28 Mitem/s │ 10.36 Mitem/s │         │

Timer precision: 20 ns
turboshake                      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ turboshake128                              │               │               │               │         │
│  ├─ msg = 2.0KB, md = 64.0B   1.387 µs      │ 3.142 µs      │ 1.426 µs      │ 1.564 µs      │ 100     │ 100
│  │                            1.417 GiB/s   │ 640.9 MiB/s   │ 1.379 GiB/s   │ 1.257 GiB/s   │         │
│  │                            720.6 Kitem/s │ 318.2 Kitem/s │ 701.2 Kitem/s │ 639.1 Kitem/s │         │
│  ├─ msg = 8.0KB, md = 64.0B   5.068 µs      │ 7.756 µs      │ 6.32 µs       │ 6.29 µs       │ 100     │ 100
│  │                            1.516 GiB/s   │ 1015 MiB/s    │ 1.216 GiB/s   │ 1.222 GiB/s   │         │
│  │                            197.2 Kitem/s │ 128.9 Kitem/s │ 158.2 Kitem/s │ 158.9 Kitem/s │         │
│  ├─ msg = 32.0B, md = 64.0B   142.6 ns      │ 323.5 ns      │ 144.9 ns      │ 147.9 ns      │ 100     │ 1600
│  │                            641.7 MiB/s   │ 282.9 MiB/s   │ 631.4 MiB/s   │ 619 MiB/s     │         │
│  │                            7.009 Mitem/s │ 3.09 Mitem/s  │ 6.897 Mitem/s │ 6.761 Mitem/s │         │
│  ├─ msg = 128.0B, md = 64.0B  142.2 ns      │ 220.8 ns      │ 149.5 ns      │ 150.5 ns      │ 100     │ 1600
│  │                            1.256 GiB/s   │ 829 MiB/s     │ 1.195 GiB/s   │ 1.187 GiB/s   │         │
│  │                            7.027 Mitem/s │ 4.527 Mitem/s │ 6.687 Mitem/s │ 6.642 Mitem/s │         │
│  ╰─ msg = 512.0B, md = 64.0B  506.8 ns      │ 598.3 ns      │ 537.3 ns      │ 541.7 ns      │ 100     │ 400
│                               1.058 GiB/s   │ 918 MiB/s     │ 1022 MiB/s    │ 1013 MiB/s    │         │
│                               1.972 Mitem/s │ 1.671 Mitem/s │ 1.86 Mitem/s  │ 1.845 Mitem/s │         │
╰─ turboshake256                              │               │               │               │         │
   ├─ msg = 2.0KB, md = 64.0B   2.09 µs       │ 4.833 µs      │ 2.117 µs      │ 2.152 µs      │ 100     │ 100
   │                            963.4 MiB/s   │ 416.6 MiB/s   │ 951.3 MiB/s   │ 935.8 MiB/s   │         │
   │                            478.3 Kitem/s │ 206.8 Kitem/s │ 472.3 Kitem/s │ 464.6 Kitem/s │         │
   ├─ msg = 8.0KB, md = 64.0B   6.267 µs      │ 11.58 µs      │ 6.329 µs      │ 6.801 µs      │ 100     │ 100
   │                            1.226 GiB/s   │ 679.3 MiB/s   │ 1.214 GiB/s   │ 1.13 GiB/s    │         │
   │                            159.5 Kitem/s │ 86.28 Kitem/s │ 158 Kitem/s   │ 147 Kitem/s   │         │
   ├─ msg = 32.0B, md = 64.0B   114.9 ns      │ 456.7 ns      │ 142.4 ns      │ 148 ns        │ 100     │ 3200
   │                            796.2 MiB/s   │ 200.4 MiB/s   │ 642.9 MiB/s   │ 618.4 MiB/s   │         │
   │                            8.697 Mitem/s │ 2.189 Mitem/s │ 7.022 Mitem/s │ 6.755 Mitem/s │         │
   ├─ msg = 128.0B, md = 64.0B  119.2 ns      │ 198.9 ns      │ 134.1 ns      │ 131 ns        │ 100     │ 1600
   │                            1.499 GiB/s   │ 920.5 MiB/s   │ 1.332 GiB/s   │ 1.364 GiB/s   │         │
   │                            8.387 Mitem/s │ 5.027 Mitem/s │ 7.451 Mitem/s │ 7.633 Mitem/s │         │
   ╰─ msg = 512.0B, md = 64.0B  486.8 ns      │ 548.1 ns      │ 505.6 ns      │ 504.4 ns      │ 100     │ 400
                                1.101 GiB/s   │ 1002 MiB/s    │ 1.06 GiB/s    │ 1.063 GiB/s   │         │
                                2.053 Mitem/s │ 1.824 Mitem/s │ 1.977 Mitem/s │ 1.982 Mitem/s │         │
```

## Usage
Using TurboSHAKE{128, 256} Xof API is fairly easy.

1) Add `turboshake` to your project's Cargo.toml.

```toml
[dependencies]
turboshake = "0.3.0"
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

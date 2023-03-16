# turboshake
TurboSHAKE: A Family of e**X**tendable **O**utput **F**unctions based on round reduced Keccak[1600] Permutation

## Overview

TurboSHAKE is a family of extendable output functions (XOFs) powered by round-reduced ( i.e. 12 -rounds ) Keccak-p[1600, 12] permutation. Keccak-p[1600, 12] has previously been used in fast hashing algorithm KangarooTwelve ( more @ https://keccak.team/kangarootwelve.html ). Recently a formal specification, describing TurboSHAKE was released ( more @ https://ia.cr/2023/342 ) which generally exposes the underlying primitive of KangarooTwelve ( also known as **K12**, see https://blake12.org ) so that post-quantum public key cryptosystems ( such as Kyber, Dilithium etc. - being standardized by NIST ) benefit from it ( more @ https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/5HveEPBsbxY ).

Here I'm maintaining a Rust library which implements TurboSHAKE{128, 256} XOF s.t. one can absorb arbitrary many bytes into sponge state, finalize sponge and squeeze arbitrary many bytes out of sponge.

## Prerequisites

Rust stable toolchain; see https://rustup.rs for installation guide.

```bash
# When developing this library, I was using
$ rustc --version
rustc 1.68.0 (2c8cc3432 2023-03-06)
```

## Testing

For ensuring functional correctness of TurboSHAKE{128, 256} implementation, I use test vectors from section 4 ( on page 9 ) and Appendix A ( on page 17 ) of https://datatracker.ietf.org/doc/draft-irtf-cfrg-kangarootwelve. Issue following command to run test cases

```bash
cargo test --lib
```

## Benchmarking

Issue following command for benchmarking round-reduced Keccak-p[1600, 12] permutation and TurboSHAKE{128, 256} XOF ( for various input sizes ). Note, squeezed output size is kept constant at 32 -bytes.

```bash
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench
```

### On **Intel(R) Core(TM) i5-8279U CPU @ 2.40GHz**

```bash
keccak-p[1600, 12] (cached)
                        time:   [170.97 ns 171.45 ns 172.03 ns]
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe

keccak-p[1600, 12] (random)
                        time:   [186.23 ns 187.54 ns 188.88 ns]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe

     Running benches/turboshake.rs (target/release/deps/turboshake-ee40f9eb0651fba3)
turboshake128/32/32 (cached)
                        time:   [193.60 ns 194.11 ns 194.67 ns]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

turboshake128/32/32 (random)
                        time:   [226.03 ns 226.81 ns 227.69 ns]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

turboshake128/64/32 (cached)
                        time:   [194.15 ns 194.94 ns 195.99 ns]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

turboshake128/64/32 (random)
                        time:   [233.17 ns 234.13 ns 235.16 ns]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

turboshake128/128/32 (cached)
                        time:   [195.02 ns 195.69 ns 196.41 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

turboshake128/128/32 (random)
                        time:   [245.22 ns 246.58 ns 248.05 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

turboshake128/256/32 (cached)
                        time:   [371.44 ns 372.54 ns 373.78 ns]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

turboshake128/256/32 (random)
                        time:   [451.83 ns 454.02 ns 456.40 ns]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

turboshake128/512/32 (cached)
                        time:   [721.75 ns 724.00 ns 726.39 ns]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

turboshake128/512/32 (random)
                        time:   [897.06 ns 909.89 ns 923.12 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

turboshake128/1024/32 (cached)
                        time:   [1.2600 µs 1.2641 µs 1.2689 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

turboshake128/1024/32 (random)
                        time:   [1.3228 µs 1.3311 µs 1.3405 µs]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

turboshake128/2048/32 (cached)
                        time:   [2.3093 µs 2.3156 µs 2.3222 µs]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

turboshake128/2048/32 (random)
                        time:   [2.3835 µs 2.3930 µs 2.4032 µs]
Found 12 outliers among 100 measurements (12.00%)
  10 (10.00%) high mild
  2 (2.00%) high severe

turboshake128/4096/32 (cached)
                        time:   [4.4126 µs 4.4259 µs 4.4407 µs]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

turboshake128/4096/32 (random)
                        time:   [4.4995 µs 4.5224 µs 4.5482 µs]
Found 10 outliers among 100 measurements (10.00%)
  8 (8.00%) high mild
  2 (2.00%) high severe

turboshake256/32/32 (cached)
                        time:   [190.16 ns 190.61 ns 191.08 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

turboshake256/32/32 (random)
                        time:   [222.62 ns 223.58 ns 224.62 ns]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

turboshake256/64/32 (cached)
                        time:   [189.81 ns 190.33 ns 190.94 ns]
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) high mild
  2 (2.00%) high severe

turboshake256/64/32 (random)
                        time:   [228.10 ns 228.91 ns 229.78 ns]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

turboshake256/128/32 (cached)
                        time:   [191.65 ns 192.49 ns 193.46 ns]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe

turboshake256/128/32 (random)
                        time:   [239.42 ns 240.75 ns 242.17 ns]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

turboshake256/256/32 (cached)
                        time:   [364.12 ns 365.34 ns 366.67 ns]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

turboshake256/256/32 (random)
                        time:   [451.02 ns 453.37 ns 455.85 ns]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

turboshake256/512/32 (cached)
                        time:   [708.38 ns 712.26 ns 717.51 ns]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

turboshake256/512/32 (random)
                        time:   [873.98 ns 884.23 ns 894.00 ns]

turboshake256/1024/32 (cached)
                        time:   [1.3822 µs 1.3854 µs 1.3889 µs]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

turboshake256/1024/32 (random)
                        time:   [1.5439 µs 1.5562 µs 1.5697 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

turboshake256/2048/32 (cached)
                        time:   [2.8448 µs 2.8567 µs 2.8693 µs]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

turboshake256/2048/32 (random)
                        time:   [2.9912 µs 3.0048 µs 3.0190 µs]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

turboshake256/4096/32 (cached)
                        time:   [5.3825 µs 5.4103 µs 5.4438 µs]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

turboshake256/4096/32 (random)
                        time:   [5.5844 µs 5.6091 µs 5.6342 µs]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
```

## Usage

Using TurboSHAKE{128, 256} XOF API is fairly easy

1) Add `turboshake` to Cargo.toml, as your project dependency

```toml
[dependencies]
# either
turboshake = { git = "https://github.com/itzmeanjan/turboshake" }
# or
turboshake = "0.1.1"
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

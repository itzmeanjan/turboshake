# turboshake
TurboSHAKE: A Family of e**X**tendable **O**utput **F**unctions based on round reduced ( 12 rounds ) Keccak[1600] Permutation

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

Issue following command for benchmarking round-reduced Keccak-p[1600, 12] permutation and TurboSHAKE{128, 256} XOF ( for various input sizes ). Note, squeezed output size ( from the XOF ) is kept constant at 32 -bytes.

```bash
# if only interested in TurboSHAKE{128, 256} XOF
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench

# also benchmarks keccak-p[1600, 12]
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench --features dev
```

### On **Intel(R) Core(TM) i5-8279U CPU @ 2.40GHz**

```bash
keccak/keccak-p[1600, 12] (cached)
                        time:   [171.08 ns 171.72 ns 172.48 ns]
                        thrpt:  [1.0799 GiB/s 1.0847 GiB/s 1.0888 GiB/s]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe
keccak/keccak-p[1600, 12] (random)
                        time:   [188.03 ns 189.51 ns 191.10 ns]
                        thrpt:  [998.07 MiB/s 1006.5 MiB/s 1014.4 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

turboshake128/32/32 (cached)
                        time:   [194.03 ns 194.42 ns 194.84 ns]
                        thrpt:  [156.63 MiB/s 156.97 MiB/s 157.28 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe
turboshake128/32/32 (random)
                        time:   [227.81 ns 228.51 ns 229.27 ns]
                        thrpt:  [133.11 MiB/s 133.55 MiB/s 133.96 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

turboshake128/64/32 (cached)
                        time:   [194.75 ns 195.23 ns 195.72 ns]
                        thrpt:  [311.85 MiB/s 312.64 MiB/s 313.40 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) high mild
  2 (2.00%) high severe
turboshake128/64/32 (random)
                        time:   [234.68 ns 235.94 ns 237.31 ns]
                        thrpt:  [257.20 MiB/s 258.69 MiB/s 260.08 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

turboshake128/128/32 (cached)
                        time:   [195.89 ns 198.55 ns 202.60 ns]
                        thrpt:  [602.51 MiB/s 614.81 MiB/s 623.16 MiB/s]
Found 14 outliers among 100 measurements (14.00%)
  8 (8.00%) high mild
  6 (6.00%) high severe
turboshake128/128/32 (random)
                        time:   [247.96 ns 249.38 ns 250.89 ns]
                        thrpt:  [486.55 MiB/s 489.50 MiB/s 492.31 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

turboshake128/256/32 (cached)
                        time:   [371.50 ns 376.57 ns 383.07 ns]
                        thrpt:  [637.33 MiB/s 648.33 MiB/s 657.18 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe
turboshake128/256/32 (random)
                        time:   [459.31 ns 462.15 ns 464.75 ns]
                        thrpt:  [525.32 MiB/s 528.27 MiB/s 531.54 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

turboshake128/512/32 (cached)
                        time:   [723.39 ns 726.10 ns 729.41 ns]
                        thrpt:  [669.42 MiB/s 672.47 MiB/s 674.99 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
turboshake128/512/32 (random)
                        time:   [879.38 ns 890.55 ns 901.22 ns]
                        thrpt:  [541.80 MiB/s 548.29 MiB/s 555.26 MiB/s]

turboshake128/1024/32 (cached)
                        time:   [1.2460 µs 1.2493 µs 1.2530 µs]
                        thrpt:  [779.37 MiB/s 781.69 MiB/s 783.75 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
turboshake128/1024/32 (random)
                        time:   [1.3155 µs 1.3219 µs 1.3286 µs]
                        thrpt:  [735.03 MiB/s 738.78 MiB/s 742.33 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

turboshake128/2048/32 (cached)
                        time:   [2.3084 µs 2.3145 µs 2.3216 µs]
                        thrpt:  [841.30 MiB/s 843.86 MiB/s 846.08 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
turboshake128/2048/32 (random)
                        time:   [2.3903 µs 2.3993 µs 2.4087 µs]
                        thrpt:  [810.86 MiB/s 814.03 MiB/s 817.10 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

turboshake128/4096/32 (cached)
                        time:   [4.4181 µs 4.4307 µs 4.4442 µs]
                        thrpt:  [878.96 MiB/s 881.62 MiB/s 884.14 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
turboshake128/4096/32 (random)
                        time:   [4.6364 µs 4.6615 µs 4.6835 µs]
                        thrpt:  [834.05 MiB/s 837.99 MiB/s 842.52 MiB/s]

turboshake256/32/32 (cached)
                        time:   [190.63 ns 191.16 ns 191.75 ns]
                        thrpt:  [159.15 MiB/s 159.64 MiB/s 160.09 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
turboshake256/32/32 (random)
                        time:   [223.73 ns 225.53 ns 227.68 ns]
                        thrpt:  [134.04 MiB/s 135.31 MiB/s 136.40 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

turboshake256/64/32 (cached)
                        time:   [191.15 ns 191.64 ns 192.17 ns]
                        thrpt:  [317.61 MiB/s 318.49 MiB/s 319.31 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
turboshake256/64/32 (random)
                        time:   [228.44 ns 229.55 ns 230.79 ns]
                        thrpt:  [264.46 MiB/s 265.89 MiB/s 267.19 MiB/s]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

turboshake256/128/32 (cached)
                        time:   [190.90 ns 191.73 ns 192.76 ns]
                        thrpt:  [633.28 MiB/s 636.67 MiB/s 639.46 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
turboshake256/128/32 (random)
                        time:   [241.36 ns 246.30 ns 252.58 ns]
                        thrpt:  [483.29 MiB/s 495.61 MiB/s 505.75 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

turboshake256/256/32 (cached)
                        time:   [456.61 ns 500.62 ns 552.86 ns]
                        thrpt:  [441.60 MiB/s 487.68 MiB/s 534.68 MiB/s]
Found 18 outliers among 100 measurements (18.00%)
  6 (6.00%) high mild
  12 (12.00%) high severe
turboshake256/256/32 (random)
                        time:   [507.76 ns 549.49 ns 601.73 ns]
                        thrpt:  [405.73 MiB/s 444.30 MiB/s 480.82 MiB/s]
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) low mild
  2 (2.00%) high mild
  8 (8.00%) high severe

turboshake256/512/32 (cached)
                        time:   [716.74 ns 719.57 ns 722.80 ns]
                        thrpt:  [675.54 MiB/s 678.58 MiB/s 681.26 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  9 (9.00%) high mild
  2 (2.00%) high severe
turboshake256/512/32 (random)
                        time:   [884.19 ns 895.35 ns 906.16 ns]
                        thrpt:  [538.84 MiB/s 545.35 MiB/s 552.23 MiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

turboshake256/1024/32 (cached)
                        time:   [1.3902 µs 1.3993 µs 1.4105 µs]
                        thrpt:  [692.34 MiB/s 697.91 MiB/s 702.47 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
turboshake256/1024/32 (random)
                        time:   [1.4778 µs 1.4842 µs 1.4915 µs]
                        thrpt:  [654.75 MiB/s 657.97 MiB/s 660.82 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

turboshake256/2048/32 (cached)
                        time:   [2.7661 µs 2.7744 µs 2.7839 µs]
                        thrpt:  [701.57 MiB/s 703.97 MiB/s 706.09 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
turboshake256/2048/32 (random)
                        time:   [2.8918 µs 2.9032 µs 2.9153 µs]
                        thrpt:  [669.95 MiB/s 672.75 MiB/s 675.40 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  8 (8.00%) high mild
  1 (1.00%) high severe

turboshake256/4096/32 (cached)
                        time:   [5.3096 µs 5.3237 µs 5.3394 µs]
                        thrpt:  [731.59 MiB/s 733.74 MiB/s 735.69 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe
turboshake256/4096/32 (random)
                        time:   [5.5134 µs 5.5296 µs 5.5474 µs]
                        thrpt:  [704.16 MiB/s 706.43 MiB/s 708.50 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
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
turboshake = "0.1.4"
# or if interested in using underlying keccak-p[1600, 12] and sponge (developer) API
turboshake = { version = "0.1.4", features = "dev" }
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

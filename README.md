# turboshake
TurboSHAKE: A Family of e**X**tendable **O**utput **F**unctions based on round reduced ( 12 rounds ) Keccak[1600] Permutation

## Overview

TurboSHAKE is a family of extendable output functions (Xofs) powered by round-reduced ( i.e. 12 -rounds ) Keccak-p[1600, 12] permutation. Keccak-p[1600, 12] has previously been used in fast parallel hashing algorithm KangarooTwelve ( more @ https://keccak.team/kangarootwelve.html ). Recently a formal specification, describing TurboSHAKE was released ( more @ https://ia.cr/2023/342 ) which generally exposes the underlying primitive of KangarooTwelve ( also known as **K12**, see https://blake12.org ) so that post-quantum public key cryptosystems ( such as Kyber, Dilithium etc. - being standardized by NIST ) benefit from it ( more @ https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/5HveEPBsbxY ).

Here I'm maintaining a Rust library which implements TurboSHAKE{128, 256} Xof s.t. one can absorb arbitrary many bytes into sponge state, finalize sponge and squeeze arbitrary many bytes out of sponge. It also exposes ( not by default, controlled by Rust feature gate `"dev"` ) raw API for keccak-p[1600, 12] permutation and sponge operations i.e. absorption, finalization and squeezing. Other features ( such as `"simdx2"` or `"simdx4"` ) expose advanced Keccak-p[1600, 12] permutation implementation s.t. using {128, 256} -bit SIMD registers for parallelly applying 2 or 4 keccak permutations. See [usage](#usage) section below for more info on how to use these.

## Prerequisites

Rust nightly toolchain; see https://rustup.rs for installation guide.

> [!NOTE]
> Nightly toolchain is required because I use `portable_simd` feature ( more @ https://doc.rust-lang.org/std/simd/struct.Simd.html ) for SIMD implementation of Keccak-p[1600, 12] permutation. See [rust-toolchain](./rust-toolchain.toml) file for understanding how toolchain version is overridden in this crate.

```bash
# When developing this library, I was using
$ rustc --version
rustc 1.75.0-nightly (df871fbf0 2023-10-24)
```

I advise you to also use `cargo-criterion` for running benchmark executable. Read more about it @ https://crates.io/crates/cargo-criterion. You can just issue following command for installing it.

```bash
cargo install cargo-criterion
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

Issue following command for benchmarking round-reduced Keccak-p[1600, 12] permutation and TurboSHAKE{128, 256} Xof, for variable input and output sizes.

> [!NOTE]
> When benchmarking on `x86`, `x86_64`, `aarch64` or `loongarch64` targets, CPU cycles and cycles/ byte metrics are reported, while for other targets, default wallclock timer of criterion.rs is used for reporting time and throughput. I found https://github.com/pornin/crrl/blob/73b33c1efc73d637f3084d197353991a22c10366/benches/util.rs pretty useful for obtaining CPU cycles when benchmarking Rust functions. But I'm using criterion.rs as benchmark harness, hence I decided to go with https://crates.io/crates/criterion-cycles-per-byte plugin, much easier to integrate. But I had to patch it for my usecase and they live in the branch `add-memfence` of my fork of `criterion-cycles-per-byte` ( see my commits @ https://github.com/itzmeanjan/criterion-cycles-per-byte/commits/add-memfence ).

> [!NOTE]
> In case you're running benchmarks on aarch64 target, consider reading https://github.com/itzmeanjan/criterion-cycles-per-byte/blob/d2f5bf8638640962a9b301966dbb3e65fbc6f283/src/lib.rs#L63-L70.

> [!WARNING]
> When benchmarking make sure you've disabled CPU frequency scaling, otherwise numbers you see can be pretty misleading. I found https://github.com/google/benchmark/blob/b40db869/docs/reducing_variance.md helpful.

```bash
# In case you didn't install `cargo-criterion`, you've to execute benchmark with
# `$ RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench ...`

# When interested in TurboSHAKE{128, 256} Xof
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo criterion turboshake

# When interested in scalar Keccak-p[1600, 12] permutation
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo criterion keccak --features="dev"

# When interested in 2x SIMD parallel Keccak-p[1600, 12] permutation
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo criterion keccak --features="dev simdx2"

# When interested in 4x SIMD parallel Keccak-p[1600, 12] permutation
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo criterion keccak --features="dev simdx4"
```

### On *12th Gen Intel(R) Core(TM) i7-1260P*

#### TurboSHAKE{128, 256} Xof

```bash
turboshake128/32B msg/32B dig (cached)                                                                            
                        time:   [287.6238 cycles 288.3253 cycles 289.0466 cycles]
                        thrpt:  [4.5164 cpb 4.5051 cpb 4.4941 cpb]
turboshake128/32B msg/32B dig (random)                                                                            
                        time:   [339.0698 cycles 340.1734 cycles 341.5481 cycles]
                        thrpt:  [5.3367 cpb 5.3152 cpb 5.2980 cpb]

turboshake128/32B msg/64B dig (cached)                                                                            
                        time:   [285.6819 cycles 286.4859 cycles 287.3474 cycles]
                        thrpt:  [2.9932 cpb 2.9842 cpb 2.9759 cpb]
turboshake128/32B msg/64B dig (random)                                                                            
                        time:   [343.5620 cycles 344.8867 cycles 346.1286 cycles]
                        thrpt:  [3.6055 cpb 3.5926 cpb 3.5788 cpb]

turboshake128/128B msg/32B dig (cached)                                                                            
                        time:   [284.3077 cycles 284.7192 cycles 285.1794 cycles]
                        thrpt:  [1.7824 cpb 1.7795 cpb 1.7769 cpb]
turboshake128/128B msg/32B dig (random)                                                                            
                        time:   [357.6439 cycles 359.5727 cycles 361.5864 cycles]
                        thrpt:  [2.2599 cpb 2.2473 cpb 2.2353 cpb]

turboshake128/128B msg/64B dig (cached)                                                                            
                        time:   [290.3613 cycles 291.2368 cycles 292.0447 cycles]
                        thrpt:  [1.5211 cpb 1.5169 cpb 1.5123 cpb]
turboshake128/128B msg/64B dig (random)                                                                            
                        time:   [357.9329 cycles 359.2733 cycles 360.6333 cycles]
                        thrpt:  [1.8783 cpb 1.8712 cpb 1.8642 cpb]

turboshake128/512B msg/32B dig (cached)                                                                            
                        time:   [1002.5072 cycles 1003.9786 cycles 1005.4542 cycles]
                        thrpt:  [1.8483 cpb 1.8455 cpb 1.8428 cpb]
turboshake128/512B msg/32B dig (random)                                                                             
                        time:   [1079.6958 cycles 1082.5829 cycles 1085.5750 cycles]
                        thrpt:  [1.9955 cpb 1.9900 cpb 1.9847 cpb]

turboshake128/512B msg/64B dig (cached)                                                                            
                        time:   [1000.4494 cycles 1001.8184 cycles 1003.2174 cycles]
                        thrpt:  [1.7417 cpb 1.7393 cpb 1.7369 cpb]
turboshake128/512B msg/64B dig (random)                                                                             
                        time:   [1076.2309 cycles 1079.3641 cycles 1082.7812 cycles]
                        thrpt:  [1.8798 cpb 1.8739 cpb 1.8685 cpb]

turboshake128/2048B msg/32B dig (cached)                                                                             
                        time:   [3211.5467 cycles 3218.5923 cycles 3225.5207 cycles]
                        thrpt:  [1.5507 cpb 1.5474 cpb 1.5440 cpb]
turboshake128/2048B msg/32B dig (random)                                                                             
                        time:   [3409.4399 cycles 3416.4178 cycles 3423.1240 cycles]
                        thrpt:  [1.6457 cpb 1.6425 cpb 1.6392 cpb]

turboshake128/2048B msg/64B dig (cached)                                                                             
                        time:   [3186.7336 cycles 3192.7819 cycles 3198.6854 cycles]
                        thrpt:  [1.5145 cpb 1.5117 cpb 1.5089 cpb]
turboshake128/2048B msg/64B dig (random)                                                                             
                        time:   [3482.1904 cycles 3493.0162 cycles 3503.8394 cycles]
                        thrpt:  [1.6590 cpb 1.6539 cpb 1.6488 cpb]

turboshake128/8192B msg/32B dig (cached)                                                                             
                        time:   [11974.2498 cycles 12005.6076 cycles 12040.8473 cycles]
                        thrpt:  [1.4641 cpb 1.4598 cpb 1.4560 cpb]
turboshake128/8192B msg/32B dig (random)                                                                             
                        time:   [12355.9025 cycles 12378.5491 cycles 12400.9973 cycles]
                        thrpt:  [1.5079 cpb 1.5052 cpb 1.5024 cpb]

turboshake128/8192B msg/64B dig (cached)                                                                             
                        time:   [12106.7616 cycles 12160.2407 cycles 12225.5836 cycles]
                        thrpt:  [1.4808 cpb 1.4729 cpb 1.4664 cpb]
turboshake128/8192B msg/64B dig (random)                                                                             
                        time:   [12588.6335 cycles 12632.5005 cycles 12675.2836 cycles]
                        thrpt:  [1.5353 cpb 1.5301 cpb 1.5248 cpb]

turboshake256/32B msg/32B dig (cached)                                                                            
                        time:   [271.7565 cycles 271.9105 cycles 272.0642 cycles]
                        thrpt:  [4.2510 cpb 4.2486 cpb 4.2462 cpb]
turboshake256/32B msg/32B dig (random)                                                                            
                        time:   [330.3118 cycles 330.8677 cycles 331.3458 cycles]
                        thrpt:  [5.1773 cpb 5.1698 cpb 5.1611 cpb]

turboshake256/32B msg/64B dig (cached)                                                                            
                        time:   [281.9165 cycles 282.3238 cycles 282.7096 cycles]
                        thrpt:  [2.9449 cpb 2.9409 cpb 2.9366 cpb]
turboshake256/32B msg/64B dig (random)                                                                            
                        time:   [341.0349 cycles 342.2477 cycles 343.3680 cycles]
                        thrpt:  [3.5768 cpb 3.5651 cpb 3.5524 cpb]

turboshake256/128B msg/32B dig (cached)                                                                            
                        time:   [272.5409 cycles 272.9458 cycles 273.4958 cycles]
                        thrpt:  [1.7093 cpb 1.7059 cpb 1.7034 cpb]
turboshake256/128B msg/32B dig (random)                                                                            
                        time:   [349.6556 cycles 352.3262 cycles 355.0041 cycles]
                        thrpt:  [2.2188 cpb 2.2020 cpb 2.1853 cpb]

turboshake256/128B msg/64B dig (cached)                                                                            
                        time:   [272.7317 cycles 273.0902 cycles 273.5082 cycles]
                        thrpt:  [1.4245 cpb 1.4223 cpb 1.4205 cpb]
turboshake256/128B msg/64B dig (random)                                                                            
                        time:   [351.3268 cycles 352.6258 cycles 353.8854 cycles]
                        thrpt:  [1.8432 cpb 1.8366 cpb 1.8298 cpb]

turboshake256/512B msg/32B dig (cached)                                                                            
                        time:   [1044.6703 cycles 1046.3649 cycles 1048.0302 cycles]
                        thrpt:  [1.9265 cpb 1.9235 cpb 1.9203 cpb]
turboshake256/512B msg/32B dig (random)                                                                             
                        time:   [1076.1872 cycles 1080.1144 cycles 1084.0237 cycles]
                        thrpt:  [1.9927 cpb 1.9855 cpb 1.9783 cpb]

turboshake256/512B msg/64B dig (cached)                                                                            
                        time:   [1011.1605 cycles 1013.7759 cycles 1016.4621 cycles]
                        thrpt:  [1.7647 cpb 1.7600 cpb 1.7555 cpb]
turboshake256/512B msg/64B dig (random)                                                                             
                        time:   [1071.2234 cycles 1073.0961 cycles 1074.9256 cycles]
                        thrpt:  [1.8662 cpb 1.8630 cpb 1.8598 cpb]

turboshake256/2048B msg/32B dig (cached)                                                                             
                        time:   [3994.8454 cycles 4004.0602 cycles 4013.4218 cycles]
                        thrpt:  [1.9295 cpb 1.9250 cpb 1.9206 cpb]
turboshake256/2048B msg/32B dig (random)                                                                             
                        time:   [4110.2656 cycles 4117.1706 cycles 4124.3807 cycles]
                        thrpt:  [1.9829 cpb 1.9794 cpb 1.9761 cpb]

turboshake256/2048B msg/64B dig (cached)                                                                             
                        time:   [3969.6332 cycles 3977.0682 cycles 3984.4061 cycles]
                        thrpt:  [1.8866 cpb 1.8831 cpb 1.8796 cpb]
turboshake256/2048B msg/64B dig (random)                                                                             
                        time:   [4098.6050 cycles 4107.0759 cycles 4116.6100 cycles]
                        thrpt:  [1.9492 cpb 1.9446 cpb 1.9406 cpb]

turboshake256/8192B msg/32B dig (cached)                                                                             
                        time:   [15140.6760 cycles 15174.1363 cycles 15207.9372 cycles]
                        thrpt:  [1.8492 cpb 1.8451 cpb 1.8410 cpb]
turboshake256/8192B msg/32B dig (random)                                                                             
                        time:   [15119.0164 cycles 15147.3455 cycles 15177.0955 cycles]
                        thrpt:  [1.8455 cpb 1.8418 cpb 1.8384 cpb]

turboshake256/8192B msg/64B dig (cached)                                                                             
                        time:   [15145.8790 cycles 15191.8807 cycles 15247.4533 cycles]
                        thrpt:  [1.8468 cpb 1.8401 cpb 1.8345 cpb]
turboshake256/8192B msg/64B dig (random)                                                                             
                        time:   [15137.1426 cycles 15167.8781 cycles 15199.0904 cycles]
                        thrpt:  [1.8410 cpb 1.8372 cpb 1.8335 cpb]
```

#### Scalar Keccak-p[1600, 12] Permutation

```bash
keccak/keccak-p[1600, 12] (cached)                                                                            
                        time:   [241.6094 cycles 241.9362 cycles 242.2867 cycles]
                        thrpt:  [1.2114 cpb 1.2097 cpb 1.2080 cpb]
keccak/keccak-p[1600, 12] (random)                                                                            
                        time:   [257.8638 cycles 258.7075 cycles 259.5606 cycles]
                        thrpt:  [1.2978 cpb 1.2935 cpb 1.2893 cpb]
```


#### 2x SIMD parallel Keccak-p[1600, 12] Permutation

```bash
keccak/keccak-p[1600, 12] x2 (cached)                                                                            
                        time:   [453.5327 cycles 453.6882 cycles 453.8566 cycles]
                        thrpt:  [1.1346 cpb 1.1342 cpb 1.1338 cpb]
keccak/keccak-p[1600, 12] x2 (random)                                                                            
                        time:   [485.1290 cycles 485.7451 cycles 486.3307 cycles]
                        thrpt:  [1.2158 cpb 1.2144 cpb 1.2128 cpb]
```

#### 4x SIMD parallel Keccak-p[1600, 12] Permutation

```bash
keccak/keccak-p[1600, 12] x4 (cached)                                                                            
                        time:   [782.1605 cycles 782.3983 cycles 782.6835 cycles]
                        thrpt:  [0.9784 cpb 0.9780 cpb 0.9777 cpb]
keccak/keccak-p[1600, 12] x4 (random)                                                                             
                        time:   [764.8180 cycles 766.6393 cycles 768.4535 cycles]
                        thrpt:  [0.9606 cpb 0.9583 cpb 0.9560 cpb]
```

### On *ARM Cortex-A72 (Raspberry Pi 4B)*

#### TurboSHAKE{128, 256} Xof

```bash
turboshake128/32B msg/32B dig (cached)                                                                             
                        time:   [1036.5341 cycles 1036.6166 cycles 1036.7102 cycles]
                        thrpt:  [16.1986 cpb 16.1971 cpb 16.1958 cpb]
turboshake128/32B msg/32B dig (random)                                                                             
                        time:   [1215.2910 cycles 1216.7328 cycles 1217.9627 cycles]
                        thrpt:  [19.0307 cpb 19.0114 cpb 18.9889 cpb]

turboshake128/32B msg/64B dig (cached)                                                                             
                        time:   [1040.4157 cycles 1040.5093 cycles 1040.6153 cycles]
                        thrpt:  [10.8397 cpb 10.8386 cpb 10.8377 cpb]
turboshake128/32B msg/64B dig (random)                                                                             
                        time:   [1289.1261 cycles 1301.3464 cycles 1312.2139 cycles]
                        thrpt:  [13.6689 cpb 13.5557 cpb 13.4284 cpb]

turboshake128/128B msg/32B dig (cached)                                                                             
                        time:   [1054.7642 cycles 1055.4314 cycles 1056.5595 cycles]
                        thrpt:  [6.6035 cpb 6.5964 cpb 6.5923 cpb]
turboshake128/128B msg/32B dig (random)                                                                             
                        time:   [1292.3021 cycles 1297.0090 cycles 1301.2525 cycles]
                        thrpt:  [8.1328 cpb 8.1063 cpb 8.0769 cpb]

turboshake128/128B msg/64B dig (cached)                                                                             
                        time:   [1059.4935 cycles 1059.5726 cycles 1059.6595 cycles]
                        thrpt:  [5.5191 cpb 5.5186 cpb 5.5182 cpb]
turboshake128/128B msg/64B dig (random)                                                                             
                        time:   [1326.6998 cycles 1334.2239 cycles 1340.5168 cycles]
                        thrpt:  [6.9819 cpb 6.9491 cpb 6.9099 cpb]

turboshake128/512B msg/32B dig (cached)                                                                             
                        time:   [3884.7425 cycles 3886.0097 cycles 3887.4237 cycles]
                        thrpt:  [7.1460 cpb 7.1434 cpb 7.1411 cpb]
turboshake128/512B msg/32B dig (random)                                                                             
                        time:   [4154.9634 cycles 4164.1096 cycles 4172.1637 cycles]
                        thrpt:  [7.6694 cpb 7.6546 cpb 7.6378 cpb]

turboshake128/512B msg/64B dig (cached)                                                                             
                        time:   [3893.3238 cycles 3894.3840 cycles 3895.5201 cycles]
                        thrpt:  [6.7631 cpb 6.7611 cpb 6.7592 cpb]
turboshake128/512B msg/64B dig (random)                                                                             
                        time:   [4180.7177 cycles 4190.1143 cycles 4198.3058 cycles]
                        thrpt:  [7.2887 cpb 7.2745 cpb 7.2582 cpb]

turboshake128/2048B msg/32B dig (cached)                                                                             
                        time:   [12497.4794 cycles 12500.0932 cycles 12502.7086 cycles]
                        thrpt:  [6.0109 cpb 6.0097 cpb 6.0084 cpb]
turboshake128/2048B msg/32B dig (random)                                                                             
                        time:   [13151.8455 cycles 13164.9347 cycles 13176.3853 cycles]
                        thrpt:  [6.3348 cpb 6.3293 cpb 6.3230 cpb]

turboshake128/2048B msg/64B dig (cached)                                                                             
                        time:   [12508.4730 cycles 12511.5676 cycles 12514.7700 cycles]
                        thrpt:  [5.9256 cpb 5.9240 cpb 5.9226 cpb]
turboshake128/2048B msg/64B dig (random)                                                                             
                        time:   [13189.5480 cycles 13203.1440 cycles 13214.1344 cycles]
                        thrpt:  [6.2567 cpb 6.2515 cpb 6.2451 cpb]

turboshake128/8192B msg/32B dig (cached)                                                                             
                        time:   [46858.3816 cycles 46865.4126 cycles 46872.6442 cycles]
                        thrpt:  [5.6995 cpb 5.6986 cpb 5.6978 cpb]
turboshake128/8192B msg/32B dig (random)                                                                             
                        time:   [47212.4290 cycles 47244.3228 cycles 47271.7703 cycles]
                        thrpt:  [5.7480 cpb 5.7447 cpb 5.7408 cpb]

turboshake128/8192B msg/64B dig (cached)                                                                             
                        time:   [46820.9954 cycles 46827.4208 cycles 46835.1945 cycles]
                        thrpt:  [5.6729 cpb 5.6719 cpb 5.6711 cpb]
turboshake128/8192B msg/64B dig (random)                                                                             
                        time:   [47069.9353 cycles 47093.0886 cycles 47115.1400 cycles]
                        thrpt:  [5.7068 cpb 5.7041 cpb 5.7013 cpb]

turboshake256/32B msg/32B dig (cached)                                                                             
                        time:   [1017.4095 cycles 1017.4914 cycles 1017.5844 cycles]
                        thrpt:  [15.8998 cpb 15.8983 cpb 15.8970 cpb]
turboshake256/32B msg/32B dig (random)                                                                             
                        time:   [1286.0145 cycles 1287.5749 cycles 1288.9537 cycles]
                        thrpt:  [20.1399 cpb 20.1184 cpb 20.0940 cpb]

turboshake256/32B msg/64B dig (cached)                                                                             
                        time:   [1025.3474 cycles 1025.4295 cycles 1025.5231 cycles]
                        thrpt:  [10.6825 cpb 10.6816 cpb 10.6807 cpb]
turboshake256/32B msg/64B dig (random)                                                                             
                        time:   [1365.6120 cycles 1379.0504 cycles 1391.1320 cycles]
                        thrpt:  [14.4910 cpb 14.3651 cpb 14.2251 cpb]

turboshake256/128B msg/32B dig (cached)                                                                             
                        time:   [1034.4059 cycles 1034.4867 cycles 1034.5856 cycles]
                        thrpt:  [6.4662 cpb 6.4655 cpb 6.4650 cpb]
turboshake256/128B msg/32B dig (random)                                                                             
                        time:   [1356.9424 cycles 1360.8001 cycles 1364.1026 cycles]
                        thrpt:  [8.5256 cpb 8.5050 cpb 8.4809 cpb]

turboshake256/128B msg/64B dig (cached)                                                                             
                        time:   [1042.3949 cycles 1042.4881 cycles 1042.5888 cycles]
                        thrpt:  [5.4301 cpb 5.4296 cpb 5.4291 cpb]
turboshake256/128B msg/64B dig (random)                                                                             
                        time:   [1402.2166 cycles 1409.0123 cycles 1414.6053 cycles]
                        thrpt:  [7.3677 cpb 7.3386 cpb 7.3032 cpb]

turboshake256/512B msg/32B dig (cached)                                                                             
                        time:   [3792.2332 cycles 3792.5436 cycles 3792.8928 cycles]
                        thrpt:  [6.9722 cpb 6.9716 cpb 6.9710 cpb]
turboshake256/512B msg/32B dig (random)                                                                             
                        time:   [4289.6772 cycles 4298.9350 cycles 4306.8622 cycles]
                        thrpt:  [7.9170 cpb 7.9025 cpb 7.8854 cpb]

turboshake256/512B msg/64B dig (cached)                                                                             
                        time:   [3797.9211 cycles 3800.9033 cycles 3804.9855 cycles]
                        thrpt:  [6.6059 cpb 6.5988 cpb 6.5936 cpb]
turboshake256/512B msg/64B dig (random)                                                                             
                        time:   [4311.9172 cycles 4324.7274 cycles 4335.3485 cycles]
                        thrpt:  [7.5266 cpb 7.5082 cpb 7.4860 cpb]

turboshake256/2048B msg/32B dig (cached)                                                                             
                        time:   [14811.9438 cycles 14813.8520 cycles 14816.2478 cycles]
                        thrpt:  [7.1232 cpb 7.1220 cpb 7.1211 cpb]
turboshake256/2048B msg/32B dig (random)                                                                             
                        time:   [15979.2734 cycles 15997.1044 cycles 16011.8727 cycles]
                        thrpt:  [7.6980 cpb 7.6909 cpb 7.6823 cpb]

turboshake256/2048B msg/64B dig (cached)                                                                             
                        time:   [14814.8523 cycles 14816.0265 cycles 14817.5088 cycles]
                        thrpt:  [7.0159 cpb 7.0152 cpb 7.0146 cpb]
turboshake256/2048B msg/64B dig (random)                                                                             
                        time:   [15988.7011 cycles 16008.5894 cycles 16024.6673 cycles]
                        thrpt:  [7.5874 cpb 7.5798 cpb 7.5704 cpb]

turboshake256/8192B msg/32B dig (cached)                                                                             
                        time:   [56143.6860 cycles 56149.5931 cycles 56156.9430 cycles]
                        thrpt:  [6.8284 cpb 6.8275 cpb 6.8268 cpb]
turboshake256/8192B msg/32B dig (random)                                                                             
                        time:   [58157.4743 cycles 58193.3756 cycles 58222.1508 cycles]
                        thrpt:  [7.0795 cpb 7.0760 cpb 7.0717 cpb]

turboshake256/8192B msg/64B dig (cached)                                                                             
                        time:   [56147.7141 cycles 56151.4327 cycles 56155.7480 cycles]
                        thrpt:  [6.8018 cpb 6.8013 cpb 6.8008 cpb]
turboshake256/8192B msg/64B dig (random)                                                                             
                        time:   [58011.4607 cycles 58044.7578 cycles 58072.0053 cycles]
                        thrpt:  [7.0339 cpb 7.0306 cpb 7.0266 cpb]
```

#### Scalar Keccak-p[1600, 12] Permutation

```bash
keccak/keccak-p[1600, 12] (cached)                                                                             
                        time:   [918.2161 cycles 918.2905 cycles 918.3706 cycles]
                        thrpt:  [4.5919 cpb 4.5915 cpb 4.5911 cpb]
keccak/keccak-p[1600, 12] (random)                                                                             
                        time:   [978.7095 cycles 979.0918 cycles 979.5673 cycles]
                        thrpt:  [4.8978 cpb 4.8955 cpb 4.8935 cpb]
```


#### 2x SIMD parallel Keccak-p[1600, 12] Permutation

```bash
keccak/keccak-p[1600, 12] x2 (cached)                                                                             
                        time:   [2054.4389 cycles 2054.6256 cycles 2054.8473 cycles]
                        thrpt:  [5.1371 cpb 5.1366 cpb 5.1361 cpb]
keccak/keccak-p[1600, 12] x2 (random)                                                                             
                        time:   [2272.4929 cycles 2273.2588 cycles 2274.0048 cycles]
                        thrpt:  [5.6850 cpb 5.6831 cpb 5.6812 cpb]
```

#### 4x SIMD parallel Keccak-p[1600, 12] Permutation

```bash
keccak/keccak-p[1600, 12] x4 (cached)                                                                             
                        time:   [5284.3498 cycles 5284.7727 cycles 5285.2543 cycles]
                        thrpt:  [6.6066 cpb 6.6060 cpb 6.6054 cpb]
keccak/keccak-p[1600, 12] x4 (random)                                                                             
                        time:   [5485.3232 cycles 5487.2797 cycles 5489.3845 cycles]
                        thrpt:  [6.8617 cpb 6.8591 cpb 6.8567 cpb]
```

## Usage

Using TurboSHAKE{128, 256} Xof API is fairly easy

1) Add `turboshake` to Cargo.toml, with proper ( or may be none if you're only using it for TurboSHAKE Xof ) feature flags ( based on your intended use case ), as your project dependency

```toml
[dependencies]
# If only interested in using TurboSHAKE{128, 256} Xof API, do
# either
turboshake = { git = "https://github.com/itzmeanjan/turboshake" }
# or
turboshake = "0.2.0"

# If interested in using underlying keccak-p[1600, 12] permutation and sponge (developer) API
turboshake = { version = "0.2.0", features = "dev" }
# or if interested in using underlying 2x SIMD parallel keccak-p[1600, 12] permutation API
turboshake = { version = "0.2.0", features = ["dev", "simdx2"] }
# or if interested in using underlying 4x SIMD parallel keccak-p[1600, 12] permutation API
turboshake = { version = "0.2.0", features = ["dev", "simdx4"] }
```

2) Create a TurboSHAKE{128, 256} Xof object.

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

I maintain two examples demonstrating use of TurboSHAKE{128, 256} Xof API.

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

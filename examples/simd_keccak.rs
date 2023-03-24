use turboshake::keccak;

fn main() {
    let mut state0 = [1u64; 25];
    let mut state1 = [2u64; 25];
    let mut state2 = [3u64; 25];
    let mut state3 = [4u64; 25];

    // Apply keccak-p[1600, 12] permutation on four states using
    // 256 -bit SIMD registers.
    keccak::permutex4(&mut state0, &mut state1, &mut state2, &mut state3);

    println!("[after] state0 = {:?}", state0);
    println!("[after] state1 = {:?}", state1);
    println!("[after] state2 = {:?}", state2);
    println!("[after] state3 = {:?}", state3);
}

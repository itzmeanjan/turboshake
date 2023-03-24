use turboshake::keccak;

fn main() {
    let mut state = [1u64; 25];

    // Apply keccak-p[1600, 12] permutation
    keccak::permute(&mut state);

    println!("[after] state = {:?}", state);
}

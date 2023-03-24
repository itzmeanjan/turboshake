use turboshake::keccak;

fn main() {
    let mut state = [1u64; 25];

    println!("[before] state = {:?}", state);
    keccak::permute(&mut state);
    println!("[after] state = {:?}", state);
}

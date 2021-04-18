use getrandom::getrandom;
use blake2b_simd::{blake2b, Hash};

fn main() {
    let mut state = [0u8; 128];
    let mut hash: Hash = blake2b(b"");
    for _ in 1..1_000_000 {
        let mut buf = [0u8; 64];
        getrandom::getrandom(&mut buf).expect("Random source not available");
        state[0..64].copy_from_slice(&buf);

        hash = blake2b(&state);
        state[64..128].copy_from_slice(&hash.as_bytes());
    }
    println!("{}", hash.to_hex());
}

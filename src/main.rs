use getrandom::getrandom;
use blake2b_simd::{blake2b, Params, Hash};

const N_ITER: u32 = 100_000;
const CHARS: &str = "abcdefghijklmnopqrstuvwxyz01234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()?";

fn main() {
    let mut args = std::env::args();

    match args.len() {
        1 => println!("0x{}", get_entropy().to_hex()),
        2 => println!("{}", get_password(32)),
        3 => println!("{}", get_password(args.nth(2).unwrap().parse::<usize>().unwrap())),
        _ => println!("Usage:\n srand\n srand pw\n srand pw 50")
    }
}

fn get_entropy() -> Hash {
    let mut state = [0u8; 128];
    for _ in 1..N_ITER {
        let mut buf = [0u8; 64];
        getrandom(&mut buf).expect("Random source not available");
        state[0..64].copy_from_slice(&buf);
        let hash = blake2b(&state);
        state[64..128].copy_from_slice(&hash.as_bytes());
    }
    let final_hash = Params::new()
        .hash_length(32)
        .hash(&state);
    final_hash
}

fn get_password(length: usize) -> String {
    use rand::{SeedableRng};
    use rand::rngs::SmallRng;
    use rand::seq::SliceRandom;

    let mut seed = [0u8; 32];
    seed[..].copy_from_slice(get_entropy().as_bytes());
    let mut rng = SmallRng::from_seed(seed);

    let chars = CHARS.to_string().into_bytes();
    let bytes: Vec<u8> = chars.choose_multiple(&mut rng, length).cloned().collect();
    let password = String::from_utf8(bytes).unwrap();
    password
}

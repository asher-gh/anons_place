use rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

fn main() {
    let (sk, pk) = gen_keypair();
    dbg!(sk, pk);
}

fn gen_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    secp.generate_keypair(&mut OsRng)
}

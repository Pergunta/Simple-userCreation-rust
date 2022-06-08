extern crate crypto;

use crypto::digest::Digest;
use crypto::sha3::Sha3;
use rand::{distributions::Alphanumeric, Rng};

pub fn hash_salt_password(password: &String) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    let mut hasher = Sha3::sha3_256();

    hasher.input_str(&(s.clone()+password));
    s+"$"+&hasher.result_str().to_string()
}
pub fn hash_verify_password(password: &String, salted_hash: &String) -> bool {
    let vec:Vec<&str> = salted_hash.split("$").collect();
    let salt = (&vec[0]).to_string();
    let true_hash = &vec[1];

    let mut hasher = Sha3::sha3_256();
    hasher.input_str(&(salt+password));

    true_hash.eq(&hasher.result_str())
}
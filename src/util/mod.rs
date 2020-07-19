pub mod cli;
pub mod debug_middleware;
pub mod email;
pub mod env;
pub mod errors;
pub mod github_information;
pub mod path;
pub mod postgresql_pool;
pub mod redis_pool;
pub mod result;
pub use self::redis_pool::RedisPool;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fmt::Write;
use tiny_keccak::{Hasher, Sha3};

/// Get random value
#[inline]
pub fn random_string(limit: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(limit)
        .collect()
}

/// Convert text to `sha3_256` hex
#[inline]
pub fn sha3_256_encode(s: String) -> String {
    let mut sha3 = Sha3::v256();
    sha3.update(s.as_ref());
    let mut res: [u8; 32] = [0; 32];
    sha3.finalize(&mut res);
    let mut hex = String::with_capacity(64);
    for byte in res.iter() {
        write!(hex, "{:02x}", byte).expect("Fail on writing to string");
    }
    hex
}

/// Get the real password, the first six is a random number
#[inline]
pub fn get_password(raw: &str) -> String {
    let (_, password) = raw.split_at(6);
    password.to_string()
}

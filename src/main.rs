extern crate rand;

use crate::hasher::Hasher;

mod hasher {
    use argon2::{self, Config as ArgonConfig};
    use rand::{distributions::Alphanumeric, Rng}; // 0.8

    pub struct Hasher<'a> {
        argon_config : ArgonConfig<'a>
    }

    pub fn salt_gen(salt_len: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(salt_len)
            .map(char::from)
            .collect()
    }

    impl Hasher<'_> {
        pub fn create() -> Self {
            Self {
                argon_config: ArgonConfig::default()
            }
        }

        pub fn hash(&self, password : String) -> String {
            return argon2::hash_encoded(password.as_ref(), salt_gen(8).as_ref(), &self.argon_config).unwrap();
        }

        pub fn check(&self, hash: String, password : String) -> bool {
            return argon2::verify_encoded(&hash, password.as_ref()).unwrap();
        }
    }
}

fn main() {
}

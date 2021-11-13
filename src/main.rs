extern crate rand;

use crate::hasher::Hasher;

mod hasher {
    use argon2::{self, Config as ArgonConfig};
    use rand::{distributions::Alphanumeric, Rng}; // 0.8

    pub struct Hasher<'a> {
        argon_config : ArgonConfig<'a>
    }

    impl Hasher<'_> {
        pub fn create() -> Self {
            Self {
                argon_config: ArgonConfig::default()
            }
        }

        fn salt_gen(&self, salt_len: usize) -> String {
            let s: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(salt_len)
                .map(char::from)
                .collect();
            return s
        }

        pub fn hash(&self, password : String) -> String {
            let salt = b"randomsalt";
            return argon2::hash_encoded(password.as_ref(), salt, &self.argon_config).unwrap();
        }

        pub fn check(&self, hash: String, password : String) -> bool {
            return argon2::verify_encoded(&hash, password.as_ref()).unwrap();
        }
    }
}

fn main() {

}

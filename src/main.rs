mod hasher {
    use argon2::{self, Config as ArgonConfig};

    pub struct Hasher<'a> {
        argon_config : ArgonConfig<'a>
    }

    impl Hasher {
        pub fn create() -> Self {
            Self {
                argon_config: ArgonConfig::default()
            }
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

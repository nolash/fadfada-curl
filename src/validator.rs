use std::collections::HashMap;

use fadafada::source::Engine;
use fadafada::validator::Validator;

pub struct ValidatorCollection {
    default: Option<Engine>,
    validator: HashMap<Engine, Box<dyn Validator>>
}

impl ValidatorCollection {
    pub fn new() -> ValidatorCollection {
        ValidatorCollection {
            default: None,
            validator: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use fadafada::validator::Validator;
    use fadafada::web2::Sha256ImmutableValidator;
    use hex;

    use env_logger;

    const HASH_OF_FOO: &str = "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae";

    #[test]
    fn test_validator_sha256_as_default() {
        env_logger::init();

        let content = vec![0x66, 0x6f, 0x6f];
        let content_digest = hex::decode(HASH_OF_FOO).unwrap();
       
        let sha256_validator = Sha256ImmutableValidator{};
        let r = sha256_validator.verify(&content_digest, Some(&content), None);
        assert!(r);
    }
}

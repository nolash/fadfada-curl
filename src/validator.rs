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
    use fadafada::validator::{
        Sha256ImmutableValidator,
        Validator,
    };
    use sha2::{Sha256, Digest};
    use env_logger;

    #[test]
    fn test_validator_sha256_as_default() {
        env_logger::init();

        let mut h = Sha256::new();
        let content = vec![0x66, 0x6f, 0x6f];
        h.update(&content);
        let content_digest: Vec<u8> = h.finalize().to_vec();
       
        let sha256_validator = Sha256ImmutableValidator{};
        let r = sha256_validator.verify(&content_digest, Some(&content), None);
    }
}

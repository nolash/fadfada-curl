use log::info;

use std::path;
use std::collections::HashMap;

use hex;

use fadfada::source::Engine;
use fadfada::validator::Validator;

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

    pub fn insert(&mut self, engine: Engine, validator: Box<dyn Validator>) {
        match self.validator.get(&engine) {
            Some(_) => {
                panic!("validator for engine {} already exists", engine);
            },
            _ => {},
        }
        match self.default {
            None => {
                self.default = Some(engine.clone());
            },
            _ => {},
        }
        self.validator.insert(engine.clone(), validator);
        info!("added validator for engine {}", engine);
    }

    pub fn get(&self, engine: &Engine) -> &dyn Validator {
        let v = self.validator.get(engine).unwrap(); 
        return v.as_ref();
    }

    pub fn verify_by_pointer(&self, engine: &Engine, pointer: &String, content: &Vec<u8>) -> bool {
        let validator = self.get(engine);
        let pointer_bytes = hex::decode(pointer).unwrap();
        let content_bytes = content.to_vec();

        return validator.verify(&pointer_bytes, Some(&content_bytes), None);
    }
}

#[cfg(test)]
mod tests {
    use hex;
    use env_logger;

    use fadfada::validator::Validator;
    use fadfada::web2::Sha256ImmutableValidator;

    use super::ValidatorCollection;

    const HASH_OF_FOO: &str = "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae";

    #[test]
    fn test_validator_sha256_as_default() {
        env_logger::init();

        let content = vec![0x66, 0x6f, 0x6f];
        let content_digest = hex::decode(HASH_OF_FOO).unwrap();
       
        let sha256_validator = Sha256ImmutableValidator{};
    
        let engine = "sha256".to_string();
        let mut collection = ValidatorCollection::new();
        collection.insert(engine.clone(), Box::new(sha256_validator));

        let sha256_validator_retrieved = collection.get(&engine);
        let r = sha256_validator_retrieved.verify(&content_digest, Some(&content), None);

        assert!(r);
    }
}

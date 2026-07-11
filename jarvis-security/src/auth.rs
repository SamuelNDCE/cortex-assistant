use sha2::{Digest, Sha256};

/// Stores a SHA-256 hash of the owner PIN for secure comparison.
pub struct OwnerAuth {
    pin_hash: String,
}

impl OwnerAuth {
    /// Create from a plaintext PIN. The PIN is hashed immediately and never stored.
    pub fn new(pin: &str) -> Self {
        Self { pin_hash: hash_pin(pin) }
    }

    /// Verify a PIN attempt. Returns true only if it matches the stored hash.
    pub fn verify(&self, attempt: &str) -> bool {
        hash_pin(attempt) == self.pin_hash
    }
}

fn hash_pin(pin: &str) -> String {
    let mut h = Sha256::new();
    h.update(pin.as_bytes());
    hex::encode(h.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_pin_accepted() {
        let auth = OwnerAuth::new("1234");
        assert!(auth.verify("1234"));
    }

    #[test]
    fn wrong_pin_rejected() {
        let auth = OwnerAuth::new("1234");
        assert!(!auth.verify("0000"));
    }

    #[test]
    fn empty_pin_rejected_against_nonempty() {
        let auth = OwnerAuth::new("secret");
        assert!(!auth.verify(""));
    }

    #[test]
    fn empty_pin_matches_empty() {
        let auth = OwnerAuth::new("");
        assert!(auth.verify(""));
    }

    #[test]
    fn different_case_rejected() {
        let auth = OwnerAuth::new("Secret");
        assert!(!auth.verify("secret"));
        assert!(!auth.verify("SECRET"));
    }

    #[test]
    fn hash_is_deterministic() {
        let a1 = OwnerAuth::new("abc");
        let a2 = OwnerAuth::new("abc");
        assert_eq!(a1.pin_hash, a2.pin_hash);
    }
}

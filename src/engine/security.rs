use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::{Rng, thread_rng};
use sha2::{Sha256, Digest};

pub struct SecurityEngine {
    key: [u8; 32],
}

impl SecurityEngine {
    pub fn new(key_str: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(key_str.as_bytes());
        let result = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(&result);
        Self { key }
    }

    /// Encrypts data using AES-256-GCM.
    pub fn encrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        let cipher = Aes256Gcm::new(&self.key.into());
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }

    /// Decrypts data using AES-256-GCM.
    #[allow(dead_code)]
    pub fn decrypt(&self, encrypted_data: &[u8]) -> anyhow::Result<Vec<u8>> {

        if encrypted_data.len() < 12 {
            return Err(anyhow::anyhow!("Invalid encrypted data: too short"));
        }
        let cipher = Aes256Gcm::new(&self.key.into());
        let nonce = Nonce::from_slice(&encrypted_data[..12]);
        let ciphertext = &encrypted_data[12..];

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))
    }

    /// Obfuscates a symbol name using SHA-256 prefix.
    pub fn obfuscate_symbol(name: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        let result = hasher.finalize();
        // Convert first 4 bytes to hex string
        let hex = result[..4].iter().map(|b| format!("{:02x}", b)).collect::<String>();
        format!("sym_{}", hex)
    }

    /// Adds small perturbations to a feature vector to prevent reverse-engineering.
    pub fn perturb_vector(vector: &mut [f32], intensity: f32) {
        let mut rng = thread_rng();
        for val in vector.iter_mut() {
            let noise: f32 = rng.gen_range(-intensity..intensity);
            *val = (*val + noise).clamp(0.0, 1.0);
        }
    }
}

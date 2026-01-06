//! Authentication utilities

use hmac::{Hmac, Mac};
use sha1::Sha1;

/// Verify TOTP code (RFC 6238)
pub fn verify_totp(secret: &str, code: &str) -> bool {
    // Decode base32 secret
    let secret_bytes = match base32::decode(base32::Alphabet::Rfc4648 { padding: false }, secret) {
        Some(bytes) => bytes,
        None => return false,
    };
    
    // Get current time step (30 second intervals)
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() / 30;
    
    // Check current and adjacent time steps for clock drift tolerance
    for offset in -1i64..=1 {
        let counter = (time as i64 + offset) as u64;
        let counter_bytes = counter.to_be_bytes();
        
        // HMAC-SHA1
        let mut mac = match Hmac::<Sha1>::new_from_slice(&secret_bytes) {
            Ok(m) => m,
            Err(_) => return false,
        };
        mac.update(&counter_bytes);
        let result = mac.finalize().into_bytes();
        
        // Dynamic truncation
        let offset_idx = (result[19] & 0x0f) as usize;
        let binary = ((result[offset_idx] & 0x7f) as u32) << 24
            | (result[offset_idx + 1] as u32) << 16
            | (result[offset_idx + 2] as u32) << 8
            | (result[offset_idx + 3] as u32);
        
        let otp = binary % 1_000_000;
        let expected = format!("{:06}", otp);
        
        if expected == code {
            return true;
        }
    }
    
    false
}

/// Generate a simple session token
pub fn generate_session_token() -> String {
    use getrandom::getrandom;
    let mut bytes = [0u8; 32];
    getrandom(&mut bytes).unwrap_or_default();
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Hash password with SHA-256 (simple implementation)
pub fn hash_password(password: &str) -> String {
    use sha1::Digest;
    let mut hasher = sha1::Sha1::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

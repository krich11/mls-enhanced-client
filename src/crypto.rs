use openmls_rust_crypto::OpenMlsRustCrypto;

pub struct CryptoProvider {
    provider: OpenMlsRustCrypto,
}

impl CryptoProvider {
    pub fn new() -> Self {
        Self {
            provider: OpenMlsRustCrypto::default(),
        }
    }

    pub fn provider(&self) -> &OpenMlsRustCrypto {
        &self.provider
    }
}

impl Default for CryptoProvider {
    fn default() -> Self {
        Self::new()
    }
}
use anyhow::Result;
use openmls::prelude::*;
use openmls_rust_crypto::OpenMlsRustCrypto;
use openmls_basic_credential::SignatureKeyPair;
use openmls_memory_storage::MemoryStorage;
use crate::crypto::CryptoProvider;

pub struct MlsClient {
    pub crypto: OpenMlsRustCrypto,
    pub storage: MemoryStorage,
    pub signer: SignatureKeyPair,
    pub credential: BasicCredential,
    pub signature_key: SignaturePublicKey,
    pub key_package: KeyPackage,
}

impl MlsClient {
    pub async fn new(username: &str, _crypto_provider: CryptoProvider) -> Result<Self> {
        let crypto = OpenMlsRustCrypto::default();
        let storage = MemoryStorage::default();
        
        // Generate signature key pair
        let signer = SignatureKeyPair::new(SignatureScheme::ED25519)?;
        
        // Store the signature key into the key store
        signer.store(&storage)?;
        
        // Create basic credential with username
        let credential = BasicCredential::new(username.as_bytes().to_vec());
        let signature_key: SignaturePublicKey = signer.public().into();
        
        // Create credential with key
        let credential_with_key = CredentialWithKey {
            credential: credential.clone().into(),
            signature_key: signature_key.clone(),
        };

        // Create key package bundle
        let key_package_bundle = KeyPackage::builder()
            .build(
                Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519,
                &crypto,
                &signer,
                credential_with_key,
            )?;

        Ok(Self {
            crypto,
            storage,
            signer,
            credential,
            signature_key,
            key_package: key_package_bundle.key_package().clone(),
        })
    }

    pub fn get_identity(&self) -> &[u8] {
        self.credential.identity()
    }

    pub fn get_key_package(&self) -> &KeyPackage {
        &self.key_package
    }

    pub fn create_group(&self, group_config: &MlsGroupCreateConfig) -> Result<MlsGroup> {
        let credential_with_key = CredentialWithKey {
            credential: self.credential.clone().into(),
            signature_key: self.signature_key.clone(),
        };

        let group = MlsGroup::new(
            &self.crypto,
            &self.signer,
            group_config,
            credential_with_key,
        )?;

        Ok(group)
    }

    pub fn join_group(&self, _welcome: Welcome) -> Result<MlsGroup> {
        // For now, we'll implement a basic version
        // In a full implementation, you'd need to handle the welcome message properly
        // This is a placeholder that creates a new group
        let credential_with_key = CredentialWithKey {
            credential: self.credential.clone().into(),
            signature_key: self.signature_key.clone(),
        };

        let group = MlsGroup::new(
            &self.crypto,
            &self.signer,
            &MlsGroupCreateConfig::default(),
            credential_with_key,
        )?;

        Ok(group)
    }
}
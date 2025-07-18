use anyhow::Result;
use openmls::prelude::*;
use openmls_rust_crypto::OpenMlsRustCrypto;
use openmls_basic_credential::SignatureKeyPair;
use openmls_memory_storage::MemoryStorage;
use crate::crypto::CryptoProvider;
use std::collections::HashMap;

pub struct MlsClient {
    pub crypto: OpenMlsRustCrypto,
    pub storage: MemoryStorage,
    pub signer: SignatureKeyPair,
    pub credential: BasicCredential,
    pub signature_key: SignaturePublicKey,
    pub key_package: KeyPackage,
    pub groups: HashMap<String, MlsGroup>,
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
            groups: HashMap::new(),
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

    pub fn get_group(&self, group_id: &str) -> Option<&MlsGroup> {
        self.groups.get(group_id)
    }

    pub fn get_group_mut(&mut self, group_id: &str) -> Option<&mut MlsGroup> {
        self.groups.get_mut(group_id)
    }

    pub fn add_group(&mut self, group_id: &str, group: MlsGroup) {
        self.groups.insert(group_id.to_string(), group);
    }
}
use crate::types::types::{Bytes, Signature};
use async_trait::async_trait;
use base64::prelude::*;
use std::fmt;
use vaultrs::api::transit::requests::VerifySignedDataRequest;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::transit::data::{sign, verify};

pub const MOUNT: &str = "transit";

#[derive(Debug)]
pub enum HsmError {
	VaultClientSetupError(String),
	VaultOperationError(String),
	Utf8ConversionError(std::string::FromUtf8Error),
}

impl fmt::Display for HsmError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			HsmError::VaultClientSetupError(msg) => write!(f, "Vault Client Setup Error: {}", msg),
			HsmError::VaultOperationError(msg) => write!(f, "Vault Operation Error: {}", msg),
			HsmError::Utf8ConversionError(err) => write!(f, "UTF-8 Conversion Error: {}", err),
		}
	}
}

impl std::error::Error for HsmError {}

#[async_trait]
pub trait Hsm {
	async fn sign(&self, data: Bytes) -> Result<Signature, HsmError>;
	async fn verify(&self, data: Bytes, sig: Signature) -> Result<bool, HsmError>;
}

pub struct HashicorpHsm {
	client: VaultClient,
	key_name: String,
}

impl HashicorpHsm {
	pub fn initialize(addr: &str, auth_token: &str, key_name: &str) -> Result<Self, HsmError> {
		let settings = VaultClientSettingsBuilder::default()
			.address(addr)
			.token(auth_token)
			.namespace(Some("admin".to_string()))
			.verify(true)
			.build()
			.map_err(|e| HsmError::VaultClientSetupError(e.to_string()))?;

		let client = VaultClient::new(settings)
			.map_err(|e| HsmError::VaultClientSetupError(e.to_string()))?;

		Ok(HashicorpHsm { client, key_name: key_name.to_string() })
	}

	fn encode(&self, data: &[u8]) -> String {
		BASE64_STANDARD.encode(data)
	}

	fn decode_signature(&self, sig: &[u8]) -> Result<String, HsmError> {
		String::from_utf8(sig.to_vec()).map_err(HsmError::Utf8ConversionError)
	}
}

#[async_trait]
impl Hsm for HashicorpHsm {
	async fn sign(&self, data: Bytes) -> Result<Signature, HsmError> {
		let encoded_data = self.encode(&data.0);

		let sign_response = sign(&self.client, "transit", &self.key_name, &encoded_data, None)
			.await
			.map_err(|e| HsmError::VaultOperationError(e.to_string()))?;

		println!("Vault signature: {}", sign_response.signature);

		Ok(Signature(sign_response.signature.into_bytes()))
	}

	async fn verify(&self, data: Bytes, sig: Signature) -> Result<bool, HsmError> {
		let encoded_data = self.encode(&data.0);

		let signature_str = self.decode_signature(&sig.0)?;

		let verify_response = verify(
			&self.client,
			MOUNT,
			&self.key_name,
			&encoded_data,
			Some(VerifySignedDataRequest::builder().signature(&signature_str)),
		)
		.await
		.map_err(|e| HsmError::VaultOperationError(e.to_string()))?;

		Ok(verify_response.valid)
	}
}

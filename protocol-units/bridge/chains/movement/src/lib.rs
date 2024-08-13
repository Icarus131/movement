use anyhow::Error;
use aptos_sdk::{
	move_types::language_storage::TypeTag, 
	rest_client::{Client, FaucetClient}, 
	types::LocalAccount
};
use aptos_types::account_address::AccountAddress;
use bridge_shared::{
	bridge_contracts::{
		BridgeContractCounterparty, BridgeContractCounterpartyError,
		BridgeContractCounterpartyResult,
	},
	types::{
		Amount, BridgeTransferDetails, BridgeTransferId, HashLock, HashLockPreImage,
		InitiatorAddress, RecipientAddress, TimeLock,
	},
};
use crate::utils::MovementAddress;
use rand::prelude::*;
use serde::Serialize;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::{Arc, Mutex, RwLock};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;

use url::Url;

pub mod utils;

const DUMMY_ADDRESS: AccountAddress = AccountAddress::new([0; 32]);
const COUNTERPARTY_MODULE_NAME: &str = "atomic_bridge_counterparty";

enum Call {
	Lock,
	Complete,
	Abort,
	GetDetails,
}

pub struct Config {
	pub rpc_url: Option<String>,
	pub ws_url: Option<String>,
	pub chain_id: String,
	pub signer_private_key: Arc<LocalAccount>,
	pub initiator_contract: Option<MovementAddress>,
	pub gas_limit: u64,
}

impl Config {

	pub fn build_for_test() -> Self {

		let seed = [3u8; 32];
		let mut rng = rand::rngs::StdRng::from_seed(seed);

		Config {
			rpc_url: Some("http://localhost:8080".parse().unwrap()),
			ws_url: Some("ws://localhost:8080".parse().unwrap()),
			chain_id: 4.to_string(),
			signer_private_key: Arc::new(LocalAccount::generate(&mut rng)),
			initiator_contract: None,
			gas_limit: 10_000_000_000,
		}
	}
}

// Todo: Local testnet rather than devnet

	//let mut child = TokioCommand::new("aptos")
        //.args(&["node", "run-local-testnet"])
        //.stdout(Stdio::piped())
        //.stderr(Stdio::piped())
        //.spawn()?;
//
    	//let stdout = child.stdout.take().expect("Failed to capture stdout");
    	//let mut reader = BufReader::new(stdout).lines();
//
    	//while let Some(line) = reader.next_line().await? {
        //	println!("Output: {}", line);
//
        //	if line.contains("Setup is complete") {
      	//      		println!("Testnet is up and running!");
        //		break;
        //	}
	//}

	// let output = Command::new("aptos")
        // .arg("node")
        // .arg("run-local-testnet")
        // .stdout(Stdio::piped())  
        // .spawn()?;  
// 
	// println!("stdout: {}", String::from_utf8_lossy(&output.stdout));

	//let rest_client = &movement_client.rest_client;

#[allow(dead_code)]
#[derive(Clone)]
pub struct MovementClient {
	///Address of the counterparty moduke
	counterparty_address: AccountAddress,
	///Address of the initiator module
	initiator_address: Vec<u8>,
	///The Apotos Rest Client
	pub rest_client: Client,
	///The Apotos Rest Client
	pub faucet_client: Arc<RwLock<FaucetClient>>,
	///The signer account
	signer: Arc<LocalAccount>,
}

impl MovementClient {
	pub async fn new(config: Config) -> Result<Self, anyhow::Error> {
		let dot_movement = dot_movement::DotMovement::try_from_env().unwrap();
		let suzuka_config =
			dot_movement.try_get_config_from_json::<suzuka_config::Config>().unwrap();
		let node_connection_address = suzuka_config
			.execution_config
			.maptos_config
			.client
			.maptos_rest_connection_hostname;
		let node_connection_port = suzuka_config
			.execution_config
			.maptos_config
			.client
			.maptos_rest_connection_port;

		let node_connection_url =
			format!("http://{}:{}", node_connection_address, node_connection_port);
		let node_connection_url = Url::from_str(node_connection_url.as_str()).unwrap();

		let rest_client = Client::new(node_connection_url.clone());

		let faucet_listen_address = suzuka_config
			.execution_config
			.maptos_config
			.client
			.maptos_faucet_rest_connection_hostname
			.clone();
		let faucet_listen_port = suzuka_config
			.execution_config
			.maptos_config
			.client
			.maptos_faucet_rest_connection_port
			.clone();

		let faucet_connection_url = format!("http://{}:{}", node_connection_address, node_connection_port);
		let faucet_listen_url = Url::from_str(faucet_connection_url.as_str()).unwrap();
		let faucet_client = Arc::new(RwLock::new(FaucetClient::new(
			faucet_listen_url.clone(),
			node_connection_url.clone()
		)));

		let seed = [3u8; 32];
		let mut rng = rand::rngs::StdRng::from_seed(seed);
		let signer = LocalAccount::generate(&mut rng);

		Ok(MovementClient {
			counterparty_address: DUMMY_ADDRESS,
			initiator_address: Vec::new(), //dummy for now
			rest_client,
			faucet_client,
			signer: Arc::new(signer),
		})
	}

	pub async fn new_for_test(config: Config) -> Result<Self, anyhow::Error> {

		//let mut child = TokioCommand::new("aptos")
		//.args(&["node", "run-local-testnet"])
		//.stdout(Stdio::piped())
		//.stderr(Stdio::piped())
		//.spawn()?;
//
		//let stdout = child.stdout.take().expect("Failed to capture stdout");
		//let stderr = child.stderr.take().expect("Failed to capture stderr");
//
		//let mut stdout_reader = BufReader::new(stdout).lines();
		//let mut stderr_reader = BufReader::new(stderr).lines();
//
//
		//loop {
		//	tokio::select! {
		//		line = stdout_reader.next_line() => {
		//			match line {
		//				Ok(Some(line)) => {
		//					println!("STDOUT: {}", line);
		//					if line.contains("Setup is complete") {
		//						println!("Testnet is up and running!");
		//						break;
		//					}
		//				},
		//				Ok(None) => break, // End of stream
		//				Err(e) => {
		//					eprintln!("Error reading stdout: {}", e);
		//					break;
		//				}
		//			}
		//		},
		//		line = stderr_reader.next_line() => {
		//			match line {
		//				Ok(Some(line)) => {
		//					println!("STDERR: {}", line);
		//				},
		//				Ok(None) => break, // End of stream
		//				Err(e) => {
		//					eprintln!("Error reading stderr: {}", e);
		//					break;
		//				}
		//			}
		//		}
		//	}
		//}

		let node_connection_url = format!("https://aptos.devnet.suzuka.movementlabs.xyz/v1");
		let node_connection_url = Url::from_str(node_connection_url.as_str()).unwrap();
		let rest_client = Client::new(node_connection_url.clone());

		let faucet_url = format!("https://faucet.devnet.suzuka.movementlabs.xyz");
		let faucet_url = Url::from_str(faucet_url.as_str()).unwrap();
		let faucet_client = Arc::new(RwLock::new(FaucetClient::new(faucet_url.clone(), node_connection_url.clone())));

		let mut rng = ::rand::rngs::StdRng::from_seed([3u8; 32]);
		Ok(MovementClient {
			counterparty_address: DUMMY_ADDRESS,
			initiator_address: Vec::new(), //dummy for now
			rest_client,
			faucet_client,
			signer: Arc::new(LocalAccount::generate(&mut rng)),
		})
	}

	pub fn rest_client(&self) -> &Client {
		&self.rest_client
	}

	pub fn faucet_client(&self) -> &Arc<RwLock<FaucetClient>> {
		&self.faucet_client
	}

}

#[async_trait::async_trait]
impl BridgeContractCounterparty for MovementClient {
	type Address = MovementAddress;
	type Hash = [u8; 32];

	async fn lock_bridge_transfer_assets(
		&mut self,
		bridge_transfer_id: BridgeTransferId<Self::Hash>,
		hash_lock: HashLock<Self::Hash>,
		time_lock: TimeLock,
		initiator: InitiatorAddress<Vec<u8>>,
		recipient: RecipientAddress<Self::Address>,
		amount: Amount,
	) -> BridgeContractCounterpartyResult<()> {
		//@TODO properly return an error instead of unwrapping
		let args = vec![
			to_bcs_bytes(&initiator.0).unwrap(),
			to_bcs_bytes(&bridge_transfer_id.0).unwrap(),
			to_bcs_bytes(&hash_lock.0).unwrap(),
			to_bcs_bytes(&time_lock.0).unwrap(),
			to_bcs_bytes(&recipient.0).unwrap(),
			to_bcs_bytes(&amount.0).unwrap(),
		];
		let payload = utils::make_aptos_payload(
			self.counterparty_address,
			COUNTERPARTY_MODULE_NAME,
			"lock_bridge_transfer_assets",
			self.counterparty_type_args(Call::Lock),
			args,
		);
		let _ = utils::send_aptos_transaction(&self.rest_client, self.signer.as_ref(), payload)
			.await
			.map_err(|_| BridgeContractCounterpartyError::LockTransferAssetsError);
		Ok(())
	}

	async fn complete_bridge_transfer(
		&mut self,
		bridge_transfer_id: BridgeTransferId<Self::Hash>,
		preimage: HashLockPreImage,
	) -> BridgeContractCounterpartyResult<()> {
		let args = vec![
			to_bcs_bytes(&self.signer.address()).unwrap(),
			to_bcs_bytes(&bridge_transfer_id.0).unwrap(),
			to_bcs_bytes(&preimage.0).unwrap(),
		];
		let payload = utils::make_aptos_payload(
			self.counterparty_address,
			COUNTERPARTY_MODULE_NAME,
			"complete_bridge_transfer",
			self.counterparty_type_args(Call::Complete),
			args,
		);

		let _ = utils::send_aptos_transaction(&self.rest_client, self.signer.as_ref(), payload)
			.await
			.map_err(|_| BridgeContractCounterpartyError::CompleteTransferError);
		Ok(())
	}

	async fn abort_bridge_transfer(
		&mut self,
		bridge_transfer_id: BridgeTransferId<Self::Hash>,
	) -> BridgeContractCounterpartyResult<()> {
		let args = vec![
			to_bcs_bytes(&self.signer.address()).unwrap(),
			to_bcs_bytes(&bridge_transfer_id.0).unwrap(),
		];
		let payload = utils::make_aptos_payload(
			self.counterparty_address,
			COUNTERPARTY_MODULE_NAME,
			"abort_bridge_transfer",
			self.counterparty_type_args(Call::Abort),
			args,
		);
		let _ = utils::send_aptos_transaction(&self.rest_client, self.signer.as_ref(), payload)
			.await
			.map_err(|_| BridgeContractCounterpartyError::AbortTransferError);
		Ok(())
	}

	async fn get_bridge_transfer_details(
		&mut self,
		_bridge_transfer_id: BridgeTransferId<Self::Hash>,
	) -> BridgeContractCounterpartyResult<Option<BridgeTransferDetails<Self::Address, Self::Hash>>>
	{
		// let _ = utils::send_view_request(
		// 	self.rest_client,
		// 	self.counterparty_address,
		// 	"atomic_bridge_counterparty".to_string(),
		// );
		todo!();
	}
}

impl MovementClient {
	fn counterparty_type_args(&self, call: Call) -> Vec<TypeTag> {
		match call {
			Call::Lock => vec![TypeTag::Address, TypeTag::U64, TypeTag::U64, TypeTag::U8],
			Call::Complete => vec![TypeTag::Address, TypeTag::U64, TypeTag::U8],
			Call::Abort => vec![TypeTag::Address, TypeTag::U64],
			Call::GetDetails => vec![TypeTag::Address, TypeTag::U64],
		}
	}
}

fn to_bcs_bytes<T>(value: &T) -> Result<Vec<u8>, anyhow::Error>
where
	T: Serialize,
{
	Ok(bcs::to_bytes(value)?)
}

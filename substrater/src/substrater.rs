use std::{
	collections::VecDeque,
	sync::{Arc, Mutex},
};
use submetadatan::{Metadata, RuntimeMetadataPrefixed};
use subtypes::Hash;

/// Substrate client
pub struct Substrater {
	pub signer: Keypair,
	pub rpc: RPC,
}

struct RPC {
	pub uri: String,
	pub websocket: Arc<Websocket>,
	pub genesis_hash: Hash,
	pub versions: Versions,
	pub metadata: Metadata,
	pub events: Arc<Mutex<VecDeque<Value>>>,
}

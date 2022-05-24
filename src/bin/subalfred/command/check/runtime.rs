// crates.io
use async_std::task;
use clap::{ArgEnum, Args};
// hack-ink
use crate::prelude::*;
use subalfred::core::{check::runtime, node};

/// Compare the local node runtime version with live's.
#[derive(Debug, Args)]
pub struct RuntimeCmd {
	/// Path to the executable.
	#[clap(long, required = true, value_name = "PATH")]
	executable: String,
	/// Chain name.
	#[clap(long, required = true, value_name = "NAME")]
	chain: String,
	/// Live chain's RPC HTTP endpoint.
	#[clap(long, required = true, value_name = "URI")]
	live: String,
	/// The properties to check.
	#[clap(
		arg_enum,
		long,
		required = true,
		multiple_values = true,
		use_value_delimiter = true,
		value_name = "[PROPERTY]"
	)]
	properties: Vec<Property>,
}
impl RuntimeCmd {
	pub fn run(&self) -> AnyResult<()> {
		let Self { executable, chain, live, properties } = self;
		// TODO: if the port is already in used
		let mut node_process = node::spawn(executable, 23333, chain)?;
		let local = "http://127.0.0.1:23333";

		// TODO: futures join all
		for property in properties {
			match property {
				Property::Storage => match task::block_on(runtime::check_storage(local, live)) {
					Ok((pallets_diff, entries_diffs)) => {
						if !pallets_diff.is_empty() {
							pallets_diff
								.into_iter()
								.for_each(|pallet_diff| println!("{pallet_diff}"));

							println!();
						}

						entries_diffs.into_iter().for_each(|(prefix, entry_diffs)| {
							println!("Pallet: {prefix}",);

							entry_diffs.into_iter().for_each(|entry_diff| println!("{entry_diff}"));

							println!();
						});
					},
					e => {
						node_process.kill()?;
						e?;
					},
				},
				Property::Version => match task::block_on(runtime::check_version(local, live)) {
					Ok(Some(diffs)) => println!("{diffs}"),
					Ok(None) => (),
					e => {
						node_process.kill()?;
						e?;
					},
				},
			}
		}

		node_process.kill()?;

		Ok(())
	}
}

/// Runtime's property.
#[derive(Clone, Debug, ArgEnum)]
pub enum Property {
	/// Check the runtime storage.
	Storage,
	/// Check the runtime version.
	Version,
}
// crates.io
use cargo_metadata::{Metadata, MetadataCommand};
// hack-ink
use super::*;

fn test_data() -> Metadata {
	MetadataCommand::new().manifest_path("./Cargo.toml").exec().unwrap()
}

#[test]
fn members_manifests_should_work() {
	fn remove_prefix(s: &str) -> String {
		s.rsplit_once("/subalfred").unwrap().1.to_owned()
	}

	let metadata = test_data();

	assert_eq!(
		members(&metadata)
			.unwrap()
			.iter()
			.map(|pkg| { (pkg.name.as_str(), remove_prefix(&pkg.manifest_path.as_str())) })
			.collect::<Vec<_>>(),
		[
			("subcryptor", "/substrate-minimal/subcryptor/Cargo.toml"),
			("subgrandpa", "/substrate-minimal/subgrandpa/Cargo.toml"),
			("subhasher", "/substrate-minimal/subhasher/Cargo.toml"),
			("submetadatan", "/substrate-minimal/submetadatan/Cargo.toml"),
			("subrpcer", "/substrate-minimal/subrpcer/Cargo.toml"),
			("subrpcer-impl", "/substrate-minimal/subrpcer/impl/Cargo.toml"),
			("subruntimer", "/substrate-minimal/subruntimer/Cargo.toml"),
			("substorager", "/substrate-minimal/substorager/Cargo.toml"),
			("subversioner", "/substrate-minimal/subversioner/Cargo.toml"),
			("subalfred", "/Cargo.toml")
		]
		.iter()
		.map(|&(name, path)| (name, path.into()))
		.collect::<Vec<_>>()
	);
}

#[test]
fn align_version_should_work() {
	["0", "0.0", "0.0.0"].iter().for_each(|from| {
		["1", "1.0", "1.0.0"].iter().for_each(|to| {
			dbg!(from, to);
			match *from {
				"0" => assert_eq!(util::align_version(from, to), "1"),
				"0.0" => assert_eq!(util::align_version(from, to), "1.0"),
				"0.0.0" => assert_eq!(util::align_version(from, to), "1.0.0"),
				_ => unreachable!(),
			}
		});
	});
}

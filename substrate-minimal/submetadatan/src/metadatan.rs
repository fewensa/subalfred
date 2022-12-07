pub use frame_metadata::{RuntimeMetadataV14 as LatestRuntimeMetadata, *};

use crate::{SubmetadatanError, SubmetadatanResult};

pub fn metadata(metadata: RuntimeMetadataPrefixed) -> SubmetadatanResult<LatestRuntimeMetadata> {
	match metadata.1 {
		RuntimeMetadata::V14(metadata) => Ok(metadata),
		metadata => Err(SubmetadatanError::UnsupportedVersion(metadata.version())),
	}
}

/// Wrap substrate metadata
pub trait Metadatan {
	//
}

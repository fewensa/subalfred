use thiserror::Error as ThisError;

pub type SubmetadatanResult<T> = std::result::Result<T, SubmetadatanError>;

#[derive(Debug, ThisError)]
pub enum SubmetadatanError {
	#[error("[submetadatan] unsupported version, {0:?}")]
	UnsupportedVersion(u32),
}

use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigErrors {
    #[error("Unable to read .bpingrc {0}")]
    CantReadFile(io::Error),
    #[error(transparent)]
    CantParseFile(#[from] toml::de::Error),
    #[error("Unable to create .bpingrc {0}")]
    CantCreateFile(io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error)
}
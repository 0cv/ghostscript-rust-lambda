extern crate ssh2;
extern crate anyhow;
extern crate thiserror;

#[derive(Debug, thiserror::Error)]
pub enum FtpError {
    #[error("IO Error")]
    Io(#[from] ::std::io::Error),
    #[error("SSH Error")]
    Ssh2(#[from] ::ssh2::Error),
}

pub mod builder;

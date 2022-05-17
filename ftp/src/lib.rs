extern crate ssh2;
extern crate anyhow;
extern crate thiserror;

use ssh2::Session;


#[derive(Clone)]
pub struct FtpBuilder<'a> {
    pub user: &'a str,
    pub pass: &'a str,
    pub addr: &'a str,
    pub sess: Session,
}

#[derive(Debug, thiserror::Error)]
pub enum FtpError {
    #[error("IO Erro: {0}")]
    Io(#[from] ::std::io::Error),
    #[error("SSH Error: {0}")]
    Ssh2(#[from] ::ssh2::Error),
}

pub mod builder;

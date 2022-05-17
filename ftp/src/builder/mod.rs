use std::io::Read;
use std::net::TcpStream;
use std::path::Path;
use anyhow::Result;
use ssh2::Session;


impl<'a> ::FtpBuilder<'a> {
    pub fn new(user: &'a str, pass: &'a str, addr: &'a str) -> Result<::FtpBuilder<'a>, ::FtpError> {
        let tcp = TcpStream::connect(&addr)?;
        let mut sess = Session::new()?;
        sess.set_timeout(15000);
        sess.set_tcp_stream(tcp);
        sess.handshake()?;

        sess.userauth_password(&user, &pass)?;
        println!("is authenticated: {}", sess.authenticated());

        Ok(::FtpBuilder {
            user,
            pass,
            addr,
            sess,
        })
    }

    pub fn list_dir(&self, dir: &'a str) -> Result<Vec<std::path::PathBuf>, ::FtpError> {
        let sftp = self.sess.sftp()?;
        let data_path = Path::new(&dir);
        Ok(sftp.readdir(&data_path)?
            .into_iter()
            .map(|(e, _)| e)
            .collect::<Vec<_>>())
    }

    pub fn download(&self, path: &'a str) -> Result<Vec<u8>, ::FtpError> {
        let sftp = self.sess.sftp()?;
        let data_path = Path::new(&path);
        let mut buf = Vec::new();

        sftp.open(data_path)?
            .read_to_end(&mut buf)?;

        Ok(buf)
    }
}
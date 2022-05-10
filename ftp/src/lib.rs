extern crate ssh2;

#[macro_use]
extern crate error_chain;
pub mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Ssh2(::ssh2::Error);
        }
    }
}

pub mod builder;

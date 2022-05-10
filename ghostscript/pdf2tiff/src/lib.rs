extern crate base64;
extern crate ghostscript;

#[macro_use]
extern crate error_chain;

pub mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Base64(::base64::DecodeError);
            // Ass();
        }

        errors {
            ConversionFailed(t: String) {
                display("conversion failed: '{}'", t)
            }

            PdfRunningInstanceException(t: String) {
                display("running instance exception: '{}'", t)
            }
        }
    }
}

// impl<T> From<::ghostscript::builder::BuilderError<T>> for errors::Error {
//     fn from(err: ::ghostscript::builder::BuilderError<T>) -> Self {
//         // errors::Error::chain_err(err.)
//         errors::Error::description("qwdqwd")
//     }
// }

pub mod builder;

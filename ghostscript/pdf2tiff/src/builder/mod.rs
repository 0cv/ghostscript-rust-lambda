pub mod display_callback;

use ghostscript::builder::{BuilderResult, GhostscriptBuilder};
use self::display_callback::PageGrabberDisplayCallback;
use std::{fs::File, io::Read};
use anyhow::Result;

#[derive(Clone)]
pub struct PdfBuilder<'a> {
    pub base64: &'a str,
}

impl<'a> PdfBuilder<'a> {
    pub fn new(base64: &'a str) -> Self {
        PdfBuilder {
            base64,
        }
    }

    pub fn convert(&self) -> Result<String> {
        let output_file_name = "/tmp/output.tif";
        let mut builder = GhostscriptBuilder::new();
    
        let bytes = base64::decode(self.base64)?;
        let mut my_callback = PageGrabberDisplayCallback::new(&bytes);
    
        builder.with_stdin(true);
        // builder.with_display(true);
    
        builder.with_init_params(&[
            "-sDEVICE=tiffg4",
            "-dNOPAUSE",
            "-r300x300",
            &format!("-sOutputFile={}", output_file_name),
            "-",
        ]);
    
        // If we used build() instead of build_simple() we could have passed any data
        // to associate with the new Ghostscript interpreter instance.
        // Such user data can also implement some useful Ghostscript callback traits.
        return match builder.build(&mut my_callback) {
            BuilderResult::Running(instance) => {
                // This is where we could get a running instance for further interpreter work.
                // But our init params above should have made the interpreter immediately quit
                // after rendering the file.
                Err(::PdfError::PdfRunningInstanceException(format!("Unexpected ghostscript instance: {:?}", instance.into_inner()).to_string()).into())
            },
            BuilderResult::Quit(user_data) => {
                // The builder can be reused to keep building new instances.
                // All the settings and parameters are preserved.
                // The following repeats the same rendering as above, but the has_quit() shorthand is used
                // to convert BuilderResult into Result in a similar way to the above match.
            
                let mut f = File::open(output_file_name)?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)?;
                Ok(String::from(base64::encode(&buffer.as_slice())))
            },
            BuilderResult::Failed(e) => {
                // Interpreter failed to build or run. The user_data is returned to us still.
                // As well as details about which part of the build process failed.
                Err(::PdfError::BuilderError{ kind: format!("{:?}", e.kind), code: e.code.to_string(), user_data: format!("{:?}", e.user_data) }.into())
            },
        };
    }
}
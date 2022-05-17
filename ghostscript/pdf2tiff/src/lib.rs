extern crate base64;
extern crate ghostscript;
extern crate anyhow;
extern crate thiserror;

#[derive(Debug, thiserror::Error)]
pub enum PdfError {
    #[error("PDF: Instance Exception: {0}")]
    PdfRunningInstanceException(String),

    #[error("PDF Failed (kind: {kind:?}, code: {code:?}, user_data: {user_data:?})")]
    BuilderError {
        kind: String,
        code: String,
        user_data: String,
    },
}

pub mod builder;

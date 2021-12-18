use std::error;

use serde_json::value::RawValue;

#[derive(Debug)]
pub struct Error {
    pub status_code: u16,
    pub message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = format!(
            "request failed with status code {}. reason: {}",
            self.status_code, self.message
        );
        f.write_str(&msg)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }
}

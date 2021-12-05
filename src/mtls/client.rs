mod builder;

pub use builder::Builder;
use url::{ParseError, Url};

/// This type contains a single X.509 certificate by value.
pub struct Certificate {
    cert: rustls::Certificate,
}

/// This type contains a private key by value.
pub struct PrivateKey {
    cert: rustls::PrivateKey,
}

pub struct Client {
    base_url: Option<Url>,
}

impl Client {
    pub fn builder() -> Builder<'static> {
        Builder::new()
    }

    fn urlify(&self, path: &str) -> Result<Url, ParseError> {
        match &self.base_url {
            Some(url) => url.join(path),
            None => url::Url::parse(path),
        }
    }
}

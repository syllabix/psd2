use std::ptr::NonNull;

use url::Url;

use crate::mtls;

pub struct Builder<'a> {
    key: Option<mtls::PrivateKey>,
    cert: Option<mtls::Certificate>,
    base_url: Option<&'a str>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Builder {
            key: None,
            cert: None,
            base_url: None,
        }
    }

    /// Sets the base_url that will be used for all requests
    pub fn base_url(&mut self, url: &'a str) -> &mut Self {
        self.base_url = Some(url);
        self
    }

    /// Sets a single certificate and matching private key for use
    /// in client authentication.
    pub fn single_cert(&mut self, cert: mtls::Certificate, key: mtls::PrivateKey) -> &mut Self {
        self.cert = Some(cert);
        self.key = Some(key);
        self
    }

    pub fn build(&self) -> Result<mtls::Client, Box<dyn std::error::Error + 'static>> {
        Ok(mtls::Client { base_url: None })
    }
}

mod builder;

use std::{
    error::Error,
    fmt,
    fs::File,
    io::{self},
};

pub use builder::Builder;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use rustls_pemfile::Item;
use serde::Deserialize;
use url::{ParseError, Url};

/// Client is used to make http requests
pub struct Client {
    base_url: Option<Url>,
    headers: hyper::HeaderMap,
    client: hyper::Client<HttpsConnector<HttpConnector>>,
}

impl Client {
    /// Use this function to initialize a builder that can be used
    /// to properly initialize an mtls Client
    pub fn builder() -> Builder<'static> {
        Builder::new()
    }

    pub fn post(uri: &str) {}

    fn urlify(&self, path: &str) -> Result<Url, ParseError> {
        match &self.base_url {
            Some(url) => url.join(path),
            None => url::Url::parse(path),
        }
    }
}

// TODO: define a better error
#[derive(Debug)]
struct PrivateKeyMissing();

impl fmt::Display for PrivateKeyMissing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no private key found")
    }
}

impl Error for PrivateKeyMissing {
    fn description(&self) -> &str {
        "no private key found"
    }
}

/// Certificate represents a x509 certificate and it's
/// corresponding private key used to sign client requests
#[derive(Debug, Clone)]
pub struct Certificate {
    key: rustls::PrivateKey,
    chain: Vec<rustls::Certificate>,
}

impl Certificate {
    /// load_from_x509_key_pair reads and parses a public/private key pair from a pair
    /// of files. The files must contain PEM encoded data.
    pub fn load_from_x509_key_pair(
        cert_file: &str,
        key_file: &str,
    ) -> Result<Certificate, Box<dyn Error>> {
        let mut cert = {
            let f = File::open(cert_file)?;
            io::BufReader::new(f)
        };

        let mut key = {
            let f = File::open(key_file)?;
            io::BufReader::new(f)
        };

        return Certificate::from_x509_key_pair(&mut cert, &mut key);
    }

    /// from_x509_key_pair reads and parses a public/private key pair from a pair
    /// of buffered readers. The buffers must contain PEM encoded data.
    pub fn from_x509_key_pair(
        cert: &mut dyn io::BufRead,
        key: &mut dyn io::BufRead,
    ) -> Result<Certificate, Box<dyn Error>> {
        let chain = {
            rustls_pemfile::certs(cert)?
                .iter()
                .map(|der| rustls::Certificate(der.to_vec()))
                .collect()
        };

        let key = {
            match rustls_pemfile::read_one(key)? {
                Some(item) => match item {
                    Item::PKCS8Key(key) => rustls::PrivateKey(key),
                    _ => return Err(Box::new(PrivateKeyMissing())),
                },
                _ => return Err(Box::new(PrivateKeyMissing())),
            }
        };

        Ok(Certificate { chain, key })
    }
}

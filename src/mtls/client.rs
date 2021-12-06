mod builder;

use std::{fs::File, io};

pub use builder::Builder;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use rustls_pemfile::Item;
use url::{ParseError, Url};

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
    ) -> Result<Certificate, Box<dyn std::error::Error>> {
        let chain = {
            let f = File::open(cert_file)?;
            let mut f = io::BufReader::new(f);
            rustls_pemfile::certs(&mut f)?
                .iter()
                .map(|der| rustls::Certificate(der.to_vec()))
                .collect()
        };

        let key = {
            let f = File::open(key_file).unwrap();
            let mut f = io::BufReader::new(f);
            match rustls_pemfile::read_one(&mut f)
                .transpose()
                .unwrap()
                .unwrap()
            {
                Item::PKCS8Key(key) => rustls::PrivateKey(key),
                _ => panic!("no private key found"),
            }
        };

        Ok(Certificate { chain, key })
    }
}

/// Client is used to make http requests
pub struct Client {
    base_url: Option<Url>,
    headers: hyper::HeaderMap,
    client: hyper::Client<HttpsConnector<HttpConnector>>,
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

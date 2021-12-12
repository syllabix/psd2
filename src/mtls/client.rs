mod builder;
pub mod request;
pub mod response;

use std::{
    error::Error,
    fmt,
    fs::File,
    io::{self},
    time::Duration,
};

pub use builder::Builder;
use futures_util::StreamExt;
use hyper::{client::HttpConnector, Body, Method, Request as HTTPRequest};
use hyper_rustls::HttpsConnector;
pub use request::Request;
pub use response::Response;
use rustls_pemfile::Item;
use serde::{de::DeserializeOwned, Serialize};
use url::{ParseError, Url};

/// Client is used to make http requests
pub struct Client {
    timeout: Option<Duration>,
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

    /// Issue a url encoded form post request to the provided path
    /// using the provided request
    pub async fn post_form<'a, T: Serialize, U: DeserializeOwned>(
        &self,
        path: &str,
        request: Request<T>,
    ) -> Result<Response<U>, Box<dyn Error + 'static>> {
        let uri = self.urlify(path)?;
        let payload = serde_urlencoded::to_string(request.data)?;
        let req = HTTPRequest::builder()
            .method(Method::POST)
            .uri(uri.as_str())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from(payload))?;

        let mut res = self.client.request(req).await?;
        let mut body = Vec::new();
        while let Some(chunk) = res.body_mut().next().await {
            body.extend_from_slice(&chunk.unwrap());
        }

        let result: U = serde_json::from_slice(&body)?;
        return Ok(Response {
            status_code: res.status().as_u16(),
            body: result,
        });
    }

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

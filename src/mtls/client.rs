use hyper::{
    client::HttpConnector,
    header::{Entry, HeaderValue},
    Body, Method, Request as HTTPRequest,
};
use hyper_rustls::HttpsConnector;
use rustls_pemfile::Item;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::value::RawValue;
use std::{
    collections::HashMap,
    error::Error,
    fmt,
    fs::File,
    io::{self},
    str::from_utf8,
    time::Duration,
};
use url::{ParseError, Url};

mod builder;
pub mod request;
pub mod response;
pub use builder::Builder;
pub use request::Request;
pub use response::Response;

use self::request::error;

/// Client is used to make http requests
pub struct Client {
    // TODO: implement timeout
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

    /// Issue a post request to the provided path
    pub async fn post<T: Serialize, U: DeserializeOwned>(
        &self,
        path: &str,
        request: Request<T>,
    ) -> Result<Response<U>, Box<dyn Error + 'static>> {
        let (data, mut headers) = request.parts();

        let payload = serde_json::to_string(&data)?;
        self.request(Method::POST, path, &mut headers, Body::from(payload))
            .await
    }

    /// Issue a url encoded form post request to the provided path
    /// using the provided request
    pub async fn post_form<T: Serialize, U: DeserializeOwned>(
        &self,
        path: &str,
        request: Request<T>,
    ) -> Result<Response<U>, Box<dyn Error + 'static>> {
        let (data, mut headers) = request.parts();

        headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let payload = serde_urlencoded::to_string(data)?;
        self.request(Method::POST, path, &mut headers, Body::from(payload))
            .await
    }

    async fn request<T: DeserializeOwned>(
        &self,
        method: hyper::Method,
        path: &str,
        headers: &mut hyper::HeaderMap,
        body: Body,
    ) -> Result<Response<T>, Box<dyn Error + 'static>> {
        let uri = self.urlify(path)?;

        for (key, value) in &self.headers {
            if let Entry::Vacant(entry) = headers.entry(key) {
                entry.insert(value.clone());
            }
        }

        let mut req = HTTPRequest::builder()
            .method(method)
            .uri(uri.as_str())
            .body(body)?;
        *req.headers_mut() = headers.clone();

        let (parts, body) = self.client.request(req).await?.into_parts();
        let bytes = hyper::body::to_bytes(body).await?;

        if parts.status.is_success() {
            let result: T = serde_json::from_slice(&bytes)?;
            return Ok(Response {
                status_code: parts.status.as_u16(),
                data: result,
            });
        }

        let json = from_utf8(bytes.as_ref())?;

        Err(Box::new(error::Error {
            status_code: parts.status.as_u16(),
            message: String::from(json),
        }))
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

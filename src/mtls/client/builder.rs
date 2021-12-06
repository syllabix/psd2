use hyper::header::{self, HeaderValue};
use rustls::OwnedTrustAnchor;

use crate::mtls;

pub struct Builder<'a> {
    cert: Option<mtls::Certificate>,
    base_url: Option<&'a str>,
    headers: hyper::HeaderMap,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Builder {
            cert: None,
            base_url: None,
            headers: hyper::HeaderMap::new(),
        }
    }

    /// Sets the base_url that will be used for all requests
    pub fn base_url(&mut self, url: &'a str) -> &mut Self {
        self.base_url = Some(url);
        self
    }

    /// Sets a single certificate and matching private key for use
    /// in client authentication.
    pub fn single_cert(&mut self, cert: mtls::Certificate) -> &mut Self {
        self.cert = Some(cert);
        self
    }

    /// Adds a default http header to that will be used on ever outgoing request from
    /// the client
    pub fn default_header(&mut self, key: &'static str, value: &'static str) -> &mut Self {
        self.headers.append(key, header::HeaderValue::from_static(value));
        self
    }

    /// Build an mtls Client
    pub fn build(&self) -> Result<mtls::Client, Box<dyn std::error::Error + 'static>> {
        let base_url = {
            match self.base_url {
                Some(base_url) => Some(url::Url::parse(base_url)?),
                None => None,
            }
        };

        let mut root_store = rustls::RootCertStore::empty();
        root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
            OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));

        let builder = rustls::ClientConfig::builder()
            .with_safe_default_cipher_suites()
            .with_safe_default_kx_groups()
            .with_safe_default_protocol_versions()?
            .with_root_certificates(root_store);

        let config = {
            // TODO: no cloning!
            match self.cert.clone() {
                Some(cert) => builder.with_single_cert(cert.chain, cert.key)?,
                None => builder.with_no_client_auth(),
            }
        };

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(config)
            .https_only()
            .enable_http1()
            .build();

        let client: hyper::Client<_, hyper::Body> = hyper::Client::builder().build(https);

        Ok(mtls::Client {
            base_url,
            headers: self.headers.to_owned(), // TODO: no cloning!
            client,
        })
    }
}

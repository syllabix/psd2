use std::{sync::Arc, time::Duration};

use hyper::header;
use rustls::{client::ServerCertVerifier, OwnedTrustAnchor};

use crate::mtls;

/// A Builder is used to construct a mtls Client
pub struct Builder<'a> {
    insecure: bool,
    cert: Option<mtls::Certificate>,
    timeout: Option<Duration>,
    base_url: Option<&'a str>,
    headers: hyper::HeaderMap,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Builder {
            insecure: false,
            cert: None,
            timeout: None,
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
    pub fn certificate(&mut self, cert: mtls::Certificate) -> &mut Self {
        self.cert = Some(cert);
        self
    }

    /// Sets a default timeout to be used for all requests
    pub fn timeout(&mut self, duration: Duration) -> &mut Self {
        self.timeout = Some(duration);
        self
    }

    /// Adds a default http header to that will be used on every outgoing request from
    /// the client
    pub fn default_header(&mut self, key: &'static str, value: &'static str) -> &mut Self {
        self.headers
            .append(key, header::HeaderValue::from_static(value));
        self
    }

    /// Insecure will set the client to trust all server certificates. This is incredibly
    /// unsafe, but is here to suppor connecting to various psd2 sandbox environments that
    /// require this. (effectively supporting the cURL -k flag)
    pub fn insecure(&mut self) -> &mut Self {
        self.insecure = true;
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
            let mut config = match self.cert.clone() {
                Some(cert) => builder.with_single_cert(cert.chain, cert.key)?,
                None => builder.with_no_client_auth(),
            };

            if self.insecure {
                config
                    .dangerous()
                    .set_certificate_verifier(Arc::new(SkipCertificationVerification));
            }
            config
        };

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(config)
            .https_only()
            .enable_http1()
            .build();

        let client: hyper::Client<_, hyper::Body> = hyper::Client::builder().build(https);

        Ok(mtls::Client {
            base_url,
            client,
            timeout: self.timeout,
            headers: self.headers.to_owned(),
        })
    }
}

/// Implementation of `ServerCertVerifier` that verifies everything as trustworthy.
/// This is incredbily unsafe, but is here to support the use case of connecting to various
/// psd2 sandbox environments that require this (the equivilant of -k with cURL)
struct SkipCertificationVerification;

impl ServerCertVerifier for SkipCertificationVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

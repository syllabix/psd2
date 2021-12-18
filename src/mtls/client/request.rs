//! # request
//!
//! The request module contains entities and utilites related
//! to issuing requests to psd2 endpoints

use hyper::header::{self, HeaderValue};
use serde::Serialize;

mod builder;
pub mod error;
pub mod sign;
use builder::Builder;

/// A Request is used to send data and additional headers to a remote
/// psd2 endpoint. Depending on the http method used to issue the Request, the underlying
/// data will be serialized appropriately
pub struct Request<T: Serialize> {
    pub(super) data: T,
    pub(super) headers: hyper::HeaderMap,
}

impl<T: Serialize> Request<T> {
    /// Constructor for a request
    pub fn new(data: T) -> Request<T> {
        Request {
            data,
            headers: hyper::HeaderMap::new(),
        }
    }

    /// Adds a http header to that will be used on the outgoing request
    /// If the value or key is invalid, instead of failing, the header will not
    /// be set.
    pub fn header(&mut self, key: &'static str, value: &str) -> &mut Self {
        if let Ok(value) = HeaderValue::from_str(value) {
            self.headers.append(key, value);
        };
        self
    }

    /// TODO: investigate a proper builder pattern for the request
    fn builder() -> Builder<T> {
        Builder::new()
    }

    pub(super) fn parts(self) -> (T, hyper::HeaderMap) {
        (self.data, self.headers)
    }
}

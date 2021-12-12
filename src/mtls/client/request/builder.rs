use std::{error::Error, fmt};

use hyper::header;
use serde::Serialize;

use crate::mtls::client;

pub struct Builder<T: Serialize> {
    data: Option<T>,
    headers: hyper::HeaderMap,
}

impl<T: Serialize> Builder<T> {
    pub fn new() -> Builder<T> {
        Builder {
            data: None,
            headers: hyper::HeaderMap::new(),
        }
    }

    /// Data sets the associated data intended to be sent along with the request
    /// Depending on http method that is used, data will be serialized into the
    /// appropriate format
    pub fn data(&mut self, data: T) -> &mut Self {
        self.data = Some(data);
        self
    }

    /// Adds a http header to that will be used on the outgoing request
    pub fn header(&mut self, key: &'static str, value: &'static str) -> &mut Self {
        self.headers
            .append(key, header::HeaderValue::from_static(value));
        self
    }

    pub fn build(self) -> Result<client::Request<T>, Box<dyn std::error::Error + 'static>> {
        if let Some(data) = self.data {
            Ok(client::Request {
                data,
                headers: self.headers.to_owned(),
            })
        } else {
            Err(Box::new(BuildError()))
        }
    }
}

// TODO: define a better error
#[derive(Debug)]
pub struct BuildError();

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "nothing to transmit! request was configured without data"
        )
    }
}

impl Error for BuildError {
    fn description(&self) -> &str {
        "nothing to transmit! request was configured without data"
    }
}

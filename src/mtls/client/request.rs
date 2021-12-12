use serde::Serialize;

mod builder;
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

    pub fn builder() -> Builder<T> {
        Builder::new()
    }
}

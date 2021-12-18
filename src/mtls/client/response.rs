//! # response
//!
//! The response module contains entities and utilites related
//! to working with the output of calls to psd2 endpoints

#[derive(Debug, Clone)]
/// Response is a simple representation of a successful
/// response from a psd2 API endpoint
pub struct Response<T> {
    pub data: T,
    pub status_code: u16,
}

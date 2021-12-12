#[derive(Debug, Clone)]
/// Response is a simple representation of a successful
/// response from a psd2 API endpoint
pub struct Response<T> {
    pub body: T,
    pub status_code: u16,
}

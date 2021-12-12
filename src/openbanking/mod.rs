//! # openbanking

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenRequest<'a> {
    pub scope: &'a str,
    pub client_id: &'a str,
    pub grant_type: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

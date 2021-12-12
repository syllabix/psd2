use std::{error::Error, time::Duration};

use psd2::mtls::client;
use psd2::mtls::client::{Request, Response};
use psd2::openbanking::AccessToken;
use psd2::{mtls, openbanking};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cert = mtls::Certificate::load_from_x509_key_pair(
        ".certificates/transport.pem",
        ".certificates/private.key",
    )?;

    let client = mtls::Client::builder()
        .base_url("https://sandbox.some-bank.com")
        .certificate(cert)
        .insecure()
        .timeout(Duration::from_secs(10))
        .build()?;

    let payload = openbanking::TokenRequest {
        grant_type: "client_credentials",
        scope: "payments",
        client_id: "<a valid cliend>",
    };

    let req = client::Request::new(payload);

    let res: Response<AccessToken> = client.post_form("/token", req).await.unwrap();

    println!("{:?}", res);
    Ok(())
}

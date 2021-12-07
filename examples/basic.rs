use psd2::mtls;

// TODO: build out complete example
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert = mtls::Certificate::load_from_x509_key_pair(
        "./examples/certs/example_cert.crt",
        "./examples/certs/example_key.key",
    )?;

    let client = mtls::Client::builder()
        .base_url("https://test.ob.com")
        .default_header("Cache-Control", "no-cache")
        .certificate(cert)
        .build();

    Ok(())
}

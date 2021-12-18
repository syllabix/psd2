use psd2::mtls::client::{Request, Response};
use psd2::openbanking::{AccessToken, DomesticPaymentConsent};
use psd2::{mtls, openbanking};
use std::{error::Error, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cert = mtls::Certificate::load_from_x509_key_pair(
        ".certificates/transport.pem",
        ".certificates/private.key",
    )?;

    let client = mtls::Client::builder()
        .base_url("https://ob.bank.com")
        .certificate(cert)
        .insecure()
        .timeout(Duration::from_secs(10))
        .build()?;

    let api = mtls::Client::builder()
        .base_url("https://ob.bank.com")
        .insecure()
        .timeout(Duration::from_secs(10))
        .build()?;

    let payload = openbanking::TokenRequest {
        grant_type: "client_credentials",
        scope: "payments",
        client_id: "<client id>",
    };

    let req = Request::new(payload);
    let res: Response<AccessToken> = client.post_form("/token", req).await?;
    println!("Auth Response: {:?}", res);
    let payload = openbanking::DomesticPaymentConsent {
        data: openbanking::DomesticPaymentConsentData {
            status: None,
            status_update_date_time: None,
            creation_date_time: None,
            consent_id: None,
            initiation: openbanking::Initiation {
                instruction_identification: String::from("ID412"),
                end_to_end_identification: String::from("E2E123"),
                instructed_amount: openbanking::InstructedAmount {
                    amount: String::from("5.00"),
                    currency: String::from("GBP"),
                },
                creditor_account: openbanking::CreditorAccount {
                    scheme_name: String::from("UK.OBIE.SortCodeAccountNumber"),
                    identification: String::from("11223321325698"),
                    name: String::from("Receiver Co."),
                },
                remittance_information: openbanking::RemittanceInformation {
                    reference: String::from("PSD2.Rust.123"),
                    unstructured: String::from("Shipment Fee"),
                },
            },
        },
        risk: openbanking::Risk {
            payment_context_code: String::from("EcommerceGoods"),
            merchant_category_code: String::from("5967"),
            merchant_customer_identification: String::from("1238808123123"),
            delivery_address: openbanking::DeliveryAddress {
                address_line: vec!["7".to_string()],
                street_name: String::from("Apple Street"),
                building_number: String::from("1"),
                post_code: String::from("E2 7AA"),
                town_name: String::from("London"),
                country: String::from("UK"),
            },
        },
        links: None,
        meta: None,
    };

    let auth = ["Bearer", res.data.access_token.as_str()].join(" ");

    let mut req = Request::new(payload);
    req.header("x-fapi-financial-id", "001580000103UAvAAM")
        .header("Content-Type", "application/json")
        .header("x-idempotency-key", "123")
        .header("Authorization", &auth);

    mtls::client::request::sign::sign_request(&mut req);

    let res: Response<DomesticPaymentConsent> = api.post("/domestic-payment-consents", req).await?;

    println!("Payment Consent Response {:?}", res);
    Ok(())
}

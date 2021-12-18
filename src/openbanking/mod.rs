//! # openbanking
//!
//! the open banking module contains various types and utilities for
//! working directly with psd2 protocol, Open Banking (primarily UK)
//!

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

/// Domestic Payment Models

#[derive(Debug, Serialize, Deserialize)]
pub struct DomesticPaymentConsent {
    #[serde(rename = "Data")]
    pub data: DomesticPaymentConsentData,

    #[serde(rename = "Risk")]
    pub risk: Risk,

    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,

    #[serde(rename = "Meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomesticPaymentConsentData {
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "StatusUpdateDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_date_time: Option<String>,

    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<String>,

    #[serde(rename = "ConsentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_id: Option<String>,

    #[serde(rename = "Initiation")]
    pub initiation: Initiation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Initiation {
    #[serde(rename = "InstructionIdentification")]
    pub instruction_identification: String,
    #[serde(rename = "EndToEndIdentification")]
    pub end_to_end_identification: String,
    #[serde(rename = "InstructedAmount")]
    pub instructed_amount: InstructedAmount,
    #[serde(rename = "CreditorAccount")]
    pub creditor_account: CreditorAccount,
    #[serde(rename = "RemittanceInformation")]
    pub remittance_information: RemittanceInformation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditorAccount {
    #[serde(rename = "SchemeName")]
    pub scheme_name: String,
    #[serde(rename = "Identification")]
    pub identification: String,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstructedAmount {
    #[serde(rename = "Amount")]
    pub amount: String,
    #[serde(rename = "Currency")]
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemittanceInformation {
    #[serde(rename = "Reference")]
    pub reference: String,
    #[serde(rename = "Unstructured")]
    pub unstructured: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "Self")]
    pub links_self: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "TotalPages")]
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Risk {
    #[serde(rename = "PaymentContextCode")]
    pub payment_context_code: String,
    #[serde(rename = "MerchantCategoryCode")]
    pub merchant_category_code: String,
    #[serde(rename = "MerchantCustomerIdentification")]
    pub merchant_customer_identification: String,
    #[serde(rename = "DeliveryAddress")]
    pub delivery_address: DeliveryAddress,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryAddress {
    #[serde(rename = "AddressLine")]
    pub address_line: Vec<String>,
    #[serde(rename = "StreetName")]
    pub street_name: String,
    #[serde(rename = "BuildingNumber")]
    pub building_number: String,
    #[serde(rename = "PostCode")]
    pub post_code: String,
    #[serde(rename = "TownName")]
    pub town_name: String,
    #[serde(rename = "Country")]
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    pub message: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Errors")]
    pub errors: Option<Vec<ErrorElement>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorElement {
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    pub message: Option<String>,
}

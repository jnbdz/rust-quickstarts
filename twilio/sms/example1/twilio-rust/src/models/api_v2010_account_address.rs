/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.29.1
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiV2010AccountAddress {
    /// The SID of the Account that is responsible for the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The city in which the address is located
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The name associated with the address
    #[serde(rename = "customer_name", skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// Whether emergency calling has been enabled on this number
    #[serde(rename = "emergency_enabled", skip_serializing_if = "Option::is_none")]
    pub emergency_enabled: Option<bool>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The ISO country code of the address
    #[serde(rename = "iso_country", skip_serializing_if = "Option::is_none")]
    pub iso_country: Option<String>,
    /// The postal code of the address
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The state or region of the address
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The number and street address of the address
    #[serde(rename = "street", skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// Whether the address has been validated to comply with local regulation
    #[serde(rename = "validated", skip_serializing_if = "Option::is_none")]
    pub validated: Option<bool>,
    /// Whether the address has been verified to comply with regulation
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

impl ApiV2010AccountAddress {
    pub fn new() -> ApiV2010AccountAddress {
        ApiV2010AccountAddress {
            account_sid: None,
            city: None,
            customer_name: None,
            date_created: None,
            date_updated: None,
            emergency_enabled: None,
            friendly_name: None,
            iso_country: None,
            postal_code: None,
            region: None,
            sid: None,
            street: None,
            uri: None,
            validated: None,
            verified: None,
        }
    }
}


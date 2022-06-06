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
pub struct ApiV2010AccountSipSipDomainSipIpAccessControlListMapping {
    /// The unique id of the Account that is responsible for this resource.
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The date that this resource was created, given as GMT in RFC 2822 format.
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date that this resource was last updated, given as GMT in RFC 2822 format.
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The unique string that identifies the SipDomain resource.
    #[serde(rename = "domain_sid", skip_serializing_if = "Option::is_none")]
    pub domain_sid: Option<String>,
    /// A human readable descriptive text for this resource, up to 64 characters long.
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// A 34 character string that uniquely identifies this resource.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The URI for this resource, relative to https://api.twilio.com
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ApiV2010AccountSipSipDomainSipIpAccessControlListMapping {
    pub fn new() -> ApiV2010AccountSipSipDomainSipIpAccessControlListMapping {
        ApiV2010AccountSipSipDomainSipIpAccessControlListMapping {
            account_sid: None,
            date_created: None,
            date_updated: None,
            domain_sid: None,
            friendly_name: None,
            sid: None,
            uri: None,
        }
    }
}



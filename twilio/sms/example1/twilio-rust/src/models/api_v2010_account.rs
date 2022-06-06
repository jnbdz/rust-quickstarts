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
pub struct ApiV2010Account {
    /// The authorization token for this account
    #[serde(rename = "auth_token", skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// The date this account was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date this account was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// A human readable description of this account
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The unique 34 character id representing the parent of this account
    #[serde(rename = "owner_account_sid", skip_serializing_if = "Option::is_none")]
    pub owner_account_sid: Option<String>,
    /// A 34 character string that uniquely identifies this resource.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The status of this account
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Account Instance Subresources
    #[serde(rename = "subresource_uris", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<serde_json::Value>,
    /// The type of this account
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The URI for this resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ApiV2010Account {
    pub fn new() -> ApiV2010Account {
        ApiV2010Account {
            auth_token: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            owner_account_sid: None,
            sid: None,
            status: None,
            subresource_uris: None,
            _type: None,
            uri: None,
        }
    }
}

/// The status of this account
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "closed")]
    Closed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
/// The type of this account
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Trial")]
    Trial,
    #[serde(rename = "Full")]
    Full,
}

impl Default for Type {
    fn default() -> Type {
        Self::Trial
    }
}


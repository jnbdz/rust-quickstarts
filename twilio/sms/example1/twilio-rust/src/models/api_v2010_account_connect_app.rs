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
pub struct ApiV2010AccountConnectApp {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The URL to redirect the user to after authorization
    #[serde(rename = "authorize_redirect_url", skip_serializing_if = "Option::is_none")]
    pub authorize_redirect_url: Option<String>,
    /// The company name set for the Connect App
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// The HTTP method we use to call deauthorize_callback_url
    #[serde(rename = "deauthorize_callback_method", skip_serializing_if = "Option::is_none")]
    pub deauthorize_callback_method: Option<DeauthorizeCallbackMethod>,
    /// The URL we call to de-authorize the Connect App
    #[serde(rename = "deauthorize_callback_url", skip_serializing_if = "Option::is_none")]
    pub deauthorize_callback_url: Option<String>,
    /// The description of the Connect App
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The URL users can obtain more information
    #[serde(rename = "homepage_url", skip_serializing_if = "Option::is_none")]
    pub homepage_url: Option<String>,
    /// The set of permissions that your ConnectApp requests
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permissions>>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ApiV2010AccountConnectApp {
    pub fn new() -> ApiV2010AccountConnectApp {
        ApiV2010AccountConnectApp {
            account_sid: None,
            authorize_redirect_url: None,
            company_name: None,
            deauthorize_callback_method: None,
            deauthorize_callback_url: None,
            description: None,
            friendly_name: None,
            homepage_url: None,
            permissions: None,
            sid: None,
            uri: None,
        }
    }
}

/// The HTTP method we use to call deauthorize_callback_url
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeauthorizeCallbackMethod {
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "DELETE")]
    DELETE,
}

impl Default for DeauthorizeCallbackMethod {
    fn default() -> DeauthorizeCallbackMethod {
        Self::HEAD
    }
}
/// The set of permissions that your ConnectApp requests
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permissions {
    #[serde(rename = "get-all")]
    GetAll,
    #[serde(rename = "post-all")]
    PostAll,
}

impl Default for Permissions {
    fn default() -> Permissions {
        Self::GetAll
    }
}

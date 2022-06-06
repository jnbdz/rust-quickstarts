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
pub struct ApiV2010AccountTranscription {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The API version used to create the transcription
    #[serde(rename = "api_version", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The duration of the transcribed audio in seconds.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// The charge for the transcription
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    /// The currency in which price is measured
    #[serde(rename = "price_unit", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
    /// The SID that identifies the transcription's recording
    #[serde(rename = "recording_sid", skip_serializing_if = "Option::is_none")]
    pub recording_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The status of the transcription
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The text content of the transcription.
    #[serde(rename = "transcription_text", skip_serializing_if = "Option::is_none")]
    pub transcription_text: Option<String>,
    /// The transcription type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ApiV2010AccountTranscription {
    pub fn new() -> ApiV2010AccountTranscription {
        ApiV2010AccountTranscription {
            account_sid: None,
            api_version: None,
            date_created: None,
            date_updated: None,
            duration: None,
            price: None,
            price_unit: None,
            recording_sid: None,
            sid: None,
            status: None,
            transcription_text: None,
            _type: None,
            uri: None,
        }
    }
}

/// The status of the transcription
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::InProgress
    }
}


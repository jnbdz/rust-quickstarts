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
pub struct ApiV2010AccountConferenceConferenceRecording {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The API version used to create the recording
    #[serde(rename = "api_version", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// The SID of the Call the resource is associated with
    #[serde(rename = "call_sid", skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<String>,
    /// The number of channels in the final recording file as an integer
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<i32>,
    /// The Conference SID that identifies the conference associated with the recording
    #[serde(rename = "conference_sid", skip_serializing_if = "Option::is_none")]
    pub conference_sid: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The length of the recording in seconds
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// How to decrypt the recording.
    #[serde(rename = "encryption_details", skip_serializing_if = "Option::is_none")]
    pub encryption_details: Option<serde_json::Value>,
    /// More information about why the recording is missing, if status is `absent`.
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    /// The one-time cost of creating the recording.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// The currency used in the price property.
    #[serde(rename = "price_unit", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// How the recording was created
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// The start time of the recording, given in RFC 2822 format
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The status of the recording
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ApiV2010AccountConferenceConferenceRecording {
    pub fn new() -> ApiV2010AccountConferenceConferenceRecording {
        ApiV2010AccountConferenceConferenceRecording {
            account_sid: None,
            api_version: None,
            call_sid: None,
            channels: None,
            conference_sid: None,
            date_created: None,
            date_updated: None,
            duration: None,
            encryption_details: None,
            error_code: None,
            price: None,
            price_unit: None,
            sid: None,
            source: None,
            start_time: None,
            status: None,
            uri: None,
        }
    }
}

/// How the recording was created
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "DialVerb")]
    DialVerb,
    #[serde(rename = "Conference")]
    Conference,
    #[serde(rename = "OutboundAPI")]
    OutboundAPI,
    #[serde(rename = "Trunking")]
    Trunking,
    #[serde(rename = "RecordVerb")]
    RecordVerb,
    #[serde(rename = "StartCallRecordingAPI")]
    StartCallRecordingAPI,
    #[serde(rename = "StartConferenceRecordingAPI")]
    StartConferenceRecordingAPI,
}

impl Default for Source {
    fn default() -> Source {
        Self::DialVerb
    }
}
/// The status of the recording
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "absent")]
    Absent,
}

impl Default for Status {
    fn default() -> Status {
        Self::InProgress
    }
}


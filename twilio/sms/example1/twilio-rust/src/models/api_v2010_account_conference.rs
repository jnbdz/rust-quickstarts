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
pub struct ApiV2010AccountConference {
    /// The SID of the Account that created this resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The API version used to create this conference
    #[serde(rename = "api_version", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// The call SID that caused the conference to end
    #[serde(rename = "call_sid_ending_conference", skip_serializing_if = "Option::is_none")]
    pub call_sid_ending_conference: Option<String>,
    /// The RFC 2822 date and time in GMT that this resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT that this resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// A string that you assigned to describe this conference room
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The reason why a conference ended.
    #[serde(rename = "reason_conference_ended", skip_serializing_if = "Option::is_none")]
    pub reason_conference_ended: Option<ReasonConferenceEnded>,
    /// A string that represents the Twilio Region where the conference was mixed
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The unique string that identifies this resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The status of this conference
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// A list of related resources identified by their relative URIs
    #[serde(rename = "subresource_uris", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<serde_json::Value>,
    /// The URI of this resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ApiV2010AccountConference {
    pub fn new() -> ApiV2010AccountConference {
        ApiV2010AccountConference {
            account_sid: None,
            api_version: None,
            call_sid_ending_conference: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            reason_conference_ended: None,
            region: None,
            sid: None,
            status: None,
            subresource_uris: None,
            uri: None,
        }
    }
}

/// The reason why a conference ended.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReasonConferenceEnded {
    #[serde(rename = "conference-ended-via-api")]
    ConferenceEndedViaApi,
    #[serde(rename = "participant-with-end-conference-on-exit-left")]
    ParticipantWithEndConferenceOnExitLeft,
    #[serde(rename = "participant-with-end-conference-on-exit-kicked")]
    ParticipantWithEndConferenceOnExitKicked,
    #[serde(rename = "last-participant-kicked")]
    LastParticipantKicked,
    #[serde(rename = "last-participant-left")]
    LastParticipantLeft,
}

impl Default for ReasonConferenceEnded {
    fn default() -> ReasonConferenceEnded {
        Self::ConferenceEndedViaApi
    }
}
/// The status of this conference
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "init")]
    Init,
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Init
    }
}

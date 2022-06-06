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
pub struct ApiV2010AccountCallCallFeedbackSummary {
    /// The unique sid that identifies this account
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The total number of calls
    #[serde(rename = "call_count", skip_serializing_if = "Option::is_none")]
    pub call_count: Option<i32>,
    /// The total number of calls with a feedback entry
    #[serde(rename = "call_feedback_count", skip_serializing_if = "Option::is_none")]
    pub call_feedback_count: Option<i32>,
    /// The date this resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date this resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The latest feedback entry date in the summary
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Whether the feedback summary includes subaccounts
    #[serde(rename = "include_subaccounts", skip_serializing_if = "Option::is_none")]
    pub include_subaccounts: Option<bool>,
    /// Issues experienced during the call
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<serde_json::Value>>,
    /// The average QualityScore of the feedback entries
    #[serde(rename = "quality_score_average", skip_serializing_if = "Option::is_none")]
    pub quality_score_average: Option<f32>,
    /// The median QualityScore of the feedback entries
    #[serde(rename = "quality_score_median", skip_serializing_if = "Option::is_none")]
    pub quality_score_median: Option<f32>,
    /// The standard deviation of the quality scores
    #[serde(rename = "quality_score_standard_deviation", skip_serializing_if = "Option::is_none")]
    pub quality_score_standard_deviation: Option<f32>,
    /// A string that uniquely identifies this feedback entry
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The earliest feedback entry date in the summary
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The status of the feedback summary
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl ApiV2010AccountCallCallFeedbackSummary {
    pub fn new() -> ApiV2010AccountCallCallFeedbackSummary {
        ApiV2010AccountCallCallFeedbackSummary {
            account_sid: None,
            call_count: None,
            call_feedback_count: None,
            date_created: None,
            date_updated: None,
            end_date: None,
            include_subaccounts: None,
            issues: None,
            quality_score_average: None,
            quality_score_median: None,
            quality_score_standard_deviation: None,
            sid: None,
            start_date: None,
            status: None,
        }
    }
}

/// The status of the feedback summary
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Queued
    }
}


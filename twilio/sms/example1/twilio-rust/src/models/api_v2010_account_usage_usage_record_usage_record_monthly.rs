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
pub struct ApiV2010AccountUsageUsageRecordUsageRecordMonthly {
    /// The SID of the Account accrued the usage
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The API version used to create the resource
    #[serde(rename = "api_version", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Usage records up to date as of this timestamp
    #[serde(rename = "as_of", skip_serializing_if = "Option::is_none")]
    pub as_of: Option<String>,
    /// The category of usage
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    /// The number of usage events
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    /// The units in which count is measured
    #[serde(rename = "count_unit", skip_serializing_if = "Option::is_none")]
    pub count_unit: Option<String>,
    /// A plain-language description of the usage category
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The last date for which usage is included in the UsageRecord
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// The total price of the usage
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    /// The currency in which `price` is measured
    #[serde(rename = "price_unit", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
    /// The first date for which usage is included in this UsageRecord
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// A list of related resources identified by their relative URIs
    #[serde(rename = "subresource_uris", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<serde_json::Value>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The amount of usage
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    /// The units in which usage is measured
    #[serde(rename = "usage_unit", skip_serializing_if = "Option::is_none")]
    pub usage_unit: Option<String>,
}

impl ApiV2010AccountUsageUsageRecordUsageRecordMonthly {
    pub fn new() -> ApiV2010AccountUsageUsageRecordUsageRecordMonthly {
        ApiV2010AccountUsageUsageRecordUsageRecordMonthly {
            account_sid: None,
            api_version: None,
            as_of: None,
            category: None,
            count: None,
            count_unit: None,
            description: None,
            end_date: None,
            price: None,
            price_unit: None,
            start_date: None,
            subresource_uris: None,
            uri: None,
            usage: None,
            usage_unit: None,
        }
    }
}

/// The category of usage
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "a2p-registration-fees")]
    A2pRegistrationFees,
    #[serde(rename = "agent-conference")]
    AgentConference,
    #[serde(rename = "answering-machine-detection")]
    AnsweringMachineDetection,
    #[serde(rename = "authy-authentications")]
    AuthyAuthentications,
    #[serde(rename = "authy-calls-outbound")]
    AuthyCallsOutbound,
    #[serde(rename = "authy-monthly-fees")]
    AuthyMonthlyFees,
    #[serde(rename = "authy-phone-intelligence")]
    AuthyPhoneIntelligence,
    #[serde(rename = "authy-phone-verifications")]
    AuthyPhoneVerifications,
    #[serde(rename = "authy-sms-outbound")]
    AuthySmsOutbound,
    #[serde(rename = "call-progess-events")]
    CallProgessEvents,
    #[serde(rename = "calleridlookups")]
    Calleridlookups,
    #[serde(rename = "calls")]
    Calls,
    #[serde(rename = "calls-client")]
    CallsClient,
    #[serde(rename = "calls-globalconference")]
    CallsGlobalconference,
    #[serde(rename = "calls-inbound")]
    CallsInbound,
    #[serde(rename = "calls-inbound-local")]
    CallsInboundLocal,
    #[serde(rename = "calls-inbound-mobile")]
    CallsInboundMobile,
    #[serde(rename = "calls-inbound-tollfree")]
    CallsInboundTollfree,
    #[serde(rename = "calls-outbound")]
    CallsOutbound,
    #[serde(rename = "calls-pay-verb-transactions")]
    CallsPayVerbTransactions,
    #[serde(rename = "calls-recordings")]
    CallsRecordings,
    #[serde(rename = "calls-sip")]
    CallsSip,
    #[serde(rename = "calls-sip-inbound")]
    CallsSipInbound,
    #[serde(rename = "calls-sip-outbound")]
    CallsSipOutbound,
    #[serde(rename = "calls-transfers")]
    CallsTransfers,
    #[serde(rename = "carrier-lookups")]
    CarrierLookups,
    #[serde(rename = "conversations")]
    Conversations,
    #[serde(rename = "conversations-api-requests")]
    ConversationsApiRequests,
    #[serde(rename = "conversations-conversation-events")]
    ConversationsConversationEvents,
    #[serde(rename = "conversations-endpoint-connectivity")]
    ConversationsEndpointConnectivity,
    #[serde(rename = "conversations-events")]
    ConversationsEvents,
    #[serde(rename = "conversations-participant-events")]
    ConversationsParticipantEvents,
    #[serde(rename = "conversations-participants")]
    ConversationsParticipants,
    #[serde(rename = "cps")]
    Cps,
    #[serde(rename = "flex-usage")]
    FlexUsage,
    #[serde(rename = "fraud-lookups")]
    FraudLookups,
    #[serde(rename = "group-rooms")]
    GroupRooms,
    #[serde(rename = "group-rooms-data-track")]
    GroupRoomsDataTrack,
    #[serde(rename = "group-rooms-encrypted-media-recorded")]
    GroupRoomsEncryptedMediaRecorded,
    #[serde(rename = "group-rooms-media-downloaded")]
    GroupRoomsMediaDownloaded,
    #[serde(rename = "group-rooms-media-recorded")]
    GroupRoomsMediaRecorded,
    #[serde(rename = "group-rooms-media-routed")]
    GroupRoomsMediaRouted,
    #[serde(rename = "group-rooms-media-stored")]
    GroupRoomsMediaStored,
    #[serde(rename = "group-rooms-participant-minutes")]
    GroupRoomsParticipantMinutes,
    #[serde(rename = "group-rooms-recorded-minutes")]
    GroupRoomsRecordedMinutes,
    #[serde(rename = "imp-v1-usage")]
    ImpV1Usage,
    #[serde(rename = "lookups")]
    Lookups,
    #[serde(rename = "marketplace")]
    Marketplace,
    #[serde(rename = "marketplace-algorithmia-named-entity-recognition")]
    MarketplaceAlgorithmiaNamedEntityRecognition,
    #[serde(rename = "marketplace-cadence-transcription")]
    MarketplaceCadenceTranscription,
    #[serde(rename = "marketplace-cadence-translation")]
    MarketplaceCadenceTranslation,
    #[serde(rename = "marketplace-capio-speech-to-text")]
    MarketplaceCapioSpeechToText,
    #[serde(rename = "marketplace-convriza-ababa")]
    MarketplaceConvrizaAbaba,
    #[serde(rename = "marketplace-deepgram-phrase-detector")]
    MarketplaceDeepgramPhraseDetector,
    #[serde(rename = "marketplace-digital-segment-business-info")]
    MarketplaceDigitalSegmentBusinessInfo,
    #[serde(rename = "marketplace-facebook-offline-conversions")]
    MarketplaceFacebookOfflineConversions,
    #[serde(rename = "marketplace-google-speech-to-text")]
    MarketplaceGoogleSpeechToText,
    #[serde(rename = "marketplace-ibm-watson-message-insights")]
    MarketplaceIbmWatsonMessageInsights,
    #[serde(rename = "marketplace-ibm-watson-message-sentiment")]
    MarketplaceIbmWatsonMessageSentiment,
    #[serde(rename = "marketplace-ibm-watson-recording-analysis")]
    MarketplaceIbmWatsonRecordingAnalysis,
    #[serde(rename = "marketplace-ibm-watson-tone-analyzer")]
    MarketplaceIbmWatsonToneAnalyzer,
    #[serde(rename = "marketplace-icehook-systems-scout")]
    MarketplaceIcehookSystemsScout,
    #[serde(rename = "marketplace-infogroup-dataaxle-bizinfo")]
    MarketplaceInfogroupDataaxleBizinfo,
    #[serde(rename = "marketplace-keen-io-contact-center-analytics")]
    MarketplaceKeenIoContactCenterAnalytics,
    #[serde(rename = "marketplace-marchex-cleancall")]
    MarketplaceMarchexCleancall,
    #[serde(rename = "marketplace-marchex-sentiment-analysis-for-sms")]
    MarketplaceMarchexSentimentAnalysisForSms,
    #[serde(rename = "marketplace-marketplace-nextcaller-social-id")]
    MarketplaceMarketplaceNextcallerSocialId,
    #[serde(rename = "marketplace-mobile-commons-opt-out-classifier")]
    MarketplaceMobileCommonsOptOutClassifier,
    #[serde(rename = "marketplace-nexiwave-voicemail-to-text")]
    MarketplaceNexiwaveVoicemailToText,
    #[serde(rename = "marketplace-nextcaller-advanced-caller-identification")]
    MarketplaceNextcallerAdvancedCallerIdentification,
    #[serde(rename = "marketplace-nomorobo-spam-score")]
    MarketplaceNomoroboSpamScore,
    #[serde(rename = "marketplace-payfone-tcpa-compliance")]
    MarketplacePayfoneTcpaCompliance,
    #[serde(rename = "marketplace-remeeting-automatic-speech-recognition")]
    MarketplaceRemeetingAutomaticSpeechRecognition,
    #[serde(rename = "marketplace-tcpa-defense-solutions-blacklist-feed")]
    MarketplaceTcpaDefenseSolutionsBlacklistFeed,
    #[serde(rename = "marketplace-telo-opencnam")]
    MarketplaceTeloOpencnam,
    #[serde(rename = "marketplace-truecnam-true-spam")]
    MarketplaceTruecnamTrueSpam,
    #[serde(rename = "marketplace-twilio-caller-name-lookup-us")]
    MarketplaceTwilioCallerNameLookupUs,
    #[serde(rename = "marketplace-twilio-carrier-information-lookup")]
    MarketplaceTwilioCarrierInformationLookup,
    #[serde(rename = "marketplace-voicebase-pci")]
    MarketplaceVoicebasePci,
    #[serde(rename = "marketplace-voicebase-transcription")]
    MarketplaceVoicebaseTranscription,
    #[serde(rename = "marketplace-voicebase-transcription-custom-vocabulary")]
    MarketplaceVoicebaseTranscriptionCustomVocabulary,
    #[serde(rename = "marketplace-whitepages-pro-caller-identification")]
    MarketplaceWhitepagesProCallerIdentification,
    #[serde(rename = "marketplace-whitepages-pro-phone-intelligence")]
    MarketplaceWhitepagesProPhoneIntelligence,
    #[serde(rename = "marketplace-whitepages-pro-phone-reputation")]
    MarketplaceWhitepagesProPhoneReputation,
    #[serde(rename = "marketplace-wolfarm-spoken-results")]
    MarketplaceWolfarmSpokenResults,
    #[serde(rename = "marketplace-wolfram-short-answer")]
    MarketplaceWolframShortAnswer,
    #[serde(rename = "marketplace-ytica-contact-center-reporting-analytics")]
    MarketplaceYticaContactCenterReportingAnalytics,
    #[serde(rename = "mediastorage")]
    Mediastorage,
    #[serde(rename = "mms")]
    Mms,
    #[serde(rename = "mms-inbound")]
    MmsInbound,
    #[serde(rename = "mms-inbound-longcode")]
    MmsInboundLongcode,
    #[serde(rename = "mms-inbound-shortcode")]
    MmsInboundShortcode,
    #[serde(rename = "mms-messages-carrierfees")]
    MmsMessagesCarrierfees,
    #[serde(rename = "mms-outbound")]
    MmsOutbound,
    #[serde(rename = "mms-outbound-longcode")]
    MmsOutboundLongcode,
    #[serde(rename = "mms-outbound-shortcode")]
    MmsOutboundShortcode,
    #[serde(rename = "monitor-reads")]
    MonitorReads,
    #[serde(rename = "monitor-storage")]
    MonitorStorage,
    #[serde(rename = "monitor-writes")]
    MonitorWrites,
    #[serde(rename = "notify")]
    Notify,
    #[serde(rename = "notify-actions-attempts")]
    NotifyActionsAttempts,
    #[serde(rename = "notify-channels")]
    NotifyChannels,
    #[serde(rename = "number-format-lookups")]
    NumberFormatLookups,
    #[serde(rename = "pchat")]
    Pchat,
    #[serde(rename = "pchat-users")]
    PchatUsers,
    #[serde(rename = "peer-to-peer-rooms-participant-minutes")]
    PeerToPeerRoomsParticipantMinutes,
    #[serde(rename = "pfax")]
    Pfax,
    #[serde(rename = "pfax-minutes")]
    PfaxMinutes,
    #[serde(rename = "pfax-minutes-inbound")]
    PfaxMinutesInbound,
    #[serde(rename = "pfax-minutes-outbound")]
    PfaxMinutesOutbound,
    #[serde(rename = "pfax-pages")]
    PfaxPages,
    #[serde(rename = "phonenumbers")]
    Phonenumbers,
    #[serde(rename = "phonenumbers-cps")]
    PhonenumbersCps,
    #[serde(rename = "phonenumbers-emergency")]
    PhonenumbersEmergency,
    #[serde(rename = "phonenumbers-local")]
    PhonenumbersLocal,
    #[serde(rename = "phonenumbers-mobile")]
    PhonenumbersMobile,
    #[serde(rename = "phonenumbers-setups")]
    PhonenumbersSetups,
    #[serde(rename = "phonenumbers-tollfree")]
    PhonenumbersTollfree,
    #[serde(rename = "premiumsupport")]
    Premiumsupport,
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "proxy-active-sessions")]
    ProxyActiveSessions,
    #[serde(rename = "pstnconnectivity")]
    Pstnconnectivity,
    #[serde(rename = "pv")]
    Pv,
    #[serde(rename = "pv-composition-media-downloaded")]
    PvCompositionMediaDownloaded,
    #[serde(rename = "pv-composition-media-encrypted")]
    PvCompositionMediaEncrypted,
    #[serde(rename = "pv-composition-media-stored")]
    PvCompositionMediaStored,
    #[serde(rename = "pv-composition-minutes")]
    PvCompositionMinutes,
    #[serde(rename = "pv-recording-compositions")]
    PvRecordingCompositions,
    #[serde(rename = "pv-room-participants")]
    PvRoomParticipants,
    #[serde(rename = "pv-room-participants-au1")]
    PvRoomParticipantsAu1,
    #[serde(rename = "pv-room-participants-br1")]
    PvRoomParticipantsBr1,
    #[serde(rename = "pv-room-participants-ie1")]
    PvRoomParticipantsIe1,
    #[serde(rename = "pv-room-participants-jp1")]
    PvRoomParticipantsJp1,
    #[serde(rename = "pv-room-participants-sg1")]
    PvRoomParticipantsSg1,
    #[serde(rename = "pv-room-participants-us1")]
    PvRoomParticipantsUs1,
    #[serde(rename = "pv-room-participants-us2")]
    PvRoomParticipantsUs2,
    #[serde(rename = "pv-rooms")]
    PvRooms,
    #[serde(rename = "pv-sip-endpoint-registrations")]
    PvSipEndpointRegistrations,
    #[serde(rename = "recordings")]
    Recordings,
    #[serde(rename = "recordingstorage")]
    Recordingstorage,
    #[serde(rename = "rooms-group-bandwidth")]
    RoomsGroupBandwidth,
    #[serde(rename = "rooms-group-minutes")]
    RoomsGroupMinutes,
    #[serde(rename = "rooms-peer-to-peer-minutes")]
    RoomsPeerToPeerMinutes,
    #[serde(rename = "shortcodes")]
    Shortcodes,
    #[serde(rename = "shortcodes-customerowned")]
    ShortcodesCustomerowned,
    #[serde(rename = "shortcodes-mms-enablement")]
    ShortcodesMmsEnablement,
    #[serde(rename = "shortcodes-mps")]
    ShortcodesMps,
    #[serde(rename = "shortcodes-random")]
    ShortcodesRandom,
    #[serde(rename = "shortcodes-uk")]
    ShortcodesUk,
    #[serde(rename = "shortcodes-vanity")]
    ShortcodesVanity,
    #[serde(rename = "small-group-rooms")]
    SmallGroupRooms,
    #[serde(rename = "small-group-rooms-data-track")]
    SmallGroupRoomsDataTrack,
    #[serde(rename = "small-group-rooms-participant-minutes")]
    SmallGroupRoomsParticipantMinutes,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "sms-inbound")]
    SmsInbound,
    #[serde(rename = "sms-inbound-longcode")]
    SmsInboundLongcode,
    #[serde(rename = "sms-inbound-shortcode")]
    SmsInboundShortcode,
    #[serde(rename = "sms-messages-carrierfees")]
    SmsMessagesCarrierfees,
    #[serde(rename = "sms-messages-features")]
    SmsMessagesFeatures,
    #[serde(rename = "sms-messages-features-senderid")]
    SmsMessagesFeaturesSenderid,
    #[serde(rename = "sms-outbound")]
    SmsOutbound,
    #[serde(rename = "sms-outbound-content-inspection")]
    SmsOutboundContentInspection,
    #[serde(rename = "sms-outbound-longcode")]
    SmsOutboundLongcode,
    #[serde(rename = "sms-outbound-shortcode")]
    SmsOutboundShortcode,
    #[serde(rename = "speech-recognition")]
    SpeechRecognition,
    #[serde(rename = "studio-engagements")]
    StudioEngagements,
    #[serde(rename = "sync")]
    Sync,
    #[serde(rename = "sync-actions")]
    SyncActions,
    #[serde(rename = "sync-endpoint-hours")]
    SyncEndpointHours,
    #[serde(rename = "sync-endpoint-hours-above-daily-cap")]
    SyncEndpointHoursAboveDailyCap,
    #[serde(rename = "taskrouter-tasks")]
    TaskrouterTasks,
    #[serde(rename = "totalprice")]
    Totalprice,
    #[serde(rename = "transcriptions")]
    Transcriptions,
    #[serde(rename = "trunking-cps")]
    TrunkingCps,
    #[serde(rename = "trunking-emergency-calls")]
    TrunkingEmergencyCalls,
    #[serde(rename = "trunking-origination")]
    TrunkingOrigination,
    #[serde(rename = "trunking-origination-local")]
    TrunkingOriginationLocal,
    #[serde(rename = "trunking-origination-mobile")]
    TrunkingOriginationMobile,
    #[serde(rename = "trunking-origination-tollfree")]
    TrunkingOriginationTollfree,
    #[serde(rename = "trunking-recordings")]
    TrunkingRecordings,
    #[serde(rename = "trunking-secure")]
    TrunkingSecure,
    #[serde(rename = "trunking-termination")]
    TrunkingTermination,
    #[serde(rename = "turnmegabytes")]
    Turnmegabytes,
    #[serde(rename = "turnmegabytes-australia")]
    TurnmegabytesAustralia,
    #[serde(rename = "turnmegabytes-brasil")]
    TurnmegabytesBrasil,
    #[serde(rename = "turnmegabytes-germany")]
    TurnmegabytesGermany,
    #[serde(rename = "turnmegabytes-india")]
    TurnmegabytesIndia,
    #[serde(rename = "turnmegabytes-ireland")]
    TurnmegabytesIreland,
    #[serde(rename = "turnmegabytes-japan")]
    TurnmegabytesJapan,
    #[serde(rename = "turnmegabytes-singapore")]
    TurnmegabytesSingapore,
    #[serde(rename = "turnmegabytes-useast")]
    TurnmegabytesUseast,
    #[serde(rename = "turnmegabytes-uswest")]
    TurnmegabytesUswest,
    #[serde(rename = "twilio-interconnect")]
    TwilioInterconnect,
    #[serde(rename = "verify-push")]
    VerifyPush,
    #[serde(rename = "verify-totp")]
    VerifyTotp,
    #[serde(rename = "verify-whatsapp-conversations-business-initiated")]
    VerifyWhatsappConversationsBusinessInitiated,
    #[serde(rename = "video-recordings")]
    VideoRecordings,
    #[serde(rename = "voice-insights")]
    VoiceInsights,
    #[serde(rename = "voice-insights-client-insights-on-demand-minute")]
    VoiceInsightsClientInsightsOnDemandMinute,
    #[serde(rename = "voice-insights-ptsn-insights-on-demand-minute")]
    VoiceInsightsPtsnInsightsOnDemandMinute,
    #[serde(rename = "voice-insights-sip-interface-insights-on-demand-minute")]
    VoiceInsightsSipInterfaceInsightsOnDemandMinute,
    #[serde(rename = "voice-insights-sip-trunking-insights-on-demand-minute")]
    VoiceInsightsSipTrunkingInsightsOnDemandMinute,
    #[serde(rename = "wireless")]
    Wireless,
    #[serde(rename = "wireless-orders")]
    WirelessOrders,
    #[serde(rename = "wireless-orders-artwork")]
    WirelessOrdersArtwork,
    #[serde(rename = "wireless-orders-bulk")]
    WirelessOrdersBulk,
    #[serde(rename = "wireless-orders-esim")]
    WirelessOrdersEsim,
    #[serde(rename = "wireless-orders-starter")]
    WirelessOrdersStarter,
    #[serde(rename = "wireless-usage")]
    WirelessUsage,
    #[serde(rename = "wireless-usage-commands")]
    WirelessUsageCommands,
    #[serde(rename = "wireless-usage-commands-africa")]
    WirelessUsageCommandsAfrica,
    #[serde(rename = "wireless-usage-commands-asia")]
    WirelessUsageCommandsAsia,
    #[serde(rename = "wireless-usage-commands-centralandsouthamerica")]
    WirelessUsageCommandsCentralandsouthamerica,
    #[serde(rename = "wireless-usage-commands-europe")]
    WirelessUsageCommandsEurope,
    #[serde(rename = "wireless-usage-commands-home")]
    WirelessUsageCommandsHome,
    #[serde(rename = "wireless-usage-commands-northamerica")]
    WirelessUsageCommandsNorthamerica,
    #[serde(rename = "wireless-usage-commands-oceania")]
    WirelessUsageCommandsOceania,
    #[serde(rename = "wireless-usage-commands-roaming")]
    WirelessUsageCommandsRoaming,
    #[serde(rename = "wireless-usage-data")]
    WirelessUsageData,
    #[serde(rename = "wireless-usage-data-africa")]
    WirelessUsageDataAfrica,
    #[serde(rename = "wireless-usage-data-asia")]
    WirelessUsageDataAsia,
    #[serde(rename = "wireless-usage-data-centralandsouthamerica")]
    WirelessUsageDataCentralandsouthamerica,
    #[serde(rename = "wireless-usage-data-custom-additionalmb")]
    WirelessUsageDataCustomAdditionalmb,
    #[serde(rename = "wireless-usage-data-custom-first5mb")]
    WirelessUsageDataCustomFirst5mb,
    #[serde(rename = "wireless-usage-data-domestic-roaming")]
    WirelessUsageDataDomesticRoaming,
    #[serde(rename = "wireless-usage-data-europe")]
    WirelessUsageDataEurope,
    #[serde(rename = "wireless-usage-data-individual-additionalgb")]
    WirelessUsageDataIndividualAdditionalgb,
    #[serde(rename = "wireless-usage-data-individual-firstgb")]
    WirelessUsageDataIndividualFirstgb,
    #[serde(rename = "wireless-usage-data-international-roaming-canada")]
    WirelessUsageDataInternationalRoamingCanada,
    #[serde(rename = "wireless-usage-data-international-roaming-india")]
    WirelessUsageDataInternationalRoamingIndia,
    #[serde(rename = "wireless-usage-data-international-roaming-mexico")]
    WirelessUsageDataInternationalRoamingMexico,
    #[serde(rename = "wireless-usage-data-northamerica")]
    WirelessUsageDataNorthamerica,
    #[serde(rename = "wireless-usage-data-oceania")]
    WirelessUsageDataOceania,
    #[serde(rename = "wireless-usage-data-pooled")]
    WirelessUsageDataPooled,
    #[serde(rename = "wireless-usage-data-pooled-downlink")]
    WirelessUsageDataPooledDownlink,
    #[serde(rename = "wireless-usage-data-pooled-uplink")]
    WirelessUsageDataPooledUplink,
    #[serde(rename = "wireless-usage-mrc")]
    WirelessUsageMrc,
    #[serde(rename = "wireless-usage-mrc-custom")]
    WirelessUsageMrcCustom,
    #[serde(rename = "wireless-usage-mrc-individual")]
    WirelessUsageMrcIndividual,
    #[serde(rename = "wireless-usage-mrc-pooled")]
    WirelessUsageMrcPooled,
    #[serde(rename = "wireless-usage-mrc-suspended")]
    WirelessUsageMrcSuspended,
    #[serde(rename = "wireless-usage-sms")]
    WirelessUsageSms,
    #[serde(rename = "wireless-usage-voice")]
    WirelessUsageVoice,
}

impl Default for Category {
    fn default() -> Category {
        Self::A2pRegistrationFees
    }
}


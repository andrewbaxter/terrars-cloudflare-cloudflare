#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataStreamLiveInputData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    live_input_identifier: terrars::PrimField<std::string::String>,
}

struct DataStreamLiveInput_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataStreamLiveInputData_>,
}

#[derive(Clone)]
pub struct DataStreamLiveInput(std::rc::Rc<DataStreamLiveInput_>);

impl DataStreamLiveInput {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderCloudflare) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nThe date and time the live input was created."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_recording_after_days` after provisioning.\nIndicates the number of days after which the live inputs recordings will be deleted. When a stream completes and the recording is ready, the value is used to calculate a scheduled deletion date for that recording. Omit the field to indicate no change, or include with a `null` value to remove an existing scheduled deletion."]
    pub fn delete_recording_after_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_recording_after_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicates whether the live input is enabled and can accept streams."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `keys_rotated_at` after provisioning.\nThe date and time the live input keys were last rotated. Omitted for live inputs that have never had their keys rotated."]
    pub fn keys_rotated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keys_rotated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `live_input_identifier` after provisioning.\nA unique identifier for a live input."]
    pub fn live_input_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.live_input_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meta` after provisioning.\nA user modifiable key-value store used to reference other systems of record for managing live inputs."]
    pub fn meta(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified` after provisioning.\nThe date and time the live input was last modified."]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `playback` after provisioning.\nDetails for playing a live input's broadcast using the HLS or DASH manifests. URLs reference the live input ID."]
    pub fn playback(&self) -> DataStreamLiveInput_PlaybackRef {
        DataStreamLiveInput_PlaybackRef::new(self.shared().clone(), format!("{}.playback", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `prefer_low_latency` after provisioning.\nWhen enabled, the live stream is delivered using Low-Latency HLS (LL-HLS), reducing glass-to-glass latency for viewers at the cost of reduced player compatibility."]
    pub fn prefer_low_latency(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefer_low_latency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recording` after provisioning.\nRecords the input to a Cloudflare Stream video. Behavior depends on the mode. In most cases, the video will initially be viewable as a live video and transition to on-demand after a condition is satisfied."]
    pub fn recording(&self) -> DataStreamLiveInput_RecordingRef {
        DataStreamLiveInput_RecordingRef::new(self.shared().clone(), format!("{}.recording", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rtmps` after provisioning.\nDetails for streaming to an live input using RTMPS."]
    pub fn rtmps(&self) -> DataStreamLiveInput_RtmpsRef {
        DataStreamLiveInput_RtmpsRef::new(self.shared().clone(), format!("{}.rtmps", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rtmps_playback` after provisioning.\nDetails for playback from an live input using RTMPS."]
    pub fn rtmps_playback(&self) -> DataStreamLiveInput_RtmpsPlaybackRef {
        DataStreamLiveInput_RtmpsPlaybackRef::new(
            self.shared().clone(),
            format!("{}.rtmps_playback", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `srt` after provisioning.\nDetails for streaming to a live input using SRT."]
    pub fn srt(&self) -> DataStreamLiveInput_SrtRef {
        DataStreamLiveInput_SrtRef::new(self.shared().clone(), format!("{}.srt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `srt_playback` after provisioning.\nDetails for playback from an live input using SRT."]
    pub fn srt_playback(&self) -> DataStreamLiveInput_SrtPlaybackRef {
        DataStreamLiveInput_SrtPlaybackRef::new(
            self.shared().clone(),
            format!("{}.srt_playback", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe connection status of a live input.\nAvailable values: \"connected\", \"reconnected\", \"reconnecting\", \"client_disconnect\", \"ttl_exceeded\", \"failed_to_connect\", \"failed_to_reconnect\", \"new_configuration_accepted\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nA unique identifier for a live input."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `web_rtc` after provisioning.\nDetails for streaming to a live input using WebRTC."]
    pub fn web_rtc(&self) -> DataStreamLiveInput_WebRtcRef {
        DataStreamLiveInput_WebRtcRef::new(self.shared().clone(), format!("{}.web_rtc", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `web_rtc_playback` after provisioning.\nDetails for playback from a live input using WebRTC."]
    pub fn web_rtc_playback(&self) -> DataStreamLiveInput_WebRtcPlaybackRef {
        DataStreamLiveInput_WebRtcPlaybackRef::new(
            self.shared().clone(),
            format!("{}.web_rtc_playback", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataStreamLiveInput {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataStreamLiveInput { }

impl terrars::ToListMappable for DataStreamLiveInput {
    type O = terrars::ListRef<DataStreamLiveInputRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataStreamLiveInput_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_stream_live_input".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataStreamLiveInput {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "A unique identifier for a live input."]
    pub live_input_identifier: terrars::PrimField<std::string::String>,
}

impl BuildDataStreamLiveInput {
    pub fn build(self, stack: &mut terrars::Stack) -> DataStreamLiveInput {
        let out = DataStreamLiveInput(std::rc::Rc::new(DataStreamLiveInput_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataStreamLiveInputData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                live_input_identifier: self.live_input_identifier,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataStreamLiveInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreamLiveInputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataStreamLiveInputRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nThe date and time the live input was created."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_recording_after_days` after provisioning.\nIndicates the number of days after which the live inputs recordings will be deleted. When a stream completes and the recording is ready, the value is used to calculate a scheduled deletion date for that recording. Omit the field to indicate no change, or include with a `null` value to remove an existing scheduled deletion."]
    pub fn delete_recording_after_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_recording_after_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicates whether the live input is enabled and can accept streams."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `keys_rotated_at` after provisioning.\nThe date and time the live input keys were last rotated. Omitted for live inputs that have never had their keys rotated."]
    pub fn keys_rotated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keys_rotated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `live_input_identifier` after provisioning.\nA unique identifier for a live input."]
    pub fn live_input_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.live_input_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meta` after provisioning.\nA user modifiable key-value store used to reference other systems of record for managing live inputs."]
    pub fn meta(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified` after provisioning.\nThe date and time the live input was last modified."]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `playback` after provisioning.\nDetails for playing a live input's broadcast using the HLS or DASH manifests. URLs reference the live input ID."]
    pub fn playback(&self) -> DataStreamLiveInput_PlaybackRef {
        DataStreamLiveInput_PlaybackRef::new(self.shared().clone(), format!("{}.playback", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `prefer_low_latency` after provisioning.\nWhen enabled, the live stream is delivered using Low-Latency HLS (LL-HLS), reducing glass-to-glass latency for viewers at the cost of reduced player compatibility."]
    pub fn prefer_low_latency(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefer_low_latency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recording` after provisioning.\nRecords the input to a Cloudflare Stream video. Behavior depends on the mode. In most cases, the video will initially be viewable as a live video and transition to on-demand after a condition is satisfied."]
    pub fn recording(&self) -> DataStreamLiveInput_RecordingRef {
        DataStreamLiveInput_RecordingRef::new(self.shared().clone(), format!("{}.recording", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rtmps` after provisioning.\nDetails for streaming to an live input using RTMPS."]
    pub fn rtmps(&self) -> DataStreamLiveInput_RtmpsRef {
        DataStreamLiveInput_RtmpsRef::new(self.shared().clone(), format!("{}.rtmps", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rtmps_playback` after provisioning.\nDetails for playback from an live input using RTMPS."]
    pub fn rtmps_playback(&self) -> DataStreamLiveInput_RtmpsPlaybackRef {
        DataStreamLiveInput_RtmpsPlaybackRef::new(
            self.shared().clone(),
            format!("{}.rtmps_playback", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `srt` after provisioning.\nDetails for streaming to a live input using SRT."]
    pub fn srt(&self) -> DataStreamLiveInput_SrtRef {
        DataStreamLiveInput_SrtRef::new(self.shared().clone(), format!("{}.srt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `srt_playback` after provisioning.\nDetails for playback from an live input using SRT."]
    pub fn srt_playback(&self) -> DataStreamLiveInput_SrtPlaybackRef {
        DataStreamLiveInput_SrtPlaybackRef::new(
            self.shared().clone(),
            format!("{}.srt_playback", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe connection status of a live input.\nAvailable values: \"connected\", \"reconnected\", \"reconnecting\", \"client_disconnect\", \"ttl_exceeded\", \"failed_to_connect\", \"failed_to_reconnect\", \"new_configuration_accepted\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nA unique identifier for a live input."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `web_rtc` after provisioning.\nDetails for streaming to a live input using WebRTC."]
    pub fn web_rtc(&self) -> DataStreamLiveInput_WebRtcRef {
        DataStreamLiveInput_WebRtcRef::new(self.shared().clone(), format!("{}.web_rtc", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `web_rtc_playback` after provisioning.\nDetails for playback from a live input using WebRTC."]
    pub fn web_rtc_playback(&self) -> DataStreamLiveInput_WebRtcPlaybackRef {
        DataStreamLiveInput_WebRtcPlaybackRef::new(
            self.shared().clone(),
            format!("{}.web_rtc_playback", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataStreamLiveInput_Playback {}

impl DataStreamLiveInput_Playback { }

impl terrars::ToListMappable for DataStreamLiveInput_Playback {
    type O = terrars::BlockAssignable<DataStreamLiveInput_Playback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreamLiveInput_Playback {}

impl BuildDataStreamLiveInput_Playback {
    pub fn build(self) -> DataStreamLiveInput_Playback {
        DataStreamLiveInput_Playback {}
    }
}

#[allow(dead_code)]
pub struct DataStreamLiveInput_PlaybackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreamLiveInput_PlaybackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreamLiveInput_PlaybackRef {
        DataStreamLiveInput_PlaybackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreamLiveInput_PlaybackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dash` after provisioning.\nThe DASH manifest URL used to play live video, referencing the live input ID."]
    pub fn dash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dash", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hls` after provisioning.\nThe HLS manifest URL used to play live video, referencing the live input ID."]
    pub fn hls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStreamLiveInput_Recording {}

impl DataStreamLiveInput_Recording { }

impl terrars::ToListMappable for DataStreamLiveInput_Recording {
    type O = terrars::BlockAssignable<DataStreamLiveInput_Recording>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreamLiveInput_Recording {}

impl BuildDataStreamLiveInput_Recording {
    pub fn build(self) -> DataStreamLiveInput_Recording {
        DataStreamLiveInput_Recording {}
    }
}

#[allow(dead_code)]
pub struct DataStreamLiveInput_RecordingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreamLiveInput_RecordingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreamLiveInput_RecordingRef {
        DataStreamLiveInput_RecordingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreamLiveInput_RecordingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_origins` after provisioning.\nLists the origins allowed to display videos created with this input. Enter allowed origin domains in an array and use `*` for wildcard subdomains. An empty array allows videos to be viewed on any origin."]
    pub fn allowed_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hide_live_viewer_count` after provisioning.\nDisables reporting the number of live viewers when this property is set to `true`."]
    pub fn hide_live_viewer_count(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hide_live_viewer_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nSpecifies the recording behavior for the live input. Set this value to `off` to prevent a recording. Set the value to `automatic` to begin a recording and transition to on-demand after Stream Live stops receiving input.\nAvailable values: \"off\", \"automatic\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_signed_urls` after provisioning.\nIndicates if a video using the live input has the `requireSignedURLs` property set. Also enforces access controls on any video recording of the livestream with the live input."]
    pub fn require_signed_urls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_signed_urls", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nDetermines the amount of time a live input configured in `automatic` mode should wait before a recording transitions from live to on-demand. `0` is recommended for most use cases and indicates the platform default should be used."]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStreamLiveInput_Rtmps {}

impl DataStreamLiveInput_Rtmps { }

impl terrars::ToListMappable for DataStreamLiveInput_Rtmps {
    type O = terrars::BlockAssignable<DataStreamLiveInput_Rtmps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreamLiveInput_Rtmps {}

impl BuildDataStreamLiveInput_Rtmps {
    pub fn build(self) -> DataStreamLiveInput_Rtmps {
        DataStreamLiveInput_Rtmps {}
    }
}

#[allow(dead_code)]
pub struct DataStreamLiveInput_RtmpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreamLiveInput_RtmpsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreamLiveInput_RtmpsRef {
        DataStreamLiveInput_RtmpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreamLiveInput_RtmpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `stream_key` after provisioning.\nThe secret key to use when streaming via RTMPS to a live input."]
    pub fn stream_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe RTMPS URL you provide to the broadcaster, which they stream live video to."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStreamLiveInput_RtmpsPlayback {}

impl DataStreamLiveInput_RtmpsPlayback { }

impl terrars::ToListMappable for DataStreamLiveInput_RtmpsPlayback {
    type O = terrars::BlockAssignable<DataStreamLiveInput_RtmpsPlayback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreamLiveInput_RtmpsPlayback {}

impl BuildDataStreamLiveInput_RtmpsPlayback {
    pub fn build(self) -> DataStreamLiveInput_RtmpsPlayback {
        DataStreamLiveInput_RtmpsPlayback {}
    }
}

#[allow(dead_code)]
pub struct DataStreamLiveInput_RtmpsPlaybackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreamLiveInput_RtmpsPlaybackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreamLiveInput_RtmpsPlaybackRef {
        DataStreamLiveInput_RtmpsPlaybackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreamLiveInput_RtmpsPlaybackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `stream_key` after provisioning.\nThe secret key to use for playback via RTMPS."]
    pub fn stream_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe URL used to play live video over RTMPS."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStreamLiveInput_Srt {}

impl DataStreamLiveInput_Srt { }

impl terrars::ToListMappable for DataStreamLiveInput_Srt {
    type O = terrars::BlockAssignable<DataStreamLiveInput_Srt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreamLiveInput_Srt {}

impl BuildDataStreamLiveInput_Srt {
    pub fn build(self) -> DataStreamLiveInput_Srt {
        DataStreamLiveInput_Srt {}
    }
}

#[allow(dead_code)]
pub struct DataStreamLiveInput_SrtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreamLiveInput_SrtRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreamLiveInput_SrtRef {
        DataStreamLiveInput_SrtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreamLiveInput_SrtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `passphrase` after provisioning.\nThe secret key to use when streaming via SRT to a live input."]
    pub fn passphrase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stream_id` after provisioning.\nThe identifier of the live input to use when streaming via SRT."]
    pub fn stream_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe SRT URL you provide to the broadcaster, which they stream live video to."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStreamLiveInput_SrtPlayback {}

impl DataStreamLiveInput_SrtPlayback { }

impl terrars::ToListMappable for DataStreamLiveInput_SrtPlayback {
    type O = terrars::BlockAssignable<DataStreamLiveInput_SrtPlayback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreamLiveInput_SrtPlayback {}

impl BuildDataStreamLiveInput_SrtPlayback {
    pub fn build(self) -> DataStreamLiveInput_SrtPlayback {
        DataStreamLiveInput_SrtPlayback {}
    }
}

#[allow(dead_code)]
pub struct DataStreamLiveInput_SrtPlaybackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreamLiveInput_SrtPlaybackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreamLiveInput_SrtPlaybackRef {
        DataStreamLiveInput_SrtPlaybackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreamLiveInput_SrtPlaybackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `passphrase` after provisioning.\nThe secret key to use for playback via SRT."]
    pub fn passphrase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stream_id` after provisioning.\nThe identifier of the live input to use for playback via SRT."]
    pub fn stream_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe URL used to play live video over SRT."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStreamLiveInput_WebRtc {}

impl DataStreamLiveInput_WebRtc { }

impl terrars::ToListMappable for DataStreamLiveInput_WebRtc {
    type O = terrars::BlockAssignable<DataStreamLiveInput_WebRtc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreamLiveInput_WebRtc {}

impl BuildDataStreamLiveInput_WebRtc {
    pub fn build(self) -> DataStreamLiveInput_WebRtc {
        DataStreamLiveInput_WebRtc {}
    }
}

#[allow(dead_code)]
pub struct DataStreamLiveInput_WebRtcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreamLiveInput_WebRtcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreamLiveInput_WebRtcRef {
        DataStreamLiveInput_WebRtcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreamLiveInput_WebRtcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe WebRTC URL you provide to the broadcaster, which they stream live video to."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStreamLiveInput_WebRtcPlayback {}

impl DataStreamLiveInput_WebRtcPlayback { }

impl terrars::ToListMappable for DataStreamLiveInput_WebRtcPlayback {
    type O = terrars::BlockAssignable<DataStreamLiveInput_WebRtcPlayback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreamLiveInput_WebRtcPlayback {}

impl BuildDataStreamLiveInput_WebRtcPlayback {
    pub fn build(self) -> DataStreamLiveInput_WebRtcPlayback {
        DataStreamLiveInput_WebRtcPlayback {}
    }
}

#[allow(dead_code)]
pub struct DataStreamLiveInput_WebRtcPlaybackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreamLiveInput_WebRtcPlaybackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreamLiveInput_WebRtcPlaybackRef {
        DataStreamLiveInput_WebRtcPlaybackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreamLiveInput_WebRtcPlaybackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe URL used to play live video over WebRTC."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

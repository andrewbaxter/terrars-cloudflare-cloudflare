#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct StreamLiveInputData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_creator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_recording_after_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    live_input_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    meta: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefer_low_latency: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recording: std::option::Option<StreamLiveInput_Recording>,
}

struct StreamLiveInput_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StreamLiveInputData_>,
}

#[derive(Clone)]
pub struct StreamLiveInput(std::rc::Rc<StreamLiveInput_>);

impl StreamLiveInput {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Set the field `default_creator`.\nSets the creator ID asssociated with this live input."]
    pub fn set_default_creator(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_creator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete_recording_after_days`.\nIndicates the number of days after which the live inputs recordings will be deleted. When a stream completes and the recording is ready, the value is used to calculate a scheduled deletion date for that recording. Omit the field to indicate no change, or include with a `null` value to remove an existing scheduled deletion."]
    pub fn set_delete_recording_after_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().delete_recording_after_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nIndicates whether the live input is enabled and can accept streams."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `live_input_identifier`.\nA unique identifier for a live input."]
    pub fn set_live_input_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().live_input_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `meta`.\nA user modifiable key-value store used to reference other systems of record for managing live inputs."]
    pub fn set_meta(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().meta = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefer_low_latency`.\nWhen enabled, the live stream is delivered using Low-Latency HLS (LL-HLS), reducing glass-to-glass latency for viewers at the cost of reduced player compatibility."]
    pub fn set_prefer_low_latency(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prefer_low_latency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `recording`.\nRecords the input to a Cloudflare Stream video. Behavior depends on the mode. In most cases, the video will initially be viewable as a live video and transition to on-demand after a condition is satisfied."]
    pub fn set_recording(self, v: impl std::convert::Into<StreamLiveInput_Recording>) -> Self {
        self.0.data.borrow_mut().recording = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `default_creator` after provisioning.\nSets the creator ID asssociated with this live input."]
    pub fn default_creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_creator", self.extract_ref()))
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
    pub fn playback(&self) -> StreamLiveInput_PlaybackRef {
        StreamLiveInput_PlaybackRef::new(self.shared().clone(), format!("{}.playback", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `prefer_low_latency` after provisioning.\nWhen enabled, the live stream is delivered using Low-Latency HLS (LL-HLS), reducing glass-to-glass latency for viewers at the cost of reduced player compatibility."]
    pub fn prefer_low_latency(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefer_low_latency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recording` after provisioning.\nRecords the input to a Cloudflare Stream video. Behavior depends on the mode. In most cases, the video will initially be viewable as a live video and transition to on-demand after a condition is satisfied."]
    pub fn recording(&self) -> StreamLiveInput_RecordingRef {
        StreamLiveInput_RecordingRef::new(self.shared().clone(), format!("{}.recording", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rtmps` after provisioning.\nDetails for streaming to an live input using RTMPS."]
    pub fn rtmps(&self) -> StreamLiveInput_RtmpsRef {
        StreamLiveInput_RtmpsRef::new(self.shared().clone(), format!("{}.rtmps", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rtmps_playback` after provisioning.\nDetails for playback from an live input using RTMPS."]
    pub fn rtmps_playback(&self) -> StreamLiveInput_RtmpsPlaybackRef {
        StreamLiveInput_RtmpsPlaybackRef::new(
            self.shared().clone(),
            format!("{}.rtmps_playback", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `srt` after provisioning.\nDetails for streaming to a live input using SRT."]
    pub fn srt(&self) -> StreamLiveInput_SrtRef {
        StreamLiveInput_SrtRef::new(self.shared().clone(), format!("{}.srt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `srt_playback` after provisioning.\nDetails for playback from an live input using SRT."]
    pub fn srt_playback(&self) -> StreamLiveInput_SrtPlaybackRef {
        StreamLiveInput_SrtPlaybackRef::new(self.shared().clone(), format!("{}.srt_playback", self.extract_ref()))
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
    pub fn web_rtc(&self) -> StreamLiveInput_WebRtcRef {
        StreamLiveInput_WebRtcRef::new(self.shared().clone(), format!("{}.web_rtc", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `web_rtc_playback` after provisioning.\nDetails for playback from a live input using WebRTC."]
    pub fn web_rtc_playback(&self) -> StreamLiveInput_WebRtcPlaybackRef {
        StreamLiveInput_WebRtcPlaybackRef::new(
            self.shared().clone(),
            format!("{}.web_rtc_playback", self.extract_ref()),
        )
    }
}

impl terrars::Referable for StreamLiveInput {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StreamLiveInput { }

impl terrars::ToListMappable for StreamLiveInput {
    type O = terrars::ListRef<StreamLiveInputRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StreamLiveInput_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_stream_live_input".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStreamLiveInput {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildStreamLiveInput {
    pub fn build(self, stack: &mut terrars::Stack) -> StreamLiveInput {
        let out = StreamLiveInput(std::rc::Rc::new(StreamLiveInput_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StreamLiveInputData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                default_creator: core::default::Default::default(),
                delete_recording_after_days: core::default::Default::default(),
                enabled: core::default::Default::default(),
                live_input_identifier: core::default::Default::default(),
                meta: core::default::Default::default(),
                prefer_low_latency: core::default::Default::default(),
                recording: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StreamLiveInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamLiveInputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StreamLiveInputRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `default_creator` after provisioning.\nSets the creator ID asssociated with this live input."]
    pub fn default_creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_creator", self.extract_ref()))
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
    pub fn playback(&self) -> StreamLiveInput_PlaybackRef {
        StreamLiveInput_PlaybackRef::new(self.shared().clone(), format!("{}.playback", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `prefer_low_latency` after provisioning.\nWhen enabled, the live stream is delivered using Low-Latency HLS (LL-HLS), reducing glass-to-glass latency for viewers at the cost of reduced player compatibility."]
    pub fn prefer_low_latency(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefer_low_latency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recording` after provisioning.\nRecords the input to a Cloudflare Stream video. Behavior depends on the mode. In most cases, the video will initially be viewable as a live video and transition to on-demand after a condition is satisfied."]
    pub fn recording(&self) -> StreamLiveInput_RecordingRef {
        StreamLiveInput_RecordingRef::new(self.shared().clone(), format!("{}.recording", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rtmps` after provisioning.\nDetails for streaming to an live input using RTMPS."]
    pub fn rtmps(&self) -> StreamLiveInput_RtmpsRef {
        StreamLiveInput_RtmpsRef::new(self.shared().clone(), format!("{}.rtmps", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rtmps_playback` after provisioning.\nDetails for playback from an live input using RTMPS."]
    pub fn rtmps_playback(&self) -> StreamLiveInput_RtmpsPlaybackRef {
        StreamLiveInput_RtmpsPlaybackRef::new(
            self.shared().clone(),
            format!("{}.rtmps_playback", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `srt` after provisioning.\nDetails for streaming to a live input using SRT."]
    pub fn srt(&self) -> StreamLiveInput_SrtRef {
        StreamLiveInput_SrtRef::new(self.shared().clone(), format!("{}.srt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `srt_playback` after provisioning.\nDetails for playback from an live input using SRT."]
    pub fn srt_playback(&self) -> StreamLiveInput_SrtPlaybackRef {
        StreamLiveInput_SrtPlaybackRef::new(self.shared().clone(), format!("{}.srt_playback", self.extract_ref()))
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
    pub fn web_rtc(&self) -> StreamLiveInput_WebRtcRef {
        StreamLiveInput_WebRtcRef::new(self.shared().clone(), format!("{}.web_rtc", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `web_rtc_playback` after provisioning.\nDetails for playback from a live input using WebRTC."]
    pub fn web_rtc_playback(&self) -> StreamLiveInput_WebRtcPlaybackRef {
        StreamLiveInput_WebRtcPlaybackRef::new(
            self.shared().clone(),
            format!("{}.web_rtc_playback", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct StreamLiveInput_Playback {}

impl StreamLiveInput_Playback { }

impl terrars::ToListMappable for StreamLiveInput_Playback {
    type O = terrars::BlockAssignable<StreamLiveInput_Playback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStreamLiveInput_Playback {}

impl BuildStreamLiveInput_Playback {
    pub fn build(self) -> StreamLiveInput_Playback {
        StreamLiveInput_Playback {}
    }
}

#[allow(dead_code)]
pub struct StreamLiveInput_PlaybackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamLiveInput_PlaybackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StreamLiveInput_PlaybackRef {
        StreamLiveInput_PlaybackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StreamLiveInput_PlaybackRef {
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
pub struct StreamLiveInput_Recording {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_origins: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hide_live_viewer_count: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_signed_urls: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl StreamLiveInput_Recording {
    #[doc =
        "Set the field `allowed_origins`.\nLists the origins allowed to display videos created with this input. Enter allowed origin domains in an array and use `*` for wildcard subdomains. An empty array allows videos to be viewed on any origin."]
    pub fn set_allowed_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hide_live_viewer_count`.\nDisables reporting the number of live viewers when this property is set to `true`."]
    pub fn set_hide_live_viewer_count(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.hide_live_viewer_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nSpecifies the recording behavior for the live input. Set this value to `off` to prevent a recording. Set the value to `automatic` to begin a recording and transition to on-demand after Stream Live stops receiving input.\nAvailable values: \"off\", \"automatic\"."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `require_signed_urls`.\nIndicates if a video using the live input has the `requireSignedURLs` property set. Also enforces access controls on any video recording of the livestream with the live input."]
    pub fn set_require_signed_urls(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_signed_urls = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_seconds`.\nDetermines the amount of time a live input configured in `automatic` mode should wait before a recording transitions from live to on-demand. `0` is recommended for most use cases and indicates the platform default should be used."]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StreamLiveInput_Recording {
    type O = terrars::BlockAssignable<StreamLiveInput_Recording>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStreamLiveInput_Recording {}

impl BuildStreamLiveInput_Recording {
    pub fn build(self) -> StreamLiveInput_Recording {
        StreamLiveInput_Recording {
            allowed_origins: core::default::Default::default(),
            hide_live_viewer_count: core::default::Default::default(),
            mode: core::default::Default::default(),
            require_signed_urls: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StreamLiveInput_RecordingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamLiveInput_RecordingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StreamLiveInput_RecordingRef {
        StreamLiveInput_RecordingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StreamLiveInput_RecordingRef {
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
pub struct StreamLiveInput_Rtmps {}

impl StreamLiveInput_Rtmps { }

impl terrars::ToListMappable for StreamLiveInput_Rtmps {
    type O = terrars::BlockAssignable<StreamLiveInput_Rtmps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStreamLiveInput_Rtmps {}

impl BuildStreamLiveInput_Rtmps {
    pub fn build(self) -> StreamLiveInput_Rtmps {
        StreamLiveInput_Rtmps {}
    }
}

#[allow(dead_code)]
pub struct StreamLiveInput_RtmpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamLiveInput_RtmpsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StreamLiveInput_RtmpsRef {
        StreamLiveInput_RtmpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StreamLiveInput_RtmpsRef {
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
pub struct StreamLiveInput_RtmpsPlayback {}

impl StreamLiveInput_RtmpsPlayback { }

impl terrars::ToListMappable for StreamLiveInput_RtmpsPlayback {
    type O = terrars::BlockAssignable<StreamLiveInput_RtmpsPlayback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStreamLiveInput_RtmpsPlayback {}

impl BuildStreamLiveInput_RtmpsPlayback {
    pub fn build(self) -> StreamLiveInput_RtmpsPlayback {
        StreamLiveInput_RtmpsPlayback {}
    }
}

#[allow(dead_code)]
pub struct StreamLiveInput_RtmpsPlaybackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamLiveInput_RtmpsPlaybackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StreamLiveInput_RtmpsPlaybackRef {
        StreamLiveInput_RtmpsPlaybackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StreamLiveInput_RtmpsPlaybackRef {
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
pub struct StreamLiveInput_Srt {}

impl StreamLiveInput_Srt { }

impl terrars::ToListMappable for StreamLiveInput_Srt {
    type O = terrars::BlockAssignable<StreamLiveInput_Srt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStreamLiveInput_Srt {}

impl BuildStreamLiveInput_Srt {
    pub fn build(self) -> StreamLiveInput_Srt {
        StreamLiveInput_Srt {}
    }
}

#[allow(dead_code)]
pub struct StreamLiveInput_SrtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamLiveInput_SrtRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StreamLiveInput_SrtRef {
        StreamLiveInput_SrtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StreamLiveInput_SrtRef {
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
pub struct StreamLiveInput_SrtPlayback {}

impl StreamLiveInput_SrtPlayback { }

impl terrars::ToListMappable for StreamLiveInput_SrtPlayback {
    type O = terrars::BlockAssignable<StreamLiveInput_SrtPlayback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStreamLiveInput_SrtPlayback {}

impl BuildStreamLiveInput_SrtPlayback {
    pub fn build(self) -> StreamLiveInput_SrtPlayback {
        StreamLiveInput_SrtPlayback {}
    }
}

#[allow(dead_code)]
pub struct StreamLiveInput_SrtPlaybackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamLiveInput_SrtPlaybackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StreamLiveInput_SrtPlaybackRef {
        StreamLiveInput_SrtPlaybackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StreamLiveInput_SrtPlaybackRef {
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
pub struct StreamLiveInput_WebRtc {}

impl StreamLiveInput_WebRtc { }

impl terrars::ToListMappable for StreamLiveInput_WebRtc {
    type O = terrars::BlockAssignable<StreamLiveInput_WebRtc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStreamLiveInput_WebRtc {}

impl BuildStreamLiveInput_WebRtc {
    pub fn build(self) -> StreamLiveInput_WebRtc {
        StreamLiveInput_WebRtc {}
    }
}

#[allow(dead_code)]
pub struct StreamLiveInput_WebRtcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamLiveInput_WebRtcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StreamLiveInput_WebRtcRef {
        StreamLiveInput_WebRtcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StreamLiveInput_WebRtcRef {
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
pub struct StreamLiveInput_WebRtcPlayback {}

impl StreamLiveInput_WebRtcPlayback { }

impl terrars::ToListMappable for StreamLiveInput_WebRtcPlayback {
    type O = terrars::BlockAssignable<StreamLiveInput_WebRtcPlayback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStreamLiveInput_WebRtcPlayback {}

impl BuildStreamLiveInput_WebRtcPlayback {
    pub fn build(self) -> StreamLiveInput_WebRtcPlayback {
        StreamLiveInput_WebRtcPlayback {}
    }
}

#[allow(dead_code)]
pub struct StreamLiveInput_WebRtcPlaybackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamLiveInput_WebRtcPlaybackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StreamLiveInput_WebRtcPlaybackRef {
        StreamLiveInput_WebRtcPlaybackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StreamLiveInput_WebRtcPlaybackRef {
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

#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct StreamAudioTrackData_ {
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
    audio_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<bool>>,
    identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct StreamAudioTrack_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StreamAudioTrackData_>,
}

#[derive(Clone)]
pub struct StreamAudioTrack(std::rc::Rc<StreamAudioTrack_>);

impl StreamAudioTrack {
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

    #[doc = "Set the field `audio_identifier`.\nThe unique identifier for an additional audio track."]
    pub fn set_audio_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().audio_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default`.\nDenotes whether the audio track will be played by default in a player."]
    pub fn set_default(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `label`.\nA string to uniquely identify the track amongst other audio track labels for the specified video."]
    pub fn set_label(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audio` after provisioning.\nArray of audio tracks for the video."]
    pub fn audio(&self) -> terrars::ListRef<StreamAudioTrack_AudioRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `audio_identifier` after provisioning.\nThe unique identifier for an additional audio track."]
    pub fn audio_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nDenotes whether the audio track will be played by default in a player."]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identifier` after provisioning.\nA Cloudflare-generated unique identifier for a media item."]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label` after provisioning.\nA string to uniquely identify the track amongst other audio track labels for the specified video."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nSpecifies the processing status of the video.\nAvailable values: \"queued\", \"ready\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nA Cloudflare-generated unique identifier for a media item."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
}

impl terrars::Referable for StreamAudioTrack {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StreamAudioTrack { }

impl terrars::ToListMappable for StreamAudioTrack {
    type O = terrars::ListRef<StreamAudioTrackRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StreamAudioTrack_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_stream_audio_track".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStreamAudioTrack {
    pub tf_id: std::string::String,
    #[doc = "The account identifier tag."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "A Cloudflare-generated unique identifier for a media item."]
    pub identifier: terrars::PrimField<std::string::String>,
}

impl BuildStreamAudioTrack {
    pub fn build(self, stack: &mut terrars::Stack) -> StreamAudioTrack {
        let out = StreamAudioTrack(std::rc::Rc::new(StreamAudioTrack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StreamAudioTrackData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                audio_identifier: core::default::Default::default(),
                default: core::default::Default::default(),
                identifier: self.identifier,
                label: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StreamAudioTrackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamAudioTrackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StreamAudioTrackRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audio` after provisioning.\nArray of audio tracks for the video."]
    pub fn audio(&self) -> terrars::ListRef<StreamAudioTrack_AudioRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `audio_identifier` after provisioning.\nThe unique identifier for an additional audio track."]
    pub fn audio_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nDenotes whether the audio track will be played by default in a player."]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identifier` after provisioning.\nA Cloudflare-generated unique identifier for a media item."]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label` after provisioning.\nA string to uniquely identify the track amongst other audio track labels for the specified video."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nSpecifies the processing status of the video.\nAvailable values: \"queued\", \"ready\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nA Cloudflare-generated unique identifier for a media item."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct StreamAudioTrack_Audio {}

impl StreamAudioTrack_Audio { }

impl terrars::ToListMappable for StreamAudioTrack_Audio {
    type O = terrars::BlockAssignable<StreamAudioTrack_Audio>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStreamAudioTrack_Audio {}

impl BuildStreamAudioTrack_Audio {
    pub fn build(self) -> StreamAudioTrack_Audio {
        StreamAudioTrack_Audio {}
    }
}

#[allow(dead_code)]
pub struct StreamAudioTrack_AudioRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamAudioTrack_AudioRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StreamAudioTrack_AudioRef {
        StreamAudioTrack_AudioRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StreamAudioTrack_AudioRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nDenotes whether the audio track will be played by default in a player."]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc =
        "Get a reference to the value of field `label` after provisioning.\nA string to uniquely identify the track amongst other audio track labels for the specified video."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nSpecifies the processing status of the video.\nAvailable values: \"queued\", \"ready\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nA Cloudflare-generated unique identifier for a media item."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

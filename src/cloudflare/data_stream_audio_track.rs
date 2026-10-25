#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataStreamAudioTrackData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    identifier: terrars::PrimField<std::string::String>,
}

struct DataStreamAudioTrack_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataStreamAudioTrackData_>,
}

#[derive(Clone)]
pub struct DataStreamAudioTrack(std::rc::Rc<DataStreamAudioTrack_>);

impl DataStreamAudioTrack {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audio` after provisioning.\nArray of audio tracks for the video."]
    pub fn audio(&self) -> terrars::ListRef<DataStreamAudioTrack_AudioRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identifier` after provisioning.\nA Cloudflare-generated unique identifier for a media item."]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }
}

impl terrars::Referable for DataStreamAudioTrack {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataStreamAudioTrack { }

impl terrars::ToListMappable for DataStreamAudioTrack {
    type O = terrars::ListRef<DataStreamAudioTrackRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataStreamAudioTrack_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_stream_audio_track".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataStreamAudioTrack {
    pub tf_id: std::string::String,
    #[doc = "The account identifier tag."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "A Cloudflare-generated unique identifier for a media item."]
    pub identifier: terrars::PrimField<std::string::String>,
}

impl BuildDataStreamAudioTrack {
    pub fn build(self, stack: &mut terrars::Stack) -> DataStreamAudioTrack {
        let out = DataStreamAudioTrack(std::rc::Rc::new(DataStreamAudioTrack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataStreamAudioTrackData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                identifier: self.identifier,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataStreamAudioTrackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreamAudioTrackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataStreamAudioTrackRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audio` after provisioning.\nArray of audio tracks for the video."]
    pub fn audio(&self) -> terrars::ListRef<DataStreamAudioTrack_AudioRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identifier` after provisioning.\nA Cloudflare-generated unique identifier for a media item."]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataStreamAudioTrack_Audio {}

impl DataStreamAudioTrack_Audio { }

impl terrars::ToListMappable for DataStreamAudioTrack_Audio {
    type O = terrars::BlockAssignable<DataStreamAudioTrack_Audio>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreamAudioTrack_Audio {}

impl BuildDataStreamAudioTrack_Audio {
    pub fn build(self) -> DataStreamAudioTrack_Audio {
        DataStreamAudioTrack_Audio {}
    }
}

#[allow(dead_code)]
pub struct DataStreamAudioTrack_AudioRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreamAudioTrack_AudioRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreamAudioTrack_AudioRef {
        DataStreamAudioTrack_AudioRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreamAudioTrack_AudioRef {
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

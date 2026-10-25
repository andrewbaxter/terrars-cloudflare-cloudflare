#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataStreamsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asc: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_counts: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    live_input_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataStreams_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataStreamsData_>,
}

#[derive(Clone)]
pub struct DataStreams(std::rc::Rc<DataStreams_>);

impl DataStreams {
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

    #[doc = "Set the field `account_id`.\nThe account identifier tag."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `after`.\nAlias for 'start'. Returns videos created after this date/time (RFC 3339 format)."]
    pub fn set_after(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `asc`.\nLists videos in ascending order of creation."]
    pub fn set_asc(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().asc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `before`.\nAlias for 'end'. Returns videos created before this date/time (RFC 3339 format)."]
    pub fn set_before(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().before = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `creator`.\nA user-defined identifier for the media creator."]
    pub fn set_creator(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().creator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end`.\nLists videos created before the specified date."]
    pub fn set_end(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nFilter by video ID(s). Can be a single ID or a comma-separated list of IDs."]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_counts`.\nIncludes the total number of videos associated with the submitted query parameters."]
    pub fn set_include_counts(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_counts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limit`.\nMaximum number of videos to return (default 1000, max 1000)."]
    pub fn set_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().limit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `live_input_id`.\nFilter by live input ID to find videos associated with a specific live stream."]
    pub fn set_live_input_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().live_input_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nFilter by video name/UID(s). Can be a single name or a comma-separated list."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `search`.\nProvides a partial word match of the `name` key in the `meta` field. Slow for medium to large video libraries. May be unavailable for very large libraries."]
    pub fn set_search(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\nLists videos created after the specified date."]
    pub fn set_start(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().start = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nSpecifies the processing status for all quality levels for a video.\nAvailable values: \"pendingupload\", \"downloading\", \"queued\", \"inprogress\", \"ready\", \"error\", \"live-inprogress\"."]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nSpecifies whether the video is `vod` or `live`."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `video_name`.\nProvides a fast, exact string match on the `name` key in the `meta` field."]
    pub fn set_video_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().video_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `after` after provisioning.\nAlias for 'start'. Returns videos created after this date/time (RFC 3339 format)."]
    pub fn after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.after", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `asc` after provisioning.\nLists videos in ascending order of creation."]
    pub fn asc(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asc", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `before` after provisioning.\nAlias for 'end'. Returns videos created before this date/time (RFC 3339 format)."]
    pub fn before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.before", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nA user-defined identifier for the media creator."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `end` after provisioning.\nLists videos created before the specified date."]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nFilter by video ID(s). Can be a single ID or a comma-separated list of IDs."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_counts` after provisioning.\nIncludes the total number of videos associated with the submitted query parameters."]
    pub fn include_counts(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_counts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nMaximum number of videos to return (default 1000, max 1000)."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `live_input_id` after provisioning.\nFilter by live input ID to find videos associated with a specific live stream."]
    pub fn live_input_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.live_input_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFilter by video name/UID(s). Can be a single name or a comma-separated list."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataStreams_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nProvides a partial word match of the `name` key in the `meta` field. Slow for medium to large video libraries. May be unavailable for very large libraries."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start` after provisioning.\nLists videos created after the specified date."]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nSpecifies the processing status for all quality levels for a video.\nAvailable values: \"pendingupload\", \"downloading\", \"queued\", \"inprogress\", \"ready\", \"error\", \"live-inprogress\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nSpecifies whether the video is `vod` or `live`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `video_name` after provisioning.\nProvides a fast, exact string match on the `name` key in the `meta` field."]
    pub fn video_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.video_name", self.extract_ref()))
    }
}

impl terrars::Referable for DataStreams {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataStreams { }

impl terrars::ToListMappable for DataStreams {
    type O = terrars::ListRef<DataStreamsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataStreams_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_streams".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataStreams {
    pub tf_id: std::string::String,
}

impl BuildDataStreams {
    pub fn build(self, stack: &mut terrars::Stack) -> DataStreams {
        let out = DataStreams(std::rc::Rc::new(DataStreams_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataStreamsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                after: core::default::Default::default(),
                asc: core::default::Default::default(),
                before: core::default::Default::default(),
                creator: core::default::Default::default(),
                end: core::default::Default::default(),
                id: core::default::Default::default(),
                include_counts: core::default::Default::default(),
                limit: core::default::Default::default(),
                live_input_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                name: core::default::Default::default(),
                search: core::default::Default::default(),
                start: core::default::Default::default(),
                status: core::default::Default::default(),
                type_: core::default::Default::default(),
                video_name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataStreamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataStreamsRef {
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

    #[doc =
        "Get a reference to the value of field `after` after provisioning.\nAlias for 'start'. Returns videos created after this date/time (RFC 3339 format)."]
    pub fn after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.after", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `asc` after provisioning.\nLists videos in ascending order of creation."]
    pub fn asc(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asc", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `before` after provisioning.\nAlias for 'end'. Returns videos created before this date/time (RFC 3339 format)."]
    pub fn before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.before", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nA user-defined identifier for the media creator."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `end` after provisioning.\nLists videos created before the specified date."]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nFilter by video ID(s). Can be a single ID or a comma-separated list of IDs."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_counts` after provisioning.\nIncludes the total number of videos associated with the submitted query parameters."]
    pub fn include_counts(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_counts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nMaximum number of videos to return (default 1000, max 1000)."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `live_input_id` after provisioning.\nFilter by live input ID to find videos associated with a specific live stream."]
    pub fn live_input_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.live_input_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFilter by video name/UID(s). Can be a single name or a comma-separated list."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataStreams_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nProvides a partial word match of the `name` key in the `meta` field. Slow for medium to large video libraries. May be unavailable for very large libraries."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start` after provisioning.\nLists videos created after the specified date."]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nSpecifies the processing status for all quality levels for a video.\nAvailable values: \"pendingupload\", \"downloading\", \"queued\", \"inprogress\", \"ready\", \"error\", \"live-inprogress\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nSpecifies whether the video is `vod` or `live`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `video_name` after provisioning.\nProvides a fast, exact string match on the `name` key in the `meta` field."]
    pub fn video_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.video_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataStreams_Result_Input {}

impl DataStreams_Result_Input { }

impl terrars::ToListMappable for DataStreams_Result_Input {
    type O = terrars::BlockAssignable<DataStreams_Result_Input>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreams_Result_Input {}

impl BuildDataStreams_Result_Input {
    pub fn build(self) -> DataStreams_Result_Input {
        DataStreams_Result_Input {}
    }
}

#[allow(dead_code)]
pub struct DataStreams_Result_InputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreams_Result_InputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreams_Result_InputRef {
        DataStreams_Result_InputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreams_Result_InputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `height` after provisioning.\nThe video height in pixels. A value of `-1` means the height is unknown. The value becomes available after the upload and before the video is ready."]
    pub fn height(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.height", self.base))
    }

    #[doc =
        "Get a reference to the value of field `width` after provisioning.\nThe video width in pixels. A value of `-1` means the width is unknown. The value becomes available after the upload and before the video is ready."]
    pub fn width(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.width", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStreams_Result_Playback {}

impl DataStreams_Result_Playback { }

impl terrars::ToListMappable for DataStreams_Result_Playback {
    type O = terrars::BlockAssignable<DataStreams_Result_Playback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreams_Result_Playback {}

impl BuildDataStreams_Result_Playback {
    pub fn build(self) -> DataStreams_Result_Playback {
        DataStreams_Result_Playback {}
    }
}

#[allow(dead_code)]
pub struct DataStreams_Result_PlaybackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreams_Result_PlaybackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreams_Result_PlaybackRef {
        DataStreams_Result_PlaybackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreams_Result_PlaybackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dash` after provisioning.\nDASH Media Presentation Description for the video."]
    pub fn dash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dash", self.base))
    }

    #[doc = "Get a reference to the value of field `hls` after provisioning.\nThe HLS manifest for the video."]
    pub fn hls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStreams_Result_PublicDetails {}

impl DataStreams_Result_PublicDetails { }

impl terrars::ToListMappable for DataStreams_Result_PublicDetails {
    type O = terrars::BlockAssignable<DataStreams_Result_PublicDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreams_Result_PublicDetails {}

impl BuildDataStreams_Result_PublicDetails {
    pub fn build(self) -> DataStreams_Result_PublicDetails {
        DataStreams_Result_PublicDetails {}
    }
}

#[allow(dead_code)]
pub struct DataStreams_Result_PublicDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreams_Result_PublicDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreams_Result_PublicDetailsRef {
        DataStreams_Result_PublicDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreams_Result_PublicDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel_link` after provisioning.\n"]
    pub fn channel_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_link", self.base))
    }

    #[doc = "Get a reference to the value of field `logo` after provisioning.\n"]
    pub fn logo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logo", self.base))
    }

    #[doc = "Get a reference to the value of field `media_id` after provisioning.\n"]
    pub fn media_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.media_id", self.base))
    }

    #[doc = "Get a reference to the value of field `share_link` after provisioning.\n"]
    pub fn share_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_link", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStreams_Result_Status {}

impl DataStreams_Result_Status { }

impl terrars::ToListMappable for DataStreams_Result_Status {
    type O = terrars::BlockAssignable<DataStreams_Result_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreams_Result_Status {}

impl BuildDataStreams_Result_Status {
    pub fn build(self) -> DataStreams_Result_Status {
        DataStreams_Result_Status {}
    }
}

#[allow(dead_code)]
pub struct DataStreams_Result_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreams_Result_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreams_Result_StatusRef {
        DataStreams_Result_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreams_Result_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `error_reason_code` after provisioning.\nSpecifies why the video failed to encode. This field is empty if the video is not in an `error` state. Preferred for programmatic use."]
    pub fn error_reason_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_reason_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `error_reason_text` after provisioning.\nSpecifies why the video failed to encode using a human readable error message in English. This field is empty if the video is not in an `error` state."]
    pub fn error_reason_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_reason_text", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pct_complete` after provisioning.\nIndicates the size of the entire upload in bytes. The value must be a non-negative integer."]
    pub fn pct_complete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pct_complete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nSpecifies the processing status for all quality levels for a video.\nAvailable values: \"pendingupload\", \"downloading\", \"queued\", \"inprogress\", \"ready\", \"error\", \"live-inprogress\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStreams_Result_Watermark {}

impl DataStreams_Result_Watermark { }

impl terrars::ToListMappable for DataStreams_Result_Watermark {
    type O = terrars::BlockAssignable<DataStreams_Result_Watermark>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreams_Result_Watermark {}

impl BuildDataStreams_Result_Watermark {
    pub fn build(self) -> DataStreams_Result_Watermark {
        DataStreams_Result_Watermark {}
    }
}

#[allow(dead_code)]
pub struct DataStreams_Result_WatermarkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreams_Result_WatermarkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreams_Result_WatermarkRef {
        DataStreams_Result_WatermarkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreams_Result_WatermarkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nThe date and a time a watermark profile was created."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc =
        "Get a reference to the value of field `downloaded_from` after provisioning.\nThe source URL for a downloaded image. If the watermark profile was created via direct upload, this field is null."]
    pub fn downloaded_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.downloaded_from", self.base))
    }

    #[doc = "Get a reference to the value of field `height` after provisioning.\nThe height of the image in pixels."]
    pub fn height(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.height", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA short description of the watermark profile."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `opacity` after provisioning.\nThe translucency of the image. A value of `0.0` makes the image completely transparent, and `1.0` makes the image completely opaque. Note that if the image is already semi-transparent, setting this to `1.0` will not make the image completely opaque."]
    pub fn opacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opacity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `padding` after provisioning.\nThe whitespace between the adjacent edges (determined by position) of the video and the image. `0.0` indicates no padding, and `1.0` indicates a fully padded video width or length, as determined by the algorithm."]
    pub fn padding(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.padding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `position` after provisioning.\nThe location of the image. Valid positions are: `upperRight`, `upperLeft`, `lowerLeft`, `lowerRight`, and `center`. Note that `center` ignores the `padding` parameter."]
    pub fn position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale` after provisioning.\nThe size of the image relative to the overall size of the video. This parameter will adapt to horizontal and vertical videos automatically. `0.0` indicates no scaling (use the size of the image as-is), and `1.0 `fills the entire video."]
    pub fn scale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\nThe size of the image in bytes."]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier for a watermark profile."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }

    #[doc = "Get a reference to the value of field `width` after provisioning.\nThe width of the image in pixels."]
    pub fn width(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.width", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStreams_Result {}

impl DataStreams_Result { }

impl terrars::ToListMappable for DataStreams_Result {
    type O = terrars::BlockAssignable<DataStreams_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStreams_Result {}

impl BuildDataStreams_Result {
    pub fn build(self) -> DataStreams_Result {
        DataStreams_Result {}
    }
}

#[allow(dead_code)]
pub struct DataStreams_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreams_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStreams_ResultRef {
        DataStreams_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStreams_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_origins` after provisioning.\nLists the origins allowed to display the video. Enter allowed origin domains in an array and use `*` for wildcard subdomains. Empty arrays allow the video to be viewed on any origin."]
    pub fn allowed_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `clipped_from` after provisioning.\nThe unique identifier of the source video this video was clipped from."]
    pub fn clipped_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clipped_from", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nThe date and time the media item was created."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nA user-defined identifier for the media creator."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nThe duration of the video in seconds. A value of `-1` means the duration is unknown. The duration becomes available after the upload and before the video is ready."]
    pub fn duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> DataStreams_Result_InputRef {
        DataStreams_Result_InputRef::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc =
        "Get a reference to the value of field `live_input` after provisioning.\nThe live input ID used to upload a video with Stream Live."]
    pub fn live_input(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.live_input", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_duration_seconds` after provisioning.\nThe maximum duration in seconds for a video upload. Can be set for a video that is not yet uploaded to limit its duration. Uploads that exceed the specified duration will fail during processing. A value of `-1` means the value is unknown."]
    pub fn max_duration_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_duration_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_size_bytes` after provisioning.\nThe maximum size in bytes for the video upload."]
    pub fn max_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_size_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `meta` after provisioning.\nA user modifiable key-value store used to reference other systems of record for managing videos."]
    pub fn meta(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.meta", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified` after provisioning.\nThe date and time the media item was last modified."]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.base))
    }

    #[doc = "Get a reference to the value of field `playback` after provisioning.\n"]
    pub fn playback(&self) -> DataStreams_Result_PlaybackRef {
        DataStreams_Result_PlaybackRef::new(self.shared().clone(), format!("{}.playback", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preview` after provisioning.\nThe video's preview page URI. This field is omitted until encoding is complete."]
    pub fn preview(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_details` after provisioning.\nPublic details for the video including title, share link, channel link, and logo."]
    pub fn public_details(&self) -> DataStreams_Result_PublicDetailsRef {
        DataStreams_Result_PublicDetailsRef::new(self.shared().clone(), format!("{}.public_details", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ready_to_stream` after provisioning.\nIndicates whether the video is playable. The field is empty if the video is not ready for viewing or the live stream is still in progress."]
    pub fn ready_to_stream(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ready_to_stream", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ready_to_stream_at` after provisioning.\nIndicates the time at which the video became playable. The field is empty if the video is not ready for viewing or the live stream is still in progress."]
    pub fn ready_to_stream_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ready_to_stream_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_signed_urls` after provisioning.\nIndicates whether the video can be a accessed using the UID. When set to `true`, a signed token must be generated with a signing key to view the video."]
    pub fn require_signed_urls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_signed_urls", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheduled_deletion` after provisioning.\nIndicates the date and time at which the video will be deleted. Omit the field to indicate no change, or include with a `null` value to remove an existing scheduled deletion. If specified, must be at least 30 days from upload time."]
    pub fn scheduled_deletion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduled_deletion", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\nThe size of the media item in bytes."]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nSpecifies a detailed status for a video. If the `state` is `inprogress` or `error`, the `step` field returns `encoding` or `manifest`. If the `state` is `inprogress`, `pctComplete` returns a number between 0 and 100 to indicate the approximate percent of completion. If the `state` is `error`, `errorReasonCode` and `errorReasonText` provide additional details."]
    pub fn status(&self) -> DataStreams_Result_StatusRef {
        DataStreams_Result_StatusRef::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `thumbnail` after provisioning.\nThe media item's thumbnail URI. This field is omitted until encoding is complete."]
    pub fn thumbnail(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thumbnail", self.base))
    }

    #[doc =
        "Get a reference to the value of field `thumbnail_timestamp_pct` after provisioning.\nThe timestamp for a thumbnail image calculated as a percentage value of the video's duration. To convert from a second-wise timestamp to a percentage, divide the desired timestamp by the total duration of the video.  If this value is not set, the default thumbnail image is taken from 0s of the video."]
    pub fn thumbnail_timestamp_pct(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thumbnail_timestamp_pct", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nA Cloudflare-generated unique identifier for a media item."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `upload_expiry` after provisioning.\nThe date and time when the video upload URL is no longer valid for direct user uploads."]
    pub fn upload_expiry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_expiry", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uploaded` after provisioning.\nThe date and time the media item was uploaded."]
    pub fn uploaded(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uploaded", self.base))
    }

    #[doc = "Get a reference to the value of field `watermark` after provisioning.\n"]
    pub fn watermark(&self) -> DataStreams_Result_WatermarkRef {
        DataStreams_Result_WatermarkRef::new(self.shared().clone(), format!("{}.watermark", self.base))
    }
}

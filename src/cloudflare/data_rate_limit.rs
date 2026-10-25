#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataRateLimitData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    rate_limit_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataRateLimit_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRateLimitData_>,
}

#[derive(Clone)]
pub struct DataRateLimit(std::rc::Rc<DataRateLimit_>);

impl DataRateLimit {
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

    #[doc = "Set the field `zone_id`.\nDefines an identifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe action to perform when the threshold of matched traffic within the configured period is exceeded."]
    pub fn action(&self) -> DataRateLimit_ActionRef {
        DataRateLimit_ActionRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bypass` after provisioning.\nCriteria specifying when the current rate limit should be bypassed. You can specify that the rate limit should not apply to one or more URLs."]
    pub fn bypass(&self) -> terrars::ListRef<DataRateLimit_BypassRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bypass", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative summary of the rule. This value is sanitized and any tags will be removed."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhen true, indicates that the rate limit is currently disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nDefines the unique identifier of the rate limit."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nDetermines which traffic the rate limit counts towards the threshold."]
    pub fn match_(&self) -> DataRateLimit_MatchRef {
        DataRateLimit_MatchRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `period` after provisioning.\nThe time in seconds (an integer value) to count matching traffic. If the count exceeds the configured threshold within this period, Cloudflare will perform the configured action."]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rate_limit_id` after provisioning.\nDefines the unique identifier of the rate limit."]
    pub fn rate_limit_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limit_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `threshold` after provisioning.\nThe threshold that will trigger the configured mitigation action. Configure this value along with the `period` property to establish a threshold per period."]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nDefines an identifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataRateLimit {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRateLimit { }

impl terrars::ToListMappable for DataRateLimit {
    type O = terrars::ListRef<DataRateLimitRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRateLimit_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_rate_limit".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRateLimit {
    pub tf_id: std::string::String,
    #[doc = "Defines the unique identifier of the rate limit."]
    pub rate_limit_id: terrars::PrimField<std::string::String>,
}

impl BuildDataRateLimit {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRateLimit {
        let out = DataRateLimit(std::rc::Rc::new(DataRateLimit_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRateLimitData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                rate_limit_id: self.rate_limit_id,
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRateLimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRateLimitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRateLimitRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe action to perform when the threshold of matched traffic within the configured period is exceeded."]
    pub fn action(&self) -> DataRateLimit_ActionRef {
        DataRateLimit_ActionRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bypass` after provisioning.\nCriteria specifying when the current rate limit should be bypassed. You can specify that the rate limit should not apply to one or more URLs."]
    pub fn bypass(&self) -> terrars::ListRef<DataRateLimit_BypassRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bypass", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative summary of the rule. This value is sanitized and any tags will be removed."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhen true, indicates that the rate limit is currently disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nDefines the unique identifier of the rate limit."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nDetermines which traffic the rate limit counts towards the threshold."]
    pub fn match_(&self) -> DataRateLimit_MatchRef {
        DataRateLimit_MatchRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `period` after provisioning.\nThe time in seconds (an integer value) to count matching traffic. If the count exceeds the configured threshold within this period, Cloudflare will perform the configured action."]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rate_limit_id` after provisioning.\nDefines the unique identifier of the rate limit."]
    pub fn rate_limit_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limit_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `threshold` after provisioning.\nThe threshold that will trigger the configured mitigation action. Configure this value along with the `period` property to establish a threshold per period."]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nDefines an identifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRateLimit_Action_Response {}

impl DataRateLimit_Action_Response { }

impl terrars::ToListMappable for DataRateLimit_Action_Response {
    type O = terrars::BlockAssignable<DataRateLimit_Action_Response>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRateLimit_Action_Response {}

impl BuildDataRateLimit_Action_Response {
    pub fn build(self) -> DataRateLimit_Action_Response {
        DataRateLimit_Action_Response {}
    }
}

#[allow(dead_code)]
pub struct DataRateLimit_Action_ResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRateLimit_Action_ResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRateLimit_Action_ResponseRef {
        DataRateLimit_Action_ResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRateLimit_Action_ResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `body` after provisioning.\nThe response body to return. The value must conform to the configured content type."]
    pub fn body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.body", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nThe content type of the body. Must be one of the following: `text/plain`, `text/xml`, or `application/json`."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRateLimit_Action {}

impl DataRateLimit_Action { }

impl terrars::ToListMappable for DataRateLimit_Action {
    type O = terrars::BlockAssignable<DataRateLimit_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRateLimit_Action {}

impl BuildDataRateLimit_Action {
    pub fn build(self) -> DataRateLimit_Action {
        DataRateLimit_Action {}
    }
}

#[allow(dead_code)]
pub struct DataRateLimit_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRateLimit_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRateLimit_ActionRef {
        DataRateLimit_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRateLimit_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe action to perform.\nAvailable values: \"simulate\", \"ban\", \"challenge\", \"js_challenge\", \"managed_challenge\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response` after provisioning.\nA custom content type and reponse to return when the threshold is exceeded. The custom response configured in this object will override the custom error for the zone. This object is optional.\nNotes: If you omit this object, Cloudflare will use the default HTML error page. If \"mode\" is \"challenge\", \"managed_challenge\", or \"js_challenge\", Cloudflare will use the zone challenge pages and you should not provide the \"response\" object."]
    pub fn response(&self) -> DataRateLimit_Action_ResponseRef {
        DataRateLimit_Action_ResponseRef::new(self.shared().clone(), format!("{}.response", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe time in seconds during which Cloudflare will perform the mitigation action. Must be an integer value greater than or equal to the period.\nNotes: If \"mode\" is \"challenge\", \"managed_challenge\", or \"js_challenge\", Cloudflare will use the zone's Challenge Passage time and you should not provide this value."]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRateLimit_Bypass {}

impl DataRateLimit_Bypass { }

impl terrars::ToListMappable for DataRateLimit_Bypass {
    type O = terrars::BlockAssignable<DataRateLimit_Bypass>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRateLimit_Bypass {}

impl BuildDataRateLimit_Bypass {
    pub fn build(self) -> DataRateLimit_Bypass {
        DataRateLimit_Bypass {}
    }
}

#[allow(dead_code)]
pub struct DataRateLimit_BypassRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRateLimit_BypassRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRateLimit_BypassRef {
        DataRateLimit_BypassRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRateLimit_BypassRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nAvailable values: \"url\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe URL to bypass."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRateLimit_Match_Headers {}

impl DataRateLimit_Match_Headers { }

impl terrars::ToListMappable for DataRateLimit_Match_Headers {
    type O = terrars::BlockAssignable<DataRateLimit_Match_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRateLimit_Match_Headers {}

impl BuildDataRateLimit_Match_Headers {
    pub fn build(self) -> DataRateLimit_Match_Headers {
        DataRateLimit_Match_Headers {}
    }
}

#[allow(dead_code)]
pub struct DataRateLimit_Match_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRateLimit_Match_HeadersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRateLimit_Match_HeadersRef {
        DataRateLimit_Match_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRateLimit_Match_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the response header to match."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `op` after provisioning.\nThe operator used when matching: `eq` means \"equal\" and `ne` means \"not equal\".\nAvailable values: \"eq\", \"ne\"."]
    pub fn op(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.op", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe value of the response header, which must match exactly."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRateLimit_Match_Request {}

impl DataRateLimit_Match_Request { }

impl terrars::ToListMappable for DataRateLimit_Match_Request {
    type O = terrars::BlockAssignable<DataRateLimit_Match_Request>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRateLimit_Match_Request {}

impl BuildDataRateLimit_Match_Request {
    pub fn build(self) -> DataRateLimit_Match_Request {
        DataRateLimit_Match_Request {}
    }
}

#[allow(dead_code)]
pub struct DataRateLimit_Match_RequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRateLimit_Match_RequestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRateLimit_Match_RequestRef {
        DataRateLimit_Match_RequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRateLimit_Match_RequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `methods` after provisioning.\nThe HTTP methods to match. You can specify a subset (for example, `['POST','PUT']`) or all methods (`['_ALL_']`). This field is optional when creating a rate limit."]
    pub fn methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schemes` after provisioning.\nThe HTTP schemes to match. You can specify one scheme (`['HTTPS']`), both schemes (`['HTTP','HTTPS']`), or all schemes (`['_ALL_']`). This field is optional."]
    pub fn schemes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schemes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe URL pattern to match, composed of a host and a path such as `example.org/path*`. Normalization is applied before the pattern is matched. `*` wildcards are expanded to match applicable traffic. Query strings are not matched. Set the value to `*` to match all traffic to your zone."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRateLimit_Match_Response {}

impl DataRateLimit_Match_Response { }

impl terrars::ToListMappable for DataRateLimit_Match_Response {
    type O = terrars::BlockAssignable<DataRateLimit_Match_Response>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRateLimit_Match_Response {}

impl BuildDataRateLimit_Match_Response {
    pub fn build(self) -> DataRateLimit_Match_Response {
        DataRateLimit_Match_Response {}
    }
}

#[allow(dead_code)]
pub struct DataRateLimit_Match_ResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRateLimit_Match_ResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRateLimit_Match_ResponseRef {
        DataRateLimit_Match_ResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRateLimit_Match_ResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `origin_traffic` after provisioning.\nWhen true, only the uncached traffic served from your origin servers will count towards rate limiting. In this case, any cached traffic served by Cloudflare will not count towards rate limiting. This field is optional.\nNotes: This field is deprecated. Instead, use response headers and set \"origin_traffic\" to \"false\" to avoid legacy behaviour interacting with the \"response_headers\" property."]
    pub fn origin_traffic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_traffic", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRateLimit_Match {}

impl DataRateLimit_Match { }

impl terrars::ToListMappable for DataRateLimit_Match {
    type O = terrars::BlockAssignable<DataRateLimit_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRateLimit_Match {}

impl BuildDataRateLimit_Match {
    pub fn build(self) -> DataRateLimit_Match {
        DataRateLimit_Match {}
    }
}

#[allow(dead_code)]
pub struct DataRateLimit_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRateLimit_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRateLimit_MatchRef {
        DataRateLimit_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRateLimit_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<DataRateLimit_Match_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc = "Get a reference to the value of field `request` after provisioning.\n"]
    pub fn request(&self) -> DataRateLimit_Match_RequestRef {
        DataRateLimit_Match_RequestRef::new(self.shared().clone(), format!("{}.request", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> DataRateLimit_Match_ResponseRef {
        DataRateLimit_Match_ResponseRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

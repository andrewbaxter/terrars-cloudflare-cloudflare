#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct RateLimitData_ {
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
    action: RateLimit_Action,
    #[serde(rename = "match")]
    match_: RateLimit_Match,
    period: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit_id: std::option::Option<terrars::PrimField<std::string::String>>,
    threshold: terrars::PrimField<f64>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct RateLimit_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RateLimitData_>,
}

#[derive(Clone)]
pub struct RateLimit(std::rc::Rc<RateLimit_>);

impl RateLimit {
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

    #[doc = "Set the field `rate_limit_id`.\nDefines the unique identifier of the rate limit."]
    pub fn set_rate_limit_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rate_limit_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe action to perform when the threshold of matched traffic within the configured period is exceeded."]
    pub fn action(&self) -> RateLimit_ActionRef {
        RateLimit_ActionRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nDetermines which traffic the rate limit counts towards the threshold."]
    pub fn match_(&self) -> RateLimit_MatchRef {
        RateLimit_MatchRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
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

impl terrars::Referable for RateLimit {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RateLimit { }

impl terrars::ToListMappable for RateLimit {
    type O = terrars::ListRef<RateLimitRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RateLimit_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_rate_limit".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRateLimit {
    pub tf_id: std::string::String,
    #[doc = "The action to perform when the threshold of matched traffic within the configured period is exceeded."]
    pub action: RateLimit_Action,
    #[doc = "Determines which traffic the rate limit counts towards the threshold."]
    pub match_: RateLimit_Match,
    #[doc =
        "The time in seconds (an integer value) to count matching traffic. If the count exceeds the configured threshold within this period, Cloudflare will perform the configured action."]
    pub period: terrars::PrimField<f64>,
    #[doc =
        "The threshold that will trigger the configured mitigation action. Configure this value along with the `period` property to establish a threshold per period."]
    pub threshold: terrars::PrimField<f64>,
    #[doc = "Defines an identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildRateLimit {
    pub fn build(self, stack: &mut terrars::Stack) -> RateLimit {
        let out = RateLimit(std::rc::Rc::new(RateLimit_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RateLimitData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action: self.action,
                match_: self.match_,
                period: self.period,
                rate_limit_id: core::default::Default::default(),
                threshold: self.threshold,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RateLimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RateLimitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RateLimitRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe action to perform when the threshold of matched traffic within the configured period is exceeded."]
    pub fn action(&self) -> RateLimit_ActionRef {
        RateLimit_ActionRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nDetermines which traffic the rate limit counts towards the threshold."]
    pub fn match_(&self) -> RateLimit_MatchRef {
        RateLimit_MatchRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
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
pub struct RateLimit_Action_Response {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RateLimit_Action_Response {
    #[doc =
        "Set the field `body`.\nThe response body to return. The value must conform to the configured content type."]
    pub fn set_body(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.body = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_type`.\nThe content type of the body. Must be one of the following: `text/plain`, `text/xml`, or `application/json`."]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RateLimit_Action_Response {
    type O = terrars::BlockAssignable<RateLimit_Action_Response>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimit_Action_Response {}

impl BuildRateLimit_Action_Response {
    pub fn build(self) -> RateLimit_Action_Response {
        RateLimit_Action_Response {
            body: core::default::Default::default(),
            content_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RateLimit_Action_ResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RateLimit_Action_ResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RateLimit_Action_ResponseRef {
        RateLimit_Action_ResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimit_Action_ResponseRef {
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
pub struct RateLimit_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<RateLimit_Action_Response>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
}

impl RateLimit_Action {
    #[doc =
        "Set the field `mode`.\nThe action to perform.\nAvailable values: \"simulate\", \"ban\", \"challenge\", \"js_challenge\", \"managed_challenge\"."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response`.\nA custom content type and reponse to return when the threshold is exceeded. The custom response configured in this object will override the custom error for the zone. This object is optional.\nNotes: If you omit this object, Cloudflare will use the default HTML error page. If \"mode\" is \"challenge\", \"managed_challenge\", or \"js_challenge\", Cloudflare will use the zone challenge pages and you should not provide the \"response\" object."]
    pub fn set_response(mut self, v: impl std::convert::Into<RateLimit_Action_Response>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout`.\nThe time in seconds during which Cloudflare will perform the mitigation action. Must be an integer value greater than or equal to the period.\nNotes: If \"mode\" is \"challenge\", \"managed_challenge\", or \"js_challenge\", Cloudflare will use the zone's Challenge Passage time and you should not provide this value."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RateLimit_Action {
    type O = terrars::BlockAssignable<RateLimit_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimit_Action {}

impl BuildRateLimit_Action {
    pub fn build(self) -> RateLimit_Action {
        RateLimit_Action {
            mode: core::default::Default::default(),
            response: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RateLimit_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RateLimit_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RateLimit_ActionRef {
        RateLimit_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimit_ActionRef {
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
    pub fn response(&self) -> RateLimit_Action_ResponseRef {
        RateLimit_Action_ResponseRef::new(self.shared().clone(), format!("{}.response", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe time in seconds during which Cloudflare will perform the mitigation action. Must be an integer value greater than or equal to the period.\nNotes: If \"mode\" is \"challenge\", \"managed_challenge\", or \"js_challenge\", Cloudflare will use the zone's Challenge Passage time and you should not provide this value."]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct RateLimit_Match_Headers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    op: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RateLimit_Match_Headers {
    #[doc = "Set the field `name`.\nThe name of the response header to match."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `op`.\nThe operator used when matching: `eq` means \"equal\" and `ne` means \"not equal\".\nAvailable values: \"eq\", \"ne\"."]
    pub fn set_op(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.op = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe value of the response header, which must match exactly."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RateLimit_Match_Headers {
    type O = terrars::BlockAssignable<RateLimit_Match_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimit_Match_Headers {}

impl BuildRateLimit_Match_Headers {
    pub fn build(self) -> RateLimit_Match_Headers {
        RateLimit_Match_Headers {
            name: core::default::Default::default(),
            op: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RateLimit_Match_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RateLimit_Match_HeadersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RateLimit_Match_HeadersRef {
        RateLimit_Match_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimit_Match_HeadersRef {
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
pub struct RateLimit_Match_Request {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schemes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RateLimit_Match_Request {
    #[doc =
        "Set the field `methods`.\nThe HTTP methods to match. You can specify a subset (for example, `['POST','PUT']`) or all methods (`['_ALL_']`). This field is optional when creating a rate limit."]
    pub fn set_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schemes`.\nThe HTTP schemes to match. You can specify one scheme (`['HTTPS']`), both schemes (`['HTTP','HTTPS']`), or all schemes (`['_ALL_']`). This field is optional."]
    pub fn set_schemes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.schemes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `url`.\nThe URL pattern to match, composed of a host and a path such as `example.org/path*`. Normalization is applied before the pattern is matched. `*` wildcards are expanded to match applicable traffic. Query strings are not matched. Set the value to `*` to match all traffic to your zone."]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RateLimit_Match_Request {
    type O = terrars::BlockAssignable<RateLimit_Match_Request>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimit_Match_Request {}

impl BuildRateLimit_Match_Request {
    pub fn build(self) -> RateLimit_Match_Request {
        RateLimit_Match_Request {
            methods: core::default::Default::default(),
            schemes: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RateLimit_Match_RequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RateLimit_Match_RequestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RateLimit_Match_RequestRef {
        RateLimit_Match_RequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimit_Match_RequestRef {
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
pub struct RateLimit_Match_Response {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_traffic: std::option::Option<terrars::PrimField<bool>>,
}

impl RateLimit_Match_Response {
    #[doc =
        "Set the field `origin_traffic`.\nWhen true, only the uncached traffic served from your origin servers will count towards rate limiting. In this case, any cached traffic served by Cloudflare will not count towards rate limiting. This field is optional.\nNotes: This field is deprecated. Instead, use response headers and set \"origin_traffic\" to \"false\" to avoid legacy behaviour interacting with the \"response_headers\" property."]
    pub fn set_origin_traffic(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.origin_traffic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RateLimit_Match_Response {
    type O = terrars::BlockAssignable<RateLimit_Match_Response>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimit_Match_Response {}

impl BuildRateLimit_Match_Response {
    pub fn build(self) -> RateLimit_Match_Response {
        RateLimit_Match_Response { origin_traffic: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct RateLimit_Match_ResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RateLimit_Match_ResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RateLimit_Match_ResponseRef {
        RateLimit_Match_ResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimit_Match_ResponseRef {
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
pub struct RateLimit_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<std::vec::Vec<RateLimit_Match_Headers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request: std::option::Option<RateLimit_Match_Request>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<RateLimit_Match_Response>,
}

impl RateLimit_Match {
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(mut self, v: impl std::convert::Into<std::vec::Vec<RateLimit_Match_Headers>>) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request`.\n"]
    pub fn set_request(mut self, v: impl std::convert::Into<RateLimit_Match_Request>) -> Self {
        self.request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl std::convert::Into<RateLimit_Match_Response>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RateLimit_Match {
    type O = terrars::BlockAssignable<RateLimit_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimit_Match {}

impl BuildRateLimit_Match {
    pub fn build(self) -> RateLimit_Match {
        RateLimit_Match {
            headers: core::default::Default::default(),
            request: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RateLimit_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RateLimit_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RateLimit_MatchRef {
        RateLimit_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimit_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<RateLimit_Match_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc = "Get a reference to the value of field `request` after provisioning.\n"]
    pub fn request(&self) -> RateLimit_Match_RequestRef {
        RateLimit_Match_RequestRef::new(self.shared().clone(), format!("{}.request", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> RateLimit_Match_ResponseRef {
        RateLimit_Match_ResponseRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

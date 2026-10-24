use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct RateLimitData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bypass_url_patterns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    period: terrars::PrimField<f64>,
    threshold: terrars::PrimField<f64>,
    zone_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<RateLimitActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    correlate: std::option::Option<std::vec::Vec<RateLimitCorrelateEl>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<RateLimitMatchEl>>,
    dynamic: RateLimitDynamic,
}

struct RateLimit_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RateLimitData>,
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

    #[doc = "Set the field `bypass_url_patterns`.\n"]
    pub fn set_bypass_url_patterns(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().bypass_url_patterns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nA note that you can use to describe the reason for a rate limit. This value is sanitized and all tags are removed."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\nWhether this ratelimit is currently disabled. Defaults to `false`."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl std::convert::Into<terrars::BlockAssignable<RateLimitActionEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `correlate`.\n"]
    pub fn set_correlate(self, v: impl std::convert::Into<terrars::BlockAssignable<RateLimitCorrelateEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().correlate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.correlate = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(self, v: impl std::convert::Into<terrars::BlockAssignable<RateLimitMatchEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `bypass_url_patterns` after provisioning.\n"]
    pub fn bypass_url_patterns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.bypass_url_patterns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA note that you can use to describe the reason for a rate limit. This value is sanitized and all tags are removed."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether this ratelimit is currently disabled. Defaults to `false`."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `period` after provisioning.\nThe time in seconds to count matching traffic. If the count exceeds threshold within this period the action will be performed."]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `threshold` after provisioning.\nThe threshold that triggers the rate limit mitigations, combine with period."]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<RateLimitActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `correlate` after provisioning.\n"]
    pub fn correlate(&self) -> terrars::ListRef<RateLimitCorrelateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.correlate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<RateLimitMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
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
    #[doc =
        "The time in seconds to count matching traffic. If the count exceeds threshold within this period the action will be performed."]
    pub period: terrars::PrimField<f64>,
    #[doc = "The threshold that triggers the rate limit mitigations, combine with period."]
    pub threshold: terrars::PrimField<f64>,
    #[doc =
        "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildRateLimit {
    pub fn build(self, stack: &mut terrars::Stack) -> RateLimit {
        let out = RateLimit(std::rc::Rc::new(RateLimit_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RateLimitData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bypass_url_patterns: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                id: core::default::Default::default(),
                period: self.period,
                threshold: self.threshold,
                zone_id: self.zone_id,
                action: core::default::Default::default(),
                correlate: core::default::Default::default(),
                match_: core::default::Default::default(),
                dynamic: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `bypass_url_patterns` after provisioning.\n"]
    pub fn bypass_url_patterns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.bypass_url_patterns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA note that you can use to describe the reason for a rate limit. This value is sanitized and all tags are removed."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether this ratelimit is currently disabled. Defaults to `false`."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `period` after provisioning.\nThe time in seconds to count matching traffic. If the count exceeds threshold within this period the action will be performed."]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `threshold` after provisioning.\nThe threshold that triggers the rate limit mitigations, combine with period."]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<RateLimitActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `correlate` after provisioning.\n"]
    pub fn correlate(&self) -> terrars::ListRef<RateLimitCorrelateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.correlate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<RateLimitMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RateLimitActionElResponseEl {
    body: terrars::PrimField<std::string::String>,
    content_type: terrars::PrimField<std::string::String>,
}

impl RateLimitActionElResponseEl { }

impl terrars::ToListMappable for RateLimitActionElResponseEl {
    type O = terrars::BlockAssignable<RateLimitActionElResponseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimitActionElResponseEl {
    #[doc = "The body to return, the content here should conform to the `content_type`."]
    pub body: terrars::PrimField<std::string::String>,
    #[doc = "The content-type of the body. Available values: `text/plain`, `text/xml`, `application/json`."]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildRateLimitActionElResponseEl {
    pub fn build(self) -> RateLimitActionElResponseEl {
        RateLimitActionElResponseEl {
            body: self.body,
            content_type: self.content_type,
        }
    }
}

pub struct RateLimitActionElResponseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RateLimitActionElResponseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RateLimitActionElResponseElRef {
        RateLimitActionElResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimitActionElResponseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `body` after provisioning.\nThe body to return, the content here should conform to the `content_type`."]
    pub fn body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.body", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nThe content-type of the body. Available values: `text/plain`, `text/xml`, `application/json`."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct RateLimitActionElDynamic {
    response: std::option::Option<terrars::DynamicBlock<RateLimitActionElResponseEl>>,
}

#[derive(Serialize)]
pub struct RateLimitActionEl {
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<std::vec::Vec<RateLimitActionElResponseEl>>,
    dynamic: RateLimitActionElDynamic,
}

impl RateLimitActionEl {
    #[doc =
        "Set the field `timeout`.\nThe time in seconds as an integer to perform the mitigation action. This field is required if the `mode` is either `simulate` or `ban`. Must be the same or greater than the period."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RateLimitActionElResponseEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RateLimitActionEl {
    type O = terrars::BlockAssignable<RateLimitActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimitActionEl {
    #[doc =
        "The type of action to perform. Available values: `simulate`, `ban`, `challenge`, `js_challenge`, `managed_challenge`."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildRateLimitActionEl {
    pub fn build(self) -> RateLimitActionEl {
        RateLimitActionEl {
            mode: self.mode,
            timeout: core::default::Default::default(),
            response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct RateLimitActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RateLimitActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RateLimitActionElRef {
        RateLimitActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimitActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe type of action to perform. Available values: `simulate`, `ban`, `challenge`, `js_challenge`, `managed_challenge`."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe time in seconds as an integer to perform the mitigation action. This field is required if the `mode` is either `simulate` or `ban`. Must be the same or greater than the period."]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::ListRef<RateLimitActionElResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct RateLimitCorrelateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    by: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RateLimitCorrelateEl {
    #[doc =
        "Set the field `by`.\nIf set to 'nat', NAT support will be enabled for rate limiting. Available values: `nat`."]
    pub fn set_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.by = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RateLimitCorrelateEl {
    type O = terrars::BlockAssignable<RateLimitCorrelateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimitCorrelateEl {}

impl BuildRateLimitCorrelateEl {
    pub fn build(self) -> RateLimitCorrelateEl {
        RateLimitCorrelateEl { by: core::default::Default::default() }
    }
}

pub struct RateLimitCorrelateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RateLimitCorrelateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RateLimitCorrelateElRef {
        RateLimitCorrelateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimitCorrelateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `by` after provisioning.\nIf set to 'nat', NAT support will be enabled for rate limiting. Available values: `nat`."]
    pub fn by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.by", self.base))
    }
}

#[derive(Serialize)]
pub struct RateLimitMatchElRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    methods: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schemes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_pattern: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RateLimitMatchElRequestEl {
    #[doc =
        "Set the field `methods`.\nHTTP Methods to match traffic on. Available values: `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`, `_ALL_`."]
    pub fn set_methods(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.methods = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schemes`.\nHTTP schemes to match traffic on. Available values: `HTTP`, `HTTPS`, `_ALL_`."]
    pub fn set_schemes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.schemes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `url_pattern`.\nThe URL pattern to match comprised of the host and path, i.e. example.org/path. Wildcard are expanded to match applicable traffic, query strings are not matched. Use _ for all traffic to your zone."]
    pub fn set_url_pattern(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url_pattern = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RateLimitMatchElRequestEl {
    type O = terrars::BlockAssignable<RateLimitMatchElRequestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimitMatchElRequestEl {}

impl BuildRateLimitMatchElRequestEl {
    pub fn build(self) -> RateLimitMatchElRequestEl {
        RateLimitMatchElRequestEl {
            methods: core::default::Default::default(),
            schemes: core::default::Default::default(),
            url_pattern: core::default::Default::default(),
        }
    }
}

pub struct RateLimitMatchElRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RateLimitMatchElRequestElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RateLimitMatchElRequestElRef {
        RateLimitMatchElRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimitMatchElRequestElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `methods` after provisioning.\nHTTP Methods to match traffic on. Available values: `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`, `_ALL_`."]
    pub fn methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schemes` after provisioning.\nHTTP schemes to match traffic on. Available values: `HTTP`, `HTTPS`, `_ALL_`."]
    pub fn schemes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.schemes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url_pattern` after provisioning.\nThe URL pattern to match comprised of the host and path, i.e. example.org/path. Wildcard are expanded to match applicable traffic, query strings are not matched. Use _ for all traffic to your zone."]
    pub fn url_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct RateLimitMatchElResponseEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_traffic: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statuses: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
}

impl RateLimitMatchElResponseEl {
    #[doc = "Set the field `headers`.\nList of HTTP headers maps to match the origin response on."]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_traffic`.\nOnly count traffic that has come from your origin servers. If true, cached items that Cloudflare serve will not count towards rate limiting."]
    pub fn set_origin_traffic(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.origin_traffic = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `statuses`.\nHTTP Status codes, can be one, many or indicate all by not providing this value."]
    pub fn set_statuses(mut self, v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>) -> Self {
        self.statuses = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RateLimitMatchElResponseEl {
    type O = terrars::BlockAssignable<RateLimitMatchElResponseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimitMatchElResponseEl {}

impl BuildRateLimitMatchElResponseEl {
    pub fn build(self) -> RateLimitMatchElResponseEl {
        RateLimitMatchElResponseEl {
            headers: core::default::Default::default(),
            origin_traffic: core::default::Default::default(),
            statuses: core::default::Default::default(),
        }
    }
}

pub struct RateLimitMatchElResponseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RateLimitMatchElResponseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RateLimitMatchElResponseElRef {
        RateLimitMatchElResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimitMatchElResponseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `headers` after provisioning.\nList of HTTP headers maps to match the origin response on."]
    pub fn headers(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_traffic` after provisioning.\nOnly count traffic that has come from your origin servers. If true, cached items that Cloudflare serve will not count towards rate limiting."]
    pub fn origin_traffic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_traffic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `statuses` after provisioning.\nHTTP Status codes, can be one, many or indicate all by not providing this value."]
    pub fn statuses(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.statuses", self.base))
    }
}

#[derive(Serialize, Default)]
struct RateLimitMatchElDynamic {
    request: std::option::Option<terrars::DynamicBlock<RateLimitMatchElRequestEl>>,
    response: std::option::Option<terrars::DynamicBlock<RateLimitMatchElResponseEl>>,
}

#[derive(Serialize)]
pub struct RateLimitMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request: std::option::Option<std::vec::Vec<RateLimitMatchElRequestEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<std::vec::Vec<RateLimitMatchElResponseEl>>,
    dynamic: RateLimitMatchElDynamic,
}

impl RateLimitMatchEl {
    #[doc = "Set the field `request`.\n"]
    pub fn set_request(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RateLimitMatchElRequestEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RateLimitMatchElResponseEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RateLimitMatchEl {
    type O = terrars::BlockAssignable<RateLimitMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimitMatchEl {}

impl BuildRateLimitMatchEl {
    pub fn build(self) -> RateLimitMatchEl {
        RateLimitMatchEl {
            request: core::default::Default::default(),
            response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct RateLimitMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RateLimitMatchElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RateLimitMatchElRef {
        RateLimitMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimitMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request` after provisioning.\n"]
    pub fn request(&self) -> terrars::ListRef<RateLimitMatchElRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::ListRef<RateLimitMatchElResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize, Default)]
struct RateLimitDynamic {
    action: std::option::Option<terrars::DynamicBlock<RateLimitActionEl>>,
    correlate: std::option::Option<terrars::DynamicBlock<RateLimitCorrelateEl>>,
    match_: std::option::Option<terrars::DynamicBlock<RateLimitMatchEl>>,
}

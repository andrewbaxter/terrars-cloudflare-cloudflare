use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct RateLimitData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_url_patterns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    period: PrimField<f64>,
    threshold: PrimField<f64>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<RateLimitActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correlate: Option<Vec<RateLimitCorrelateEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<RateLimitMatchEl>>,
    dynamic: RateLimitDynamic,
}

struct RateLimit_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RateLimitData>,
}

#[derive(Clone)]
pub struct RateLimit(Rc<RateLimit_>);

impl RateLimit {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `bypass_url_patterns`.\n"]
    pub fn set_bypass_url_patterns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().bypass_url_patterns = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nA note that you can use to describe the reason for a rate limit. This value is sanitized and all tags are removed."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `disabled`.\nWhether this ratelimit is currently disabled. Defaults to `false`."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `action`.\n"]
    pub fn set_action(self, v: impl Into<BlockAssignable<RateLimitActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `correlate`.\n"]
    pub fn set_correlate(self, v: impl Into<BlockAssignable<RateLimitCorrelateEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().correlate = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.correlate = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `match_`.\n"]
    pub fn set_match(self, v: impl Into<BlockAssignable<RateLimitMatchEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().match_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.match_ = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `bypass_url_patterns` after provisioning.\n"]
    pub fn bypass_url_patterns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.bypass_url_patterns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA note that you can use to describe the reason for a rate limit. This value is sanitized and all tags are removed."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nWhether this ratelimit is currently disabled. Defaults to `false`."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `period` after provisioning.\nThe time in seconds to count matching traffic. If the count exceeds threshold within this period the action will be performed."]
    pub fn period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `threshold` after provisioning.\nThe threshold that triggers the rate limit mitigations, combine with period."]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<RateLimitActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `correlate` after provisioning.\n"]
    pub fn correlate(&self) -> ListRef<RateLimitCorrelateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.correlate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<RateLimitMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }
}

impl Referable for RateLimit {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RateLimit { }

impl ToListMappable for RateLimit {
    type O = ListRef<RateLimitRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RateLimit_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_rate_limit".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRateLimit {
    pub tf_id: String,
    #[doc= "The time in seconds to count matching traffic. If the count exceeds threshold within this period the action will be performed."]
    pub period: PrimField<f64>,
    #[doc= "The threshold that triggers the rate limit mitigations, combine with period."]
    pub threshold: PrimField<f64>,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildRateLimit {
    pub fn build(self, stack: &mut Stack) -> RateLimit {
        let out = RateLimit(Rc::new(RateLimit_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RateLimitData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RateLimitRef {
    shared: StackShared,
    base: String,
}

impl Ref for RateLimitRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RateLimitRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bypass_url_patterns` after provisioning.\n"]
    pub fn bypass_url_patterns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.bypass_url_patterns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA note that you can use to describe the reason for a rate limit. This value is sanitized and all tags are removed."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nWhether this ratelimit is currently disabled. Defaults to `false`."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `period` after provisioning.\nThe time in seconds to count matching traffic. If the count exceeds threshold within this period the action will be performed."]
    pub fn period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `threshold` after provisioning.\nThe threshold that triggers the rate limit mitigations, combine with period."]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<RateLimitActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `correlate` after provisioning.\n"]
    pub fn correlate(&self) -> ListRef<RateLimitCorrelateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.correlate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<RateLimitMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RateLimitActionElResponseEl {
    body: PrimField<String>,
    content_type: PrimField<String>,
}

impl RateLimitActionElResponseEl { }

impl ToListMappable for RateLimitActionElResponseEl {
    type O = BlockAssignable<RateLimitActionElResponseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimitActionElResponseEl {
    #[doc= "The body to return, the content here should conform to the `content_type`."]
    pub body: PrimField<String>,
    #[doc= "The content-type of the body. Available values: `text/plain`, `text/xml`, `application/json`."]
    pub content_type: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for RateLimitActionElResponseElRef {
    fn new(shared: StackShared, base: String) -> RateLimitActionElResponseElRef {
        RateLimitActionElResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimitActionElResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `body` after provisioning.\nThe body to return, the content here should conform to the `content_type`."]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.base))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\nThe content-type of the body. Available values: `text/plain`, `text/xml`, `application/json`."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct RateLimitActionElDynamic {
    response: Option<DynamicBlock<RateLimitActionElResponseEl>>,
}

#[derive(Serialize)]
pub struct RateLimitActionEl {
    mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<Vec<RateLimitActionElResponseEl>>,
    dynamic: RateLimitActionElDynamic,
}

impl RateLimitActionEl {
    #[doc= "Set the field `timeout`.\nThe time in seconds as an integer to perform the mitigation action. This field is required if the `mode` is either `simulate` or `ban`. Must be the same or greater than the period."]
    pub fn set_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl Into<BlockAssignable<RateLimitActionElResponseEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RateLimitActionEl {
    type O = BlockAssignable<RateLimitActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRateLimitActionEl {
    #[doc= "The type of action to perform. Available values: `simulate`, `ban`, `challenge`, `js_challenge`, `managed_challenge`."]
    pub mode: PrimField<String>,
}

impl BuildRateLimitActionEl {
    pub fn build(self) -> RateLimitActionEl {
        RateLimitActionEl {
            mode: self.mode,
            timeout: core::default::Default::default(),
            response: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RateLimitActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RateLimitActionElRef {
    fn new(shared: StackShared, base: String) -> RateLimitActionElRef {
        RateLimitActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimitActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nThe type of action to perform. Available values: `simulate`, `ban`, `challenge`, `js_challenge`, `managed_challenge`."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\nThe time in seconds as an integer to perform the mitigation action. This field is required if the `mode` is either `simulate` or `ban`. Must be the same or greater than the period."]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> ListRef<RateLimitActionElResponseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct RateLimitCorrelateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    by: Option<PrimField<String>>,
}

impl RateLimitCorrelateEl {
    #[doc= "Set the field `by`.\nIf set to 'nat', NAT support will be enabled for rate limiting. Available values: `nat`."]
    pub fn set_by(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.by = Some(v.into());
        self
    }
}

impl ToListMappable for RateLimitCorrelateEl {
    type O = BlockAssignable<RateLimitCorrelateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for RateLimitCorrelateElRef {
    fn new(shared: StackShared, base: String) -> RateLimitCorrelateElRef {
        RateLimitCorrelateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimitCorrelateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `by` after provisioning.\nIf set to 'nat', NAT support will be enabled for rate limiting. Available values: `nat`."]
    pub fn by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.by", self.base))
    }
}

#[derive(Serialize)]
pub struct RateLimitMatchElRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    methods: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schemes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_pattern: Option<PrimField<String>>,
}

impl RateLimitMatchElRequestEl {
    #[doc= "Set the field `methods`.\nHTTP Methods to match traffic on. Available values: `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`, `_ALL_`."]
    pub fn set_methods(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.methods = Some(v.into());
        self
    }

    #[doc= "Set the field `schemes`.\nHTTP schemes to match traffic on. Available values: `HTTP`, `HTTPS`, `_ALL_`."]
    pub fn set_schemes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.schemes = Some(v.into());
        self
    }

    #[doc= "Set the field `url_pattern`.\nThe URL pattern to match comprised of the host and path, i.e. example.org/path. Wildcard are expanded to match applicable traffic, query strings are not matched. Use _ for all traffic to your zone."]
    pub fn set_url_pattern(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url_pattern = Some(v.into());
        self
    }
}

impl ToListMappable for RateLimitMatchElRequestEl {
    type O = BlockAssignable<RateLimitMatchElRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for RateLimitMatchElRequestElRef {
    fn new(shared: StackShared, base: String) -> RateLimitMatchElRequestElRef {
        RateLimitMatchElRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimitMatchElRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `methods` after provisioning.\nHTTP Methods to match traffic on. Available values: `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`, `_ALL_`."]
    pub fn methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }

    #[doc= "Get a reference to the value of field `schemes` after provisioning.\nHTTP schemes to match traffic on. Available values: `HTTP`, `HTTPS`, `_ALL_`."]
    pub fn schemes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.schemes", self.base))
    }

    #[doc= "Get a reference to the value of field `url_pattern` after provisioning.\nThe URL pattern to match comprised of the host and path, i.e. example.org/path. Wildcard are expanded to match applicable traffic, query strings are not matched. Use _ for all traffic to your zone."]
    pub fn url_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct RateLimitMatchElResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<ListField<RecField<PrimField<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_traffic: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statuses: Option<SetField<PrimField<f64>>>,
}

impl RateLimitMatchElResponseEl {
    #[doc= "Set the field `headers`.\nList of HTTP headers maps to match the origin response on."]
    pub fn set_headers(mut self, v: impl Into<ListField<RecField<PrimField<String>>>>) -> Self {
        self.headers = Some(v.into());
        self
    }

    #[doc= "Set the field `origin_traffic`.\nOnly count traffic that has come from your origin servers. If true, cached items that Cloudflare serve will not count towards rate limiting."]
    pub fn set_origin_traffic(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.origin_traffic = Some(v.into());
        self
    }

    #[doc= "Set the field `statuses`.\nHTTP Status codes, can be one, many or indicate all by not providing this value."]
    pub fn set_statuses(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.statuses = Some(v.into());
        self
    }
}

impl ToListMappable for RateLimitMatchElResponseEl {
    type O = BlockAssignable<RateLimitMatchElResponseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for RateLimitMatchElResponseElRef {
    fn new(shared: StackShared, base: String) -> RateLimitMatchElResponseElRef {
        RateLimitMatchElResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimitMatchElResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `headers` after provisioning.\nList of HTTP headers maps to match the origin response on."]
    pub fn headers(&self) -> ListRef<RecRef<PrimExpr<String>>> {
        ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc= "Get a reference to the value of field `origin_traffic` after provisioning.\nOnly count traffic that has come from your origin servers. If true, cached items that Cloudflare serve will not count towards rate limiting."]
    pub fn origin_traffic(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_traffic", self.base))
    }

    #[doc= "Get a reference to the value of field `statuses` after provisioning.\nHTTP Status codes, can be one, many or indicate all by not providing this value."]
    pub fn statuses(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.statuses", self.base))
    }
}

#[derive(Serialize, Default)]
struct RateLimitMatchElDynamic {
    request: Option<DynamicBlock<RateLimitMatchElRequestEl>>,
    response: Option<DynamicBlock<RateLimitMatchElResponseEl>>,
}

#[derive(Serialize)]
pub struct RateLimitMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<Vec<RateLimitMatchElRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<Vec<RateLimitMatchElResponseEl>>,
    dynamic: RateLimitMatchElDynamic,
}

impl RateLimitMatchEl {
    #[doc= "Set the field `request`.\n"]
    pub fn set_request(mut self, v: impl Into<BlockAssignable<RateLimitMatchElRequestEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl Into<BlockAssignable<RateLimitMatchElResponseEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RateLimitMatchEl {
    type O = BlockAssignable<RateLimitMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct RateLimitMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RateLimitMatchElRef {
    fn new(shared: StackShared, base: String) -> RateLimitMatchElRef {
        RateLimitMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RateLimitMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `request` after provisioning.\n"]
    pub fn request(&self) -> ListRef<RateLimitMatchElRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request", self.base))
    }

    #[doc= "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> ListRef<RateLimitMatchElResponseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize, Default)]
struct RateLimitDynamic {
    action: Option<DynamicBlock<RateLimitActionEl>>,
    correlate: Option<DynamicBlock<RateLimitCorrelateEl>>,
    match_: Option<DynamicBlock<RateLimitMatchEl>>,
}

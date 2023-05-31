use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct RulesetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    kind: PrimField<String>,
    name: PrimField<String>,
    phase: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shareable_entitlement_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<RulesetRulesEl>>,
    dynamic: RulesetDynamic,
}

struct Ruleset_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RulesetData>,
}

#[derive(Clone)]
pub struct Ruleset(Rc<Ruleset_>);

impl Ruleset {
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

    #[doc= "Set the field `account_id`.\nThe account identifier to target for the resource."]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nBrief summary of the ruleset and its intended use."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `shareable_entitlement_name`.\nName of entitlement that is shareable between entities."]
    pub fn set_shareable_entitlement_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().shareable_entitlement_name = Some(v.into());
        self
    }

    #[doc= "Set the field `zone_id`.\nThe zone identifier to target for the resource."]
    pub fn set_zone_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone_id = Some(v.into());
        self
    }

    #[doc= "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl Into<BlockAssignable<RulesetRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nBrief summary of the ruleset and its intended use."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe identifier of this resource."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\nType of Ruleset to create. Available values: `custom`, `managed`, `root`, `schema`, `zone`."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the ruleset."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `phase` after provisioning.\nPoint in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub fn phase(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phase", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shareable_entitlement_name` after provisioning.\nName of entitlement that is shareable between entities."]
    pub fn shareable_entitlement_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shareable_entitlement_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<RulesetRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl Referable for Ruleset {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Ruleset { }

impl ToListMappable for Ruleset {
    type O = ListRef<RulesetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Ruleset_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_ruleset".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRuleset {
    pub tf_id: String,
    #[doc= "Type of Ruleset to create. Available values: `custom`, `managed`, `root`, `schema`, `zone`."]
    pub kind: PrimField<String>,
    #[doc= "Name of the ruleset."]
    pub name: PrimField<String>,
    #[doc= "Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub phase: PrimField<String>,
}

impl BuildRuleset {
    pub fn build(self, stack: &mut Stack) -> Ruleset {
        let out = Ruleset(Rc::new(Ruleset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RulesetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                description: core::default::Default::default(),
                kind: self.kind,
                name: self.name,
                phase: self.phase,
                shareable_entitlement_name: core::default::Default::default(),
                zone_id: core::default::Default::default(),
                rules: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RulesetRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RulesetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nBrief summary of the ruleset and its intended use."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe identifier of this resource."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\nType of Ruleset to create. Available values: `custom`, `managed`, `root`, `schema`, `zone`."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the ruleset."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `phase` after provisioning.\nPoint in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub fn phase(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phase", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shareable_entitlement_name` after provisioning.\nName of entitlement that is shareable between entities."]
    pub fn shareable_entitlement_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shareable_entitlement_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<RulesetRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElAlgorithmsEl {
    name: PrimField<String>,
}

impl RulesetRulesElActionParametersElAlgorithmsEl { }

impl ToListMappable for RulesetRulesElActionParametersElAlgorithmsEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElAlgorithmsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElAlgorithmsEl {
    #[doc= "Name of the compression algorithm to use. Available values: `gzip`, `brotli`, `auto`, `default`, `none`"]
    pub name: PrimField<String>,
}

impl BuildRulesetRulesElActionParametersElAlgorithmsEl {
    pub fn build(self) -> RulesetRulesElActionParametersElAlgorithmsEl {
        RulesetRulesElActionParametersElAlgorithmsEl { name: self.name }
    }
}

pub struct RulesetRulesElActionParametersElAlgorithmsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElAlgorithmsElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElAlgorithmsElRef {
        RulesetRulesElActionParametersElAlgorithmsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElAlgorithmsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the compression algorithm to use. Available values: `gzip`, `brotli`, `auto`, `default`, `none`"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElAutominifyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    css: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    js: Option<PrimField<bool>>,
}

impl RulesetRulesElActionParametersElAutominifyEl {
    #[doc= "Set the field `css`.\nCSS minification."]
    pub fn set_css(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.css = Some(v.into());
        self
    }

    #[doc= "Set the field `html`.\nHTML minification."]
    pub fn set_html(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.html = Some(v.into());
        self
    }

    #[doc= "Set the field `js`.\nJS minification."]
    pub fn set_js(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.js = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElAutominifyEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElAutominifyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElAutominifyEl {}

impl BuildRulesetRulesElActionParametersElAutominifyEl {
    pub fn build(self) -> RulesetRulesElActionParametersElAutominifyEl {
        RulesetRulesElActionParametersElAutominifyEl {
            css: core::default::Default::default(),
            html: core::default::Default::default(),
            js: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElAutominifyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElAutominifyElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElAutominifyElRef {
        RulesetRulesElActionParametersElAutominifyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElAutominifyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `css` after provisioning.\nCSS minification."]
    pub fn css(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.css", self.base))
    }

    #[doc= "Get a reference to the value of field `html` after provisioning.\nHTML minification."]
    pub fn html(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }

    #[doc= "Get a reference to the value of field `js` after provisioning.\nJS minification."]
    pub fn js(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.js", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElBrowserTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<f64>>,
    mode: PrimField<String>,
}

impl RulesetRulesElActionParametersElBrowserTtlEl {
    #[doc= "Set the field `default`.\nDefault browser TTL. This value is required when override_origin is set"]
    pub fn set_default(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElBrowserTtlEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElBrowserTtlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElBrowserTtlEl {
    #[doc= "Mode of the browser TTL."]
    pub mode: PrimField<String>,
}

impl BuildRulesetRulesElActionParametersElBrowserTtlEl {
    pub fn build(self) -> RulesetRulesElActionParametersElBrowserTtlEl {
        RulesetRulesElActionParametersElBrowserTtlEl {
            default: core::default::Default::default(),
            mode: self.mode,
        }
    }
}

pub struct RulesetRulesElActionParametersElBrowserTtlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElBrowserTtlElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElBrowserTtlElRef {
        RulesetRulesElActionParametersElBrowserTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElBrowserTtlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default` after provisioning.\nDefault browser TTL. This value is required when override_origin is set"]
    pub fn default(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nMode of the browser TTL."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    check_presence: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<SetField<PrimField<String>>>,
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    #[doc= "Set the field `check_presence`.\nList of cookies to check for presence in the custom key."]
    pub fn set_check_presence(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.check_presence = Some(v.into());
        self
    }

    #[doc= "Set the field `include`.\nList of cookies to include in the custom key."]
    pub fn set_include(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.include = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {}

impl BuildRulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    pub fn build(self) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
            check_presence: core::default::Default::default(),
            include: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `check_presence` after provisioning.\nList of cookies to check for presence in the custom key."]
    pub fn check_presence(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\nList of cookies to include in the custom key."]
    pub fn include(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    check_presence: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_origin: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<SetField<PrimField<String>>>,
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    #[doc= "Set the field `check_presence`.\nList of headers to check for presence in the custom key."]
    pub fn set_check_presence(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.check_presence = Some(v.into());
        self
    }

    #[doc= "Set the field `exclude_origin`.\nExclude the origin header from the custom key."]
    pub fn set_exclude_origin(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.exclude_origin = Some(v.into());
        self
    }

    #[doc= "Set the field `include`.\nList of headers to include in the custom key."]
    pub fn set_include(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.include = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {}

impl BuildRulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    pub fn build(self) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
            check_presence: core::default::Default::default(),
            exclude_origin: core::default::Default::default(),
            include: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `check_presence` after provisioning.\nList of headers to check for presence in the custom key."]
    pub fn check_presence(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc= "Get a reference to the value of field `exclude_origin` after provisioning.\nExclude the origin header from the custom key."]
    pub fn exclude_origin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_origin", self.base))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\nList of headers to include in the custom key."]
    pub fn include(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resolved: Option<PrimField<bool>>,
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    #[doc= "Set the field `resolved`.\nResolve hostname to IP address."]
    pub fn set_resolved(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.resolved = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl {}

impl BuildRulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    pub fn build(self) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl { resolved: core::default::Default::default() }
    }
}

pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `resolved` after provisioning.\nResolve hostname to IP address."]
    pub fn resolved(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolved", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<SetField<PrimField<String>>>,
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    #[doc= "Set the field `exclude`.\nList of query string parameters to exclude from the custom key."]
    pub fn set_exclude(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exclude = Some(v.into());
        self
    }

    #[doc= "Set the field `include`.\nList of query string parameters to include in the custom key."]
    pub fn set_include(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.include = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {}

impl BuildRulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    pub fn build(self) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
            exclude: core::default::Default::default(),
            include: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exclude` after provisioning.\nList of query string parameters to exclude from the custom key."]
    pub fn exclude(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\nList of query string parameters to include in the custom key."]
    pub fn include(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<PrimField<bool>>,
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
    #[doc= "Set the field `device_type`.\nAdd device type to the custom key."]
    pub fn set_device_type(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.device_type = Some(v.into());
        self
    }

    #[doc= "Set the field `geo`.\nAdd geo data to the custom key."]
    pub fn set_geo(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.geo = Some(v.into());
        self
    }

    #[doc= "Set the field `lang`.\nAdd language data to the custom key."]
    pub fn set_lang(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.lang = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl {}

impl BuildRulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
    pub fn build(self) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
            device_type: core::default::Default::default(),
            geo: core::default::Default::default(),
            lang: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `device_type` after provisioning.\nAdd device type to the custom key."]
    pub fn device_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.base))
    }

    #[doc= "Get a reference to the value of field `geo` after provisioning.\nAdd geo data to the custom key."]
    pub fn geo(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc= "Get a reference to the value of field `lang` after provisioning.\nAdd language data to the custom key."]
    pub fn lang(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lang", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElDynamic {
    cookie: Option<DynamicBlock<RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>>,
    header: Option<DynamicBlock<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>>,
    host: Option<DynamicBlock<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl>>,
    query_string: Option<DynamicBlock<RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>>,
    user: Option<DynamicBlock<RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie: Option<Vec<RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<Vec<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<Vec<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string: Option<Vec<RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<Vec<RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl>>,
    dynamic: RulesetRulesElActionParametersElCacheKeyElCustomKeyElDynamic,
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyEl {
    #[doc= "Set the field `cookie`.\n"]
    pub fn set_cookie(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cookie = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cookie = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `host`.\n"]
    pub fn set_host(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.host = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.host = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_string = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `user`.\n"]
    pub fn set_user(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElCacheKeyElCustomKeyEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElCacheKeyElCustomKeyEl {}

impl BuildRulesetRulesElActionParametersElCacheKeyElCustomKeyEl {
    pub fn build(self) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyEl {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyEl {
            cookie: core::default::Default::default(),
            header: core::default::Default::default(),
            host: core::default::Default::default(),
            query_string: core::default::Default::default(),
            user: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElCacheKeyElCustomKeyElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElRef {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cookie` after provisioning.\n"]
    pub fn cookie(&self) -> ListRef<RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cookie", self.base))
    }

    #[doc= "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> ListRef<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> ListRef<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc= "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> ListRef<RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef> {
        ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> ListRef<RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElCacheKeyElDynamic {
    custom_key: Option<DynamicBlock<RulesetRulesElActionParametersElCacheKeyElCustomKeyEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_by_device_type: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_deception_armor: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_query_strings_order: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_key: Option<Vec<RulesetRulesElActionParametersElCacheKeyElCustomKeyEl>>,
    dynamic: RulesetRulesElActionParametersElCacheKeyElDynamic,
}

impl RulesetRulesElActionParametersElCacheKeyEl {
    #[doc= "Set the field `cache_by_device_type`.\nCache by device type."]
    pub fn set_cache_by_device_type(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cache_by_device_type = Some(v.into());
        self
    }

    #[doc= "Set the field `cache_deception_armor`.\nCache deception armor."]
    pub fn set_cache_deception_armor(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cache_deception_armor = Some(v.into());
        self
    }

    #[doc= "Set the field `ignore_query_strings_order`.\nIgnore query strings order."]
    pub fn set_ignore_query_strings_order(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_query_strings_order = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_key`.\n"]
    pub fn set_custom_key(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_key = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_key = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElCacheKeyEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElCacheKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElCacheKeyEl {}

impl BuildRulesetRulesElActionParametersElCacheKeyEl {
    pub fn build(self) -> RulesetRulesElActionParametersElCacheKeyEl {
        RulesetRulesElActionParametersElCacheKeyEl {
            cache_by_device_type: core::default::Default::default(),
            cache_deception_armor: core::default::Default::default(),
            ignore_query_strings_order: core::default::Default::default(),
            custom_key: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElCacheKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElCacheKeyElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElCacheKeyElRef {
        RulesetRulesElActionParametersElCacheKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cache_by_device_type` after provisioning.\nCache by device type."]
    pub fn cache_by_device_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_by_device_type", self.base))
    }

    #[doc= "Get a reference to the value of field `cache_deception_armor` after provisioning.\nCache deception armor."]
    pub fn cache_deception_armor(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_deception_armor", self.base))
    }

    #[doc= "Get a reference to the value of field `ignore_query_strings_order` after provisioning.\nIgnore query strings order."]
    pub fn ignore_query_strings_order(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_query_strings_order", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_key` after provisioning.\n"]
    pub fn custom_key(&self) -> ListRef<RulesetRulesElActionParametersElCacheKeyElCustomKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_key", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    #[doc= "Set the field `from`.\nFrom status code."]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc= "Set the field `to`.\nTo status code."]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {}

impl BuildRulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    pub fn build(self) -> RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
        RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
        RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `from` after provisioning.\nFrom status code."]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc= "Get a reference to the value of field `to` after provisioning.\nTo status code."]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElDynamic {
    status_code_range: Option<
        DynamicBlock<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl>,
    >,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code_range: Option<Vec<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl>>,
    dynamic: RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElDynamic,
}

impl RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    #[doc= "Set the field `status_code`.\nStatus code for which the edge TTL is applied."]
    pub fn set_status_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.status_code = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\nStatus code edge TTL value."]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }

    #[doc= "Set the field `status_code_range`.\n"]
    pub fn set_status_code_range(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.status_code_range = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.status_code_range = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {}

impl BuildRulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    pub fn build(self) -> RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
        RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
            status_code: core::default::Default::default(),
            value: core::default::Default::default(),
            status_code_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
        RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `status_code` after provisioning.\nStatus code for which the edge TTL is applied."]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nStatus code edge TTL value."]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code_range` after provisioning.\n"]
    pub fn status_code_range(
        &self,
    ) -> ListRef<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.status_code_range", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElEdgeTtlElDynamic {
    status_code_ttl: Option<DynamicBlock<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElEdgeTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<f64>>,
    mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code_ttl: Option<Vec<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>>,
    dynamic: RulesetRulesElActionParametersElEdgeTtlElDynamic,
}

impl RulesetRulesElActionParametersElEdgeTtlEl {
    #[doc= "Set the field `default`.\nDefault edge TTL"]
    pub fn set_default(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default = Some(v.into());
        self
    }

    #[doc= "Set the field `status_code_ttl`.\n"]
    pub fn set_status_code_ttl(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.status_code_ttl = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.status_code_ttl = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElEdgeTtlEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElEdgeTtlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElEdgeTtlEl {
    #[doc= "Mode of the edge TTL."]
    pub mode: PrimField<String>,
}

impl BuildRulesetRulesElActionParametersElEdgeTtlEl {
    pub fn build(self) -> RulesetRulesElActionParametersElEdgeTtlEl {
        RulesetRulesElActionParametersElEdgeTtlEl {
            default: core::default::Default::default(),
            mode: self.mode,
            status_code_ttl: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElEdgeTtlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElEdgeTtlElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElEdgeTtlElRef {
        RulesetRulesElActionParametersElEdgeTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElEdgeTtlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default` after provisioning.\nDefault edge TTL"]
    pub fn default(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nMode of the edge TTL."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code_ttl` after provisioning.\n"]
    pub fn status_code_ttl(&self) -> ListRef<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.status_code_ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElFromListEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl RulesetRulesElActionParametersElFromListEl {
    #[doc= "Set the field `key`.\nExpression to use for the list lookup."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nName of the list."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElFromListEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElFromListEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElFromListEl {}

impl BuildRulesetRulesElActionParametersElFromListEl {
    pub fn build(self) -> RulesetRulesElActionParametersElFromListEl {
        RulesetRulesElActionParametersElFromListEl {
            key: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElFromListElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElFromListElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElFromListElRef {
        RulesetRulesElActionParametersElFromListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElFromListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nExpression to use for the list lookup."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the list."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElFromValueElTargetUrlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl RulesetRulesElActionParametersElFromValueElTargetUrlEl {
    #[doc= "Set the field `expression`.\nUse a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\nStatic value to provide as the HTTP request header value."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElFromValueElTargetUrlEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElFromValueElTargetUrlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElFromValueElTargetUrlEl {}

impl BuildRulesetRulesElActionParametersElFromValueElTargetUrlEl {
    pub fn build(self) -> RulesetRulesElActionParametersElFromValueElTargetUrlEl {
        RulesetRulesElActionParametersElFromValueElTargetUrlEl {
            expression: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElFromValueElTargetUrlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElFromValueElTargetUrlElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElFromValueElTargetUrlElRef {
        RulesetRulesElActionParametersElFromValueElTargetUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElFromValueElTargetUrlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\nUse a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nStatic value to provide as the HTTP request header value."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElFromValueElDynamic {
    target_url: Option<DynamicBlock<RulesetRulesElActionParametersElFromValueElTargetUrlEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElFromValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_query_string: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_url: Option<Vec<RulesetRulesElActionParametersElFromValueElTargetUrlEl>>,
    dynamic: RulesetRulesElActionParametersElFromValueElDynamic,
}

impl RulesetRulesElActionParametersElFromValueEl {
    #[doc= "Set the field `preserve_query_string`.\nPreserve query string for redirect URL."]
    pub fn set_preserve_query_string(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preserve_query_string = Some(v.into());
        self
    }

    #[doc= "Set the field `status_code`.\nStatus code for redirect."]
    pub fn set_status_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.status_code = Some(v.into());
        self
    }

    #[doc= "Set the field `target_url`.\n"]
    pub fn set_target_url(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElFromValueElTargetUrlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_url = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_url = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElFromValueEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElFromValueEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElFromValueEl {}

impl BuildRulesetRulesElActionParametersElFromValueEl {
    pub fn build(self) -> RulesetRulesElActionParametersElFromValueEl {
        RulesetRulesElActionParametersElFromValueEl {
            preserve_query_string: core::default::Default::default(),
            status_code: core::default::Default::default(),
            target_url: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElFromValueElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElFromValueElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElFromValueElRef {
        RulesetRulesElActionParametersElFromValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElFromValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `preserve_query_string` after provisioning.\nPreserve query string for redirect URL."]
    pub fn preserve_query_string(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_query_string", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code` after provisioning.\nStatus code for redirect."]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc= "Get a reference to the value of field `target_url` after provisioning.\n"]
    pub fn target_url(&self) -> ListRef<RulesetRulesElActionParametersElFromValueElTargetUrlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_url", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl RulesetRulesElActionParametersElHeadersEl {
    #[doc= "Set the field `expression`.\nUse a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nName of the HTTP request header to target."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `operation`.\nAction to perform on the HTTP request header. Available values: `remove`, `set`, `add`."]
    pub fn set_operation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operation = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\nStatic value to provide as the HTTP request header value."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElHeadersEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElHeadersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElHeadersEl {}

impl BuildRulesetRulesElActionParametersElHeadersEl {
    pub fn build(self) -> RulesetRulesElActionParametersElHeadersEl {
        RulesetRulesElActionParametersElHeadersEl {
            expression: core::default::Default::default(),
            name: core::default::Default::default(),
            operation: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElHeadersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElHeadersElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElHeadersElRef {
        RulesetRulesElActionParametersElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\nUse a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the HTTP request header to target."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `operation` after provisioning.\nAction to perform on the HTTP request header. Available values: `remove`, `set`, `add`."]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nStatic value to provide as the HTTP request header value."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElMatchedDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<PrimField<String>>,
}

impl RulesetRulesElActionParametersElMatchedDataEl {
    #[doc= "Set the field `public_key`.\nPublic key to use within WAF Ruleset payload logging to view the HTTP request parameters. You can generate a public key [using the `matched-data-cli` command-line tool](https://developers.cloudflare.com/waf/managed-rulesets/payload-logging/command-line/generate-key-pair) or [in the Cloudflare dashboard](https://developers.cloudflare.com/waf/managed-rulesets/payload-logging/configure)."]
    pub fn set_public_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_key = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElMatchedDataEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElMatchedDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElMatchedDataEl {}

impl BuildRulesetRulesElActionParametersElMatchedDataEl {
    pub fn build(self) -> RulesetRulesElActionParametersElMatchedDataEl {
        RulesetRulesElActionParametersElMatchedDataEl { public_key: core::default::Default::default() }
    }
}

pub struct RulesetRulesElActionParametersElMatchedDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElMatchedDataElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElMatchedDataElRef {
        RulesetRulesElActionParametersElMatchedDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElMatchedDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\nPublic key to use within WAF Ruleset payload logging to view the HTTP request parameters. You can generate a public key [using the `matched-data-cli` command-line tool](https://developers.cloudflare.com/waf/managed-rulesets/payload-logging/command-line/generate-key-pair) or [in the Cloudflare dashboard](https://developers.cloudflare.com/waf/managed-rulesets/payload-logging/configure)."]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElOriginEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}

impl RulesetRulesElActionParametersElOriginEl {
    #[doc= "Set the field `host`.\nOrigin Hostname where request is sent."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\nOrigin Port where request is sent."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElOriginEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElOriginEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElOriginEl {}

impl BuildRulesetRulesElActionParametersElOriginEl {
    pub fn build(self) -> RulesetRulesElActionParametersElOriginEl {
        RulesetRulesElActionParametersElOriginEl {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElOriginElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElOriginElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElOriginElRef {
        RulesetRulesElActionParametersElOriginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElOriginElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\nOrigin Hostname where request is sent."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\nOrigin Port where request is sent."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElOverridesElCategoriesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl RulesetRulesElActionParametersElOverridesElCategoriesEl {
    #[doc= "Set the field `action`.\nAction to perform in the tag-level override. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc= "Set the field `category`.\nTag name to apply the ruleset rule override to."]
    pub fn set_category(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.category = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nDefines if the current tag-level override enables or disables the ruleset rules with the specified tag."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElOverridesElCategoriesEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElOverridesElCategoriesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElOverridesElCategoriesEl {}

impl BuildRulesetRulesElActionParametersElOverridesElCategoriesEl {
    pub fn build(self) -> RulesetRulesElActionParametersElOverridesElCategoriesEl {
        RulesetRulesElActionParametersElOverridesElCategoriesEl {
            action: core::default::Default::default(),
            category: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElOverridesElCategoriesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElOverridesElCategoriesElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElOverridesElCategoriesElRef {
        RulesetRulesElActionParametersElOverridesElCategoriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElOverridesElCategoriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\nAction to perform in the tag-level override. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `category` after provisioning.\nTag name to apply the ruleset rule override to."]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nDefines if the current tag-level override enables or disables the ruleset rules with the specified tag."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElOverridesElRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitivity_level: Option<PrimField<String>>,
}

impl RulesetRulesElActionParametersElOverridesElRulesEl {
    #[doc= "Set the field `action`.\nAction to perform in the rule-level override. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nDefines if the current rule-level override enables or disables the rule."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\nRule ID to apply the override to."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `score_threshold`.\nAnomaly score threshold to apply in the ruleset rule override. Only applicable to modsecurity-based rulesets."]
    pub fn set_score_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.score_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `sensitivity_level`.\nSensitivity level for a ruleset rule override."]
    pub fn set_sensitivity_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sensitivity_level = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElOverridesElRulesEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElOverridesElRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElOverridesElRulesEl {}

impl BuildRulesetRulesElActionParametersElOverridesElRulesEl {
    pub fn build(self) -> RulesetRulesElActionParametersElOverridesElRulesEl {
        RulesetRulesElActionParametersElOverridesElRulesEl {
            action: core::default::Default::default(),
            enabled: core::default::Default::default(),
            id: core::default::Default::default(),
            score_threshold: core::default::Default::default(),
            sensitivity_level: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElOverridesElRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElOverridesElRulesElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElOverridesElRulesElRef {
        RulesetRulesElActionParametersElOverridesElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElOverridesElRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\nAction to perform in the rule-level override. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nDefines if the current rule-level override enables or disables the rule."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nRule ID to apply the override to."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `score_threshold` after provisioning.\nAnomaly score threshold to apply in the ruleset rule override. Only applicable to modsecurity-based rulesets."]
    pub fn score_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.score_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `sensitivity_level` after provisioning.\nSensitivity level for a ruleset rule override."]
    pub fn sensitivity_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElOverridesElDynamic {
    categories: Option<DynamicBlock<RulesetRulesElActionParametersElOverridesElCategoriesEl>>,
    rules: Option<DynamicBlock<RulesetRulesElActionParametersElOverridesElRulesEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElOverridesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitivity_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<Vec<RulesetRulesElActionParametersElOverridesElCategoriesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<RulesetRulesElActionParametersElOverridesElRulesEl>>,
    dynamic: RulesetRulesElActionParametersElOverridesElDynamic,
}

impl RulesetRulesElActionParametersElOverridesEl {
    #[doc= "Set the field `action`.\nAction to perform in the rule-level override. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nDefines if the current ruleset-level override enables or disables the ruleset."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `sensitivity_level`.\nSensitivity level to override for all ruleset rules. Available values: `default`, `medium`, `low`, `eoff`."]
    pub fn set_sensitivity_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sensitivity_level = Some(v.into());
        self
    }

    #[doc= "Set the field `categories`.\n"]
    pub fn set_categories(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElOverridesElCategoriesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.categories = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.categories = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElOverridesElRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rules = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElOverridesEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElOverridesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElOverridesEl {}

impl BuildRulesetRulesElActionParametersElOverridesEl {
    pub fn build(self) -> RulesetRulesElActionParametersElOverridesEl {
        RulesetRulesElActionParametersElOverridesEl {
            action: core::default::Default::default(),
            enabled: core::default::Default::default(),
            sensitivity_level: core::default::Default::default(),
            categories: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElOverridesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElOverridesElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElOverridesElRef {
        RulesetRulesElActionParametersElOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\nAction to perform in the rule-level override. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nDefines if the current ruleset-level override enables or disables the ruleset."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `sensitivity_level` after provisioning.\nSensitivity level to override for all ruleset rules. Available values: `default`, `medium`, `low`, `eoff`."]
    pub fn sensitivity_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }

    #[doc= "Get a reference to the value of field `categories` after provisioning.\n"]
    pub fn categories(&self) -> ListRef<RulesetRulesElActionParametersElOverridesElCategoriesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.categories", self.base))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<RulesetRulesElActionParametersElOverridesElRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<f64>>,
}

impl RulesetRulesElActionParametersElResponseEl {
    #[doc= "Set the field `content`.\nBody content to include in the response."]
    pub fn set_content(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content = Some(v.into());
        self
    }

    #[doc= "Set the field `content_type`.\nHTTP content type to send in the response."]
    pub fn set_content_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_type = Some(v.into());
        self
    }

    #[doc= "Set the field `status_code`.\nHTTP status code to send in the response."]
    pub fn set_status_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.status_code = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElResponseEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElResponseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElResponseEl {}

impl BuildRulesetRulesElActionParametersElResponseEl {
    pub fn build(self) -> RulesetRulesElActionParametersElResponseEl {
        RulesetRulesElActionParametersElResponseEl {
            content: core::default::Default::default(),
            content_type: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElResponseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElResponseElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElResponseElRef {
        RulesetRulesElActionParametersElResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nBody content to include in the response."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\nHTTP content type to send in the response."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code` after provisioning.\nHTTP status code to send in the response."]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElServeStaleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_stale_while_updating: Option<PrimField<bool>>,
}

impl RulesetRulesElActionParametersElServeStaleEl {
    #[doc= "Set the field `disable_stale_while_updating`.\nDisable stale while updating."]
    pub fn set_disable_stale_while_updating(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_stale_while_updating = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElServeStaleEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElServeStaleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElServeStaleEl {}

impl BuildRulesetRulesElActionParametersElServeStaleEl {
    pub fn build(self) -> RulesetRulesElActionParametersElServeStaleEl {
        RulesetRulesElActionParametersElServeStaleEl {
            disable_stale_while_updating: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElServeStaleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElServeStaleElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElServeStaleElRef {
        RulesetRulesElActionParametersElServeStaleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElServeStaleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disable_stale_while_updating` after provisioning.\nDisable stale while updating."]
    pub fn disable_stale_while_updating(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_stale_while_updating", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElSniEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl RulesetRulesElActionParametersElSniEl {
    #[doc= "Set the field `value`.\nValue to define for SNI."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElSniEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElSniEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElSniEl {}

impl BuildRulesetRulesElActionParametersElSniEl {
    pub fn build(self) -> RulesetRulesElActionParametersElSniEl {
        RulesetRulesElActionParametersElSniEl { value: core::default::Default::default() }
    }
}

pub struct RulesetRulesElActionParametersElSniElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElSniElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElSniElRef {
        RulesetRulesElActionParametersElSniElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElSniElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nValue to define for SNI."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElUriElPathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl RulesetRulesElActionParametersElUriElPathEl {
    #[doc= "Set the field `expression`.\nExpression that defines the updated (dynamic) value of the URI path or query string component. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\nStatic string value of the updated URI path or query string component."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElUriElPathEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElUriElPathEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElUriElPathEl {}

impl BuildRulesetRulesElActionParametersElUriElPathEl {
    pub fn build(self) -> RulesetRulesElActionParametersElUriElPathEl {
        RulesetRulesElActionParametersElUriElPathEl {
            expression: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElUriElPathElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElUriElPathElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElUriElPathElRef {
        RulesetRulesElActionParametersElUriElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElUriElPathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\nExpression that defines the updated (dynamic) value of the URI path or query string component. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nStatic string value of the updated URI path or query string component."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElUriElQueryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl RulesetRulesElActionParametersElUriElQueryEl {
    #[doc= "Set the field `expression`.\nExpression that defines the updated (dynamic) value of the URI path or query string component. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\nStatic string value of the updated URI path or query string component."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElUriElQueryEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElUriElQueryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElUriElQueryEl {}

impl BuildRulesetRulesElActionParametersElUriElQueryEl {
    pub fn build(self) -> RulesetRulesElActionParametersElUriElQueryEl {
        RulesetRulesElActionParametersElUriElQueryEl {
            expression: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElUriElQueryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElUriElQueryElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElUriElQueryElRef {
        RulesetRulesElActionParametersElUriElQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElUriElQueryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\nExpression that defines the updated (dynamic) value of the URI path or query string component. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nStatic string value of the updated URI path or query string component."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElUriElDynamic {
    path: Option<DynamicBlock<RulesetRulesElActionParametersElUriElPathEl>>,
    query: Option<DynamicBlock<RulesetRulesElActionParametersElUriElQueryEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElUriEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<Vec<RulesetRulesElActionParametersElUriElPathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<Vec<RulesetRulesElActionParametersElUriElQueryEl>>,
    dynamic: RulesetRulesElActionParametersElUriElDynamic,
}

impl RulesetRulesElActionParametersElUriEl {
    #[doc= "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.origin = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<BlockAssignable<RulesetRulesElActionParametersElUriElPathEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.path = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.path = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `query`.\n"]
    pub fn set_query(mut self, v: impl Into<BlockAssignable<RulesetRulesElActionParametersElUriElQueryEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersElUriEl {
    type O = BlockAssignable<RulesetRulesElActionParametersElUriEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElUriEl {}

impl BuildRulesetRulesElActionParametersElUriEl {
    pub fn build(self) -> RulesetRulesElActionParametersElUriEl {
        RulesetRulesElActionParametersElUriEl {
            origin: core::default::Default::default(),
            path: core::default::Default::default(),
            query: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElUriElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElUriElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElUriElRef {
        RulesetRulesElActionParametersElUriElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElUriElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> ListRef<RulesetRulesElActionParametersElUriElPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> ListRef<RulesetRulesElActionParametersElUriElQueryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.query", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElDynamic {
    algorithms: Option<DynamicBlock<RulesetRulesElActionParametersElAlgorithmsEl>>,
    autominify: Option<DynamicBlock<RulesetRulesElActionParametersElAutominifyEl>>,
    browser_ttl: Option<DynamicBlock<RulesetRulesElActionParametersElBrowserTtlEl>>,
    cache_key: Option<DynamicBlock<RulesetRulesElActionParametersElCacheKeyEl>>,
    edge_ttl: Option<DynamicBlock<RulesetRulesElActionParametersElEdgeTtlEl>>,
    from_list: Option<DynamicBlock<RulesetRulesElActionParametersElFromListEl>>,
    from_value: Option<DynamicBlock<RulesetRulesElActionParametersElFromValueEl>>,
    headers: Option<DynamicBlock<RulesetRulesElActionParametersElHeadersEl>>,
    matched_data: Option<DynamicBlock<RulesetRulesElActionParametersElMatchedDataEl>>,
    origin: Option<DynamicBlock<RulesetRulesElActionParametersElOriginEl>>,
    overrides: Option<DynamicBlock<RulesetRulesElActionParametersElOverridesEl>>,
    response: Option<DynamicBlock<RulesetRulesElActionParametersElResponseEl>>,
    serve_stale: Option<DynamicBlock<RulesetRulesElActionParametersElServeStaleEl>>,
    sni: Option<DynamicBlock<RulesetRulesElActionParametersElSniEl>>,
    uri: Option<DynamicBlock<RulesetRulesElActionParametersElUriEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_https_rewrites: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bic: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie_fields: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_apps: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_railgun: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_zaraz: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_obfuscation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hotlink_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    increment: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirage: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opportunistic_encryption: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_error_page_passthru: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phases: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    polish: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    products: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_fields: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    respect_strong_etags: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_fields: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rocket_loader: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ruleset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rulesets: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_excludes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sxg: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithms: Option<Vec<RulesetRulesElActionParametersElAlgorithmsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autominify: Option<Vec<RulesetRulesElActionParametersElAutominifyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    browser_ttl: Option<Vec<RulesetRulesElActionParametersElBrowserTtlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_key: Option<Vec<RulesetRulesElActionParametersElCacheKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_ttl: Option<Vec<RulesetRulesElActionParametersElEdgeTtlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_list: Option<Vec<RulesetRulesElActionParametersElFromListEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_value: Option<Vec<RulesetRulesElActionParametersElFromValueEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Vec<RulesetRulesElActionParametersElHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matched_data: Option<Vec<RulesetRulesElActionParametersElMatchedDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<Vec<RulesetRulesElActionParametersElOriginEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<Vec<RulesetRulesElActionParametersElOverridesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<Vec<RulesetRulesElActionParametersElResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serve_stale: Option<Vec<RulesetRulesElActionParametersElServeStaleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sni: Option<Vec<RulesetRulesElActionParametersElSniEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<Vec<RulesetRulesElActionParametersElUriEl>>,
    dynamic: RulesetRulesElActionParametersElDynamic,
}

impl RulesetRulesElActionParametersEl {
    #[doc= "Set the field `automatic_https_rewrites`.\nTurn on or off Cloudflare Automatic HTTPS rewrites."]
    pub fn set_automatic_https_rewrites(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.automatic_https_rewrites = Some(v.into());
        self
    }

    #[doc= "Set the field `bic`.\nInspect the visitor's browser for headers commonly associated with spammers and certain bots."]
    pub fn set_bic(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bic = Some(v.into());
        self
    }

    #[doc= "Set the field `cache`.\nWhether to cache if expression matches."]
    pub fn set_cache(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cache = Some(v.into());
        self
    }

    #[doc= "Set the field `content`.\nContent of the custom error response."]
    pub fn set_content(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content = Some(v.into());
        self
    }

    #[doc= "Set the field `content_type`.\nContent-Type of the custom error response."]
    pub fn set_content_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_type = Some(v.into());
        self
    }

    #[doc= "Set the field `cookie_fields`.\nList of cookie values to include as part of custom fields logging."]
    pub fn set_cookie_fields(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.cookie_fields = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_apps`.\nTurn off all active Cloudflare Apps."]
    pub fn set_disable_apps(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_apps = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_railgun`.\nTurn off railgun feature of the Cloudflare Speed app."]
    pub fn set_disable_railgun(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_railgun = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_zaraz`.\nTurn off zaraz feature."]
    pub fn set_disable_zaraz(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_zaraz = Some(v.into());
        self
    }

    #[doc= "Set the field `email_obfuscation`.\nTurn on or off the Cloudflare Email Obfuscation feature of the Cloudflare Scrape Shield app."]
    pub fn set_email_obfuscation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.email_obfuscation = Some(v.into());
        self
    }

    #[doc= "Set the field `host_header`.\nHost Header that request origin receives."]
    pub fn set_host_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_header = Some(v.into());
        self
    }

    #[doc= "Set the field `hotlink_protection`.\nTurn on or off the hotlink protection feature."]
    pub fn set_hotlink_protection(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.hotlink_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\nIdentifier of the action parameter to modify."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `increment`.\n"]
    pub fn set_increment(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.increment = Some(v.into());
        self
    }

    #[doc= "Set the field `mirage`.\nTurn on or off Cloudflare Mirage of the Cloudflare Speed app."]
    pub fn set_mirage(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mirage = Some(v.into());
        self
    }

    #[doc= "Set the field `opportunistic_encryption`.\nTurn on or off the Cloudflare Opportunistic Encryption feature of the Edge Certificates tab in the Cloudflare SSL/TLS app."]
    pub fn set_opportunistic_encryption(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.opportunistic_encryption = Some(v.into());
        self
    }

    #[doc= "Set the field `origin_error_page_passthru`.\nPass-through error page for origin."]
    pub fn set_origin_error_page_passthru(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.origin_error_page_passthru = Some(v.into());
        self
    }

    #[doc= "Set the field `phases`.\nPoint in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub fn set_phases(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.phases = Some(v.into());
        self
    }

    #[doc= "Set the field `polish`.\nApply options from the Polish feature of the Cloudflare Speed app."]
    pub fn set_polish(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.polish = Some(v.into());
        self
    }

    #[doc= "Set the field `products`.\nProducts to target with the actions. Available values: `bic`, `hot`, `ratelimit`, `securityLevel`, `uablock`, `waf`, `zonelockdown`."]
    pub fn set_products(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.products = Some(v.into());
        self
    }

    #[doc= "Set the field `request_fields`.\nList of request headers to include as part of custom fields logging, in lowercase."]
    pub fn set_request_fields(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.request_fields = Some(v.into());
        self
    }

    #[doc= "Set the field `respect_strong_etags`.\nRespect strong ETags."]
    pub fn set_respect_strong_etags(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.respect_strong_etags = Some(v.into());
        self
    }

    #[doc= "Set the field `response_fields`.\nList of response headers to include as part of custom fields logging, in lowercase."]
    pub fn set_response_fields(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.response_fields = Some(v.into());
        self
    }

    #[doc= "Set the field `rocket_loader`.\nTurn on or off Cloudflare Rocket Loader in the Cloudflare Speed app."]
    pub fn set_rocket_loader(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.rocket_loader = Some(v.into());
        self
    }

    #[doc= "Set the field `rules`.\nMap of managed WAF rule ID to comma-delimited string of ruleset rule IDs. Example: `rules = { \"efb7b8c949ac4650a09736fc376e9aee\" = \"5de7edfa648c4d6891dc3e7f84534ffa,e3a567afc347477d9702d9047e97d760\" }`."]
    pub fn set_rules(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.rules = Some(v.into());
        self
    }

    #[doc= "Set the field `ruleset`.\nWhich ruleset ID to target."]
    pub fn set_ruleset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ruleset = Some(v.into());
        self
    }

    #[doc= "Set the field `rulesets`.\nList of managed WAF rule IDs to target. Only valid when the `\"action\"` is set to skip."]
    pub fn set_rulesets(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.rulesets = Some(v.into());
        self
    }

    #[doc= "Set the field `security_level`.\nControl options for the Security Level feature from the Security app."]
    pub fn set_security_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_level = Some(v.into());
        self
    }

    #[doc= "Set the field `server_side_excludes`.\nTurn on or off the Server Side Excludes feature of the Cloudflare Scrape Shield app."]
    pub fn set_server_side_excludes(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.server_side_excludes = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl`.\nControl options for the SSL feature of the Edge Certificates tab in the Cloudflare SSL/TLS app."]
    pub fn set_ssl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl = Some(v.into());
        self
    }

    #[doc= "Set the field `status_code`.\nHTTP status code of the custom error response."]
    pub fn set_status_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.status_code = Some(v.into());
        self
    }

    #[doc= "Set the field `sxg`.\nTurn on or off the SXG feature."]
    pub fn set_sxg(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.sxg = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\nVersion of the ruleset to deploy."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }

    #[doc= "Set the field `algorithms`.\n"]
    pub fn set_algorithms(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElAlgorithmsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.algorithms = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.algorithms = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `autominify`.\n"]
    pub fn set_autominify(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElAutominifyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autominify = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autominify = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `browser_ttl`.\n"]
    pub fn set_browser_ttl(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElBrowserTtlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.browser_ttl = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.browser_ttl = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `cache_key`.\n"]
    pub fn set_cache_key(mut self, v: impl Into<BlockAssignable<RulesetRulesElActionParametersElCacheKeyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_key = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_key = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `edge_ttl`.\n"]
    pub fn set_edge_ttl(mut self, v: impl Into<BlockAssignable<RulesetRulesElActionParametersElEdgeTtlEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.edge_ttl = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.edge_ttl = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `from_list`.\n"]
    pub fn set_from_list(mut self, v: impl Into<BlockAssignable<RulesetRulesElActionParametersElFromListEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.from_list = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.from_list = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `from_value`.\n"]
    pub fn set_from_value(mut self, v: impl Into<BlockAssignable<RulesetRulesElActionParametersElFromValueEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.from_value = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.from_value = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `headers`.\n"]
    pub fn set_headers(mut self, v: impl Into<BlockAssignable<RulesetRulesElActionParametersElHeadersEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.headers = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.headers = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `matched_data`.\n"]
    pub fn set_matched_data(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElMatchedDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.matched_data = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.matched_data = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl Into<BlockAssignable<RulesetRulesElActionParametersElOriginEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.origin = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.origin = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `overrides`.\n"]
    pub fn set_overrides(mut self, v: impl Into<BlockAssignable<RulesetRulesElActionParametersElOverridesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.overrides = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.overrides = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl Into<BlockAssignable<RulesetRulesElActionParametersElResponseEl>>) -> Self {
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

    #[doc= "Set the field `serve_stale`.\n"]
    pub fn set_serve_stale(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElActionParametersElServeStaleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.serve_stale = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.serve_stale = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `sni`.\n"]
    pub fn set_sni(mut self, v: impl Into<BlockAssignable<RulesetRulesElActionParametersElSniEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sni = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sni = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<BlockAssignable<RulesetRulesElActionParametersElUriEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.uri = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.uri = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RulesetRulesElActionParametersEl {
    type O = BlockAssignable<RulesetRulesElActionParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersEl {}

impl BuildRulesetRulesElActionParametersEl {
    pub fn build(self) -> RulesetRulesElActionParametersEl {
        RulesetRulesElActionParametersEl {
            automatic_https_rewrites: core::default::Default::default(),
            bic: core::default::Default::default(),
            cache: core::default::Default::default(),
            content: core::default::Default::default(),
            content_type: core::default::Default::default(),
            cookie_fields: core::default::Default::default(),
            disable_apps: core::default::Default::default(),
            disable_railgun: core::default::Default::default(),
            disable_zaraz: core::default::Default::default(),
            email_obfuscation: core::default::Default::default(),
            host_header: core::default::Default::default(),
            hotlink_protection: core::default::Default::default(),
            id: core::default::Default::default(),
            increment: core::default::Default::default(),
            mirage: core::default::Default::default(),
            opportunistic_encryption: core::default::Default::default(),
            origin_error_page_passthru: core::default::Default::default(),
            phases: core::default::Default::default(),
            polish: core::default::Default::default(),
            products: core::default::Default::default(),
            request_fields: core::default::Default::default(),
            respect_strong_etags: core::default::Default::default(),
            response_fields: core::default::Default::default(),
            rocket_loader: core::default::Default::default(),
            rules: core::default::Default::default(),
            ruleset: core::default::Default::default(),
            rulesets: core::default::Default::default(),
            security_level: core::default::Default::default(),
            server_side_excludes: core::default::Default::default(),
            ssl: core::default::Default::default(),
            status_code: core::default::Default::default(),
            sxg: core::default::Default::default(),
            version: core::default::Default::default(),
            algorithms: core::default::Default::default(),
            autominify: core::default::Default::default(),
            browser_ttl: core::default::Default::default(),
            cache_key: core::default::Default::default(),
            edge_ttl: core::default::Default::default(),
            from_list: core::default::Default::default(),
            from_value: core::default::Default::default(),
            headers: core::default::Default::default(),
            matched_data: core::default::Default::default(),
            origin: core::default::Default::default(),
            overrides: core::default::Default::default(),
            response: core::default::Default::default(),
            serve_stale: core::default::Default::default(),
            sni: core::default::Default::default(),
            uri: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElActionParametersElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElActionParametersElRef {
        RulesetRulesElActionParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `automatic_https_rewrites` after provisioning.\nTurn on or off Cloudflare Automatic HTTPS rewrites."]
    pub fn automatic_https_rewrites(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_https_rewrites", self.base))
    }

    #[doc= "Get a reference to the value of field `bic` after provisioning.\nInspect the visitor's browser for headers commonly associated with spammers and certain bots."]
    pub fn bic(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bic", self.base))
    }

    #[doc= "Get a reference to the value of field `cache` after provisioning.\nWhether to cache if expression matches."]
    pub fn cache(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache", self.base))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nContent of the custom error response."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\nContent-Type of the custom error response."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc= "Get a reference to the value of field `cookie_fields` after provisioning.\nList of cookie values to include as part of custom fields logging."]
    pub fn cookie_fields(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.cookie_fields", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_apps` after provisioning.\nTurn off all active Cloudflare Apps."]
    pub fn disable_apps(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_apps", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_railgun` after provisioning.\nTurn off railgun feature of the Cloudflare Speed app."]
    pub fn disable_railgun(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_railgun", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_zaraz` after provisioning.\nTurn off zaraz feature."]
    pub fn disable_zaraz(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_zaraz", self.base))
    }

    #[doc= "Get a reference to the value of field `email_obfuscation` after provisioning.\nTurn on or off the Cloudflare Email Obfuscation feature of the Cloudflare Scrape Shield app."]
    pub fn email_obfuscation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_obfuscation", self.base))
    }

    #[doc= "Get a reference to the value of field `host_header` after provisioning.\nHost Header that request origin receives."]
    pub fn host_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_header", self.base))
    }

    #[doc= "Get a reference to the value of field `hotlink_protection` after provisioning.\nTurn on or off the hotlink protection feature."]
    pub fn hotlink_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.hotlink_protection", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nIdentifier of the action parameter to modify."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `increment` after provisioning.\n"]
    pub fn increment(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.increment", self.base))
    }

    #[doc= "Get a reference to the value of field `mirage` after provisioning.\nTurn on or off Cloudflare Mirage of the Cloudflare Speed app."]
    pub fn mirage(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirage", self.base))
    }

    #[doc= "Get a reference to the value of field `opportunistic_encryption` after provisioning.\nTurn on or off the Cloudflare Opportunistic Encryption feature of the Edge Certificates tab in the Cloudflare SSL/TLS app."]
    pub fn opportunistic_encryption(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_encryption", self.base))
    }

    #[doc= "Get a reference to the value of field `origin_error_page_passthru` after provisioning.\nPass-through error page for origin."]
    pub fn origin_error_page_passthru(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_error_page_passthru", self.base))
    }

    #[doc= "Get a reference to the value of field `phases` after provisioning.\nPoint in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub fn phases(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.phases", self.base))
    }

    #[doc= "Get a reference to the value of field `polish` after provisioning.\nApply options from the Polish feature of the Cloudflare Speed app."]
    pub fn polish(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.polish", self.base))
    }

    #[doc= "Get a reference to the value of field `products` after provisioning.\nProducts to target with the actions. Available values: `bic`, `hot`, `ratelimit`, `securityLevel`, `uablock`, `waf`, `zonelockdown`."]
    pub fn products(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.products", self.base))
    }

    #[doc= "Get a reference to the value of field `request_fields` after provisioning.\nList of request headers to include as part of custom fields logging, in lowercase."]
    pub fn request_fields(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.request_fields", self.base))
    }

    #[doc= "Get a reference to the value of field `respect_strong_etags` after provisioning.\nRespect strong ETags."]
    pub fn respect_strong_etags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.respect_strong_etags", self.base))
    }

    #[doc= "Get a reference to the value of field `response_fields` after provisioning.\nList of response headers to include as part of custom fields logging, in lowercase."]
    pub fn response_fields(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.response_fields", self.base))
    }

    #[doc= "Get a reference to the value of field `rocket_loader` after provisioning.\nTurn on or off Cloudflare Rocket Loader in the Cloudflare Speed app."]
    pub fn rocket_loader(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rocket_loader", self.base))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\nMap of managed WAF rule ID to comma-delimited string of ruleset rule IDs. Example: `rules = { \"efb7b8c949ac4650a09736fc376e9aee\" = \"5de7edfa648c4d6891dc3e7f84534ffa,e3a567afc347477d9702d9047e97d760\" }`."]
    pub fn rules(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc= "Get a reference to the value of field `ruleset` after provisioning.\nWhich ruleset ID to target."]
    pub fn ruleset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ruleset", self.base))
    }

    #[doc= "Get a reference to the value of field `rulesets` after provisioning.\nList of managed WAF rule IDs to target. Only valid when the `\"action\"` is set to skip."]
    pub fn rulesets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.rulesets", self.base))
    }

    #[doc= "Get a reference to the value of field `security_level` after provisioning.\nControl options for the Security Level feature from the Security app."]
    pub fn security_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_level", self.base))
    }

    #[doc= "Get a reference to the value of field `server_side_excludes` after provisioning.\nTurn on or off the Server Side Excludes feature of the Cloudflare Scrape Shield app."]
    pub fn server_side_excludes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_side_excludes", self.base))
    }

    #[doc= "Get a reference to the value of field `ssl` after provisioning.\nControl options for the SSL feature of the Edge Certificates tab in the Cloudflare SSL/TLS app."]
    pub fn ssl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code` after provisioning.\nHTTP status code of the custom error response."]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc= "Get a reference to the value of field `sxg` after provisioning.\nTurn on or off the SXG feature."]
    pub fn sxg(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sxg", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nVersion of the ruleset to deploy."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc= "Get a reference to the value of field `algorithms` after provisioning.\n"]
    pub fn algorithms(&self) -> ListRef<RulesetRulesElActionParametersElAlgorithmsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.algorithms", self.base))
    }

    #[doc= "Get a reference to the value of field `autominify` after provisioning.\n"]
    pub fn autominify(&self) -> ListRef<RulesetRulesElActionParametersElAutominifyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autominify", self.base))
    }

    #[doc= "Get a reference to the value of field `browser_ttl` after provisioning.\n"]
    pub fn browser_ttl(&self) -> ListRef<RulesetRulesElActionParametersElBrowserTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.browser_ttl", self.base))
    }

    #[doc= "Get a reference to the value of field `cache_key` after provisioning.\n"]
    pub fn cache_key(&self) -> ListRef<RulesetRulesElActionParametersElCacheKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cache_key", self.base))
    }

    #[doc= "Get a reference to the value of field `edge_ttl` after provisioning.\n"]
    pub fn edge_ttl(&self) -> ListRef<RulesetRulesElActionParametersElEdgeTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.edge_ttl", self.base))
    }

    #[doc= "Get a reference to the value of field `from_list` after provisioning.\n"]
    pub fn from_list(&self) -> ListRef<RulesetRulesElActionParametersElFromListElRef> {
        ListRef::new(self.shared().clone(), format!("{}.from_list", self.base))
    }

    #[doc= "Get a reference to the value of field `from_value` after provisioning.\n"]
    pub fn from_value(&self) -> ListRef<RulesetRulesElActionParametersElFromValueElRef> {
        ListRef::new(self.shared().clone(), format!("{}.from_value", self.base))
    }

    #[doc= "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> ListRef<RulesetRulesElActionParametersElHeadersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc= "Get a reference to the value of field `matched_data` after provisioning.\n"]
    pub fn matched_data(&self) -> ListRef<RulesetRulesElActionParametersElMatchedDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.matched_data", self.base))
    }

    #[doc= "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> ListRef<RulesetRulesElActionParametersElOriginElRef> {
        ListRef::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc= "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(&self) -> ListRef<RulesetRulesElActionParametersElOverridesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.overrides", self.base))
    }

    #[doc= "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> ListRef<RulesetRulesElActionParametersElResponseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }

    #[doc= "Get a reference to the value of field `serve_stale` after provisioning.\n"]
    pub fn serve_stale(&self) -> ListRef<RulesetRulesElActionParametersElServeStaleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.serve_stale", self.base))
    }

    #[doc= "Get a reference to the value of field `sni` after provisioning.\n"]
    pub fn sni(&self) -> ListRef<RulesetRulesElActionParametersElSniElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sni", self.base))
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> ListRef<RulesetRulesElActionParametersElUriElRef> {
        ListRef::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElExposedCredentialCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username_expression: Option<PrimField<String>>,
}

impl RulesetRulesElExposedCredentialCheckEl {
    #[doc= "Set the field `password_expression`.\nFirewall Rules expression language based on Wireshark display filters for where to check for the \"password\" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language)."]
    pub fn set_password_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_expression = Some(v.into());
        self
    }

    #[doc= "Set the field `username_expression`.\nFirewall Rules expression language based on Wireshark display filters for where to check for the \"username\" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language)."]
    pub fn set_username_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username_expression = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElExposedCredentialCheckEl {
    type O = BlockAssignable<RulesetRulesElExposedCredentialCheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElExposedCredentialCheckEl {}

impl BuildRulesetRulesElExposedCredentialCheckEl {
    pub fn build(self) -> RulesetRulesElExposedCredentialCheckEl {
        RulesetRulesElExposedCredentialCheckEl {
            password_expression: core::default::Default::default(),
            username_expression: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElExposedCredentialCheckElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElExposedCredentialCheckElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElExposedCredentialCheckElRef {
        RulesetRulesElExposedCredentialCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElExposedCredentialCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `password_expression` after provisioning.\nFirewall Rules expression language based on Wireshark display filters for where to check for the \"password\" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language)."]
    pub fn password_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_expression", self.base))
    }

    #[doc= "Get a reference to the value of field `username_expression` after provisioning.\nFirewall Rules expression language based on Wireshark display filters for where to check for the \"username\" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language)."]
    pub fn username_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElLoggingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl RulesetRulesElLoggingEl {
    #[doc= "Set the field `enabled`.\nOverride the default logging behavior when a rule is matched."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElLoggingEl {
    type O = BlockAssignable<RulesetRulesElLoggingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElLoggingEl {}

impl BuildRulesetRulesElLoggingEl {
    pub fn build(self) -> RulesetRulesElLoggingEl {
        RulesetRulesElLoggingEl { enabled: core::default::Default::default() }
    }
}

pub struct RulesetRulesElLoggingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElLoggingElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElLoggingElRef {
        RulesetRulesElLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nOverride the default logging behavior when a rule is matched."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElRatelimitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    characteristics: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    counting_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mitigation_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requests_per_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requests_to_origin: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score_per_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score_response_header_name: Option<PrimField<String>>,
}

impl RulesetRulesElRatelimitEl {
    #[doc= "Set the field `characteristics`.\nList of parameters that define how Cloudflare tracks the request rate for this rule."]
    pub fn set_characteristics(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.characteristics = Some(v.into());
        self
    }

    #[doc= "Set the field `counting_expression`.\nCriteria for counting HTTP requests to trigger the Rate Limiting action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn set_counting_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.counting_expression = Some(v.into());
        self
    }

    #[doc= "Set the field `mitigation_timeout`.\nOnce the request rate is reached, the Rate Limiting rule blocks further requests for the period of time defined in this field."]
    pub fn set_mitigation_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.mitigation_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `period`.\nThe period of time to consider (in seconds) when evaluating the request rate."]
    pub fn set_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period = Some(v.into());
        self
    }

    #[doc= "Set the field `requests_per_period`.\nThe number of requests over the period of time that will trigger the Rate Limiting rule."]
    pub fn set_requests_per_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.requests_per_period = Some(v.into());
        self
    }

    #[doc= "Set the field `requests_to_origin`.\nWhether to include requests to origin within the Rate Limiting count."]
    pub fn set_requests_to_origin(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.requests_to_origin = Some(v.into());
        self
    }

    #[doc= "Set the field `score_per_period`.\nThe maximum aggregate score over the period of time that will trigger Rate Limiting rule."]
    pub fn set_score_per_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.score_per_period = Some(v.into());
        self
    }

    #[doc= "Set the field `score_response_header_name`.\nName of HTTP header in the response, set by the origin server, with the score for the current request."]
    pub fn set_score_response_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.score_response_header_name = Some(v.into());
        self
    }
}

impl ToListMappable for RulesetRulesElRatelimitEl {
    type O = BlockAssignable<RulesetRulesElRatelimitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElRatelimitEl {}

impl BuildRulesetRulesElRatelimitEl {
    pub fn build(self) -> RulesetRulesElRatelimitEl {
        RulesetRulesElRatelimitEl {
            characteristics: core::default::Default::default(),
            counting_expression: core::default::Default::default(),
            mitigation_timeout: core::default::Default::default(),
            period: core::default::Default::default(),
            requests_per_period: core::default::Default::default(),
            requests_to_origin: core::default::Default::default(),
            score_per_period: core::default::Default::default(),
            score_response_header_name: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElRatelimitElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElRatelimitElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElRatelimitElRef {
        RulesetRulesElRatelimitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElRatelimitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `characteristics` after provisioning.\nList of parameters that define how Cloudflare tracks the request rate for this rule."]
    pub fn characteristics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.characteristics", self.base))
    }

    #[doc= "Get a reference to the value of field `counting_expression` after provisioning.\nCriteria for counting HTTP requests to trigger the Rate Limiting action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn counting_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.counting_expression", self.base))
    }

    #[doc= "Get a reference to the value of field `mitigation_timeout` after provisioning.\nOnce the request rate is reached, the Rate Limiting rule blocks further requests for the period of time defined in this field."]
    pub fn mitigation_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mitigation_timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `period` after provisioning.\nThe period of time to consider (in seconds) when evaluating the request rate."]
    pub fn period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }

    #[doc= "Get a reference to the value of field `requests_per_period` after provisioning.\nThe number of requests over the period of time that will trigger the Rate Limiting rule."]
    pub fn requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.requests_per_period", self.base))
    }

    #[doc= "Get a reference to the value of field `requests_to_origin` after provisioning.\nWhether to include requests to origin within the Rate Limiting count."]
    pub fn requests_to_origin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.requests_to_origin", self.base))
    }

    #[doc= "Get a reference to the value of field `score_per_period` after provisioning.\nThe maximum aggregate score over the period of time that will trigger Rate Limiting rule."]
    pub fn score_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.score_per_period", self.base))
    }

    #[doc= "Get a reference to the value of field `score_response_header_name` after provisioning.\nName of HTTP header in the response, set by the origin server, with the score for the current request."]
    pub fn score_response_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.score_response_header_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElDynamic {
    action_parameters: Option<DynamicBlock<RulesetRulesElActionParametersEl>>,
    exposed_credential_check: Option<DynamicBlock<RulesetRulesElExposedCredentialCheckEl>>,
    logging: Option<DynamicBlock<RulesetRulesElLoggingEl>>,
    ratelimit: Option<DynamicBlock<RulesetRulesElRatelimitEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_updated: Option<PrimField<String>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_parameters: Option<Vec<RulesetRulesElActionParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exposed_credential_check: Option<Vec<RulesetRulesElExposedCredentialCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Vec<RulesetRulesElLoggingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ratelimit: Option<Vec<RulesetRulesElRatelimitEl>>,
    dynamic: RulesetRulesElDynamic,
}

impl RulesetRulesEl {
    #[doc= "Set the field `action`.\nAction to perform in the ruleset rule. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nBrief summary of the ruleset rule and its intended use."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nWhether the rule is active."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\nUnique rule identifier."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `last_updated`.\nThe most recent update to this rule."]
    pub fn set_last_updated(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_updated = Some(v.into());
        self
    }

    #[doc= "Set the field `ref_`.\nRule reference."]
    pub fn set_ref(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ref_ = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\nVersion of the ruleset to deploy."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }

    #[doc= "Set the field `action_parameters`.\n"]
    pub fn set_action_parameters(mut self, v: impl Into<BlockAssignable<RulesetRulesElActionParametersEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action_parameters = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `exposed_credential_check`.\n"]
    pub fn set_exposed_credential_check(
        mut self,
        v: impl Into<BlockAssignable<RulesetRulesElExposedCredentialCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exposed_credential_check = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exposed_credential_check = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `logging`.\n"]
    pub fn set_logging(mut self, v: impl Into<BlockAssignable<RulesetRulesElLoggingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logging = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logging = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `ratelimit`.\n"]
    pub fn set_ratelimit(mut self, v: impl Into<BlockAssignable<RulesetRulesElRatelimitEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ratelimit = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ratelimit = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RulesetRulesEl {
    type O = BlockAssignable<RulesetRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesEl {
    #[doc= "Criteria for an HTTP request to trigger the ruleset rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub expression: PrimField<String>,
}

impl BuildRulesetRulesEl {
    pub fn build(self) -> RulesetRulesEl {
        RulesetRulesEl {
            action: core::default::Default::default(),
            description: core::default::Default::default(),
            enabled: core::default::Default::default(),
            expression: self.expression,
            id: core::default::Default::default(),
            last_updated: core::default::Default::default(),
            ref_: core::default::Default::default(),
            version: core::default::Default::default(),
            action_parameters: core::default::Default::default(),
            exposed_credential_check: core::default::Default::default(),
            logging: core::default::Default::default(),
            ratelimit: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RulesetRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RulesetRulesElRef {
    fn new(shared: StackShared, base: String) -> RulesetRulesElRef {
        RulesetRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\nAction to perform in the ruleset rule. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nBrief summary of the ruleset rule and its intended use."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether the rule is active."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\nCriteria for an HTTP request to trigger the ruleset rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique rule identifier."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `last_updated` after provisioning.\nThe most recent update to this rule."]
    pub fn last_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\nRule reference."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nVersion of the ruleset to deploy."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc= "Get a reference to the value of field `action_parameters` after provisioning.\n"]
    pub fn action_parameters(&self) -> ListRef<RulesetRulesElActionParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action_parameters", self.base))
    }

    #[doc= "Get a reference to the value of field `exposed_credential_check` after provisioning.\n"]
    pub fn exposed_credential_check(&self) -> ListRef<RulesetRulesElExposedCredentialCheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exposed_credential_check", self.base))
    }

    #[doc= "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<RulesetRulesElLoggingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc= "Get a reference to the value of field `ratelimit` after provisioning.\n"]
    pub fn ratelimit(&self) -> ListRef<RulesetRulesElRatelimitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ratelimit", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetDynamic {
    rules: Option<DynamicBlock<RulesetRulesEl>>,
}

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct RulesetData {
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
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    kind: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    phase: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shareable_entitlement_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<RulesetRulesEl>>,
    dynamic: RulesetDynamic,
}

struct Ruleset_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RulesetData>,
}

#[derive(Clone)]
pub struct Ruleset(std::rc::Rc<Ruleset_>);

impl Ruleset {
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

    #[doc = "Set the field `account_id`.\nThe account identifier to target for the resource."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nBrief summary of the ruleset and its intended use."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shareable_entitlement_name`.\nName of entitlement that is shareable between entities."]
    pub fn set_shareable_entitlement_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().shareable_entitlement_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nThe zone identifier to target for the resource."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nBrief summary of the ruleset and its intended use."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe identifier of this resource."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nType of Ruleset to create. Available values: `custom`, `managed`, `root`, `schema`, `zone`."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the ruleset."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `phase` after provisioning.\nPoint in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shareable_entitlement_name` after provisioning.\nName of entitlement that is shareable between entities."]
    pub fn shareable_entitlement_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shareable_entitlement_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<RulesetRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl terrars::Referable for Ruleset {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Ruleset { }

impl terrars::ToListMappable for Ruleset {
    type O = terrars::ListRef<RulesetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Ruleset_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_ruleset".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRuleset {
    pub tf_id: std::string::String,
    #[doc = "Type of Ruleset to create. Available values: `custom`, `managed`, `root`, `schema`, `zone`."]
    pub kind: terrars::PrimField<std::string::String>,
    #[doc = "Name of the ruleset."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub phase: terrars::PrimField<std::string::String>,
}

impl BuildRuleset {
    pub fn build(self, stack: &mut terrars::Stack) -> Ruleset {
        let out = Ruleset(std::rc::Rc::new(Ruleset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RulesetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                description: core::default::Default::default(),
                kind: self.kind,
                name: self.name,
                phase: self.phase,
                shareable_entitlement_name: core::default::Default::default(),
                zone_id: core::default::Default::default(),
                rules: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RulesetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RulesetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nBrief summary of the ruleset and its intended use."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe identifier of this resource."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nType of Ruleset to create. Available values: `custom`, `managed`, `root`, `schema`, `zone`."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the ruleset."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `phase` after provisioning.\nPoint in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shareable_entitlement_name` after provisioning.\nName of entitlement that is shareable between entities."]
    pub fn shareable_entitlement_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shareable_entitlement_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<RulesetRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElAlgorithmsEl {
    name: terrars::PrimField<std::string::String>,
}

impl RulesetRulesElActionParametersElAlgorithmsEl { }

impl terrars::ToListMappable for RulesetRulesElActionParametersElAlgorithmsEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElAlgorithmsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElAlgorithmsEl {
    #[doc = "Name of the compression algorithm to use. Available values: `gzip`, `brotli`, `auto`, `default`, `none`"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildRulesetRulesElActionParametersElAlgorithmsEl {
    pub fn build(self) -> RulesetRulesElActionParametersElAlgorithmsEl {
        RulesetRulesElActionParametersElAlgorithmsEl { name: self.name }
    }
}

pub struct RulesetRulesElActionParametersElAlgorithmsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElAlgorithmsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElAlgorithmsElRef {
        RulesetRulesElActionParametersElAlgorithmsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElAlgorithmsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the compression algorithm to use. Available values: `gzip`, `brotli`, `auto`, `default`, `none`"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElAutominifyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    css: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    html: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    js: std::option::Option<terrars::PrimField<bool>>,
}

impl RulesetRulesElActionParametersElAutominifyEl {
    #[doc = "Set the field `css`.\nCSS minification."]
    pub fn set_css(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.css = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `html`.\nHTML minification."]
    pub fn set_html(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.html = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `js`.\nJS minification."]
    pub fn set_js(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.js = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElAutominifyEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElAutominifyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElAutominifyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElAutominifyElRef {
        RulesetRulesElActionParametersElAutominifyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElAutominifyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `css` after provisioning.\nCSS minification."]
    pub fn css(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.css", self.base))
    }

    #[doc = "Get a reference to the value of field `html` after provisioning.\nHTML minification."]
    pub fn html(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }

    #[doc = "Get a reference to the value of field `js` after provisioning.\nJS minification."]
    pub fn js(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.js", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElBrowserTtlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<f64>>,
    mode: terrars::PrimField<std::string::String>,
}

impl RulesetRulesElActionParametersElBrowserTtlEl {
    #[doc = "Set the field `default`.\nDefault browser TTL. This value is required when override_origin is set"]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElBrowserTtlEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElBrowserTtlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElBrowserTtlEl {
    #[doc = "Mode of the browser TTL."]
    pub mode: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElBrowserTtlElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElBrowserTtlElRef {
        RulesetRulesElActionParametersElBrowserTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElBrowserTtlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nDefault browser TTL. This value is required when override_origin is set"]
    pub fn default(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\nMode of the browser TTL."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_presence: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    #[doc = "Set the field `check_presence`.\nList of cookies to check for presence in the custom key."]
    pub fn set_check_presence(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.check_presence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include`.\nList of cookies to include in the custom key."]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `check_presence` after provisioning.\nList of cookies to check for presence in the custom key."]
    pub fn check_presence(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nList of cookies to include in the custom key."]
    pub fn include(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_presence: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_origin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    #[doc = "Set the field `check_presence`.\nList of headers to check for presence in the custom key."]
    pub fn set_check_presence(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.check_presence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_origin`.\nExclude the origin header from the custom key."]
    pub fn set_exclude_origin(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include`.\nList of headers to include in the custom key."]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `check_presence` after provisioning.\nList of headers to check for presence in the custom key."]
    pub fn check_presence(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_origin` after provisioning.\nExclude the origin header from the custom key."]
    pub fn exclude_origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nList of headers to include in the custom key."]
    pub fn include(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolved: std::option::Option<terrars::PrimField<bool>>,
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    #[doc = "Set the field `resolved`.\nResolve hostname to IP address."]
    pub fn set_resolved(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.resolved = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resolved` after provisioning.\nResolve hostname to IP address."]
    pub fn resolved(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolved", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    #[doc = "Set the field `exclude`.\nList of query string parameters to exclude from the custom key."]
    pub fn set_exclude(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include`.\nList of query string parameters to include in the custom key."]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nList of query string parameters to exclude from the custom key."]
    pub fn exclude(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nList of query string parameters to include in the custom key."]
    pub fn include(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lang: std::option::Option<terrars::PrimField<bool>>,
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
    #[doc = "Set the field `device_type`.\nAdd device type to the custom key."]
    pub fn set_device_type(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.device_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo`.\nAdd geo data to the custom key."]
    pub fn set_geo(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.geo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lang`.\nAdd language data to the custom key."]
    pub fn set_lang(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.lang = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `device_type` after provisioning.\nAdd device type to the custom key."]
    pub fn device_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.base))
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\nAdd geo data to the custom key."]
    pub fn geo(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `lang` after provisioning.\nAdd language data to the custom key."]
    pub fn lang(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lang", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElDynamic {
    cookie: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>>,
    header: std
    ::option
    ::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>>,
    host: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl>>,
    query_string: std
    ::option
    ::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>>,
    user: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl>>,
    dynamic: RulesetRulesElActionParametersElCacheKeyElCustomKeyElDynamic,
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyEl {
    #[doc = "Set the field `cookie`.\n"]
    pub fn set_cookie(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cookie = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cookie = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `host`.\n"]
    pub fn set_host(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.host = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.host = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user`.\n"]
    pub fn set_user(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElCacheKeyElCustomKeyEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElCacheKeyElCustomKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElCacheKeyElCustomKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElCacheKeyElCustomKeyElRef {
        RulesetRulesElActionParametersElCacheKeyElCustomKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElCustomKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookie` after provisioning.\n"]
    pub fn cookie(&self) -> terrars::ListRef<RulesetRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookie", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::ListRef<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::ListRef<RulesetRulesElActionParametersElCacheKeyElCustomKeyElHostElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(
        &self,
    ) -> terrars::ListRef<RulesetRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> terrars::ListRef<RulesetRulesElActionParametersElCacheKeyElCustomKeyElUserElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElCacheKeyElDynamic {
    custom_key: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElCacheKeyElCustomKeyEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElCacheKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_by_device_type: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_deception_armor: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_query_strings_order: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_key: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElCacheKeyElCustomKeyEl>>,
    dynamic: RulesetRulesElActionParametersElCacheKeyElDynamic,
}

impl RulesetRulesElActionParametersElCacheKeyEl {
    #[doc = "Set the field `cache_by_device_type`.\nCache by device type."]
    pub fn set_cache_by_device_type(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cache_by_device_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_deception_armor`.\nCache deception armor."]
    pub fn set_cache_deception_armor(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cache_deception_armor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_query_strings_order`.\nIgnore query strings order."]
    pub fn set_ignore_query_strings_order(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_query_strings_order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_key`.\n"]
    pub fn set_custom_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyElCustomKeyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_key = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElCacheKeyEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElCacheKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElCacheKeyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RulesetRulesElActionParametersElCacheKeyElRef {
        RulesetRulesElActionParametersElCacheKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElCacheKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_by_device_type` after provisioning.\nCache by device type."]
    pub fn cache_by_device_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_by_device_type", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_deception_armor` after provisioning.\nCache deception armor."]
    pub fn cache_deception_armor(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_deception_armor", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_query_strings_order` after provisioning.\nIgnore query strings order."]
    pub fn ignore_query_strings_order(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_query_strings_order", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_key` after provisioning.\n"]
    pub fn custom_key(&self) -> terrars::ListRef<RulesetRulesElActionParametersElCacheKeyElCustomKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_key", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    #[doc = "Set the field `from`.\nFrom status code."]
    pub fn set_from(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\nTo status code."]
    pub fn set_to(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
        RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\nFrom status code."]
    pub fn from(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\nTo status code."]
    pub fn to(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElDynamic {
    status_code_range: std
    ::option
    ::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code_range: std
    ::option
    ::Option<std::vec::Vec<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl>>,
    dynamic: RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElDynamic,
}

impl RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    #[doc = "Set the field `status_code`.\nStatus code for which the edge TTL is applied."]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nStatus code edge TTL value."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code_range`.\n"]
    pub fn set_status_code_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.status_code_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.status_code_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
        RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `status_code` after provisioning.\nStatus code for which the edge TTL is applied."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nStatus code edge TTL value."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code_range` after provisioning.\n"]
    pub fn status_code_range(
        &self,
    ) -> terrars::ListRef<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status_code_range", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElEdgeTtlElDynamic {
    status_code_ttl: std
    ::option
    ::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElEdgeTtlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<f64>>,
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code_ttl: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>>,
    dynamic: RulesetRulesElActionParametersElEdgeTtlElDynamic,
}

impl RulesetRulesElActionParametersElEdgeTtlEl {
    #[doc = "Set the field `default`.\nDefault edge TTL"]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code_ttl`.\n"]
    pub fn set_status_code_ttl(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.status_code_ttl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.status_code_ttl = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElEdgeTtlEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElEdgeTtlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesElActionParametersElEdgeTtlEl {
    #[doc = "Mode of the edge TTL."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildRulesetRulesElActionParametersElEdgeTtlEl {
    pub fn build(self) -> RulesetRulesElActionParametersElEdgeTtlEl {
        RulesetRulesElActionParametersElEdgeTtlEl {
            default: core::default::Default::default(),
            mode: self.mode,
            status_code_ttl: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElEdgeTtlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElEdgeTtlElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RulesetRulesElActionParametersElEdgeTtlElRef {
        RulesetRulesElActionParametersElEdgeTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElEdgeTtlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default` after provisioning.\nDefault edge TTL"]
    pub fn default(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\nMode of the edge TTL."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code_ttl` after provisioning.\n"]
    pub fn status_code_ttl(&self) -> terrars::ListRef<RulesetRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status_code_ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElFromListEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RulesetRulesElActionParametersElFromListEl {
    #[doc = "Set the field `key`.\nExpression to use for the list lookup."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the list."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElFromListEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElFromListEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElFromListElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RulesetRulesElActionParametersElFromListElRef {
        RulesetRulesElActionParametersElFromListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElFromListElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nExpression to use for the list lookup."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the list."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElFromValueElTargetUrlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RulesetRulesElActionParametersElFromValueElTargetUrlEl {
    #[doc =
        "Set the field `expression`.\nUse a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nStatic value to provide as the HTTP request header value."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElFromValueElTargetUrlEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElFromValueElTargetUrlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElFromValueElTargetUrlElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElFromValueElTargetUrlElRef {
        RulesetRulesElActionParametersElFromValueElTargetUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElFromValueElTargetUrlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nUse a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nStatic value to provide as the HTTP request header value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElFromValueElDynamic {
    target_url: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElFromValueElTargetUrlEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElFromValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_query_string: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_url: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElFromValueElTargetUrlEl>>,
    dynamic: RulesetRulesElActionParametersElFromValueElDynamic,
}

impl RulesetRulesElActionParametersElFromValueEl {
    #[doc = "Set the field `preserve_query_string`.\nPreserve query string for redirect URL."]
    pub fn set_preserve_query_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_query_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\nStatus code for redirect."]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_url`.\n"]
    pub fn set_target_url(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElFromValueElTargetUrlEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_url = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_url = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElFromValueEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElFromValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElFromValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElFromValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElFromValueElRef {
        RulesetRulesElActionParametersElFromValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElFromValueElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `preserve_query_string` after provisioning.\nPreserve query string for redirect URL."]
    pub fn preserve_query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\nStatus code for redirect."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc = "Get a reference to the value of field `target_url` after provisioning.\n"]
    pub fn target_url(&self) -> terrars::ListRef<RulesetRulesElActionParametersElFromValueElTargetUrlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_url", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElHeadersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RulesetRulesElActionParametersElHeadersEl {
    #[doc =
        "Set the field `expression`.\nUse a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the HTTP request header to target."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `operation`.\nAction to perform on the HTTP request header. Available values: `remove`, `set`, `add`."]
    pub fn set_operation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nStatic value to provide as the HTTP request header value."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElHeadersEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElHeadersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElHeadersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RulesetRulesElActionParametersElHeadersElRef {
        RulesetRulesElActionParametersElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElHeadersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nUse a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the HTTP request header to target."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nAction to perform on the HTTP request header. Available values: `remove`, `set`, `add`."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nStatic value to provide as the HTTP request header value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElMatchedDataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RulesetRulesElActionParametersElMatchedDataEl {
    #[doc =
        "Set the field `public_key`.\nPublic key to use within WAF Ruleset payload logging to view the HTTP request parameters. You can generate a public key [using the `matched-data-cli` command-line tool](https://developers.cloudflare.com/waf/managed-rulesets/payload-logging/command-line/generate-key-pair) or [in the Cloudflare dashboard](https://developers.cloudflare.com/waf/managed-rulesets/payload-logging/configure)."]
    pub fn set_public_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElMatchedDataEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElMatchedDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElMatchedDataElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElMatchedDataElRef {
        RulesetRulesElActionParametersElMatchedDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElMatchedDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `public_key` after provisioning.\nPublic key to use within WAF Ruleset payload logging to view the HTTP request parameters. You can generate a public key [using the `matched-data-cli` command-line tool](https://developers.cloudflare.com/waf/managed-rulesets/payload-logging/command-line/generate-key-pair) or [in the Cloudflare dashboard](https://developers.cloudflare.com/waf/managed-rulesets/payload-logging/configure)."]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElOriginEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl RulesetRulesElActionParametersElOriginEl {
    #[doc = "Set the field `host`.\nOrigin Hostname where request is sent."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nOrigin Port where request is sent."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElOriginEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElOriginEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElOriginElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RulesetRulesElActionParametersElOriginElRef {
        RulesetRulesElActionParametersElOriginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElOriginElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nOrigin Hostname where request is sent."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nOrigin Port where request is sent."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElOverridesElCategoriesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    category: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl RulesetRulesElActionParametersElOverridesElCategoriesEl {
    #[doc =
        "Set the field `action`.\nAction to perform in the tag-level override. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `category`.\nTag name to apply the ruleset rule override to."]
    pub fn set_category(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.category = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enabled`.\nDefines if the current tag-level override enables or disables the ruleset rules with the specified tag."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElOverridesElCategoriesEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElOverridesElCategoriesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElOverridesElCategoriesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElOverridesElCategoriesElRef {
        RulesetRulesElActionParametersElOverridesElCategoriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElOverridesElCategoriesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAction to perform in the tag-level override. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `category` after provisioning.\nTag name to apply the ruleset rule override to."]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nDefines if the current tag-level override enables or disables the ruleset rules with the specified tag."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElOverridesElRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RulesetRulesElActionParametersElOverridesElRulesEl {
    #[doc =
        "Set the field `action`.\nAction to perform in the rule-level override. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nDefines if the current rule-level override enables or disables the rule."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nRule ID to apply the override to."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `score_threshold`.\nAnomaly score threshold to apply in the ruleset rule override. Only applicable to modsecurity-based rulesets."]
    pub fn set_score_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.score_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_level`.\nSensitivity level for a ruleset rule override."]
    pub fn set_sensitivity_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sensitivity_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElOverridesElRulesEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElOverridesElRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElOverridesElRulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElOverridesElRulesElRef {
        RulesetRulesElActionParametersElOverridesElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElOverridesElRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAction to perform in the rule-level override. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nDefines if the current rule-level override enables or disables the rule."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nRule ID to apply the override to."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `score_threshold` after provisioning.\nAnomaly score threshold to apply in the ruleset rule override. Only applicable to modsecurity-based rulesets."]
    pub fn score_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sensitivity_level` after provisioning.\nSensitivity level for a ruleset rule override."]
    pub fn sensitivity_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElOverridesElDynamic {
    categories: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElOverridesElCategoriesEl>>,
    rules: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElOverridesElRulesEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElOverridesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    categories: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElOverridesElCategoriesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElOverridesElRulesEl>>,
    dynamic: RulesetRulesElActionParametersElOverridesElDynamic,
}

impl RulesetRulesElActionParametersElOverridesEl {
    #[doc =
        "Set the field `action`.\nAction to perform in the rule-level override. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nDefines if the current ruleset-level override enables or disables the ruleset."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sensitivity_level`.\nSensitivity level to override for all ruleset rules. Available values: `default`, `medium`, `low`, `eoff`."]
    pub fn set_sensitivity_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sensitivity_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `categories`.\n"]
    pub fn set_categories(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElOverridesElCategoriesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.categories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.categories = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElOverridesElRulesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElOverridesEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElOverridesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElOverridesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElOverridesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElOverridesElRef {
        RulesetRulesElActionParametersElOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElOverridesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAction to perform in the rule-level override. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nDefines if the current ruleset-level override enables or disables the ruleset."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sensitivity_level` after provisioning.\nSensitivity level to override for all ruleset rules. Available values: `default`, `medium`, `low`, `eoff`."]
    pub fn sensitivity_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }

    #[doc = "Get a reference to the value of field `categories` after provisioning.\n"]
    pub fn categories(&self) -> terrars::ListRef<RulesetRulesElActionParametersElOverridesElCategoriesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categories", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<RulesetRulesElActionParametersElOverridesElRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElResponseEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
}

impl RulesetRulesElActionParametersElResponseEl {
    #[doc = "Set the field `content`.\nBody content to include in the response."]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_type`.\nHTTP content type to send in the response."]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\nHTTP status code to send in the response."]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElResponseEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElResponseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElResponseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RulesetRulesElActionParametersElResponseElRef {
        RulesetRulesElActionParametersElResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElResponseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nBody content to include in the response."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nHTTP content type to send in the response."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status_code` after provisioning.\nHTTP status code to send in the response."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElServeStaleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_stale_while_updating: std::option::Option<terrars::PrimField<bool>>,
}

impl RulesetRulesElActionParametersElServeStaleEl {
    #[doc = "Set the field `disable_stale_while_updating`.\nDisable stale while updating."]
    pub fn set_disable_stale_while_updating(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_stale_while_updating = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElServeStaleEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElServeStaleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElServeStaleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElServeStaleElRef {
        RulesetRulesElActionParametersElServeStaleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElServeStaleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_stale_while_updating` after provisioning.\nDisable stale while updating."]
    pub fn disable_stale_while_updating(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_stale_while_updating", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElSniEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RulesetRulesElActionParametersElSniEl {
    #[doc = "Set the field `value`.\nValue to define for SNI."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElSniEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElSniEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElSniElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RulesetRulesElActionParametersElSniElRef {
        RulesetRulesElActionParametersElSniElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElSniElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue to define for SNI."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElUriElPathEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RulesetRulesElActionParametersElUriElPathEl {
    #[doc =
        "Set the field `expression`.\nExpression that defines the updated (dynamic) value of the URI path or query string component. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nStatic string value of the updated URI path or query string component."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElUriElPathEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElUriElPathEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElUriElPathElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElUriElPathElRef {
        RulesetRulesElActionParametersElUriElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElUriElPathElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nExpression that defines the updated (dynamic) value of the URI path or query string component. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nStatic string value of the updated URI path or query string component."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElUriElQueryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RulesetRulesElActionParametersElUriElQueryEl {
    #[doc =
        "Set the field `expression`.\nExpression that defines the updated (dynamic) value of the URI path or query string component. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nStatic string value of the updated URI path or query string component."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElUriElQueryEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElUriElQueryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElUriElQueryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RulesetRulesElActionParametersElUriElQueryElRef {
        RulesetRulesElActionParametersElUriElQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElUriElQueryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nExpression that defines the updated (dynamic) value of the URI path or query string component. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nStatic string value of the updated URI path or query string component."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElUriElDynamic {
    path: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElUriElPathEl>>,
    query: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElUriElQueryEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersElUriEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElUriElPathEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElUriElQueryEl>>,
    dynamic: RulesetRulesElActionParametersElUriElDynamic,
}

impl RulesetRulesElActionParametersElUriEl {
    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElUriElPathEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.path = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query`.\n"]
    pub fn set_query(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElUriElQueryEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersElUriEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersElUriEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElUriElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElUriElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RulesetRulesElActionParametersElUriElRef {
        RulesetRulesElActionParametersElUriElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElUriElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::ListRef<RulesetRulesElActionParametersElUriElPathElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> terrars::ListRef<RulesetRulesElActionParametersElUriElQueryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElActionParametersElDynamic {
    algorithms: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElAlgorithmsEl>>,
    autominify: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElAutominifyEl>>,
    browser_ttl: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElBrowserTtlEl>>,
    cache_key: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElCacheKeyEl>>,
    edge_ttl: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElEdgeTtlEl>>,
    from_list: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElFromListEl>>,
    from_value: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElFromValueEl>>,
    headers: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElHeadersEl>>,
    matched_data: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElMatchedDataEl>>,
    origin: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElOriginEl>>,
    overrides: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElOverridesEl>>,
    response: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElResponseEl>>,
    serve_stale: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElServeStaleEl>>,
    sni: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElSniEl>>,
    uri: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersElUriEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesElActionParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_https_rewrites: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bic: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_fields: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_apps: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_railgun: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_zaraz: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_obfuscation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotlink_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    increment: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mirage: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opportunistic_encryption: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_error_page_passthru: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phases: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    polish: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    products: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_fields: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    respect_strong_etags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_fields: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rocket_loader: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ruleset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rulesets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_excludes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sxg: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithms: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElAlgorithmsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autominify: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElAutominifyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browser_ttl: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElBrowserTtlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_key: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElCacheKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_ttl: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElEdgeTtlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_list: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElFromListEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_value: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElFromValueEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElHeadersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matched_data: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElMatchedDataEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElOriginEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overrides: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElOverridesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElResponseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serve_stale: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElServeStaleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sni: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElSniEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersElUriEl>>,
    dynamic: RulesetRulesElActionParametersElDynamic,
}

impl RulesetRulesElActionParametersEl {
    #[doc = "Set the field `automatic_https_rewrites`.\nTurn on or off Cloudflare Automatic HTTPS rewrites."]
    pub fn set_automatic_https_rewrites(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.automatic_https_rewrites = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bic`.\nInspect the visitor's browser for headers commonly associated with spammers and certain bots."]
    pub fn set_bic(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache`.\nWhether to cache if expression matches."]
    pub fn set_cache(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cache = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content`.\nContent of the custom error response."]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_type`.\nContent-Type of the custom error response."]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cookie_fields`.\nList of cookie values to include as part of custom fields logging."]
    pub fn set_cookie_fields(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cookie_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_apps`.\nTurn off all active Cloudflare Apps."]
    pub fn set_disable_apps(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_apps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_railgun`.\nTurn off railgun feature of the Cloudflare Speed app."]
    pub fn set_disable_railgun(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_railgun = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_zaraz`.\nTurn off zaraz feature."]
    pub fn set_disable_zaraz(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_zaraz = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `email_obfuscation`.\nTurn on or off the Cloudflare Email Obfuscation feature of the Cloudflare Scrape Shield app."]
    pub fn set_email_obfuscation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.email_obfuscation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_header`.\nHost Header that request origin receives."]
    pub fn set_host_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hotlink_protection`.\nTurn on or off the hotlink protection feature."]
    pub fn set_hotlink_protection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.hotlink_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nIdentifier of the action parameter to modify."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `increment`.\n"]
    pub fn set_increment(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.increment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mirage`.\nTurn on or off Cloudflare Mirage of the Cloudflare Speed app."]
    pub fn set_mirage(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mirage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `opportunistic_encryption`.\nTurn on or off the Cloudflare Opportunistic Encryption feature of the Edge Certificates tab in the Cloudflare SSL/TLS app."]
    pub fn set_opportunistic_encryption(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.opportunistic_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_error_page_passthru`.\nPass-through error page for origin."]
    pub fn set_origin_error_page_passthru(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.origin_error_page_passthru = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `phases`.\nPoint in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub fn set_phases(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.phases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `polish`.\nApply options from the Polish feature of the Cloudflare Speed app."]
    pub fn set_polish(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.polish = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `products`.\nProducts to target with the actions. Available values: `bic`, `hot`, `ratelimit`, `securityLevel`, `uablock`, `waf`, `zonelockdown`."]
    pub fn set_products(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.products = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_fields`.\nList of request headers to include as part of custom fields logging, in lowercase."]
    pub fn set_request_fields(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.request_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `respect_strong_etags`.\nRespect strong ETags."]
    pub fn set_respect_strong_etags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.respect_strong_etags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response_fields`.\nList of response headers to include as part of custom fields logging, in lowercase."]
    pub fn set_response_fields(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.response_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rocket_loader`.\nTurn on or off Cloudflare Rocket Loader in the Cloudflare Speed app."]
    pub fn set_rocket_loader(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.rocket_loader = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rules`.\nMap of managed WAF rule ID to comma-delimited string of ruleset rule IDs. Example: `rules = { \"efb7b8c949ac4650a09736fc376e9aee\" = \"5de7edfa648c4d6891dc3e7f84534ffa,e3a567afc347477d9702d9047e97d760\" }`."]
    pub fn set_rules(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.rules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ruleset`.\nWhich ruleset ID to target."]
    pub fn set_ruleset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ruleset = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rulesets`.\nList of managed WAF rule IDs to target. Only valid when the `\"action\"` is set to skip."]
    pub fn set_rulesets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.rulesets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_level`.\nControl options for the Security Level feature from the Security app."]
    pub fn set_security_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `server_side_excludes`.\nTurn on or off the Server Side Excludes feature of the Cloudflare Scrape Shield app."]
    pub fn set_server_side_excludes(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.server_side_excludes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl`.\nControl options for the SSL feature of the Edge Certificates tab in the Cloudflare SSL/TLS app."]
    pub fn set_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\nHTTP status code of the custom error response."]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sxg`.\nTurn on or off the SXG feature."]
    pub fn set_sxg(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sxg = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nVersion of the ruleset to deploy."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `algorithms`.\n"]
    pub fn set_algorithms(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElAlgorithmsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.algorithms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.algorithms = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `autominify`.\n"]
    pub fn set_autominify(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElAutominifyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autominify = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autominify = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `browser_ttl`.\n"]
    pub fn set_browser_ttl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElBrowserTtlEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.browser_ttl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.browser_ttl = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cache_key`.\n"]
    pub fn set_cache_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElCacheKeyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `edge_ttl`.\n"]
    pub fn set_edge_ttl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElEdgeTtlEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.edge_ttl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.edge_ttl = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `from_list`.\n"]
    pub fn set_from_list(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElFromListEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.from_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.from_list = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `from_value`.\n"]
    pub fn set_from_value(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElFromValueEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.from_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.from_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElHeadersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.headers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `matched_data`.\n"]
    pub fn set_matched_data(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElMatchedDataEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.matched_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.matched_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElOriginEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.origin = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.origin = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `overrides`.\n"]
    pub fn set_overrides(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElOverridesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.overrides = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElResponseEl>>,
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

    #[doc = "Set the field `serve_stale`.\n"]
    pub fn set_serve_stale(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElServeStaleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.serve_stale = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.serve_stale = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sni`.\n"]
    pub fn set_sni(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElSniEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sni = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sni = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersElUriEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.uri = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.uri = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElActionParametersEl {
    type O = terrars::BlockAssignable<RulesetRulesElActionParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElActionParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElActionParametersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RulesetRulesElActionParametersElRef {
        RulesetRulesElActionParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElActionParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `automatic_https_rewrites` after provisioning.\nTurn on or off Cloudflare Automatic HTTPS rewrites."]
    pub fn automatic_https_rewrites(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_https_rewrites", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bic` after provisioning.\nInspect the visitor's browser for headers commonly associated with spammers and certain bots."]
    pub fn bic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cache` after provisioning.\nWhether to cache if expression matches."]
    pub fn cache(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nContent of the custom error response."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nContent-Type of the custom error response."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cookie_fields` after provisioning.\nList of cookie values to include as part of custom fields logging."]
    pub fn cookie_fields(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cookie_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_apps` after provisioning.\nTurn off all active Cloudflare Apps."]
    pub fn disable_apps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_apps", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_railgun` after provisioning.\nTurn off railgun feature of the Cloudflare Speed app."]
    pub fn disable_railgun(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_railgun", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_zaraz` after provisioning.\nTurn off zaraz feature."]
    pub fn disable_zaraz(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_zaraz", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email_obfuscation` after provisioning.\nTurn on or off the Cloudflare Email Obfuscation feature of the Cloudflare Scrape Shield app."]
    pub fn email_obfuscation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_obfuscation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host_header` after provisioning.\nHost Header that request origin receives."]
    pub fn host_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hotlink_protection` after provisioning.\nTurn on or off the hotlink protection feature."]
    pub fn hotlink_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hotlink_protection", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIdentifier of the action parameter to modify."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `increment` after provisioning.\n"]
    pub fn increment(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.increment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mirage` after provisioning.\nTurn on or off Cloudflare Mirage of the Cloudflare Speed app."]
    pub fn mirage(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mirage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `opportunistic_encryption` after provisioning.\nTurn on or off the Cloudflare Opportunistic Encryption feature of the Edge Certificates tab in the Cloudflare SSL/TLS app."]
    pub fn opportunistic_encryption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_encryption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_error_page_passthru` after provisioning.\nPass-through error page for origin."]
    pub fn origin_error_page_passthru(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_error_page_passthru", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phases` after provisioning.\nPoint in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub fn phases(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.phases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `polish` after provisioning.\nApply options from the Polish feature of the Cloudflare Speed app."]
    pub fn polish(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.polish", self.base))
    }

    #[doc =
        "Get a reference to the value of field `products` after provisioning.\nProducts to target with the actions. Available values: `bic`, `hot`, `ratelimit`, `securityLevel`, `uablock`, `waf`, `zonelockdown`."]
    pub fn products(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.products", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_fields` after provisioning.\nList of request headers to include as part of custom fields logging, in lowercase."]
    pub fn request_fields(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.request_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `respect_strong_etags` after provisioning.\nRespect strong ETags."]
    pub fn respect_strong_etags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.respect_strong_etags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_fields` after provisioning.\nList of response headers to include as part of custom fields logging, in lowercase."]
    pub fn response_fields(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.response_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rocket_loader` after provisioning.\nTurn on or off Cloudflare Rocket Loader in the Cloudflare Speed app."]
    pub fn rocket_loader(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rocket_loader", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nMap of managed WAF rule ID to comma-delimited string of ruleset rule IDs. Example: `rules = { \"efb7b8c949ac4650a09736fc376e9aee\" = \"5de7edfa648c4d6891dc3e7f84534ffa,e3a567afc347477d9702d9047e97d760\" }`."]
    pub fn rules(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc = "Get a reference to the value of field `ruleset` after provisioning.\nWhich ruleset ID to target."]
    pub fn ruleset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ruleset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rulesets` after provisioning.\nList of managed WAF rule IDs to target. Only valid when the `\"action\"` is set to skip."]
    pub fn rulesets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.rulesets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_level` after provisioning.\nControl options for the Security Level feature from the Security app."]
    pub fn security_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_side_excludes` after provisioning.\nTurn on or off the Server Side Excludes feature of the Cloudflare Scrape Shield app."]
    pub fn server_side_excludes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_side_excludes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl` after provisioning.\nControl options for the SSL feature of the Edge Certificates tab in the Cloudflare SSL/TLS app."]
    pub fn ssl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status_code` after provisioning.\nHTTP status code of the custom error response."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc = "Get a reference to the value of field `sxg` after provisioning.\nTurn on or off the SXG feature."]
    pub fn sxg(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sxg", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the ruleset to deploy."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `algorithms` after provisioning.\n"]
    pub fn algorithms(&self) -> terrars::ListRef<RulesetRulesElActionParametersElAlgorithmsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.algorithms", self.base))
    }

    #[doc = "Get a reference to the value of field `autominify` after provisioning.\n"]
    pub fn autominify(&self) -> terrars::ListRef<RulesetRulesElActionParametersElAutominifyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autominify", self.base))
    }

    #[doc = "Get a reference to the value of field `browser_ttl` after provisioning.\n"]
    pub fn browser_ttl(&self) -> terrars::ListRef<RulesetRulesElActionParametersElBrowserTtlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.browser_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_key` after provisioning.\n"]
    pub fn cache_key(&self) -> terrars::ListRef<RulesetRulesElActionParametersElCacheKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_key", self.base))
    }

    #[doc = "Get a reference to the value of field `edge_ttl` after provisioning.\n"]
    pub fn edge_ttl(&self) -> terrars::ListRef<RulesetRulesElActionParametersElEdgeTtlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edge_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `from_list` after provisioning.\n"]
    pub fn from_list(&self) -> terrars::ListRef<RulesetRulesElActionParametersElFromListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.from_list", self.base))
    }

    #[doc = "Get a reference to the value of field `from_value` after provisioning.\n"]
    pub fn from_value(&self) -> terrars::ListRef<RulesetRulesElActionParametersElFromValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.from_value", self.base))
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<RulesetRulesElActionParametersElHeadersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc = "Get a reference to the value of field `matched_data` after provisioning.\n"]
    pub fn matched_data(&self) -> terrars::ListRef<RulesetRulesElActionParametersElMatchedDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matched_data", self.base))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> terrars::ListRef<RulesetRulesElActionParametersElOriginElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc = "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(&self) -> terrars::ListRef<RulesetRulesElActionParametersElOverridesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::ListRef<RulesetRulesElActionParametersElResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }

    #[doc = "Get a reference to the value of field `serve_stale` after provisioning.\n"]
    pub fn serve_stale(&self) -> terrars::ListRef<RulesetRulesElActionParametersElServeStaleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serve_stale", self.base))
    }

    #[doc = "Get a reference to the value of field `sni` after provisioning.\n"]
    pub fn sni(&self) -> terrars::ListRef<RulesetRulesElActionParametersElSniElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sni", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::ListRef<RulesetRulesElActionParametersElUriElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElExposedCredentialCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RulesetRulesElExposedCredentialCheckEl {
    #[doc =
        "Set the field `password_expression`.\nFirewall Rules expression language based on Wireshark display filters for where to check for the \"password\" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language)."]
    pub fn set_password_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `username_expression`.\nFirewall Rules expression language based on Wireshark display filters for where to check for the \"username\" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language)."]
    pub fn set_username_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.username_expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElExposedCredentialCheckEl {
    type O = terrars::BlockAssignable<RulesetRulesElExposedCredentialCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElExposedCredentialCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RulesetRulesElExposedCredentialCheckElRef {
        RulesetRulesElExposedCredentialCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElExposedCredentialCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `password_expression` after provisioning.\nFirewall Rules expression language based on Wireshark display filters for where to check for the \"password\" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language)."]
    pub fn password_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username_expression` after provisioning.\nFirewall Rules expression language based on Wireshark display filters for where to check for the \"username\" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language)."]
    pub fn username_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElLoggingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl RulesetRulesElLoggingEl {
    #[doc = "Set the field `enabled`.\nOverride the default logging behavior when a rule is matched."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElLoggingEl {
    type O = terrars::BlockAssignable<RulesetRulesElLoggingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElLoggingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RulesetRulesElLoggingElRef {
        RulesetRulesElLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElLoggingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nOverride the default logging behavior when a rule is matched."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct RulesetRulesElRatelimitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    characteristics: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    counting_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mitigation_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests_per_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests_to_origin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score_per_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score_response_header_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RulesetRulesElRatelimitEl {
    #[doc =
        "Set the field `characteristics`.\nList of parameters that define how Cloudflare tracks the request rate for this rule."]
    pub fn set_characteristics(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.characteristics = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `counting_expression`.\nCriteria for counting HTTP requests to trigger the Rate Limiting action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn set_counting_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.counting_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mitigation_timeout`.\nOnce the request rate is reached, the Rate Limiting rule blocks further requests for the period of time defined in this field."]
    pub fn set_mitigation_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mitigation_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `period`.\nThe period of time to consider (in seconds) when evaluating the request rate."]
    pub fn set_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `requests_per_period`.\nThe number of requests over the period of time that will trigger the Rate Limiting rule."]
    pub fn set_requests_per_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.requests_per_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `requests_to_origin`.\nWhether to include requests to origin within the Rate Limiting count."]
    pub fn set_requests_to_origin(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.requests_to_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `score_per_period`.\nThe maximum aggregate score over the period of time that will trigger Rate Limiting rule."]
    pub fn set_score_per_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.score_per_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `score_response_header_name`.\nName of HTTP header in the response, set by the origin server, with the score for the current request."]
    pub fn set_score_response_header_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.score_response_header_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RulesetRulesElRatelimitEl {
    type O = terrars::BlockAssignable<RulesetRulesElRatelimitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElRatelimitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RulesetRulesElRatelimitElRef {
        RulesetRulesElRatelimitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElRatelimitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `characteristics` after provisioning.\nList of parameters that define how Cloudflare tracks the request rate for this rule."]
    pub fn characteristics(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.characteristics", self.base))
    }

    #[doc =
        "Get a reference to the value of field `counting_expression` after provisioning.\nCriteria for counting HTTP requests to trigger the Rate Limiting action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn counting_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.counting_expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mitigation_timeout` after provisioning.\nOnce the request rate is reached, the Rate Limiting rule blocks further requests for the period of time defined in this field."]
    pub fn mitigation_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mitigation_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period` after provisioning.\nThe period of time to consider (in seconds) when evaluating the request rate."]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requests_per_period` after provisioning.\nThe number of requests over the period of time that will trigger the Rate Limiting rule."]
    pub fn requests_per_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requests_per_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requests_to_origin` after provisioning.\nWhether to include requests to origin within the Rate Limiting count."]
    pub fn requests_to_origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requests_to_origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `score_per_period` after provisioning.\nThe maximum aggregate score over the period of time that will trigger Rate Limiting rule."]
    pub fn score_per_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_per_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `score_response_header_name` after provisioning.\nName of HTTP header in the response, set by the origin server, with the score for the current request."]
    pub fn score_response_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_response_header_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetRulesElDynamic {
    action_parameters: std::option::Option<terrars::DynamicBlock<RulesetRulesElActionParametersEl>>,
    exposed_credential_check: std::option::Option<terrars::DynamicBlock<RulesetRulesElExposedCredentialCheckEl>>,
    logging: std::option::Option<terrars::DynamicBlock<RulesetRulesElLoggingEl>>,
    ratelimit: std::option::Option<terrars::DynamicBlock<RulesetRulesElRatelimitEl>>,
}

#[derive(Serialize)]
pub struct RulesetRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_updated: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "ref", skip_serializing_if = "std::option::Option::is_none")]
    ref_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_parameters: std::option::Option<std::vec::Vec<RulesetRulesElActionParametersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exposed_credential_check: std::option::Option<std::vec::Vec<RulesetRulesElExposedCredentialCheckEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<std::vec::Vec<RulesetRulesElLoggingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ratelimit: std::option::Option<std::vec::Vec<RulesetRulesElRatelimitEl>>,
    dynamic: RulesetRulesElDynamic,
}

impl RulesetRulesEl {
    #[doc =
        "Set the field `action`.\nAction to perform in the ruleset rule. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nBrief summary of the ruleset rule and its intended use."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether the rule is active."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nUnique rule identifier."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_updated`.\nThe most recent update to this rule."]
    pub fn set_last_updated(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_updated = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ref_`.\nRule reference."]
    pub fn set_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ref_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nVersion of the ruleset to deploy."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action_parameters`.\n"]
    pub fn set_action_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElActionParametersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exposed_credential_check`.\n"]
    pub fn set_exposed_credential_check(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElExposedCredentialCheckEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exposed_credential_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exposed_credential_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(mut self, v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElLoggingEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ratelimit`.\n"]
    pub fn set_ratelimit(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RulesetRulesElRatelimitEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ratelimit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ratelimit = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RulesetRulesEl {
    type O = terrars::BlockAssignable<RulesetRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRulesetRulesEl {
    #[doc =
        "Criteria for an HTTP request to trigger the ruleset rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub expression: terrars::PrimField<std::string::String>,
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct RulesetRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RulesetRulesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RulesetRulesElRef {
        RulesetRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RulesetRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAction to perform in the ruleset rule. Available values: `allow`, `block`, `challenge`, `ddos_dynamic`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `set_cache_settings`, `set_config`, `serve_error`, `skip`, `compress_response`."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nBrief summary of the ruleset rule and its intended use."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the rule is active."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nCriteria for an HTTP request to trigger the ruleset rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique rule identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `last_updated` after provisioning.\nThe most recent update to this rule."]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc = "Get a reference to the value of field `ref_` after provisioning.\nRule reference."]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the ruleset to deploy."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `action_parameters` after provisioning.\n"]
    pub fn action_parameters(&self) -> terrars::ListRef<RulesetRulesElActionParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `exposed_credential_check` after provisioning.\n"]
    pub fn exposed_credential_check(&self) -> terrars::ListRef<RulesetRulesElExposedCredentialCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exposed_credential_check", self.base))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<RulesetRulesElLoggingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc = "Get a reference to the value of field `ratelimit` after provisioning.\n"]
    pub fn ratelimit(&self) -> terrars::ListRef<RulesetRulesElRatelimitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ratelimit", self.base))
    }
}

#[derive(Serialize, Default)]
struct RulesetDynamic {
    rules: std::option::Option<terrars::DynamicBlock<RulesetRulesEl>>,
}

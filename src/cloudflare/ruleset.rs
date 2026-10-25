#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct RulesetData_ {
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
    rules: std::option::Option<std::vec::Vec<Ruleset_Rules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct Ruleset_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RulesetData_>,
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

    #[doc = "Set the field `account_id`.\nThe unique ID of the account."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn informative description of the ruleset."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\nThe list of rules in the ruleset."]
    pub fn set_rules(self, v: impl std::convert::Into<std::vec::Vec<Ruleset_Rules>>) -> Self {
        self.0.data.borrow_mut().rules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nThe unique ID of the zone."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe unique ID of the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative description of the ruleset."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the ruleset."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe kind of the ruleset.\nAvailable values: \"managed\", \"custom\", \"root\", \"zone\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_updated` after provisioning.\nThe timestamp of when the ruleset was last modified."]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe human-readable name of the ruleset."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `phase` after provisioning.\nThe phase of the ruleset.\nAvailable values: \"ddos_l4\", \"ddos_l7\", \"http_config_settings\", \"http_custom_errors\", \"http_log_custom_fields\", \"http_ratelimit\", \"http_request_cache_settings\", \"http_request_dynamic_redirect\", \"http_request_firewall_custom\", \"http_request_firewall_managed\", \"http_request_late_transform\", \"http_request_origin\", \"http_request_redirect\", \"http_request_sanitize\", \"http_request_sbfm\", \"http_request_transform\", \"http_response_cache_settings\", \"http_response_compression\", \"http_response_firewall_managed\", \"http_response_headers_transform\", \"magic_transit\", \"magic_transit_ids_managed\", \"magic_transit_managed\", \"magic_transit_ratelimit\"."]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nThe list of rules in the ruleset."]
    pub fn rules(&self) -> terrars::ListRef<Ruleset_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe version of the ruleset."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nThe unique ID of the zone."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
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
    #[doc = "The kind of the ruleset.\nAvailable values: \"managed\", \"custom\", \"root\", \"zone\"."]
    pub kind: terrars::PrimField<std::string::String>,
    #[doc = "The human-readable name of the ruleset."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The phase of the ruleset.\nAvailable values: \"ddos_l4\", \"ddos_l7\", \"http_config_settings\", \"http_custom_errors\", \"http_log_custom_fields\", \"http_ratelimit\", \"http_request_cache_settings\", \"http_request_dynamic_redirect\", \"http_request_firewall_custom\", \"http_request_firewall_managed\", \"http_request_late_transform\", \"http_request_origin\", \"http_request_redirect\", \"http_request_sanitize\", \"http_request_sbfm\", \"http_request_transform\", \"http_response_cache_settings\", \"http_response_compression\", \"http_response_firewall_managed\", \"http_response_headers_transform\", \"magic_transit\", \"magic_transit_ids_managed\", \"magic_transit_managed\", \"magic_transit_ratelimit\"."]
    pub phase: terrars::PrimField<std::string::String>,
}

impl BuildRuleset {
    pub fn build(self, stack: &mut terrars::Stack) -> Ruleset {
        let out = Ruleset(std::rc::Rc::new(Ruleset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RulesetData_ {
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
                rules: core::default::Default::default(),
                zone_id: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe unique ID of the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative description of the ruleset."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the ruleset."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe kind of the ruleset.\nAvailable values: \"managed\", \"custom\", \"root\", \"zone\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_updated` after provisioning.\nThe timestamp of when the ruleset was last modified."]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe human-readable name of the ruleset."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `phase` after provisioning.\nThe phase of the ruleset.\nAvailable values: \"ddos_l4\", \"ddos_l7\", \"http_config_settings\", \"http_custom_errors\", \"http_log_custom_fields\", \"http_ratelimit\", \"http_request_cache_settings\", \"http_request_dynamic_redirect\", \"http_request_firewall_custom\", \"http_request_firewall_managed\", \"http_request_late_transform\", \"http_request_origin\", \"http_request_redirect\", \"http_request_sanitize\", \"http_request_sbfm\", \"http_request_transform\", \"http_response_cache_settings\", \"http_response_compression\", \"http_response_firewall_managed\", \"http_response_headers_transform\", \"magic_transit\", \"magic_transit_ids_managed\", \"magic_transit_managed\", \"magic_transit_ratelimit\"."]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nThe list of rules in the ruleset."]
    pub fn rules(&self) -> terrars::ListRef<Ruleset_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe version of the ruleset."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nThe unique ID of the zone."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Algorithms {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ruleset_Rules_ActionParameters_Algorithms {
    #[doc =
        "Set the field `name`.\nName of the compression algorithm to enable.\nAvailable values: \"none\", \"auto\", \"default\", \"gzip\", \"brotli\", \"zstd\"."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Algorithms {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Algorithms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Algorithms {}

impl BuildRuleset_Rules_ActionParameters_Algorithms {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Algorithms {
        Ruleset_Rules_ActionParameters_Algorithms { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_AlgorithmsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_AlgorithmsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_AlgorithmsRef {
        Ruleset_Rules_ActionParameters_AlgorithmsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_AlgorithmsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the compression algorithm to enable.\nAvailable values: \"none\", \"auto\", \"default\", \"gzip\", \"brotli\", \"zstd\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Autominify {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    css: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    html: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    js: std::option::Option<terrars::PrimField<bool>>,
}

impl Ruleset_Rules_ActionParameters_Autominify {
    #[doc = "Set the field `css`.\nWhether to minify CSS files."]
    pub fn set_css(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.css = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `html`.\nWhether to minify HTML files."]
    pub fn set_html(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.html = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `js`.\nWhether to minify JavaScript files."]
    pub fn set_js(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.js = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Autominify {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Autominify>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Autominify {}

impl BuildRuleset_Rules_ActionParameters_Autominify {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Autominify {
        Ruleset_Rules_ActionParameters_Autominify {
            css: core::default::Default::default(),
            html: core::default::Default::default(),
            js: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_AutominifyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_AutominifyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_AutominifyRef {
        Ruleset_Rules_ActionParameters_AutominifyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_AutominifyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `css` after provisioning.\nWhether to minify CSS files."]
    pub fn css(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.css", self.base))
    }

    #[doc = "Get a reference to the value of field `html` after provisioning.\nWhether to minify HTML files."]
    pub fn html(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }

    #[doc = "Get a reference to the value of field `js` after provisioning.\nWhether to minify JavaScript files."]
    pub fn js(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.js", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_BrowserTtl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<f64>>,
    mode: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_BrowserTtl {
    #[doc = "Set the field `default`.\nThe browser TTL (in seconds) if you choose the \"override_origin\" mode."]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_BrowserTtl {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_BrowserTtl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_BrowserTtl {
    #[doc =
        "The browser TTL mode.\nAvailable values: \"respect_origin\", \"bypass_by_default\", \"override_origin\", \"bypass\"."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_BrowserTtl {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_BrowserTtl {
        Ruleset_Rules_ActionParameters_BrowserTtl {
            default: core::default::Default::default(),
            mode: self.mode,
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_BrowserTtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_BrowserTtlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_BrowserTtlRef {
        Ruleset_Rules_ActionParameters_BrowserTtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_BrowserTtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nThe browser TTL (in seconds) if you choose the \"override_origin\" mode."]
    pub fn default(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe browser TTL mode.\nAvailable values: \"respect_origin\", \"bypass_by_default\", \"override_origin\", \"bypass\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_presence: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie {
    #[doc =
        "Set the field `check_presence`.\nA list of cookies to check for the presence of. The presence of these cookies is included in the cache key."]
    pub fn set_check_presence(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.check_presence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include`.\nA list of cookies to include in the cache key."]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie {}

impl BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie {
            check_presence: core::default::Default::default(),
            include: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `check_presence` after provisioning.\nA list of cookies to check for the presence of. The presence of these cookies is included in the cache key."]
    pub fn check_presence(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nA list of cookies to include in the cache key."]
    pub fn include(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Header {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_presence: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_origin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Header {
    #[doc =
        "Set the field `check_presence`.\nA list of headers to check for the presence of. The presence of these headers is included in the cache key."]
    pub fn set_check_presence(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.check_presence = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `contains`.\nA mapping of header names to a list of values. If a header is present in the request and contains any of the values provided, its value is included in the cache key."]
    pub fn set_contains(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_origin`.\nWhether to exclude the origin header in the cache key."]
    pub fn set_exclude_origin(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include`.\nA list of headers to include in the cache key."]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Header {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_Header {}

impl BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_Header {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Header {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Header {
            check_presence: core::default::Default::default(),
            contains: core::default::Default::default(),
            exclude_origin: core::default::Default::default(),
            include: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `check_presence` after provisioning.\nA list of headers to check for the presence of. The presence of these headers is included in the cache key."]
    pub fn check_presence(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nA mapping of header names to a list of values. If a header is present in the request and contains any of the values provided, its value is included in the cache key."]
    pub fn contains(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_origin` after provisioning.\nWhether to exclude the origin header in the cache key."]
    pub fn exclude_origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nA list of headers to include in the cache key."]
    pub fn include(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Host {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolved: std::option::Option<terrars::PrimField<bool>>,
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Host {
    #[doc = "Set the field `resolved`.\nWhether to use the resolved host in the cache key."]
    pub fn set_resolved(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.resolved = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Host {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Host>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_Host {}

impl BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_Host {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Host {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Host { resolved: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resolved` after provisioning.\nWhether to use the resolved host in the cache key."]
    pub fn resolved(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolved", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude {
    #[doc = "Set the field `all`.\nWhether to exclude all query string parameters from the cache key."]
    pub fn set_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `list`.\nA list of query string parameters to exclude from the cache key."]
    pub fn set_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.list = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude {}

impl BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude {
            all: core::default::Default::default(),
            list: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `all` after provisioning.\nWhether to exclude all query string parameters from the cache key."]
    pub fn all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `list` after provisioning.\nA list of query string parameters to exclude from the cache key."]
    pub fn list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.list", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include {
    #[doc = "Set the field `all`.\nWhether to include all query string parameters in the cache key."]
    pub fn set_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `list`.\nA list of query string parameters to include in the cache key."]
    pub fn set_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.list = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include {}

impl BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include {
            all: core::default::Default::default(),
            list: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `all` after provisioning.\nWhether to include all query string parameters in the cache key."]
    pub fn all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `list` after provisioning.\nA list of query string parameters to include in the cache key."]
    pub fn list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.list", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude: std::option::Option<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include>,
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString {
    #[doc = "Set the field `exclude`.\nWhich query string parameters to exclude from the cache key."]
    pub fn set_exclude(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude>,
    ) -> Self {
        self.exclude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include`.\nWhich query string parameters to include in the cache key."]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString {}

impl BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString {
            exclude: core::default::Default::default(),
            include: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nWhich query string parameters to exclude from the cache key."]
    pub fn exclude(&self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef::new(
            self.shared().clone(),
            format!("{}.exclude", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nWhich query string parameters to include in the cache key."]
    pub fn include(&self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef::new(
            self.shared().clone(),
            format!("{}.include", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_User {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lang: std::option::Option<terrars::PrimField<bool>>,
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_User {
    #[doc = "Set the field `device_type`.\nWhether to use the user agent's device type in the cache key."]
    pub fn set_device_type(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.device_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo`.\nWhether to use the user agents's country in the cache key."]
    pub fn set_geo(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.geo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lang`.\nWhether to use the user agent's language in the cache key."]
    pub fn set_lang(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.lang = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_User {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_User>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_User {}

impl BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey_User {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_User {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_User {
            device_type: core::default::Default::default(),
            geo: core::default::Default::default(),
            lang: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `device_type` after provisioning.\nWhether to use the user agent's device type in the cache key."]
    pub fn device_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `geo` after provisioning.\nWhether to use the user agents's country in the cache key."]
    pub fn geo(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lang` after provisioning.\nWhether to use the user agent's language in the cache key."]
    pub fn lang(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lang", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie: std::option::Option<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Header>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Host>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std::option::Option<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_User>,
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKey {
    #[doc = "Set the field `cookie`.\nWhich cookies to include in the cache key."]
    pub fn set_cookie(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie>,
    ) -> Self {
        self.cookie = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header`.\nWhich headers to include in the cache key."]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Header>,
    ) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\nHow to use the host in the cache key."]
    pub fn set_host(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_Host>,
    ) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_string`.\nWhich query string parameters to include in or exclude from the cache key."]
    pub fn set_query_string(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString>,
    ) -> Self {
        self.query_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\nHow to use characteristics of the request user agent in the cache key."]
    pub fn set_user(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_CacheKey_CustomKey_User>,
    ) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_CacheKey_CustomKey {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_CacheKey_CustomKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey {}

impl BuildRuleset_Rules_ActionParameters_CacheKey_CustomKey {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey {
            cookie: core::default::Default::default(),
            header: core::default::Default::default(),
            host: core::default::Default::default(),
            query_string: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_CacheKey_CustomKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_CacheKey_CustomKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKeyRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_CacheKey_CustomKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cookie` after provisioning.\nWhich cookies to include in the cache key."]
    pub fn cookie(&self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef::new(
            self.shared().clone(),
            format!("{}.cookie", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `header` after provisioning.\nWhich headers to include in the cache key."]
    pub fn header(&self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef::new(
            self.shared().clone(),
            format!("{}.header", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nHow to use the host in the cache key."]
    pub fn host(&self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef::new(
            self.shared().clone(),
            format!("{}.host", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `query_string` after provisioning.\nWhich query string parameters to include in or exclude from the cache key."]
    pub fn query_string(&self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef::new(
            self.shared().clone(),
            format!("{}.query_string", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nHow to use characteristics of the request user agent in the cache key."]
    pub fn user(&self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef::new(
            self.shared().clone(),
            format!("{}.user", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_CacheKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_by_device_type: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_deception_armor: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_key: std::option::Option<Ruleset_Rules_ActionParameters_CacheKey_CustomKey>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_query_strings_order: std::option::Option<terrars::PrimField<bool>>,
}

impl Ruleset_Rules_ActionParameters_CacheKey {
    #[doc =
        "Set the field `cache_by_device_type`.\nWhether to separate cached content based on the visitor's device type."]
    pub fn set_cache_by_device_type(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cache_by_device_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cache_deception_armor`.\nWhether to protect from web cache deception attacks, while allowing static assets to be cached."]
    pub fn set_cache_deception_armor(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cache_deception_armor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_key`.\nWhich components of the request are included or excluded from the cache key."]
    pub fn set_custom_key(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_CacheKey_CustomKey>,
    ) -> Self {
        self.custom_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_query_strings_order`.\nWhether to treat requests with the same query parameters the same, regardless of the order those query parameters are in."]
    pub fn set_ignore_query_strings_order(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_query_strings_order = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_CacheKey {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_CacheKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_CacheKey {}

impl BuildRuleset_Rules_ActionParameters_CacheKey {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_CacheKey {
        Ruleset_Rules_ActionParameters_CacheKey {
            cache_by_device_type: core::default::Default::default(),
            cache_deception_armor: core::default::Default::default(),
            custom_key: core::default::Default::default(),
            ignore_query_strings_order: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_CacheKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_CacheKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_CacheKeyRef {
        Ruleset_Rules_ActionParameters_CacheKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_CacheKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache_by_device_type` after provisioning.\nWhether to separate cached content based on the visitor's device type."]
    pub fn cache_by_device_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_by_device_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cache_deception_armor` after provisioning.\nWhether to protect from web cache deception attacks, while allowing static assets to be cached."]
    pub fn cache_deception_armor(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_deception_armor", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_key` after provisioning.\nWhich components of the request are included or excluded from the cache key."]
    pub fn custom_key(&self) -> Ruleset_Rules_ActionParameters_CacheKey_CustomKeyRef {
        Ruleset_Rules_ActionParameters_CacheKey_CustomKeyRef::new(
            self.shared().clone(),
            format!("{}.custom_key", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `ignore_query_strings_order` after provisioning.\nWhether to treat requests with the same query parameters the same, regardless of the order those query parameters are in."]
    pub fn ignore_query_strings_order(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_query_strings_order", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_CacheReserve {
    eligible: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_file_size: std::option::Option<terrars::PrimField<f64>>,
}

impl Ruleset_Rules_ActionParameters_CacheReserve {
    #[doc = "Set the field `minimum_file_size`.\nThe minimum file size eligible for storage in Cache Reserve."]
    pub fn set_minimum_file_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_file_size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_CacheReserve {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_CacheReserve>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_CacheReserve {
    #[doc =
        "Whether Cache Reserve is enabled. If this is true and a request meets eligibility criteria, Cloudflare will write the resource to Cache Reserve."]
    pub eligible: terrars::PrimField<bool>,
}

impl BuildRuleset_Rules_ActionParameters_CacheReserve {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_CacheReserve {
        Ruleset_Rules_ActionParameters_CacheReserve {
            eligible: self.eligible,
            minimum_file_size: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_CacheReserveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_CacheReserveRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_CacheReserveRef {
        Ruleset_Rules_ActionParameters_CacheReserveRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_CacheReserveRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `eligible` after provisioning.\nWhether Cache Reserve is enabled. If this is true and a request meets eligibility criteria, Cloudflare will write the resource to Cache Reserve."]
    pub fn eligible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eligible", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum_file_size` after provisioning.\nThe minimum file size eligible for storage in Cache Reserve."]
    pub fn minimum_file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_file_size", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_CookieFields {
    name: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_CookieFields { }

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_CookieFields {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_CookieFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_CookieFields {
    #[doc = "The name of the cookie."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_CookieFields {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_CookieFields {
        Ruleset_Rules_ActionParameters_CookieFields { name: self.name }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_CookieFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_CookieFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_CookieFieldsRef {
        Ruleset_Rules_ActionParameters_CookieFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_CookieFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cookie."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange {
    #[doc = "Set the field `from`.\nThe lower bound of the range."]
    pub fn set_from(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\nThe upper bound of the range."]
    pub fn set_to(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange {}

impl BuildRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange {
        Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef {
        Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\nThe lower bound of the range."]
    pub fn from(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\nThe upper bound of the range."]
    pub fn to(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code_range: std::option::Option<Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange>,
    value: terrars::PrimField<f64>,
}

impl Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl {
    #[doc = "Set the field `status_code`.\nA single status code to apply the TTL to."]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code_range`.\nA range of status codes to apply the TTL to."]
    pub fn set_status_code_range(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange>,
    ) -> Self {
        self.status_code_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl {
    #[doc =
        "The time to cache the response for (in seconds). A value of 0 is equivalent to setting the cache control header with the value \"no-cache\". A value of -1 is equivalent to setting the cache control header with the value of \"no-store\"."]
    pub value: terrars::PrimField<f64>,
}

impl BuildRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl {
        Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl {
            status_code: core::default::Default::default(),
            status_code_range: core::default::Default::default(),
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtlRef {
        Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `status_code` after provisioning.\nA single status code to apply the TTL to."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status_code_range` after provisioning.\nA range of status codes to apply the TTL to."]
    pub fn status_code_range(&self) -> Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef {
        Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef::new(
            self.shared().clone(),
            format!("{}.status_code_range", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe time to cache the response for (in seconds). A value of 0 is equivalent to setting the cache control header with the value \"no-cache\". A value of -1 is equivalent to setting the cache control header with the value of \"no-store\"."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_EdgeTtl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<f64>>,
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code_ttl: std::option::Option<std::vec::Vec<Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl>>,
}

impl Ruleset_Rules_ActionParameters_EdgeTtl {
    #[doc = "Set the field `default`.\nThe edge TTL (in seconds) if you choose the \"override_origin\" mode."]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code_ttl`.\nA list of TTLs to apply to specific status codes or status code ranges."]
    pub fn set_status_code_ttl(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl>>,
    ) -> Self {
        self.status_code_ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_EdgeTtl {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_EdgeTtl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_EdgeTtl {
    #[doc = "The edge TTL mode.\nAvailable values: \"respect_origin\", \"bypass_by_default\", \"override_origin\"."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_EdgeTtl {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_EdgeTtl {
        Ruleset_Rules_ActionParameters_EdgeTtl {
            default: core::default::Default::default(),
            mode: self.mode,
            status_code_ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_EdgeTtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_EdgeTtlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_EdgeTtlRef {
        Ruleset_Rules_ActionParameters_EdgeTtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_EdgeTtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nThe edge TTL (in seconds) if you choose the \"override_origin\" mode."]
    pub fn default(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe edge TTL mode.\nAvailable values: \"respect_origin\", \"bypass_by_default\", \"override_origin\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status_code_ttl` after provisioning.\nA list of TTLs to apply to specific status codes or status code ranges."]
    pub fn status_code_ttl(&self) -> terrars::ListRef<Ruleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status_code_ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_FromList {
    key: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_FromList { }

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_FromList {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_FromList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_FromList {
    #[doc = "An expression that evaluates to the list lookup key."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "The name of the list to match against."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_FromList {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_FromList {
        Ruleset_Rules_ActionParameters_FromList {
            key: self.key,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_FromListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_FromListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_FromListRef {
        Ruleset_Rules_ActionParameters_FromListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_FromListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nAn expression that evaluates to the list lookup key."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the list to match against."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_FromValue_TargetUrl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ruleset_Rules_ActionParameters_FromValue_TargetUrl {
    #[doc = "Set the field `expression`.\nAn expression that evaluates to a URL to redirect the request to."]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nA URL to redirect the request to."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_FromValue_TargetUrl {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_FromValue_TargetUrl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_FromValue_TargetUrl {}

impl BuildRuleset_Rules_ActionParameters_FromValue_TargetUrl {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_FromValue_TargetUrl {
        Ruleset_Rules_ActionParameters_FromValue_TargetUrl {
            expression: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_FromValue_TargetUrlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_FromValue_TargetUrlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_FromValue_TargetUrlRef {
        Ruleset_Rules_ActionParameters_FromValue_TargetUrlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_FromValue_TargetUrlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nAn expression that evaluates to a URL to redirect the request to."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nA URL to redirect the request to."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_FromValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_query_string: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
    target_url: Ruleset_Rules_ActionParameters_FromValue_TargetUrl,
}

impl Ruleset_Rules_ActionParameters_FromValue {
    #[doc = "Set the field `preserve_query_string`.\nWhether to keep the query string of the original request."]
    pub fn set_preserve_query_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_query_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\nThe status code to use for the redirect."]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_FromValue {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_FromValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_FromValue {
    #[doc = "A URL to redirect the request to."]
    pub target_url: Ruleset_Rules_ActionParameters_FromValue_TargetUrl,
}

impl BuildRuleset_Rules_ActionParameters_FromValue {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_FromValue {
        Ruleset_Rules_ActionParameters_FromValue {
            preserve_query_string: core::default::Default::default(),
            status_code: core::default::Default::default(),
            target_url: self.target_url,
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_FromValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_FromValueRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_FromValueRef {
        Ruleset_Rules_ActionParameters_FromValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_FromValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `preserve_query_string` after provisioning.\nWhether to keep the query string of the original request."]
    pub fn preserve_query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_query_string", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status_code` after provisioning.\nThe status code to use for the redirect."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc = "Get a reference to the value of field `target_url` after provisioning.\nA URL to redirect the request to."]
    pub fn target_url(&self) -> Ruleset_Rules_ActionParameters_FromValue_TargetUrlRef {
        Ruleset_Rules_ActionParameters_FromValue_TargetUrlRef::new(
            self.shared().clone(),
            format!("{}.target_url", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Headers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    operation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ruleset_Rules_ActionParameters_Headers {
    #[doc = "Set the field `expression`.\nAn expression that evaluates to a value for the header."]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nA static value for the header."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Headers {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Headers {
    #[doc = "The operation to perform on the header.\nAvailable values: \"add\", \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_Headers {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Headers {
        Ruleset_Rules_ActionParameters_Headers {
            expression: core::default::Default::default(),
            operation: self.operation,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_HeadersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_HeadersRef {
        Ruleset_Rules_ActionParameters_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nAn expression that evaluates to a value for the header."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform on the header.\nAvailable values: \"add\", \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nA static value for the header."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Immutable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_only: std::option::Option<terrars::PrimField<bool>>,
    operation: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_Immutable {
    #[doc = "Set the field `cloudflare_only`.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn set_cloudflare_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Immutable {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Immutable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Immutable {
    #[doc = "The operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_Immutable {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Immutable {
        Ruleset_Rules_ActionParameters_Immutable {
            cloudflare_only: core::default::Default::default(),
            operation: self.operation,
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_ImmutableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_ImmutableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_ImmutableRef {
        Ruleset_Rules_ActionParameters_ImmutableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_ImmutableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_MatchedData {
    public_key: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_MatchedData { }

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_MatchedData {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_MatchedData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_MatchedData {
    #[doc = "The public key to encrypt matched data logs with."]
    pub public_key: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_MatchedData {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_MatchedData {
        Ruleset_Rules_ActionParameters_MatchedData { public_key: self.public_key }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_MatchedDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_MatchedDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_MatchedDataRef {
        Ruleset_Rules_ActionParameters_MatchedDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_MatchedDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `public_key` after provisioning.\nThe public key to encrypt matched data logs with."]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_MaxAge {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_only: std::option::Option<terrars::PrimField<bool>>,
    operation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl Ruleset_Rules_ActionParameters_MaxAge {
    #[doc = "Set the field `cloudflare_only`.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn set_cloudflare_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe value for the directive in seconds."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_MaxAge {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_MaxAge>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_MaxAge {
    #[doc = "The operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_MaxAge {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_MaxAge {
        Ruleset_Rules_ActionParameters_MaxAge {
            cloudflare_only: core::default::Default::default(),
            operation: self.operation,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_MaxAgeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_MaxAgeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_MaxAgeRef {
        Ruleset_Rules_ActionParameters_MaxAgeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_MaxAgeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe value for the directive in seconds."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_MustRevalidate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_only: std::option::Option<terrars::PrimField<bool>>,
    operation: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_MustRevalidate {
    #[doc = "Set the field `cloudflare_only`.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn set_cloudflare_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_MustRevalidate {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_MustRevalidate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_MustRevalidate {
    #[doc = "The operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_MustRevalidate {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_MustRevalidate {
        Ruleset_Rules_ActionParameters_MustRevalidate {
            cloudflare_only: core::default::Default::default(),
            operation: self.operation,
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_MustRevalidateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_MustRevalidateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_MustRevalidateRef {
        Ruleset_Rules_ActionParameters_MustRevalidateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_MustRevalidateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_MustUnderstand {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_only: std::option::Option<terrars::PrimField<bool>>,
    operation: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_MustUnderstand {
    #[doc = "Set the field `cloudflare_only`.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn set_cloudflare_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_MustUnderstand {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_MustUnderstand>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_MustUnderstand {
    #[doc = "The operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_MustUnderstand {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_MustUnderstand {
        Ruleset_Rules_ActionParameters_MustUnderstand {
            cloudflare_only: core::default::Default::default(),
            operation: self.operation,
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_MustUnderstandRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_MustUnderstandRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_MustUnderstandRef {
        Ruleset_Rules_ActionParameters_MustUnderstandRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_MustUnderstandRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_NoCache {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_only: std::option::Option<terrars::PrimField<bool>>,
    operation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualifiers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Ruleset_Rules_ActionParameters_NoCache {
    #[doc = "Set the field `cloudflare_only`.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn set_cloudflare_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `qualifiers`.\nThe qualifiers for the directive."]
    pub fn set_qualifiers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.qualifiers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_NoCache {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_NoCache>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_NoCache {
    #[doc = "The operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_NoCache {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_NoCache {
        Ruleset_Rules_ActionParameters_NoCache {
            cloudflare_only: core::default::Default::default(),
            operation: self.operation,
            qualifiers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_NoCacheRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_NoCacheRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_NoCacheRef {
        Ruleset_Rules_ActionParameters_NoCacheRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_NoCacheRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc = "Get a reference to the value of field `qualifiers` after provisioning.\nThe qualifiers for the directive."]
    pub fn qualifiers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qualifiers", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_NoStore {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_only: std::option::Option<terrars::PrimField<bool>>,
    operation: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_NoStore {
    #[doc = "Set the field `cloudflare_only`.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn set_cloudflare_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_NoStore {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_NoStore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_NoStore {
    #[doc = "The operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_NoStore {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_NoStore {
        Ruleset_Rules_ActionParameters_NoStore {
            cloudflare_only: core::default::Default::default(),
            operation: self.operation,
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_NoStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_NoStoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_NoStoreRef {
        Ruleset_Rules_ActionParameters_NoStoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_NoStoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_NoTransform {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_only: std::option::Option<terrars::PrimField<bool>>,
    operation: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_NoTransform {
    #[doc = "Set the field `cloudflare_only`.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn set_cloudflare_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_NoTransform {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_NoTransform>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_NoTransform {
    #[doc = "The operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_NoTransform {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_NoTransform {
        Ruleset_Rules_ActionParameters_NoTransform {
            cloudflare_only: core::default::Default::default(),
            operation: self.operation,
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_NoTransformRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_NoTransformRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_NoTransformRef {
        Ruleset_Rules_ActionParameters_NoTransformRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_NoTransformRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Origin {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl Ruleset_Rules_ActionParameters_Origin {
    #[doc = "Set the field `host`.\nA resolved host to route to."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nA destination port to route to."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Origin {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Origin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Origin {}

impl BuildRuleset_Rules_ActionParameters_Origin {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Origin {
        Ruleset_Rules_ActionParameters_Origin {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_OriginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_OriginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_OriginRef {
        Ruleset_Rules_ActionParameters_OriginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_OriginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nA resolved host to route to."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nA destination port to route to."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_OriginRangeRequests {
    mode: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_OriginRangeRequests { }

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_OriginRangeRequests {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_OriginRangeRequests>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_OriginRangeRequests {
    #[doc =
        "Whether to use range requests. `default` is the behaviour the zone gets without this rule.\nAvailable values: \"on\", \"off\", \"default\"."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_OriginRangeRequests {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_OriginRangeRequests {
        Ruleset_Rules_ActionParameters_OriginRangeRequests { mode: self.mode }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_OriginRangeRequestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_OriginRangeRequestsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_OriginRangeRequestsRef {
        Ruleset_Rules_ActionParameters_OriginRangeRequestsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_OriginRangeRequestsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nWhether to use range requests. `default` is the behaviour the zone gets without this rule.\nAvailable values: \"on\", \"off\", \"default\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Overrides_Categories {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    category: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ruleset_Rules_ActionParameters_Overrides_Categories {
    #[doc = "Set the field `action`.\nThe action to override rules in the category with."]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether to enable execution of rules in the category."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sensitivity_level`.\nThe sensitivity level to use for rules in the category. This option is only applicable for DDoS phases.\nAvailable values: \"default\", \"medium\", \"low\", \"eoff\"."]
    pub fn set_sensitivity_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sensitivity_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Overrides_Categories {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Overrides_Categories>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Overrides_Categories {
    #[doc = "The name of the category to override."]
    pub category: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_Overrides_Categories {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Overrides_Categories {
        Ruleset_Rules_ActionParameters_Overrides_Categories {
            action: core::default::Default::default(),
            category: self.category,
            enabled: core::default::Default::default(),
            sensitivity_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_Overrides_CategoriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_Overrides_CategoriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_Overrides_CategoriesRef {
        Ruleset_Rules_ActionParameters_Overrides_CategoriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_Overrides_CategoriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe action to override rules in the category with."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `category` after provisioning.\nThe name of the category to override."]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable execution of rules in the category."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sensitivity_level` after provisioning.\nThe sensitivity level to use for rules in the category. This option is only applicable for DDoS phases.\nAvailable values: \"default\", \"medium\", \"low\", \"eoff\"."]
    pub fn sensitivity_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Overrides_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ruleset_Rules_ActionParameters_Overrides_Rules {
    #[doc = "Set the field `action`.\nThe action to override the rule with."]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether to enable execution of the rule."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `score_threshold`.\nThe score threshold to use for the rule."]
    pub fn set_score_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.score_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sensitivity_level`.\nThe sensitivity level to use for the rule. This option is only applicable for DDoS phases.\nAvailable values: \"default\", \"medium\", \"low\", \"eoff\"."]
    pub fn set_sensitivity_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sensitivity_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Overrides_Rules {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Overrides_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Overrides_Rules {
    #[doc = "The ID of the rule to override."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_Overrides_Rules {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Overrides_Rules {
        Ruleset_Rules_ActionParameters_Overrides_Rules {
            action: core::default::Default::default(),
            enabled: core::default::Default::default(),
            id: self.id,
            score_threshold: core::default::Default::default(),
            sensitivity_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_Overrides_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_Overrides_RulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_Overrides_RulesRef {
        Ruleset_Rules_ActionParameters_Overrides_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_Overrides_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe action to override the rule with."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable execution of the rule."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the rule to override."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `score_threshold` after provisioning.\nThe score threshold to use for the rule."]
    pub fn score_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sensitivity_level` after provisioning.\nThe sensitivity level to use for the rule. This option is only applicable for DDoS phases.\nAvailable values: \"default\", \"medium\", \"low\", \"eoff\"."]
    pub fn sensitivity_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Overrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    categories: std::option::Option<std::vec::Vec<Ruleset_Rules_ActionParameters_Overrides_Categories>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<Ruleset_Rules_ActionParameters_Overrides_Rules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ruleset_Rules_ActionParameters_Overrides {
    #[doc =
        "Set the field `action`.\nAn action to override all rules with. This option has lower precedence than rule and category overrides."]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `categories`.\nA list of category-level overrides. This option has the second-highest precedence after rule-level overrides."]
    pub fn set_categories(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<Ruleset_Rules_ActionParameters_Overrides_Categories>>,
    ) -> Self {
        self.categories = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enabled`.\nWhether to enable execution of all rules. This option has lower precedence than rule and category overrides."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\nA list of rule-level overrides. This option has the highest precedence."]
    pub fn set_rules(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<Ruleset_Rules_ActionParameters_Overrides_Rules>>,
    ) -> Self {
        self.rules = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sensitivity_level`.\nA sensitivity level to set for all rules. This option has lower precedence than rule and category overrides and is only applicable for DDoS phases.\nAvailable values: \"default\", \"medium\", \"low\", \"eoff\"."]
    pub fn set_sensitivity_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sensitivity_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Overrides {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Overrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Overrides {}

impl BuildRuleset_Rules_ActionParameters_Overrides {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Overrides {
        Ruleset_Rules_ActionParameters_Overrides {
            action: core::default::Default::default(),
            categories: core::default::Default::default(),
            enabled: core::default::Default::default(),
            rules: core::default::Default::default(),
            sensitivity_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_OverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_OverridesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_OverridesRef {
        Ruleset_Rules_ActionParameters_OverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_OverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAn action to override all rules with. This option has lower precedence than rule and category overrides."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `categories` after provisioning.\nA list of category-level overrides. This option has the second-highest precedence after rule-level overrides."]
    pub fn categories(&self) -> terrars::ListRef<Ruleset_Rules_ActionParameters_Overrides_CategoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categories", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable execution of all rules. This option has lower precedence than rule and category overrides."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nA list of rule-level overrides. This option has the highest precedence."]
    pub fn rules(&self) -> terrars::ListRef<Ruleset_Rules_ActionParameters_Overrides_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sensitivity_level` after provisioning.\nA sensitivity level to set for all rules. This option has lower precedence than rule and category overrides and is only applicable for DDoS phases.\nAvailable values: \"default\", \"medium\", \"low\", \"eoff\"."]
    pub fn sensitivity_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Private {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_only: std::option::Option<terrars::PrimField<bool>>,
    operation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualifiers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Ruleset_Rules_ActionParameters_Private {
    #[doc = "Set the field `cloudflare_only`.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn set_cloudflare_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `qualifiers`.\nThe qualifiers for the directive."]
    pub fn set_qualifiers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.qualifiers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Private {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Private>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Private {
    #[doc = "The operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_Private {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Private {
        Ruleset_Rules_ActionParameters_Private {
            cloudflare_only: core::default::Default::default(),
            operation: self.operation,
            qualifiers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_PrivateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_PrivateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_PrivateRef {
        Ruleset_Rules_ActionParameters_PrivateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_PrivateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc = "Get a reference to the value of field `qualifiers` after provisioning.\nThe qualifiers for the directive."]
    pub fn qualifiers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qualifiers", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_ProxyRevalidate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_only: std::option::Option<terrars::PrimField<bool>>,
    operation: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_ProxyRevalidate {
    #[doc = "Set the field `cloudflare_only`.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn set_cloudflare_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_ProxyRevalidate {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_ProxyRevalidate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_ProxyRevalidate {
    #[doc = "The operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_ProxyRevalidate {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_ProxyRevalidate {
        Ruleset_Rules_ActionParameters_ProxyRevalidate {
            cloudflare_only: core::default::Default::default(),
            operation: self.operation,
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_ProxyRevalidateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_ProxyRevalidateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_ProxyRevalidateRef {
        Ruleset_Rules_ActionParameters_ProxyRevalidateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_ProxyRevalidateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Public {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_only: std::option::Option<terrars::PrimField<bool>>,
    operation: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_Public {
    #[doc = "Set the field `cloudflare_only`.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn set_cloudflare_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Public {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Public>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Public {
    #[doc = "The operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_Public {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Public {
        Ruleset_Rules_ActionParameters_Public {
            cloudflare_only: core::default::Default::default(),
            operation: self.operation,
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_PublicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_PublicRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_PublicRef {
        Ruleset_Rules_ActionParameters_PublicRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_PublicRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_RawResponseFields {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_duplicates: std::option::Option<terrars::PrimField<bool>>,
}

impl Ruleset_Rules_ActionParameters_RawResponseFields {
    #[doc = "Set the field `preserve_duplicates`.\nWhether to log duplicate values of the same header."]
    pub fn set_preserve_duplicates(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_duplicates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_RawResponseFields {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_RawResponseFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_RawResponseFields {
    #[doc = "The name of the response header."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_RawResponseFields {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_RawResponseFields {
        Ruleset_Rules_ActionParameters_RawResponseFields {
            name: self.name,
            preserve_duplicates: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_RawResponseFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_RawResponseFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_RawResponseFieldsRef {
        Ruleset_Rules_ActionParameters_RawResponseFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_RawResponseFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the response header."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preserve_duplicates` after provisioning.\nWhether to log duplicate values of the same header."]
    pub fn preserve_duplicates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_duplicates", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_RequestFields {
    name: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_RequestFields { }

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_RequestFields {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_RequestFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_RequestFields {
    #[doc = "The name of the header."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_RequestFields {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_RequestFields {
        Ruleset_Rules_ActionParameters_RequestFields { name: self.name }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_RequestFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_RequestFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_RequestFieldsRef {
        Ruleset_Rules_ActionParameters_RequestFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_RequestFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the header."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Response {
    content: terrars::PrimField<std::string::String>,
    content_type: terrars::PrimField<std::string::String>,
    status_code: terrars::PrimField<f64>,
}

impl Ruleset_Rules_ActionParameters_Response { }

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Response {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Response>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Response {
    #[doc = "The content to return."]
    pub content: terrars::PrimField<std::string::String>,
    #[doc = "The type of the content to return."]
    pub content_type: terrars::PrimField<std::string::String>,
    #[doc = "The status code to return."]
    pub status_code: terrars::PrimField<f64>,
}

impl BuildRuleset_Rules_ActionParameters_Response {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Response {
        Ruleset_Rules_ActionParameters_Response {
            content: self.content,
            content_type: self.content_type,
            status_code: self.status_code,
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_ResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_ResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_ResponseRef {
        Ruleset_Rules_ActionParameters_ResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_ResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe content to return."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nThe type of the content to return."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\nThe status code to return."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_ResponseFields {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_duplicates: std::option::Option<terrars::PrimField<bool>>,
}

impl Ruleset_Rules_ActionParameters_ResponseFields {
    #[doc = "Set the field `preserve_duplicates`.\nWhether to log duplicate values of the same header."]
    pub fn set_preserve_duplicates(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_duplicates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_ResponseFields {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_ResponseFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_ResponseFields {
    #[doc = "The name of the response header."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_ResponseFields {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_ResponseFields {
        Ruleset_Rules_ActionParameters_ResponseFields {
            name: self.name,
            preserve_duplicates: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_ResponseFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_ResponseFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_ResponseFieldsRef {
        Ruleset_Rules_ActionParameters_ResponseFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_ResponseFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the response header."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preserve_duplicates` after provisioning.\nWhether to log duplicate values of the same header."]
    pub fn preserve_duplicates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_duplicates", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_SMaxage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_only: std::option::Option<terrars::PrimField<bool>>,
    operation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl Ruleset_Rules_ActionParameters_SMaxage {
    #[doc = "Set the field `cloudflare_only`.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn set_cloudflare_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe value for the directive in seconds."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_SMaxage {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_SMaxage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_SMaxage {
    #[doc = "The operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_SMaxage {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_SMaxage {
        Ruleset_Rules_ActionParameters_SMaxage {
            cloudflare_only: core::default::Default::default(),
            operation: self.operation,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_SMaxageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_SMaxageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_SMaxageRef {
        Ruleset_Rules_ActionParameters_SMaxageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_SMaxageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe value for the directive in seconds."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_ServeStale {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_stale_while_updating: std::option::Option<terrars::PrimField<bool>>,
}

impl Ruleset_Rules_ActionParameters_ServeStale {
    #[doc =
        "Set the field `disable_stale_while_updating`.\nWhether Cloudflare should disable serving stale content while getting the latest content from the origin."]
    pub fn set_disable_stale_while_updating(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_stale_while_updating = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_ServeStale {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_ServeStale>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_ServeStale {}

impl BuildRuleset_Rules_ActionParameters_ServeStale {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_ServeStale {
        Ruleset_Rules_ActionParameters_ServeStale { disable_stale_while_updating: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_ServeStaleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_ServeStaleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_ServeStaleRef {
        Ruleset_Rules_ActionParameters_ServeStaleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_ServeStaleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_stale_while_updating` after provisioning.\nWhether Cloudflare should disable serving stale content while getting the latest content from the origin."]
    pub fn disable_stale_while_updating(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_stale_while_updating", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Sni {
    value: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_Sni { }

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Sni {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Sni>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Sni {
    #[doc = "A value to override the SNI to."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_Sni {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Sni {
        Ruleset_Rules_ActionParameters_Sni { value: self.value }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_SniRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_SniRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_SniRef {
        Ruleset_Rules_ActionParameters_SniRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_SniRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nA value to override the SNI to."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_StaleIfError {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_only: std::option::Option<terrars::PrimField<bool>>,
    operation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl Ruleset_Rules_ActionParameters_StaleIfError {
    #[doc = "Set the field `cloudflare_only`.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn set_cloudflare_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe value for the directive in seconds."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_StaleIfError {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_StaleIfError>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_StaleIfError {
    #[doc = "The operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_StaleIfError {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_StaleIfError {
        Ruleset_Rules_ActionParameters_StaleIfError {
            cloudflare_only: core::default::Default::default(),
            operation: self.operation,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_StaleIfErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_StaleIfErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_StaleIfErrorRef {
        Ruleset_Rules_ActionParameters_StaleIfErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_StaleIfErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe value for the directive in seconds."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_StaleWhileRevalidate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_only: std::option::Option<terrars::PrimField<bool>>,
    operation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl Ruleset_Rules_ActionParameters_StaleWhileRevalidate {
    #[doc = "Set the field `cloudflare_only`.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn set_cloudflare_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe value for the directive in seconds."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_StaleWhileRevalidate {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_StaleWhileRevalidate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_StaleWhileRevalidate {
    #[doc = "The operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_StaleWhileRevalidate {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_StaleWhileRevalidate {
        Ruleset_Rules_ActionParameters_StaleWhileRevalidate {
            cloudflare_only: core::default::Default::default(),
            operation: self.operation,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_StaleWhileRevalidateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_StaleWhileRevalidateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_StaleWhileRevalidateRef {
        Ruleset_Rules_ActionParameters_StaleWhileRevalidateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_StaleWhileRevalidateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe value for the directive in seconds."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_TransformedRequestFields {
    name: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_TransformedRequestFields { }

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_TransformedRequestFields {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_TransformedRequestFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_TransformedRequestFields {
    #[doc = "The name of the header."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_TransformedRequestFields {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_TransformedRequestFields {
        Ruleset_Rules_ActionParameters_TransformedRequestFields { name: self.name }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_TransformedRequestFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_TransformedRequestFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_TransformedRequestFieldsRef {
        Ruleset_Rules_ActionParameters_TransformedRequestFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_TransformedRequestFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the header."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Uri_Path {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ruleset_Rules_ActionParameters_Uri_Path {
    #[doc = "Set the field `expression`.\nAn expression that evaluates to a value to rewrite the URI path to."]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nA value to rewrite the URI path to."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Uri_Path {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Uri_Path>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Uri_Path {}

impl BuildRuleset_Rules_ActionParameters_Uri_Path {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Uri_Path {
        Ruleset_Rules_ActionParameters_Uri_Path {
            expression: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_Uri_PathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_Uri_PathRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_Uri_PathRef {
        Ruleset_Rules_ActionParameters_Uri_PathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_Uri_PathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nAn expression that evaluates to a value to rewrite the URI path to."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nA value to rewrite the URI path to."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Uri_Query {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ruleset_Rules_ActionParameters_Uri_Query {
    #[doc = "Set the field `expression`.\nAn expression that evaluates to a value to rewrite the URI query to."]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nA value to rewrite the URI query to."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Uri_Query {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Uri_Query>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Uri_Query {}

impl BuildRuleset_Rules_ActionParameters_Uri_Query {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Uri_Query {
        Ruleset_Rules_ActionParameters_Uri_Query {
            expression: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_Uri_QueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_Uri_QueryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_Uri_QueryRef {
        Ruleset_Rules_ActionParameters_Uri_QueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_Uri_QueryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nAn expression that evaluates to a value to rewrite the URI query to."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nA value to rewrite the URI query to."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Uri {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<Ruleset_Rules_ActionParameters_Uri_Path>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query: std::option::Option<Ruleset_Rules_ActionParameters_Uri_Query>,
}

impl Ruleset_Rules_ActionParameters_Uri {
    #[doc = "Set the field `path`.\nA URI path rewrite."]
    pub fn set_path(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_Uri_Path>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query`.\nA URI query rewrite."]
    pub fn set_query(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_Uri_Query>) -> Self {
        self.query = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Uri {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Uri>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Uri {}

impl BuildRuleset_Rules_ActionParameters_Uri {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Uri {
        Ruleset_Rules_ActionParameters_Uri {
            path: core::default::Default::default(),
            query: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_UriRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_UriRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_UriRef {
        Ruleset_Rules_ActionParameters_UriRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_UriRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nA URI path rewrite."]
    pub fn path(&self) -> Ruleset_Rules_ActionParameters_Uri_PathRef {
        Ruleset_Rules_ActionParameters_Uri_PathRef::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\nA URI query rewrite."]
    pub fn query(&self) -> Ruleset_Rules_ActionParameters_Uri_QueryRef {
        Ruleset_Rules_ActionParameters_Uri_QueryRef::new(self.shared().clone(), format!("{}.query", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Vary_Default {
    action: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ActionParameters_Vary_Default { }

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Vary_Default {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Vary_Default>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Vary_Default {
    #[doc =
        "How the header value is treated when building the cache key.\nAvailable values: \"bypass\", \"passthrough\", \"normalize\"."]
    pub action: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_Vary_Default {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Vary_Default {
        Ruleset_Rules_ActionParameters_Vary_Default { action: self.action }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_Vary_DefaultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_Vary_DefaultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_Vary_DefaultRef {
        Ruleset_Rules_ActionParameters_Vary_DefaultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_Vary_DefaultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nHow the header value is treated when building the cache key.\nAvailable values: \"bypass\", \"passthrough\", \"normalize\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Vary_Headers {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    languages: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Ruleset_Rules_ActionParameters_Vary_Headers {
    #[doc =
        "Set the field `languages`.\nThe set of languages to normalize against. Only valid for the `accept-language` header."]
    pub fn set_languages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.languages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `media_types`.\nThe set of media types to normalize against. Only valid for the `accept` header."]
    pub fn set_media_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.media_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Vary_Headers {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Vary_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Vary_Headers {
    #[doc =
        "How the header value is treated when building the cache key.\nAvailable values: \"bypass\", \"passthrough\", \"normalize\"."]
    pub action: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ActionParameters_Vary_Headers {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Vary_Headers {
        Ruleset_Rules_ActionParameters_Vary_Headers {
            action: self.action,
            languages: core::default::Default::default(),
            media_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_Vary_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_Vary_HeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ruleset_Rules_ActionParameters_Vary_HeadersRef {
        Ruleset_Rules_ActionParameters_Vary_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_Vary_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nHow the header value is treated when building the cache key.\nAvailable values: \"bypass\", \"passthrough\", \"normalize\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `languages` after provisioning.\nThe set of languages to normalize against. Only valid for the `accept-language` header."]
    pub fn languages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.languages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `media_types` after provisioning.\nThe set of media types to normalize against. Only valid for the `accept` header."]
    pub fn media_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.media_types", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters_Vary {
    default: Ruleset_Rules_ActionParameters_Vary_Default,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, Ruleset_Rules_ActionParameters_Vary_Headers>>,
}

impl Ruleset_Rules_ActionParameters_Vary {
    #[doc = "Set the field `headers`.\nA mapping of lowercase request header names to their vary configuration."]
    pub fn set_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std::collections::HashMap<std::string::String, Ruleset_Rules_ActionParameters_Vary_Headers>,
                    >,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters_Vary {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters_Vary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters_Vary {
    #[doc = "Controls how response Vary headers without a per-header override contribute to the cache key."]
    pub default: Ruleset_Rules_ActionParameters_Vary_Default,
}

impl BuildRuleset_Rules_ActionParameters_Vary {
    pub fn build(self) -> Ruleset_Rules_ActionParameters_Vary {
        Ruleset_Rules_ActionParameters_Vary {
            default: self.default,
            headers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParameters_VaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParameters_VaryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParameters_VaryRef {
        Ruleset_Rules_ActionParameters_VaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParameters_VaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nControls how response Vary headers without a per-header override contribute to the cache key."]
    pub fn default(&self) -> Ruleset_Rules_ActionParameters_Vary_DefaultRef {
        Ruleset_Rules_ActionParameters_Vary_DefaultRef::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc =
        "Get a reference to the value of field `headers` after provisioning.\nA mapping of lowercase request header names to their vary configuration."]
    pub fn headers(&self) -> terrars::RecRef<Ruleset_Rules_ActionParameters_Vary_HeadersRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ActionParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_cacheable_ports: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithms: std::option::Option<std::vec::Vec<Ruleset_Rules_ActionParameters_Algorithms>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asset_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_https_rewrites: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autominify: std::option::Option<Ruleset_Rules_ActionParameters_Autominify>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bic: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browser_ttl: std::option::Option<Ruleset_Rules_ActionParameters_BrowserTtl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_key: std::option::Option<Ruleset_Rules_ActionParameters_CacheKey>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_reserve: std::option::Option<Ruleset_Rules_ActionParameters_CacheReserve>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_converter: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_fields: std::option::Option<std::vec::Vec<Ruleset_Rules_ActionParameters_CookieFields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_apps: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_rum: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_zaraz: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_ttl: std::option::Option<Ruleset_Rules_ActionParameters_EdgeTtl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_obfuscation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fonts: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_list: std::option::Option<Ruleset_Rules_ActionParameters_FromList>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_value: std::option::Option<Ruleset_Rules_ActionParameters_FromValue>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<std::collections::HashMap<std::string::String, Ruleset_Rules_ActionParameters_Headers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotlink_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    immutable: std::option::Option<Ruleset_Rules_ActionParameters_Immutable>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    increment: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matched_data: std::option::Option<Ruleset_Rules_ActionParameters_MatchedData>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<Ruleset_Rules_ActionParameters_MaxAge>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mirage: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    must_revalidate: std::option::Option<Ruleset_Rules_ActionParameters_MustRevalidate>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    must_understand: std::option::Option<Ruleset_Rules_ActionParameters_MustUnderstand>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_cache: std::option::Option<Ruleset_Rules_ActionParameters_NoCache>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_store: std::option::Option<Ruleset_Rules_ActionParameters_NoStore>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_transform: std::option::Option<Ruleset_Rules_ActionParameters_NoTransform>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opportunistic_encryption: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<Ruleset_Rules_ActionParameters_Origin>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_cache_control: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_error_page_passthru: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_range_requests: std::option::Option<Ruleset_Rules_ActionParameters_OriginRangeRequests>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overrides: std::option::Option<Ruleset_Rules_ActionParameters_Overrides>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phases: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    polish: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private: std::option::Option<Ruleset_Rules_ActionParameters_Private>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    products: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_revalidate: std::option::Option<Ruleset_Rules_ActionParameters_ProxyRevalidate>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public: std::option::Option<Ruleset_Rules_ActionParameters_Public>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_response_fields: std::option::Option<std::vec::Vec<Ruleset_Rules_ActionParameters_RawResponseFields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirects_for_ai_training: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_body_buffering: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_fields: std::option::Option<std::vec::Vec<Ruleset_Rules_ActionParameters_RequestFields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    respect_strong_etags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<Ruleset_Rules_ActionParameters_Response>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_body_buffering: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_fields: std::option::Option<std::vec::Vec<Ruleset_Rules_ActionParameters_ResponseFields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rocket_loader: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ruleset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rulesets: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s_maxage: std::option::Option<Ruleset_Rules_ActionParameters_SMaxage>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serve_stale: std::option::Option<Ruleset_Rules_ActionParameters_ServeStale>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_excludes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sni: std::option::Option<Ruleset_Rules_ActionParameters_Sni>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stale_if_error: std::option::Option<Ruleset_Rules_ActionParameters_StaleIfError>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stale_while_revalidate: std::option::Option<Ruleset_Rules_ActionParameters_StaleWhileRevalidate>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strip_etags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strip_last_modified: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strip_set_cookie: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sxg: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformed_request_fields: std
    ::option
    ::Option<std::vec::Vec<Ruleset_Rules_ActionParameters_TransformedRequestFields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<Ruleset_Rules_ActionParameters_Uri>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vary: std::option::Option<Ruleset_Rules_ActionParameters_Vary>,
}

impl Ruleset_Rules_ActionParameters {
    #[doc =
        "Set the field `additional_cacheable_ports`.\nA list of additional ports that caching should be enabled on."]
    pub fn set_additional_cacheable_ports(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.additional_cacheable_ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `algorithms`.\nCustom order for compression algorithms."]
    pub fn set_algorithms(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<Ruleset_Rules_ActionParameters_Algorithms>>,
    ) -> Self {
        self.algorithms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `asset_name`.\nThe name of a custom asset to serve as the response."]
    pub fn set_asset_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.asset_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automatic_https_rewrites`.\nWhether to enable Automatic HTTPS Rewrites."]
    pub fn set_automatic_https_rewrites(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.automatic_https_rewrites = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autominify`.\nWhich file extensions to minify automatically."]
    pub fn set_autominify(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_Autominify>) -> Self {
        self.autominify = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bic`.\nWhether to enable Browser Integrity Check (BIC)."]
    pub fn set_bic(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bic = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `browser_ttl`.\nHow long client browsers should cache the response. Cloudflare cache purge will not purge content cached on client browsers, so high browser TTLs may lead to stale content."]
    pub fn set_browser_ttl(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_BrowserTtl>) -> Self {
        self.browser_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cache`.\nWhether the request's response from the origin is eligible for caching. Caching itself will still depend on the cache control header and your other caching configurations."]
    pub fn set_cache(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cache = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cache_key`.\nWhich components of the request are included in or excluded from the cache key Cloudflare uses to store the response in cache."]
    pub fn set_cache_key(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_CacheKey>) -> Self {
        self.cache_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cache_reserve`.\nSettings to determine whether the request's response from origin is eligible for Cache Reserve (requires a Cache Reserve add-on plan)."]
    pub fn set_cache_reserve(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_CacheReserve>) -> Self {
        self.cache_reserve = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content`.\nThe response content."]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_converter`.\nWhether to enable content conversion (e.g., HTML to Markdown)."]
    pub fn set_content_converter(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.content_converter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_type`.\nThe content type header to set with the error response.\nAvailable values: \"application/json\", \"text/html\", \"text/plain\", \"text/xml\"."]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cookie_fields`.\nThe cookie fields to log."]
    pub fn set_cookie_fields(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<Ruleset_Rules_ActionParameters_CookieFields>>,
    ) -> Self {
        self.cookie_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_apps`.\nWhether to disable Cloudflare Apps."]
    pub fn set_disable_apps(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_apps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_rum`.\nWhether to disable Real User Monitoring (RUM)."]
    pub fn set_disable_rum(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_rum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_zaraz`.\nWhether to disable Zaraz."]
    pub fn set_disable_zaraz(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_zaraz = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edge_ttl`.\nHow long the Cloudflare edge network should cache the response."]
    pub fn set_edge_ttl(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_EdgeTtl>) -> Self {
        self.edge_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_obfuscation`.\nWhether to enable Email Obfuscation."]
    pub fn set_email_obfuscation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.email_obfuscation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expression`.\nAn expression to generate cache tags for set_cache_tags action."]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fonts`.\nWhether to enable Cloudflare Fonts."]
    pub fn set_fonts(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fonts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_list`.\nA redirect based on a bulk list lookup."]
    pub fn set_from_list(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_FromList>) -> Self {
        self.from_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_value`.\nA redirect based on the request properties."]
    pub fn set_from_value(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_FromValue>) -> Self {
        self.from_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `headers`.\nA map of headers to rewrite."]
    pub fn set_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<std::collections::HashMap<std::string::String, Ruleset_Rules_ActionParameters_Headers>>,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_header`.\nA value to rewrite the HTTP host header to."]
    pub fn set_host_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hotlink_protection`.\nWhether to enable Hotlink Protection."]
    pub fn set_hotlink_protection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.hotlink_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nThe ID of the ruleset to execute."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `immutable`.\nSet the immutable cache control directive."]
    pub fn set_immutable(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_Immutable>) -> Self {
        self.immutable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `increment`.\nA delta to change the score by, which can be either positive or negative."]
    pub fn set_increment(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.increment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `matched_data`.\nThe configuration to use for matched data logging."]
    pub fn set_matched_data(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_MatchedData>) -> Self {
        self.matched_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\nSet the max-age cache control directive."]
    pub fn set_max_age(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_MaxAge>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mirage`.\nWhether to enable Mirage."]
    pub fn set_mirage(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mirage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `must_revalidate`.\nSet the must-revalidate cache control directive."]
    pub fn set_must_revalidate(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_MustRevalidate>,
    ) -> Self {
        self.must_revalidate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `must_understand`.\nSet the must-understand cache control directive."]
    pub fn set_must_understand(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_MustUnderstand>,
    ) -> Self {
        self.must_understand = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_cache`.\nSet the no-cache cache control directive."]
    pub fn set_no_cache(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_NoCache>) -> Self {
        self.no_cache = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_store`.\nSet the no-store cache control directive."]
    pub fn set_no_store(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_NoStore>) -> Self {
        self.no_store = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_transform`.\nSet the no-transform cache control directive."]
    pub fn set_no_transform(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_NoTransform>) -> Self {
        self.no_transform = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `operation`.\nThe operation to perform for set_cache_tags action.\nAvailable values: \"set\", \"add\", \"remove\"."]
    pub fn set_operation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `opportunistic_encryption`.\nWhether to enable Opportunistic Encryption."]
    pub fn set_opportunistic_encryption(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.opportunistic_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin`.\nAn origin to route to."]
    pub fn set_origin(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_Origin>) -> Self {
        self.origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_cache_control`.\nWhether Cloudflare will aim to strictly adhere to RFC 7234."]
    pub fn set_origin_cache_control(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.origin_cache_control = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_error_page_passthru`.\nWhether to generate Cloudflare error pages for issues from the origin server."]
    pub fn set_origin_error_page_passthru(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.origin_error_page_passthru = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_range_requests`.\nControls whether Cloudflare fetches a large asset from the origin as a series of range requests instead of one whole-body request."]
    pub fn set_origin_range_requests(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_OriginRangeRequests>,
    ) -> Self {
        self.origin_range_requests = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `overrides`.\nA set of overrides to apply to the target ruleset."]
    pub fn set_overrides(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_Overrides>) -> Self {
        self.overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `phases`.\nA list of phases to skip the execution of. This option is incompatible with the rulesets option.\nAvailable values: \"ddos_l4\", \"ddos_l7\", \"http_config_settings\", \"http_custom_errors\", \"http_log_custom_fields\", \"http_ratelimit\", \"http_request_cache_settings\", \"http_request_dynamic_redirect\", \"http_request_firewall_custom\", \"http_request_firewall_managed\", \"http_request_late_transform\", \"http_request_origin\", \"http_request_redirect\", \"http_request_sanitize\", \"http_request_sbfm\", \"http_request_transform\", \"http_response_cache_settings\", \"http_response_compression\", \"http_response_firewall_managed\", \"http_response_headers_transform\", \"magic_transit\", \"magic_transit_ids_managed\", \"magic_transit_managed\", \"magic_transit_ratelimit\"."]
    pub fn set_phases(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.phases = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `polish`.\nThe Polish level to configure.\nAvailable values: \"off\", \"lossless\", \"lossy\", \"webp\"."]
    pub fn set_polish(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.polish = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private`.\nSet the private cache control directive."]
    pub fn set_private(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_Private>) -> Self {
        self.private = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `products`.\nA list of legacy security products to skip the execution of.\nAvailable values: \"bic\", \"hot\", \"rateLimit\", \"securityLevel\", \"uaBlock\", \"waf\", \"zoneLockdown\"."]
    pub fn set_products(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.products = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proxy_revalidate`.\nSet the proxy-revalidate cache control directive."]
    pub fn set_proxy_revalidate(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_ProxyRevalidate>,
    ) -> Self {
        self.proxy_revalidate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public`.\nSet the public cache control directive."]
    pub fn set_public(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_Public>) -> Self {
        self.public = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `raw_response_fields`.\nThe raw response fields to log."]
    pub fn set_raw_response_fields(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<Ruleset_Rules_ActionParameters_RawResponseFields>>,
    ) -> Self {
        self.raw_response_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `read_timeout`.\nA timeout value between two successive read operations to use for your origin server. Historically, the timeout value between two read options from Cloudflare to an origin server is 100 seconds. If you are attempting to reduce HTTP 524 errors because of timeouts from an origin server, try increasing this timeout value."]
    pub fn set_read_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.read_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redirects_for_ai_training`.\nWhether to redirect verified AI training crawlers to canonical URLs."]
    pub fn set_redirects_for_ai_training(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.redirects_for_ai_training = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_body_buffering`.\nThe request body buffering mode to configure.\nAvailable values: \"none\", \"standard\", \"full\"."]
    pub fn set_request_body_buffering(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.request_body_buffering = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_fields`.\nThe raw request fields to log."]
    pub fn set_request_fields(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<Ruleset_Rules_ActionParameters_RequestFields>>,
    ) -> Self {
        self.request_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `respect_strong_etags`.\nWhether Cloudflare should respect strong ETag (entity tag) headers. If false, Cloudflare converts strong ETag headers to weak ETag headers."]
    pub fn set_respect_strong_etags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.respect_strong_etags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response`.\nThe response to show when the block is applied."]
    pub fn set_response(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_Response>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response_body_buffering`.\nThe response body buffering mode to configure.\nAvailable values: \"none\", \"standard\"."]
    pub fn set_response_body_buffering(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.response_body_buffering = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_fields`.\nThe transformed response fields to log."]
    pub fn set_response_fields(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<Ruleset_Rules_ActionParameters_ResponseFields>>,
    ) -> Self {
        self.response_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rocket_loader`.\nWhether to enable Rocket Loader."]
    pub fn set_rocket_loader(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.rocket_loader = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rules`.\nA mapping of ruleset IDs to a list of rule IDs in that ruleset to skip the execution of. This option is incompatible with the ruleset option."]
    pub fn set_rules(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.rules = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ruleset`.\nA ruleset to skip the execution of. This option is incompatible with the rulesets option.\nAvailable values: \"current\"."]
    pub fn set_ruleset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ruleset = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rulesets`.\nA list of ruleset IDs to skip the execution of. This option is incompatible with the ruleset and phases options."]
    pub fn set_rulesets(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.rulesets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s_maxage`.\nSet the s-maxage cache control directive."]
    pub fn set_s_maxage(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_SMaxage>) -> Self {
        self.s_maxage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_level`.\nThe Security Level to configure.\nAvailable values: \"off\", \"essentially_off\", \"low\", \"medium\", \"high\", \"under_attack\"."]
    pub fn set_security_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serve_stale`.\nWhen to serve stale content from cache."]
    pub fn set_serve_stale(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_ServeStale>) -> Self {
        self.serve_stale = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_side_excludes`.\nWhether to enable Server-Side Excludes."]
    pub fn set_server_side_excludes(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.server_side_excludes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sni`.\nA Server Name Indication (SNI) override."]
    pub fn set_sni(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_Sni>) -> Self {
        self.sni = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl`.\nThe SSL level to configure.\nAvailable values: \"off\", \"flexible\", \"full\", \"strict\", \"origin_pull\"."]
    pub fn set_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stale_if_error`.\nSet the stale-if-error cache control directive."]
    pub fn set_stale_if_error(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_StaleIfError>) -> Self {
        self.stale_if_error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stale_while_revalidate`.\nSet the stale-while-revalidate cache control directive."]
    pub fn set_stale_while_revalidate(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ActionParameters_StaleWhileRevalidate>,
    ) -> Self {
        self.stale_while_revalidate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\nThe status code to use for the error."]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strip_etags`.\nWhether to strip the ETag header from the response."]
    pub fn set_strip_etags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strip_etags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strip_last_modified`.\nWhether to strip the Last-Modified header from the response."]
    pub fn set_strip_last_modified(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strip_last_modified = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strip_set_cookie`.\nWhether to strip the Set-Cookie header from the response."]
    pub fn set_strip_set_cookie(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strip_set_cookie = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sxg`.\nWhether to enable Signed Exchanges (SXG)."]
    pub fn set_sxg(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sxg = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transformed_request_fields`.\nThe transformed request fields to log."]
    pub fn set_transformed_request_fields(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<Ruleset_Rules_ActionParameters_TransformedRequestFields>>,
    ) -> Self {
        self.transformed_request_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\nA URI rewrite."]
    pub fn set_uri(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_Uri>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\nThe cache tag values for set_cache_tags action."]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vary`.\nControls how cached responses vary based on request headers. `default` is required and applies to any Vary response header that does not have a per-header override."]
    pub fn set_vary(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters_Vary>) -> Self {
        self.vary = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_ActionParameters {
    type O = terrars::BlockAssignable<Ruleset_Rules_ActionParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ActionParameters {}

impl BuildRuleset_Rules_ActionParameters {
    pub fn build(self) -> Ruleset_Rules_ActionParameters {
        Ruleset_Rules_ActionParameters {
            additional_cacheable_ports: core::default::Default::default(),
            algorithms: core::default::Default::default(),
            asset_name: core::default::Default::default(),
            automatic_https_rewrites: core::default::Default::default(),
            autominify: core::default::Default::default(),
            bic: core::default::Default::default(),
            browser_ttl: core::default::Default::default(),
            cache: core::default::Default::default(),
            cache_key: core::default::Default::default(),
            cache_reserve: core::default::Default::default(),
            content: core::default::Default::default(),
            content_converter: core::default::Default::default(),
            content_type: core::default::Default::default(),
            cookie_fields: core::default::Default::default(),
            disable_apps: core::default::Default::default(),
            disable_rum: core::default::Default::default(),
            disable_zaraz: core::default::Default::default(),
            edge_ttl: core::default::Default::default(),
            email_obfuscation: core::default::Default::default(),
            expression: core::default::Default::default(),
            fonts: core::default::Default::default(),
            from_list: core::default::Default::default(),
            from_value: core::default::Default::default(),
            headers: core::default::Default::default(),
            host_header: core::default::Default::default(),
            hotlink_protection: core::default::Default::default(),
            id: core::default::Default::default(),
            immutable: core::default::Default::default(),
            increment: core::default::Default::default(),
            matched_data: core::default::Default::default(),
            max_age: core::default::Default::default(),
            mirage: core::default::Default::default(),
            must_revalidate: core::default::Default::default(),
            must_understand: core::default::Default::default(),
            no_cache: core::default::Default::default(),
            no_store: core::default::Default::default(),
            no_transform: core::default::Default::default(),
            operation: core::default::Default::default(),
            opportunistic_encryption: core::default::Default::default(),
            origin: core::default::Default::default(),
            origin_cache_control: core::default::Default::default(),
            origin_error_page_passthru: core::default::Default::default(),
            origin_range_requests: core::default::Default::default(),
            overrides: core::default::Default::default(),
            phases: core::default::Default::default(),
            polish: core::default::Default::default(),
            private: core::default::Default::default(),
            products: core::default::Default::default(),
            proxy_revalidate: core::default::Default::default(),
            public: core::default::Default::default(),
            raw_response_fields: core::default::Default::default(),
            read_timeout: core::default::Default::default(),
            redirects_for_ai_training: core::default::Default::default(),
            request_body_buffering: core::default::Default::default(),
            request_fields: core::default::Default::default(),
            respect_strong_etags: core::default::Default::default(),
            response: core::default::Default::default(),
            response_body_buffering: core::default::Default::default(),
            response_fields: core::default::Default::default(),
            rocket_loader: core::default::Default::default(),
            rules: core::default::Default::default(),
            ruleset: core::default::Default::default(),
            rulesets: core::default::Default::default(),
            s_maxage: core::default::Default::default(),
            security_level: core::default::Default::default(),
            serve_stale: core::default::Default::default(),
            server_side_excludes: core::default::Default::default(),
            sni: core::default::Default::default(),
            ssl: core::default::Default::default(),
            stale_if_error: core::default::Default::default(),
            stale_while_revalidate: core::default::Default::default(),
            status_code: core::default::Default::default(),
            strip_etags: core::default::Default::default(),
            strip_last_modified: core::default::Default::default(),
            strip_set_cookie: core::default::Default::default(),
            sxg: core::default::Default::default(),
            transformed_request_fields: core::default::Default::default(),
            uri: core::default::Default::default(),
            values: core::default::Default::default(),
            vary: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ActionParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ActionParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ActionParametersRef {
        Ruleset_Rules_ActionParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ActionParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_cacheable_ports` after provisioning.\nA list of additional ports that caching should be enabled on."]
    pub fn additional_cacheable_ports(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_cacheable_ports", self.base))
    }

    #[doc =
        "Get a reference to the value of field `algorithms` after provisioning.\nCustom order for compression algorithms."]
    pub fn algorithms(&self) -> terrars::ListRef<Ruleset_Rules_ActionParameters_AlgorithmsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.algorithms", self.base))
    }

    #[doc =
        "Get a reference to the value of field `asset_name` after provisioning.\nThe name of a custom asset to serve as the response."]
    pub fn asset_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `automatic_https_rewrites` after provisioning.\nWhether to enable Automatic HTTPS Rewrites."]
    pub fn automatic_https_rewrites(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_https_rewrites", self.base))
    }

    #[doc =
        "Get a reference to the value of field `autominify` after provisioning.\nWhich file extensions to minify automatically."]
    pub fn autominify(&self) -> Ruleset_Rules_ActionParameters_AutominifyRef {
        Ruleset_Rules_ActionParameters_AutominifyRef::new(self.shared().clone(), format!("{}.autominify", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bic` after provisioning.\nWhether to enable Browser Integrity Check (BIC)."]
    pub fn bic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `browser_ttl` after provisioning.\nHow long client browsers should cache the response. Cloudflare cache purge will not purge content cached on client browsers, so high browser TTLs may lead to stale content."]
    pub fn browser_ttl(&self) -> Ruleset_Rules_ActionParameters_BrowserTtlRef {
        Ruleset_Rules_ActionParameters_BrowserTtlRef::new(
            self.shared().clone(),
            format!("{}.browser_ttl", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `cache` after provisioning.\nWhether the request's response from the origin is eligible for caching. Caching itself will still depend on the cache control header and your other caching configurations."]
    pub fn cache(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cache_key` after provisioning.\nWhich components of the request are included in or excluded from the cache key Cloudflare uses to store the response in cache."]
    pub fn cache_key(&self) -> Ruleset_Rules_ActionParameters_CacheKeyRef {
        Ruleset_Rules_ActionParameters_CacheKeyRef::new(self.shared().clone(), format!("{}.cache_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cache_reserve` after provisioning.\nSettings to determine whether the request's response from origin is eligible for Cache Reserve (requires a Cache Reserve add-on plan)."]
    pub fn cache_reserve(&self) -> Ruleset_Rules_ActionParameters_CacheReserveRef {
        Ruleset_Rules_ActionParameters_CacheReserveRef::new(
            self.shared().clone(),
            format!("{}.cache_reserve", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe response content."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_converter` after provisioning.\nWhether to enable content conversion (e.g., HTML to Markdown)."]
    pub fn content_converter(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_converter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nThe content type header to set with the error response.\nAvailable values: \"application/json\", \"text/html\", \"text/plain\", \"text/xml\"."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `cookie_fields` after provisioning.\nThe cookie fields to log."]
    pub fn cookie_fields(&self) -> terrars::ListRef<Ruleset_Rules_ActionParameters_CookieFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookie_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_apps` after provisioning.\nWhether to disable Cloudflare Apps."]
    pub fn disable_apps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_apps", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_rum` after provisioning.\nWhether to disable Real User Monitoring (RUM)."]
    pub fn disable_rum(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_rum", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_zaraz` after provisioning.\nWhether to disable Zaraz."]
    pub fn disable_zaraz(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_zaraz", self.base))
    }

    #[doc =
        "Get a reference to the value of field `edge_ttl` after provisioning.\nHow long the Cloudflare edge network should cache the response."]
    pub fn edge_ttl(&self) -> Ruleset_Rules_ActionParameters_EdgeTtlRef {
        Ruleset_Rules_ActionParameters_EdgeTtlRef::new(self.shared().clone(), format!("{}.edge_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email_obfuscation` after provisioning.\nWhether to enable Email Obfuscation."]
    pub fn email_obfuscation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_obfuscation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nAn expression to generate cache tags for set_cache_tags action."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `fonts` after provisioning.\nWhether to enable Cloudflare Fonts."]
    pub fn fonts(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fonts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `from_list` after provisioning.\nA redirect based on a bulk list lookup."]
    pub fn from_list(&self) -> Ruleset_Rules_ActionParameters_FromListRef {
        Ruleset_Rules_ActionParameters_FromListRef::new(self.shared().clone(), format!("{}.from_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `from_value` after provisioning.\nA redirect based on the request properties."]
    pub fn from_value(&self) -> Ruleset_Rules_ActionParameters_FromValueRef {
        Ruleset_Rules_ActionParameters_FromValueRef::new(self.shared().clone(), format!("{}.from_value", self.base))
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\nA map of headers to rewrite."]
    pub fn headers(&self) -> terrars::RecRef<Ruleset_Rules_ActionParameters_HeadersRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host_header` after provisioning.\nA value to rewrite the HTTP host header to."]
    pub fn host_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hotlink_protection` after provisioning.\nWhether to enable Hotlink Protection."]
    pub fn hotlink_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hotlink_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the ruleset to execute."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `immutable` after provisioning.\nSet the immutable cache control directive."]
    pub fn immutable(&self) -> Ruleset_Rules_ActionParameters_ImmutableRef {
        Ruleset_Rules_ActionParameters_ImmutableRef::new(self.shared().clone(), format!("{}.immutable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `increment` after provisioning.\nA delta to change the score by, which can be either positive or negative."]
    pub fn increment(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.increment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `matched_data` after provisioning.\nThe configuration to use for matched data logging."]
    pub fn matched_data(&self) -> Ruleset_Rules_ActionParameters_MatchedDataRef {
        Ruleset_Rules_ActionParameters_MatchedDataRef::new(
            self.shared().clone(),
            format!("{}.matched_data", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nSet the max-age cache control directive."]
    pub fn max_age(&self) -> Ruleset_Rules_ActionParameters_MaxAgeRef {
        Ruleset_Rules_ActionParameters_MaxAgeRef::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc = "Get a reference to the value of field `mirage` after provisioning.\nWhether to enable Mirage."]
    pub fn mirage(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mirage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `must_revalidate` after provisioning.\nSet the must-revalidate cache control directive."]
    pub fn must_revalidate(&self) -> Ruleset_Rules_ActionParameters_MustRevalidateRef {
        Ruleset_Rules_ActionParameters_MustRevalidateRef::new(
            self.shared().clone(),
            format!("{}.must_revalidate", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `must_understand` after provisioning.\nSet the must-understand cache control directive."]
    pub fn must_understand(&self) -> Ruleset_Rules_ActionParameters_MustUnderstandRef {
        Ruleset_Rules_ActionParameters_MustUnderstandRef::new(
            self.shared().clone(),
            format!("{}.must_understand", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `no_cache` after provisioning.\nSet the no-cache cache control directive."]
    pub fn no_cache(&self) -> Ruleset_Rules_ActionParameters_NoCacheRef {
        Ruleset_Rules_ActionParameters_NoCacheRef::new(self.shared().clone(), format!("{}.no_cache", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_store` after provisioning.\nSet the no-store cache control directive."]
    pub fn no_store(&self) -> Ruleset_Rules_ActionParameters_NoStoreRef {
        Ruleset_Rules_ActionParameters_NoStoreRef::new(self.shared().clone(), format!("{}.no_store", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_transform` after provisioning.\nSet the no-transform cache control directive."]
    pub fn no_transform(&self) -> Ruleset_Rules_ActionParameters_NoTransformRef {
        Ruleset_Rules_ActionParameters_NoTransformRef::new(
            self.shared().clone(),
            format!("{}.no_transform", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform for set_cache_tags action.\nAvailable values: \"set\", \"add\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `opportunistic_encryption` after provisioning.\nWhether to enable Opportunistic Encryption."]
    pub fn opportunistic_encryption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\nAn origin to route to."]
    pub fn origin(&self) -> Ruleset_Rules_ActionParameters_OriginRef {
        Ruleset_Rules_ActionParameters_OriginRef::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_cache_control` after provisioning.\nWhether Cloudflare will aim to strictly adhere to RFC 7234."]
    pub fn origin_cache_control(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_cache_control", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_error_page_passthru` after provisioning.\nWhether to generate Cloudflare error pages for issues from the origin server."]
    pub fn origin_error_page_passthru(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_error_page_passthru", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_range_requests` after provisioning.\nControls whether Cloudflare fetches a large asset from the origin as a series of range requests instead of one whole-body request."]
    pub fn origin_range_requests(&self) -> Ruleset_Rules_ActionParameters_OriginRangeRequestsRef {
        Ruleset_Rules_ActionParameters_OriginRangeRequestsRef::new(
            self.shared().clone(),
            format!("{}.origin_range_requests", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `overrides` after provisioning.\nA set of overrides to apply to the target ruleset."]
    pub fn overrides(&self) -> Ruleset_Rules_ActionParameters_OverridesRef {
        Ruleset_Rules_ActionParameters_OverridesRef::new(self.shared().clone(), format!("{}.overrides", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phases` after provisioning.\nA list of phases to skip the execution of. This option is incompatible with the rulesets option.\nAvailable values: \"ddos_l4\", \"ddos_l7\", \"http_config_settings\", \"http_custom_errors\", \"http_log_custom_fields\", \"http_ratelimit\", \"http_request_cache_settings\", \"http_request_dynamic_redirect\", \"http_request_firewall_custom\", \"http_request_firewall_managed\", \"http_request_late_transform\", \"http_request_origin\", \"http_request_redirect\", \"http_request_sanitize\", \"http_request_sbfm\", \"http_request_transform\", \"http_response_cache_settings\", \"http_response_compression\", \"http_response_firewall_managed\", \"http_response_headers_transform\", \"magic_transit\", \"magic_transit_ids_managed\", \"magic_transit_managed\", \"magic_transit_ratelimit\"."]
    pub fn phases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `polish` after provisioning.\nThe Polish level to configure.\nAvailable values: \"off\", \"lossless\", \"lossy\", \"webp\"."]
    pub fn polish(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.polish", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private` after provisioning.\nSet the private cache control directive."]
    pub fn private(&self) -> Ruleset_Rules_ActionParameters_PrivateRef {
        Ruleset_Rules_ActionParameters_PrivateRef::new(self.shared().clone(), format!("{}.private", self.base))
    }

    #[doc =
        "Get a reference to the value of field `products` after provisioning.\nA list of legacy security products to skip the execution of.\nAvailable values: \"bic\", \"hot\", \"rateLimit\", \"securityLevel\", \"uaBlock\", \"waf\", \"zoneLockdown\"."]
    pub fn products(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.products", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_revalidate` after provisioning.\nSet the proxy-revalidate cache control directive."]
    pub fn proxy_revalidate(&self) -> Ruleset_Rules_ActionParameters_ProxyRevalidateRef {
        Ruleset_Rules_ActionParameters_ProxyRevalidateRef::new(
            self.shared().clone(),
            format!("{}.proxy_revalidate", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `public` after provisioning.\nSet the public cache control directive."]
    pub fn public(&self) -> Ruleset_Rules_ActionParameters_PublicRef {
        Ruleset_Rules_ActionParameters_PublicRef::new(self.shared().clone(), format!("{}.public", self.base))
    }

    #[doc =
        "Get a reference to the value of field `raw_response_fields` after provisioning.\nThe raw response fields to log."]
    pub fn raw_response_fields(&self) -> terrars::ListRef<Ruleset_Rules_ActionParameters_RawResponseFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.raw_response_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_timeout` after provisioning.\nA timeout value between two successive read operations to use for your origin server. Historically, the timeout value between two read options from Cloudflare to an origin server is 100 seconds. If you are attempting to reduce HTTP 524 errors because of timeouts from an origin server, try increasing this timeout value."]
    pub fn read_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redirects_for_ai_training` after provisioning.\nWhether to redirect verified AI training crawlers to canonical URLs."]
    pub fn redirects_for_ai_training(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirects_for_ai_training", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_body_buffering` after provisioning.\nThe request body buffering mode to configure.\nAvailable values: \"none\", \"standard\", \"full\"."]
    pub fn request_body_buffering(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_body_buffering", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_fields` after provisioning.\nThe raw request fields to log."]
    pub fn request_fields(&self) -> terrars::ListRef<Ruleset_Rules_ActionParameters_RequestFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `respect_strong_etags` after provisioning.\nWhether Cloudflare should respect strong ETag (entity tag) headers. If false, Cloudflare converts strong ETag headers to weak ETag headers."]
    pub fn respect_strong_etags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.respect_strong_etags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response` after provisioning.\nThe response to show when the block is applied."]
    pub fn response(&self) -> Ruleset_Rules_ActionParameters_ResponseRef {
        Ruleset_Rules_ActionParameters_ResponseRef::new(self.shared().clone(), format!("{}.response", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_body_buffering` after provisioning.\nThe response body buffering mode to configure.\nAvailable values: \"none\", \"standard\"."]
    pub fn response_body_buffering(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_body_buffering", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_fields` after provisioning.\nThe transformed response fields to log."]
    pub fn response_fields(&self) -> terrars::ListRef<Ruleset_Rules_ActionParameters_ResponseFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rocket_loader` after provisioning.\nWhether to enable Rocket Loader."]
    pub fn rocket_loader(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rocket_loader", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nA mapping of ruleset IDs to a list of rule IDs in that ruleset to skip the execution of. This option is incompatible with the ruleset option."]
    pub fn rules(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ruleset` after provisioning.\nA ruleset to skip the execution of. This option is incompatible with the rulesets option.\nAvailable values: \"current\"."]
    pub fn ruleset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ruleset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rulesets` after provisioning.\nA list of ruleset IDs to skip the execution of. This option is incompatible with the ruleset and phases options."]
    pub fn rulesets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rulesets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `s_maxage` after provisioning.\nSet the s-maxage cache control directive."]
    pub fn s_maxage(&self) -> Ruleset_Rules_ActionParameters_SMaxageRef {
        Ruleset_Rules_ActionParameters_SMaxageRef::new(self.shared().clone(), format!("{}.s_maxage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_level` after provisioning.\nThe Security Level to configure.\nAvailable values: \"off\", \"essentially_off\", \"low\", \"medium\", \"high\", \"under_attack\"."]
    pub fn security_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serve_stale` after provisioning.\nWhen to serve stale content from cache."]
    pub fn serve_stale(&self) -> Ruleset_Rules_ActionParameters_ServeStaleRef {
        Ruleset_Rules_ActionParameters_ServeStaleRef::new(
            self.shared().clone(),
            format!("{}.serve_stale", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `server_side_excludes` after provisioning.\nWhether to enable Server-Side Excludes."]
    pub fn server_side_excludes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_side_excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `sni` after provisioning.\nA Server Name Indication (SNI) override."]
    pub fn sni(&self) -> Ruleset_Rules_ActionParameters_SniRef {
        Ruleset_Rules_ActionParameters_SniRef::new(self.shared().clone(), format!("{}.sni", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl` after provisioning.\nThe SSL level to configure.\nAvailable values: \"off\", \"flexible\", \"full\", \"strict\", \"origin_pull\"."]
    pub fn ssl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stale_if_error` after provisioning.\nSet the stale-if-error cache control directive."]
    pub fn stale_if_error(&self) -> Ruleset_Rules_ActionParameters_StaleIfErrorRef {
        Ruleset_Rules_ActionParameters_StaleIfErrorRef::new(
            self.shared().clone(),
            format!("{}.stale_if_error", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `stale_while_revalidate` after provisioning.\nSet the stale-while-revalidate cache control directive."]
    pub fn stale_while_revalidate(&self) -> Ruleset_Rules_ActionParameters_StaleWhileRevalidateRef {
        Ruleset_Rules_ActionParameters_StaleWhileRevalidateRef::new(
            self.shared().clone(),
            format!("{}.stale_while_revalidate", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `status_code` after provisioning.\nThe status code to use for the error."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strip_etags` after provisioning.\nWhether to strip the ETag header from the response."]
    pub fn strip_etags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_etags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strip_last_modified` after provisioning.\nWhether to strip the Last-Modified header from the response."]
    pub fn strip_last_modified(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_last_modified", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strip_set_cookie` after provisioning.\nWhether to strip the Set-Cookie header from the response."]
    pub fn strip_set_cookie(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_set_cookie", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sxg` after provisioning.\nWhether to enable Signed Exchanges (SXG)."]
    pub fn sxg(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sxg", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transformed_request_fields` after provisioning.\nThe transformed request fields to log."]
    pub fn transformed_request_fields(
        &self,
    ) -> terrars::ListRef<Ruleset_Rules_ActionParameters_TransformedRequestFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transformed_request_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nA URI rewrite."]
    pub fn uri(&self) -> Ruleset_Rules_ActionParameters_UriRef {
        Ruleset_Rules_ActionParameters_UriRef::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nThe cache tag values for set_cache_tags action."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vary` after provisioning.\nControls how cached responses vary based on request headers. `default` is required and applies to any Vary response header that does not have a per-header override."]
    pub fn vary(&self) -> Ruleset_Rules_ActionParameters_VaryRef {
        Ruleset_Rules_ActionParameters_VaryRef::new(self.shared().clone(), format!("{}.vary", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_ExposedCredentialCheck {
    password_expression: terrars::PrimField<std::string::String>,
    username_expression: terrars::PrimField<std::string::String>,
}

impl Ruleset_Rules_ExposedCredentialCheck { }

impl terrars::ToListMappable for Ruleset_Rules_ExposedCredentialCheck {
    type O = terrars::BlockAssignable<Ruleset_Rules_ExposedCredentialCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_ExposedCredentialCheck {
    #[doc = "An expression that selects the password used in the credentials check."]
    pub password_expression: terrars::PrimField<std::string::String>,
    #[doc = "An expression that selects the user ID used in the credentials check."]
    pub username_expression: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules_ExposedCredentialCheck {
    pub fn build(self) -> Ruleset_Rules_ExposedCredentialCheck {
        Ruleset_Rules_ExposedCredentialCheck {
            password_expression: self.password_expression,
            username_expression: self.username_expression,
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_ExposedCredentialCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_ExposedCredentialCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_ExposedCredentialCheckRef {
        Ruleset_Rules_ExposedCredentialCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_ExposedCredentialCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `password_expression` after provisioning.\nAn expression that selects the password used in the credentials check."]
    pub fn password_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username_expression` after provisioning.\nAn expression that selects the user ID used in the credentials check."]
    pub fn username_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_Logging {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl Ruleset_Rules_Logging {
    #[doc = "Set the field `enabled`.\nWhether to generate a log when the rule matches."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_Logging {
    type O = terrars::BlockAssignable<Ruleset_Rules_Logging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_Logging {}

impl BuildRuleset_Rules_Logging {
    pub fn build(self) -> Ruleset_Rules_Logging {
        Ruleset_Rules_Logging { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_LoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_LoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_LoggingRef {
        Ruleset_Rules_LoggingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_LoggingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to generate a log when the rule matches."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules_Ratelimit {
    characteristics: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    counting_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mitigation_timeout: std::option::Option<terrars::PrimField<f64>>,
    period: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests_per_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests_to_origin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score_per_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score_response_header_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ruleset_Rules_Ratelimit {
    #[doc =
        "Set the field `counting_expression`.\nAn expression that defines when the rate limit counter should be incremented. It defaults to the same as the rule's expression."]
    pub fn set_counting_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.counting_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mitigation_timeout`.\nPeriod of time in seconds after which the action will be disabled following its first execution."]
    pub fn set_mitigation_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mitigation_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `requests_per_period`.\nThe threshold of requests per period after which the action will be executed for the first time."]
    pub fn set_requests_per_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.requests_per_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requests_to_origin`.\nWhether counting is only performed when an origin is reached."]
    pub fn set_requests_to_origin(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.requests_to_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `score_per_period`.\nThe score threshold per period for which the action will be executed the first time."]
    pub fn set_score_per_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.score_per_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `score_response_header_name`.\nA response header name provided by the origin, which contains the score to increment rate limit counter with."]
    pub fn set_score_response_header_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.score_response_header_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules_Ratelimit {
    type O = terrars::BlockAssignable<Ruleset_Rules_Ratelimit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules_Ratelimit {
    #[doc = "Characteristics of the request on which the rate limit counter will be incremented."]
    pub characteristics: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Period in seconds over which the counter is being incremented."]
    pub period: terrars::PrimField<f64>,
}

impl BuildRuleset_Rules_Ratelimit {
    pub fn build(self) -> Ruleset_Rules_Ratelimit {
        Ruleset_Rules_Ratelimit {
            characteristics: self.characteristics,
            counting_expression: core::default::Default::default(),
            mitigation_timeout: core::default::Default::default(),
            period: self.period,
            requests_per_period: core::default::Default::default(),
            requests_to_origin: core::default::Default::default(),
            score_per_period: core::default::Default::default(),
            score_response_header_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_Rules_RatelimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_Rules_RatelimitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_Rules_RatelimitRef {
        Ruleset_Rules_RatelimitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_Rules_RatelimitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `characteristics` after provisioning.\nCharacteristics of the request on which the rate limit counter will be incremented."]
    pub fn characteristics(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.characteristics", self.base))
    }

    #[doc =
        "Get a reference to the value of field `counting_expression` after provisioning.\nAn expression that defines when the rate limit counter should be incremented. It defaults to the same as the rule's expression."]
    pub fn counting_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.counting_expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mitigation_timeout` after provisioning.\nPeriod of time in seconds after which the action will be disabled following its first execution."]
    pub fn mitigation_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mitigation_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period` after provisioning.\nPeriod in seconds over which the counter is being incremented."]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requests_per_period` after provisioning.\nThe threshold of requests per period after which the action will be executed for the first time."]
    pub fn requests_per_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requests_per_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requests_to_origin` after provisioning.\nWhether counting is only performed when an origin is reached."]
    pub fn requests_to_origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requests_to_origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `score_per_period` after provisioning.\nThe score threshold per period for which the action will be executed the first time."]
    pub fn score_per_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_per_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `score_response_header_name` after provisioning.\nA response header name provided by the origin, which contains the score to increment rate limit counter with."]
    pub fn score_response_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_response_header_name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ruleset_Rules {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_parameters: std::option::Option<Ruleset_Rules_ActionParameters>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exposed_credential_check: std::option::Option<Ruleset_Rules_ExposedCredentialCheck>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<Ruleset_Rules_Logging>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ratelimit: std::option::Option<Ruleset_Rules_Ratelimit>,
    #[serde(rename = "ref", skip_serializing_if = "std::option::Option::is_none")]
    ref_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ruleset_Rules {
    #[doc = "Set the field `action_parameters`.\nThe parameters configuring the rule's action."]
    pub fn set_action_parameters(mut self, v: impl std::convert::Into<Ruleset_Rules_ActionParameters>) -> Self {
        self.action_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn informative description of the rule."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether the rule should be executed."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exposed_credential_check`.\nConfiguration for exposed credential checking."]
    pub fn set_exposed_credential_check(
        mut self,
        v: impl std::convert::Into<Ruleset_Rules_ExposedCredentialCheck>,
    ) -> Self {
        self.exposed_credential_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging`.\nAn object configuring the rule's logging behavior."]
    pub fn set_logging(mut self, v: impl std::convert::Into<Ruleset_Rules_Logging>) -> Self {
        self.logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ratelimit`.\nAn object configuring the rule's rate limit behavior."]
    pub fn set_ratelimit(mut self, v: impl std::convert::Into<Ruleset_Rules_Ratelimit>) -> Self {
        self.ratelimit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ref_`.\nThe reference of the rule (the rule's ID by default)."]
    pub fn set_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ref_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ruleset_Rules {
    type O = terrars::BlockAssignable<Ruleset_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRuleset_Rules {
    #[doc =
        "The action to perform when the rule matches.\nAvailable values: \"block\", \"challenge\", \"compress_response\", \"ddos_dynamic\", \"execute\", \"force_connection_close\", \"js_challenge\", \"log\", \"log_custom_field\", \"managed_challenge\", \"redirect\", \"rewrite\", \"route\", \"score\", \"serve_error\", \"set_cache_control\", \"set_cache_settings\", \"set_cache_tags\", \"set_config\", \"skip\"."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = "The expression defining which traffic will match the rule."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildRuleset_Rules {
    pub fn build(self) -> Ruleset_Rules {
        Ruleset_Rules {
            action: self.action,
            action_parameters: core::default::Default::default(),
            description: core::default::Default::default(),
            enabled: core::default::Default::default(),
            exposed_credential_check: core::default::Default::default(),
            expression: self.expression,
            logging: core::default::Default::default(),
            ratelimit: core::default::Default::default(),
            ref_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ruleset_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ruleset_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ruleset_RulesRef {
        Ruleset_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ruleset_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe action to perform when the rule matches.\nAvailable values: \"block\", \"challenge\", \"compress_response\", \"ddos_dynamic\", \"execute\", \"force_connection_close\", \"js_challenge\", \"log\", \"log_custom_field\", \"managed_challenge\", \"redirect\", \"rewrite\", \"route\", \"score\", \"serve_error\", \"set_cache_control\", \"set_cache_settings\", \"set_cache_tags\", \"set_config\", \"skip\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `action_parameters` after provisioning.\nThe parameters configuring the rule's action."]
    pub fn action_parameters(&self) -> Ruleset_Rules_ActionParametersRef {
        Ruleset_Rules_ActionParametersRef::new(self.shared().clone(), format!("{}.action_parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the rule should be executed."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exposed_credential_check` after provisioning.\nConfiguration for exposed credential checking."]
    pub fn exposed_credential_check(&self) -> Ruleset_Rules_ExposedCredentialCheckRef {
        Ruleset_Rules_ExposedCredentialCheckRef::new(
            self.shared().clone(),
            format!("{}.exposed_credential_check", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nThe expression defining which traffic will match the rule."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logging` after provisioning.\nAn object configuring the rule's logging behavior."]
    pub fn logging(&self) -> Ruleset_Rules_LoggingRef {
        Ruleset_Rules_LoggingRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ratelimit` after provisioning.\nAn object configuring the rule's rate limit behavior."]
    pub fn ratelimit(&self) -> Ruleset_Rules_RatelimitRef {
        Ruleset_Rules_RatelimitRef::new(self.shared().clone(), format!("{}.ratelimit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ref_` after provisioning.\nThe reference of the rule (the rule's ID by default)."]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }
}

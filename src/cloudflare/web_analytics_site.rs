#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct WebAnalyticsSiteData_ {
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
    auto_install: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lite: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct WebAnalyticsSite_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WebAnalyticsSiteData_>,
}

#[derive(Clone)]
pub struct WebAnalyticsSite(std::rc::Rc<WebAnalyticsSite_>);

impl WebAnalyticsSite {
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

    #[doc =
        "Set the field `auto_install`.\nIf enabled, the JavaScript snippet is automatically injected for orange-clouded sites."]
    pub fn set_auto_install(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_install = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enabled`.\nEnables or disables RUM. This option can be used only when auto_install is set to true."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\nThe hostname to use for gray-clouded sites."]
    pub fn set_host(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lite`.\nIf enabled, the JavaScript snippet will not be injected for visitors from the EU."]
    pub fn set_lite(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().lite = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_tag`.\nThe zone identifier."]
    pub fn set_zone_tag(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_install` after provisioning.\nIf enabled, the JavaScript snippet is automatically injected for orange-clouded sites."]
    pub fn auto_install(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_install", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnables or disables RUM. This option can be used only when auto_install is set to true."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe hostname to use for gray-clouded sites."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe Web Analytics site identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lite` after provisioning.\nIf enabled, the JavaScript snippet will not be injected for visitors from the EU."]
    pub fn lite(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lite", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nA list of rules."]
    pub fn rules(&self) -> terrars::ListRef<WebAnalyticsSite_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ruleset` after provisioning.\n"]
    pub fn ruleset(&self) -> WebAnalyticsSite_RulesetRef {
        WebAnalyticsSite_RulesetRef::new(self.shared().clone(), format!("{}.ruleset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_tag` after provisioning.\nThe Web Analytics site identifier."]
    pub fn site_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_token` after provisioning.\nThe Web Analytics site token."]
    pub fn site_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snippet` after provisioning.\nEncoded JavaScript snippet."]
    pub fn snippet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snippet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_tag` after provisioning.\nThe zone identifier."]
    pub fn zone_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_tag", self.extract_ref()))
    }
}

impl terrars::Referable for WebAnalyticsSite {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WebAnalyticsSite { }

impl terrars::ToListMappable for WebAnalyticsSite {
    type O = terrars::ListRef<WebAnalyticsSiteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WebAnalyticsSite_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_web_analytics_site".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWebAnalyticsSite {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildWebAnalyticsSite {
    pub fn build(self, stack: &mut terrars::Stack) -> WebAnalyticsSite {
        let out = WebAnalyticsSite(std::rc::Rc::new(WebAnalyticsSite_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WebAnalyticsSiteData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                auto_install: core::default::Default::default(),
                enabled: core::default::Default::default(),
                host: core::default::Default::default(),
                lite: core::default::Default::default(),
                zone_tag: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WebAnalyticsSiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WebAnalyticsSiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WebAnalyticsSiteRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_install` after provisioning.\nIf enabled, the JavaScript snippet is automatically injected for orange-clouded sites."]
    pub fn auto_install(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_install", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnables or disables RUM. This option can be used only when auto_install is set to true."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe hostname to use for gray-clouded sites."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe Web Analytics site identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lite` after provisioning.\nIf enabled, the JavaScript snippet will not be injected for visitors from the EU."]
    pub fn lite(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lite", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nA list of rules."]
    pub fn rules(&self) -> terrars::ListRef<WebAnalyticsSite_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ruleset` after provisioning.\n"]
    pub fn ruleset(&self) -> WebAnalyticsSite_RulesetRef {
        WebAnalyticsSite_RulesetRef::new(self.shared().clone(), format!("{}.ruleset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_tag` after provisioning.\nThe Web Analytics site identifier."]
    pub fn site_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_token` after provisioning.\nThe Web Analytics site token."]
    pub fn site_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snippet` after provisioning.\nEncoded JavaScript snippet."]
    pub fn snippet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snippet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_tag` after provisioning.\nThe zone identifier."]
    pub fn zone_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_tag", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WebAnalyticsSite_Rules {}

impl WebAnalyticsSite_Rules { }

impl terrars::ToListMappable for WebAnalyticsSite_Rules {
    type O = terrars::BlockAssignable<WebAnalyticsSite_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWebAnalyticsSite_Rules {}

impl BuildWebAnalyticsSite_Rules {
    pub fn build(self) -> WebAnalyticsSite_Rules {
        WebAnalyticsSite_Rules {}
    }
}

#[allow(dead_code)]
pub struct WebAnalyticsSite_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WebAnalyticsSite_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WebAnalyticsSite_RulesRef {
        WebAnalyticsSite_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WebAnalyticsSite_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe hostname the rule will be applied to."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe Web Analytics rule identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inclusive` after provisioning.\nWhether the rule includes or excludes traffic from being measured."]
    pub fn inclusive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `is_paused` after provisioning.\nWhether the rule is paused or not."]
    pub fn is_paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_paused", self.base))
    }

    #[doc = "Get a reference to the value of field `paths` after provisioning.\nThe paths the rule will be applied to."]
    pub fn paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
}

#[derive(Serialize)]
pub struct WebAnalyticsSite_Ruleset {}

impl WebAnalyticsSite_Ruleset { }

impl terrars::ToListMappable for WebAnalyticsSite_Ruleset {
    type O = terrars::BlockAssignable<WebAnalyticsSite_Ruleset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWebAnalyticsSite_Ruleset {}

impl BuildWebAnalyticsSite_Ruleset {
    pub fn build(self) -> WebAnalyticsSite_Ruleset {
        WebAnalyticsSite_Ruleset {}
    }
}

#[allow(dead_code)]
pub struct WebAnalyticsSite_RulesetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WebAnalyticsSite_RulesetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WebAnalyticsSite_RulesetRef {
        WebAnalyticsSite_RulesetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WebAnalyticsSite_RulesetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the ruleset is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe Web Analytics ruleset identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_name` after provisioning.\n"]
    pub fn zone_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_name", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_tag` after provisioning.\nThe zone identifier."]
    pub fn zone_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_tag", self.base))
    }
}

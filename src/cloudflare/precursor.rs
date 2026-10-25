#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct PrecursorData_ {
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
    default_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforcement_rules: std::option::Option<std::vec::Vec<Precursor_EnforcementRules>>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct Precursor_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PrecursorData_>,
}

#[derive(Clone)]
pub struct Precursor(std::rc::Rc<Precursor_>);

impl Precursor {
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
        "Set the field `default_mode`.\nThe zone-level Precursor enforcement mode applied to requests that do\nnot match a more specific enforcement rule.\nAvailable values: \"off\", \"min-friction\", \"max-security\"."]
    pub fn set_default_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforcement_rules`.\nThe ordered list of enforcement rules for the zone."]
    pub fn set_enforcement_rules(self, v: impl std::convert::Into<std::vec::Vec<Precursor_EnforcementRules>>) -> Self {
        self.0.data.borrow_mut().enforcement_rules = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `default_mode` after provisioning.\nThe zone-level Precursor enforcement mode applied to requests that do\nnot match a more specific enforcement rule.\nAvailable values: \"off\", \"min-friction\", \"max-security\"."]
    pub fn default_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enforcement_rules` after provisioning.\nThe ordered list of enforcement rules for the zone."]
    pub fn enforcement_rules(&self) -> terrars::ListRef<Precursor_EnforcementRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enforcement_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for Precursor {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Precursor { }

impl terrars::ToListMappable for Precursor {
    type O = terrars::ListRef<PrecursorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Precursor_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_precursor".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPrecursor {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildPrecursor {
    pub fn build(self, stack: &mut terrars::Stack) -> Precursor {
        let out = Precursor(std::rc::Rc::new(Precursor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PrecursorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_mode: core::default::Default::default(),
                enforcement_rules: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PrecursorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrecursorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PrecursorRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_mode` after provisioning.\nThe zone-level Precursor enforcement mode applied to requests that do\nnot match a more specific enforcement rule.\nAvailable values: \"off\", \"min-friction\", \"max-security\"."]
    pub fn default_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enforcement_rules` after provisioning.\nThe ordered list of enforcement rules for the zone."]
    pub fn enforcement_rules(&self) -> terrars::ListRef<Precursor_EnforcementRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enforcement_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Precursor_EnforcementRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    expression: terrars::PrimField<std::string::String>,
    mode: terrars::PrimField<std::string::String>,
}

impl Precursor_EnforcementRules {
    #[doc = "Set the field `description`.\nAn informative description of the rule."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether the rule is active."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Precursor_EnforcementRules {
    type O = terrars::BlockAssignable<Precursor_EnforcementRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrecursor_EnforcementRules {
    #[doc = "The filter expression that determines which requests the rule matches."]
    pub expression: terrars::PrimField<std::string::String>,
    #[doc =
        "The override mode Precursor applies to requests matching an enforcement\nrule. Unlike `default_mode`, this cannot be `off`.\nAvailable values: \"min-friction\", \"max-security\"."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildPrecursor_EnforcementRules {
    pub fn build(self) -> Precursor_EnforcementRules {
        Precursor_EnforcementRules {
            description: core::default::Default::default(),
            enabled: core::default::Default::default(),
            expression: self.expression,
            mode: self.mode,
        }
    }
}

#[allow(dead_code)]
pub struct Precursor_EnforcementRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Precursor_EnforcementRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Precursor_EnforcementRulesRef {
        Precursor_EnforcementRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Precursor_EnforcementRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the rule is active."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nThe filter expression that determines which requests the rule matches."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe read-only identifier that Cloudflare assigns to the rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe override mode Precursor applies to requests matching an enforcement\nrule. Unlike `default_mode`, this cannot be `off`.\nAvailable values: \"min-friction\", \"max-security\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

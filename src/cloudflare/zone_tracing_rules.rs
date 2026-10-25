#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZoneTracingRulesData_ {
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
    rules: std::vec::Vec<ZoneTracingRules_Rules>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct ZoneTracingRules_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZoneTracingRulesData_>,
}

#[derive(Clone)]
pub struct ZoneTracingRules(std::rc::Rc<ZoneTracingRules_>);

impl ZoneTracingRules {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\nSpecify the zone ID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nTrace rules in evaluation order."]
    pub fn rules(&self) -> terrars::ListRef<ZoneTracingRules_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nSpecify the zone ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for ZoneTracingRules {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZoneTracingRules { }

impl terrars::ToListMappable for ZoneTracingRules {
    type O = terrars::ListRef<ZoneTracingRulesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZoneTracingRules_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zone_tracing_rules".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZoneTracingRules {
    pub tf_id: std::string::String,
    #[doc = "Trace rules in evaluation order."]
    pub rules: std::vec::Vec<ZoneTracingRules_Rules>,
    #[doc = "Specify the zone ID."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildZoneTracingRules {
    pub fn build(self, stack: &mut terrars::Stack) -> ZoneTracingRules {
        let out = ZoneTracingRules(std::rc::Rc::new(ZoneTracingRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZoneTracingRulesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                rules: self.rules,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZoneTracingRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneTracingRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZoneTracingRulesRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nSpecify the zone ID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nTrace rules in evaluation order."]
    pub fn rules(&self) -> terrars::ListRef<ZoneTracingRules_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nSpecify the zone ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZoneTracingRules_Rules_ActionParameters {
    sampling_ratio: terrars::PrimField<f64>,
}

impl ZoneTracingRules_Rules_ActionParameters { }

impl terrars::ToListMappable for ZoneTracingRules_Rules_ActionParameters {
    type O = terrars::BlockAssignable<ZoneTracingRules_Rules_ActionParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneTracingRules_Rules_ActionParameters {
    #[doc = "The ratio of requests sampled for tracing, from 0 to 1."]
    pub sampling_ratio: terrars::PrimField<f64>,
}

impl BuildZoneTracingRules_Rules_ActionParameters {
    pub fn build(self) -> ZoneTracingRules_Rules_ActionParameters {
        ZoneTracingRules_Rules_ActionParameters { sampling_ratio: self.sampling_ratio }
    }
}

#[allow(dead_code)]
pub struct ZoneTracingRules_Rules_ActionParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneTracingRules_Rules_ActionParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZoneTracingRules_Rules_ActionParametersRef {
        ZoneTracingRules_Rules_ActionParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneTracingRules_Rules_ActionParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sampling_ratio` after provisioning.\nThe ratio of requests sampled for tracing, from 0 to 1."]
    pub fn sampling_ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sampling_ratio", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneTracingRules_Rules {
    action: terrars::PrimField<std::string::String>,
    action_parameters: ZoneTracingRules_Rules_ActionParameters,
    description: terrars::PrimField<std::string::String>,
    enabled: terrars::PrimField<bool>,
    expression: terrars::PrimField<std::string::String>,
}

impl ZoneTracingRules_Rules { }

impl terrars::ToListMappable for ZoneTracingRules_Rules {
    type O = terrars::BlockAssignable<ZoneTracingRules_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneTracingRules_Rules {
    #[doc = "Available values: \"set_trace_settings\"."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub action_parameters: ZoneTracingRules_Rules_ActionParameters,
    #[doc = ""]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "A Rules language expression that selects requests."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildZoneTracingRules_Rules {
    pub fn build(self) -> ZoneTracingRules_Rules {
        ZoneTracingRules_Rules {
            action: self.action,
            action_parameters: self.action_parameters,
            description: self.description,
            enabled: self.enabled,
            expression: self.expression,
        }
    }
}

#[allow(dead_code)]
pub struct ZoneTracingRules_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneTracingRules_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZoneTracingRules_RulesRef {
        ZoneTracingRules_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneTracingRules_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAvailable values: \"set_trace_settings\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `action_parameters` after provisioning.\n"]
    pub fn action_parameters(&self) -> ZoneTracingRules_Rules_ActionParametersRef {
        ZoneTracingRules_Rules_ActionParametersRef::new(
            self.shared().clone(),
            format!("{}.action_parameters", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nA Rules language expression that selects requests."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}

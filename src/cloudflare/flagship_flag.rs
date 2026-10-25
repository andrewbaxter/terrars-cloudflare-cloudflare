#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct FlagshipFlagData_ {
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
    app_id: terrars::PrimField<std::string::String>,
    default_variation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    key: terrars::PrimField<std::string::String>,
    rules: std::vec::Vec<FlagshipFlag_Rules>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    variations: terrars::RecField<terrars::PrimField<std::string::String>>,
}

struct FlagshipFlag_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FlagshipFlagData_>,
}

#[derive(Clone)]
pub struct FlagshipFlag(std::rc::Rc<FlagshipFlag_>);

impl FlagshipFlag {
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

    #[doc = "Set the field `description`.\nOptional operator-facing description. It does not affect flag evaluation."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nDeprecated compatibility field. Omit it; the API ignores this value and infers the type from the flag's variations.\nAvailable values: \"boolean\", \"string\", \"number\", \"json\"."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID that owns the Flagship app."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_id` after provisioning.\nFlagship app ID returned when the app was created."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_variation` after provisioning.\nVariation served when no rule matches or the flag is disabled. Must be a key in `variations`."]
    pub fn default_variation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_variation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional operator-facing description. It does not affect flag evaluation."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhen false, the flag bypasses all rules and always serves `default_variation`."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the flag within an app. Used in all evaluation and SDK calls."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nUnique identifier for the flag within an app. Used in all evaluation and SDK calls."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nTargeting rules evaluated in ascending `priority`; the first matching rule wins. An empty array means the flag always serves `default_variation`."]
    pub fn rules(&self) -> terrars::ListRef<FlagshipFlag_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nDeprecated compatibility field. Omit it; the API ignores this value and infers the type from the flag's variations.\nAvailable values: \"boolean\", \"string\", \"number\", \"json\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_by` after provisioning.\n"]
    pub fn updated_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `variations` after provisioning.\nMap of variation name to value. All values must be the same type (boolean, string, number, or JSON object/array). Each serialized value must be 10KB or smaller."]
    pub fn variations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.variations", self.extract_ref()))
    }
}

impl terrars::Referable for FlagshipFlag {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FlagshipFlag { }

impl terrars::ToListMappable for FlagshipFlag {
    type O = terrars::ListRef<FlagshipFlagRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FlagshipFlag_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_flagship_flag".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFlagshipFlag {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account ID that owns the Flagship app."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Flagship app ID returned when the app was created."]
    pub app_id: terrars::PrimField<std::string::String>,
    #[doc = "Variation served when no rule matches or the flag is disabled. Must be a key in `variations`."]
    pub default_variation: terrars::PrimField<std::string::String>,
    #[doc = "When false, the flag bypasses all rules and always serves `default_variation`."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "Unique identifier for the flag within an app. Used in all evaluation and SDK calls."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc =
        "Targeting rules evaluated in ascending `priority`; the first matching rule wins. An empty array means the flag always serves `default_variation`."]
    pub rules: std::vec::Vec<FlagshipFlag_Rules>,
    #[doc =
        "Map of variation name to value. All values must be the same type (boolean, string, number, or JSON object/array). Each serialized value must be 10KB or smaller."]
    pub variations: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildFlagshipFlag {
    pub fn build(self, stack: &mut terrars::Stack) -> FlagshipFlag {
        let out = FlagshipFlag(std::rc::Rc::new(FlagshipFlag_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FlagshipFlagData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                app_id: self.app_id,
                default_variation: self.default_variation,
                description: core::default::Default::default(),
                enabled: self.enabled,
                key: self.key,
                rules: self.rules,
                type_: core::default::Default::default(),
                variations: self.variations,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FlagshipFlagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FlagshipFlagRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FlagshipFlagRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID that owns the Flagship app."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_id` after provisioning.\nFlagship app ID returned when the app was created."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_variation` after provisioning.\nVariation served when no rule matches or the flag is disabled. Must be a key in `variations`."]
    pub fn default_variation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_variation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional operator-facing description. It does not affect flag evaluation."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhen false, the flag bypasses all rules and always serves `default_variation`."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the flag within an app. Used in all evaluation and SDK calls."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nUnique identifier for the flag within an app. Used in all evaluation and SDK calls."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nTargeting rules evaluated in ascending `priority`; the first matching rule wins. An empty array means the flag always serves `default_variation`."]
    pub fn rules(&self) -> terrars::ListRef<FlagshipFlag_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nDeprecated compatibility field. Omit it; the API ignores this value and infers the type from the flag's variations.\nAvailable values: \"boolean\", \"string\", \"number\", \"json\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_by` after provisioning.\n"]
    pub fn updated_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `variations` after provisioning.\nMap of variation name to value. All values must be the same type (boolean, string, number, or JSON object/array). Each serialized value must be 10KB or smaller."]
    pub fn variations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.variations", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clauses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logical_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `clauses`.\n"]
    pub fn set_clauses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.clauses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logical_operator`.\nAvailable values: \"AND\", \"OR\"."]
    pub fn set_logical_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logical_operator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `operator`.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `value`.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses {
    type O = terrars::BlockAssignable<FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses {}

impl BuildFlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses {
    pub fn build(self) -> FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses {
        FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses {
            attribute: core::default::Default::default(),
            clauses: core::default::Default::default(),
            logical_operator: core::default::Default::default(),
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_ClausesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_ClausesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_ClausesRef {
        FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_ClausesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_ClausesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `clauses` after provisioning.\n"]
    pub fn clauses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clauses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logical_operator` after provisioning.\nAvailable values: \"AND\", \"OR\"."]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clauses: std::option::Option<std::vec::Vec<FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logical_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `clauses`.\n"]
    pub fn set_clauses(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<std::vec::Vec<FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses>>,
    ) -> Self {
        self.clauses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logical_operator`.\nAvailable values: \"AND\", \"OR\"."]
    pub fn set_logical_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logical_operator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `operator`.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `value`.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses {
    type O = terrars::BlockAssignable<FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses {}

impl BuildFlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses {
    pub fn build(self) -> FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses {
        FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses {
            attribute: core::default::Default::default(),
            clauses: core::default::Default::default(),
            logical_operator: core::default::Default::default(),
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_ClausesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_ClausesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_ClausesRef {
        FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_ClausesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_ClausesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `clauses` after provisioning.\n"]
    pub fn clauses(&self) -> terrars::ListRef<FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_ClausesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clauses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logical_operator` after provisioning.\nAvailable values: \"AND\", \"OR\"."]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clauses: std::option::Option<std::vec::Vec<FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logical_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `clauses`.\n"]
    pub fn set_clauses(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_Clauses>>,
    ) -> Self {
        self.clauses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logical_operator`.\nAvailable values: \"AND\", \"OR\"."]
    pub fn set_logical_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logical_operator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `operator`.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `value`.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses {
    type O = terrars::BlockAssignable<FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses {}

impl BuildFlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses {
    pub fn build(self) -> FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses {
        FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses {
            attribute: core::default::Default::default(),
            clauses: core::default::Default::default(),
            logical_operator: core::default::Default::default(),
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FlagshipFlag_Rules_Conditions_Clauses_Clauses_ClausesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FlagshipFlag_Rules_Conditions_Clauses_Clauses_ClausesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FlagshipFlag_Rules_Conditions_Clauses_Clauses_ClausesRef {
        FlagshipFlag_Rules_Conditions_Clauses_Clauses_ClausesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FlagshipFlag_Rules_Conditions_Clauses_Clauses_ClausesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `clauses` after provisioning.\n"]
    pub fn clauses(&self) -> terrars::ListRef<FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses_ClausesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clauses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logical_operator` after provisioning.\nAvailable values: \"AND\", \"OR\"."]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FlagshipFlag_Rules_Conditions_Clauses_Clauses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clauses: std::option::Option<std::vec::Vec<FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logical_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FlagshipFlag_Rules_Conditions_Clauses_Clauses {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `clauses`.\n"]
    pub fn set_clauses(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<FlagshipFlag_Rules_Conditions_Clauses_Clauses_Clauses>>,
    ) -> Self {
        self.clauses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logical_operator`.\nAvailable values: \"AND\", \"OR\"."]
    pub fn set_logical_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logical_operator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `operator`.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `value`.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FlagshipFlag_Rules_Conditions_Clauses_Clauses {
    type O = terrars::BlockAssignable<FlagshipFlag_Rules_Conditions_Clauses_Clauses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFlagshipFlag_Rules_Conditions_Clauses_Clauses {}

impl BuildFlagshipFlag_Rules_Conditions_Clauses_Clauses {
    pub fn build(self) -> FlagshipFlag_Rules_Conditions_Clauses_Clauses {
        FlagshipFlag_Rules_Conditions_Clauses_Clauses {
            attribute: core::default::Default::default(),
            clauses: core::default::Default::default(),
            logical_operator: core::default::Default::default(),
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FlagshipFlag_Rules_Conditions_Clauses_ClausesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FlagshipFlag_Rules_Conditions_Clauses_ClausesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FlagshipFlag_Rules_Conditions_Clauses_ClausesRef {
        FlagshipFlag_Rules_Conditions_Clauses_ClausesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FlagshipFlag_Rules_Conditions_Clauses_ClausesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `clauses` after provisioning.\n"]
    pub fn clauses(&self) -> terrars::ListRef<FlagshipFlag_Rules_Conditions_Clauses_Clauses_ClausesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clauses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logical_operator` after provisioning.\nAvailable values: \"AND\", \"OR\"."]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FlagshipFlag_Rules_Conditions_Clauses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clauses: std::option::Option<std::vec::Vec<FlagshipFlag_Rules_Conditions_Clauses_Clauses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logical_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FlagshipFlag_Rules_Conditions_Clauses {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `clauses`.\n"]
    pub fn set_clauses(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<FlagshipFlag_Rules_Conditions_Clauses_Clauses>>,
    ) -> Self {
        self.clauses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logical_operator`.\nAvailable values: \"AND\", \"OR\"."]
    pub fn set_logical_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logical_operator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `operator`.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `value`.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FlagshipFlag_Rules_Conditions_Clauses {
    type O = terrars::BlockAssignable<FlagshipFlag_Rules_Conditions_Clauses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFlagshipFlag_Rules_Conditions_Clauses {}

impl BuildFlagshipFlag_Rules_Conditions_Clauses {
    pub fn build(self) -> FlagshipFlag_Rules_Conditions_Clauses {
        FlagshipFlag_Rules_Conditions_Clauses {
            attribute: core::default::Default::default(),
            clauses: core::default::Default::default(),
            logical_operator: core::default::Default::default(),
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FlagshipFlag_Rules_Conditions_ClausesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FlagshipFlag_Rules_Conditions_ClausesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FlagshipFlag_Rules_Conditions_ClausesRef {
        FlagshipFlag_Rules_Conditions_ClausesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FlagshipFlag_Rules_Conditions_ClausesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `clauses` after provisioning.\n"]
    pub fn clauses(&self) -> terrars::ListRef<FlagshipFlag_Rules_Conditions_Clauses_ClausesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clauses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logical_operator` after provisioning.\nAvailable values: \"AND\", \"OR\"."]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FlagshipFlag_Rules_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clauses: std::option::Option<std::vec::Vec<FlagshipFlag_Rules_Conditions_Clauses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logical_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FlagshipFlag_Rules_Conditions {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `clauses`.\n"]
    pub fn set_clauses(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<FlagshipFlag_Rules_Conditions_Clauses>>,
    ) -> Self {
        self.clauses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logical_operator`.\nAvailable values: \"AND\", \"OR\"."]
    pub fn set_logical_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logical_operator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `operator`.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `value`.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FlagshipFlag_Rules_Conditions {
    type O = terrars::BlockAssignable<FlagshipFlag_Rules_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFlagshipFlag_Rules_Conditions {}

impl BuildFlagshipFlag_Rules_Conditions {
    pub fn build(self) -> FlagshipFlag_Rules_Conditions {
        FlagshipFlag_Rules_Conditions {
            attribute: core::default::Default::default(),
            clauses: core::default::Default::default(),
            logical_operator: core::default::Default::default(),
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FlagshipFlag_Rules_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FlagshipFlag_Rules_ConditionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FlagshipFlag_Rules_ConditionsRef {
        FlagshipFlag_Rules_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FlagshipFlag_Rules_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `clauses` after provisioning.\n"]
    pub fn clauses(&self) -> terrars::ListRef<FlagshipFlag_Rules_Conditions_ClausesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clauses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logical_operator` after provisioning.\nAvailable values: \"AND\", \"OR\"."]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FlagshipFlag_Rules_Rollout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    percentage: terrars::PrimField<f64>,
}

impl FlagshipFlag_Rules_Rollout {
    #[doc =
        "Set the field `attribute`.\nContext attribute used for sticky bucketing. Defaults to `targetingKey`. If absent at evaluation time, bucketing is random per request."]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FlagshipFlag_Rules_Rollout {
    type O = terrars::BlockAssignable<FlagshipFlag_Rules_Rollout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFlagshipFlag_Rules_Rollout {
    #[doc =
        "Percentage of matching traffic (0–100, up to 2 decimal places) served this variation. For multi-way splits, use cumulative upper bounds across rules (e.g. 30, 70, 100)."]
    pub percentage: terrars::PrimField<f64>,
}

impl BuildFlagshipFlag_Rules_Rollout {
    pub fn build(self) -> FlagshipFlag_Rules_Rollout {
        FlagshipFlag_Rules_Rollout {
            attribute: core::default::Default::default(),
            percentage: self.percentage,
        }
    }
}

#[allow(dead_code)]
pub struct FlagshipFlag_Rules_RolloutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FlagshipFlag_Rules_RolloutRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FlagshipFlag_Rules_RolloutRef {
        FlagshipFlag_Rules_RolloutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FlagshipFlag_Rules_RolloutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute` after provisioning.\nContext attribute used for sticky bucketing. Defaults to `targetingKey`. If absent at evaluation time, bucketing is random per request."]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nPercentage of matching traffic (0–100, up to 2 decimal places) served this variation. For multi-way splits, use cumulative upper bounds across rules (e.g. 30, 70, 100)."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct FlagshipFlag_Rules {
    conditions: std::vec::Vec<FlagshipFlag_Rules_Conditions>,
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout: std::option::Option<FlagshipFlag_Rules_Rollout>,
    serve_variation: terrars::PrimField<std::string::String>,
}

impl FlagshipFlag_Rules {
    #[doc = "Set the field `rollout`.\n"]
    pub fn set_rollout(mut self, v: impl std::convert::Into<FlagshipFlag_Rules_Rollout>) -> Self {
        self.rollout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FlagshipFlag_Rules {
    type O = terrars::BlockAssignable<FlagshipFlag_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFlagshipFlag_Rules {
    #[doc = "Conditions the context must satisfy for this rule to match. An empty array matches all contexts."]
    pub conditions: std::vec::Vec<FlagshipFlag_Rules_Conditions>,
    #[doc = "Evaluation order; lower numbers are evaluated first. Must be unique across the flag's rules."]
    pub priority: terrars::PrimField<f64>,
    #[doc = "Variation served when this rule matches. Must be a key in `variations`."]
    pub serve_variation: terrars::PrimField<std::string::String>,
}

impl BuildFlagshipFlag_Rules {
    pub fn build(self) -> FlagshipFlag_Rules {
        FlagshipFlag_Rules {
            conditions: self.conditions,
            priority: self.priority,
            rollout: core::default::Default::default(),
            serve_variation: self.serve_variation,
        }
    }
}

#[allow(dead_code)]
pub struct FlagshipFlag_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FlagshipFlag_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FlagshipFlag_RulesRef {
        FlagshipFlag_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FlagshipFlag_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `conditions` after provisioning.\nConditions the context must satisfy for this rule to match. An empty array matches all contexts."]
    pub fn conditions(&self) -> terrars::ListRef<FlagshipFlag_Rules_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nEvaluation order; lower numbers are evaluated first. Must be unique across the flag's rules."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(&self) -> FlagshipFlag_Rules_RolloutRef {
        FlagshipFlag_Rules_RolloutRef::new(self.shared().clone(), format!("{}.rollout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serve_variation` after provisioning.\nVariation served when this rule matches. Must be a key in `variations`."]
    pub fn serve_variation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serve_variation", self.base))
    }
}

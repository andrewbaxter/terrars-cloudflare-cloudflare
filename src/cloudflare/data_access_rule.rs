#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAccessRuleData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataAccessRule_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAccessRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccessRuleData_>,
}

#[derive(Clone)]
pub struct DataAccessRule(std::rc::Rc<DataAccessRule_>);

impl DataAccessRule {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderCloudflare) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        "Set the field `account_id`.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataAccessRule_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_id`.\nUnique identifier for a rule."]
    pub fn set_rule_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rule_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_modes` after provisioning.\nThe available actions that a rule can apply to a matched request."]
    pub fn allowed_modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_modes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\nThe rule configuration."]
    pub fn configuration(&self) -> DataAccessRule_ConfigurationRef {
        DataAccessRule_ConfigurationRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe timestamp of when the rule was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataAccessRule_FilterRef {
        DataAccessRule_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for a rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe action to apply to a matched request.\nAvailable values: \"block\", \"challenge\", \"whitelist\", \"js_challenge\", \"managed_challenge\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe timestamp of when the rule was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notes` after provisioning.\nAn informative summary of the rule, typically used as a reminder or explanation."]
    pub fn notes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nUnique identifier for a rule."]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nAll zones owned by the user will have the rule applied."]
    pub fn scope(&self) -> DataAccessRule_ScopeRef {
        DataAccessRule_ScopeRef::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccessRule {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccessRule { }

impl terrars::ToListMappable for DataAccessRule {
    type O = terrars::ListRef<DataAccessRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccessRule_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_access_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccessRule {
    pub tf_id: std::string::String,
}

impl BuildDataAccessRule {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccessRule {
        let out = DataAccessRule(std::rc::Rc::new(DataAccessRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAccessRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                rule_id: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccessRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccessRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccessRuleRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_modes` after provisioning.\nThe available actions that a rule can apply to a matched request."]
    pub fn allowed_modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_modes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\nThe rule configuration."]
    pub fn configuration(&self) -> DataAccessRule_ConfigurationRef {
        DataAccessRule_ConfigurationRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe timestamp of when the rule was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataAccessRule_FilterRef {
        DataAccessRule_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for a rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe action to apply to a matched request.\nAvailable values: \"block\", \"challenge\", \"whitelist\", \"js_challenge\", \"managed_challenge\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe timestamp of when the rule was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notes` after provisioning.\nAn informative summary of the rule, typically used as a reminder or explanation."]
    pub fn notes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nUnique identifier for a rule."]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nAll zones owned by the user will have the rule applied."]
    pub fn scope(&self) -> DataAccessRule_ScopeRef {
        DataAccessRule_ScopeRef::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccessRule_Configuration {}

impl DataAccessRule_Configuration { }

impl terrars::ToListMappable for DataAccessRule_Configuration {
    type O = terrars::BlockAssignable<DataAccessRule_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccessRule_Configuration {}

impl BuildDataAccessRule_Configuration {
    pub fn build(self) -> DataAccessRule_Configuration {
        DataAccessRule_Configuration {}
    }
}

#[allow(dead_code)]
pub struct DataAccessRule_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccessRule_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccessRule_ConfigurationRef {
        DataAccessRule_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccessRule_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nThe configuration target. You must set the target to `ip` when specifying an IP address in the rule.\nAvailable values: \"ip\", \"ip6\", \"ip_range\", \"asn\", \"country\"."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe IP address to match. This address will be compared to the IP address of incoming requests."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccessRule_Filter_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAccessRule_Filter_Configuration {
    #[doc =
        "Set the field `target`.\nDefines the target to search in existing rules.\nAvailable values: \"ip\", \"ip_range\", \"asn\", \"country\"."]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `value`.\nDefines the target value to search for in existing rules: an IP address, an IP address range, or a country code, depending on the provided `configuration.target`.\nNotes: You can search for a single IPv4 address, an IP address range with a subnet of '/16' or '/24', or a two-letter ISO-3166-1 alpha-2 country code."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccessRule_Filter_Configuration {
    type O = terrars::BlockAssignable<DataAccessRule_Filter_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccessRule_Filter_Configuration {}

impl BuildDataAccessRule_Filter_Configuration {
    pub fn build(self) -> DataAccessRule_Filter_Configuration {
        DataAccessRule_Filter_Configuration {
            target: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccessRule_Filter_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccessRule_Filter_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccessRule_Filter_ConfigurationRef {
        DataAccessRule_Filter_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccessRule_Filter_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nDefines the target to search in existing rules.\nAvailable values: \"ip\", \"ip_range\", \"asn\", \"country\"."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nDefines the target value to search for in existing rules: an IP address, an IP address range, or a country code, depending on the provided `configuration.target`.\nNotes: You can search for a single IPv4 address, an IP address range with a subnet of '/16' or '/24', or a two-letter ISO-3166-1 alpha-2 country code."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccessRule_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<DataAccessRule_Filter_Configuration>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAccessRule_Filter {
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(mut self, v: impl std::convert::Into<DataAccessRule_Filter_Configuration>) -> Self {
        self.configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `direction`.\nDefines the direction used to sort returned rules.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `match_`.\nDefines the search requirements. When set to `all`, all the search requirements must match. When set to `any`, only one of the search requirements has to match.\nAvailable values: \"any\", \"all\"."]
    pub fn set_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nThe action to apply to a matched request.\nAvailable values: \"block\", \"challenge\", \"whitelist\", \"js_challenge\", \"managed_challenge\"."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `notes`.\nDefines the string to search for in the notes of existing IP Access rules.\nNotes: For example, the string 'attack' would match IP Access rules with notes 'Attack 26/02' and 'Attack 27/02'. The search is case insensitive."]
    pub fn set_notes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.notes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nDefines the field used to sort returned rules.\nAvailable values: \"configuration.target\", \"configuration.value\", \"mode\"."]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccessRule_Filter {
    type O = terrars::BlockAssignable<DataAccessRule_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccessRule_Filter {}

impl BuildDataAccessRule_Filter {
    pub fn build(self) -> DataAccessRule_Filter {
        DataAccessRule_Filter {
            configuration: core::default::Default::default(),
            direction: core::default::Default::default(),
            match_: core::default::Default::default(),
            mode: core::default::Default::default(),
            notes: core::default::Default::default(),
            order: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccessRule_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccessRule_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccessRule_FilterRef {
        DataAccessRule_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccessRule_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> DataAccessRule_Filter_ConfigurationRef {
        DataAccessRule_Filter_ConfigurationRef::new(self.shared().clone(), format!("{}.configuration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDefines the direction used to sort returned rules.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nDefines the search requirements. When set to `all`, all the search requirements must match. When set to `any`, only one of the search requirements has to match.\nAvailable values: \"any\", \"all\"."]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe action to apply to a matched request.\nAvailable values: \"block\", \"challenge\", \"whitelist\", \"js_challenge\", \"managed_challenge\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `notes` after provisioning.\nDefines the string to search for in the notes of existing IP Access rules.\nNotes: For example, the string 'attack' would match IP Access rules with notes 'Attack 26/02' and 'Attack 27/02'. The search is case insensitive."]
    pub fn notes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nDefines the field used to sort returned rules.\nAvailable values: \"configuration.target\", \"configuration.value\", \"mode\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccessRule_Scope {}

impl DataAccessRule_Scope { }

impl terrars::ToListMappable for DataAccessRule_Scope {
    type O = terrars::BlockAssignable<DataAccessRule_Scope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccessRule_Scope {}

impl BuildDataAccessRule_Scope {
    pub fn build(self) -> DataAccessRule_Scope {
        DataAccessRule_Scope {}
    }
}

#[allow(dead_code)]
pub struct DataAccessRule_ScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccessRule_ScopeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccessRule_ScopeRef {
        DataAccessRule_ScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccessRule_ScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe contact email address of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nDefines an identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nDefines the scope of the rule.\nAvailable values: \"user\", \"organization\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

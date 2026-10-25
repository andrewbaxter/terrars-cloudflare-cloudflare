#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataTokenValidationRulesListData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_configuration: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataTokenValidationRulesList_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataTokenValidationRulesListData_>,
}

#[derive(Clone)]
pub struct DataTokenValidationRulesList(std::rc::Rc<DataTokenValidationRulesList_>);

impl DataTokenValidationRulesList {
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
        "Set the field `action`.\nAction to take on requests that match operations included in `selector` and fail `expression`.\nAvailable values: \"log\", \"block\"."]
    pub fn set_action(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nToggle rule on or off."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\nSelect rules with this host in `include`."]
    pub fn set_host(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nSelect rules with this host in `include`."]
    pub fn set_hostname(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nSelect rules with these IDs."]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_id`.\nSelect rules with these IDs."]
    pub fn set_rule_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rule_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_configuration`.\nSelect rules using any of these token configurations."]
    pub fn set_token_configuration(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().token_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAction to take on requests that match operations included in `selector` and fail `expression`.\nAvailable values: \"log\", \"block\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nToggle rule on or off."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nSelect rules with this host in `include`."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nSelect rules with this host in `include`."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nSelect rules with these IDs."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataTokenValidationRulesList_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nSelect rules with these IDs."]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `token_configuration` after provisioning.\nSelect rules using any of these token configurations."]
    pub fn token_configuration(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataTokenValidationRulesList {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataTokenValidationRulesList { }

impl terrars::ToListMappable for DataTokenValidationRulesList {
    type O = terrars::ListRef<DataTokenValidationRulesListRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataTokenValidationRulesList_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_token_validation_rules_list".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataTokenValidationRulesList {
    pub tf_id: std::string::String,
}

impl BuildDataTokenValidationRulesList {
    pub fn build(self, stack: &mut terrars::Stack) -> DataTokenValidationRulesList {
        let out = DataTokenValidationRulesList(std::rc::Rc::new(DataTokenValidationRulesList_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataTokenValidationRulesListData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                action: core::default::Default::default(),
                enabled: core::default::Default::default(),
                host: core::default::Default::default(),
                hostname: core::default::Default::default(),
                id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                rule_id: core::default::Default::default(),
                token_configuration: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataTokenValidationRulesListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTokenValidationRulesListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataTokenValidationRulesListRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAction to take on requests that match operations included in `selector` and fail `expression`.\nAvailable values: \"log\", \"block\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nToggle rule on or off."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nSelect rules with this host in `include`."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nSelect rules with this host in `include`."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nSelect rules with these IDs."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataTokenValidationRulesList_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nSelect rules with these IDs."]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `token_configuration` after provisioning.\nSelect rules using any of these token configurations."]
    pub fn token_configuration(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataTokenValidationRulesList_Result_Selector_Exclude {}

impl DataTokenValidationRulesList_Result_Selector_Exclude { }

impl terrars::ToListMappable for DataTokenValidationRulesList_Result_Selector_Exclude {
    type O = terrars::BlockAssignable<DataTokenValidationRulesList_Result_Selector_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTokenValidationRulesList_Result_Selector_Exclude {}

impl BuildDataTokenValidationRulesList_Result_Selector_Exclude {
    pub fn build(self) -> DataTokenValidationRulesList_Result_Selector_Exclude {
        DataTokenValidationRulesList_Result_Selector_Exclude {}
    }
}

#[allow(dead_code)]
pub struct DataTokenValidationRulesList_Result_Selector_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTokenValidationRulesList_Result_Selector_ExcludeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataTokenValidationRulesList_Result_Selector_ExcludeRef {
        DataTokenValidationRulesList_Result_Selector_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTokenValidationRulesList_Result_Selector_ExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operation_ids` after provisioning.\nExcluded operation IDs."]
    pub fn operation_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operation_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTokenValidationRulesList_Result_Selector_Include {}

impl DataTokenValidationRulesList_Result_Selector_Include { }

impl terrars::ToListMappable for DataTokenValidationRulesList_Result_Selector_Include {
    type O = terrars::BlockAssignable<DataTokenValidationRulesList_Result_Selector_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTokenValidationRulesList_Result_Selector_Include {}

impl BuildDataTokenValidationRulesList_Result_Selector_Include {
    pub fn build(self) -> DataTokenValidationRulesList_Result_Selector_Include {
        DataTokenValidationRulesList_Result_Selector_Include {}
    }
}

#[allow(dead_code)]
pub struct DataTokenValidationRulesList_Result_Selector_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTokenValidationRulesList_Result_Selector_IncludeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataTokenValidationRulesList_Result_Selector_IncludeRef {
        DataTokenValidationRulesList_Result_Selector_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTokenValidationRulesList_Result_Selector_IncludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nIncluded hostnames."]
    pub fn host(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTokenValidationRulesList_Result_Selector {}

impl DataTokenValidationRulesList_Result_Selector { }

impl terrars::ToListMappable for DataTokenValidationRulesList_Result_Selector {
    type O = terrars::BlockAssignable<DataTokenValidationRulesList_Result_Selector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTokenValidationRulesList_Result_Selector {}

impl BuildDataTokenValidationRulesList_Result_Selector {
    pub fn build(self) -> DataTokenValidationRulesList_Result_Selector {
        DataTokenValidationRulesList_Result_Selector {}
    }
}

#[allow(dead_code)]
pub struct DataTokenValidationRulesList_Result_SelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTokenValidationRulesList_Result_SelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataTokenValidationRulesList_Result_SelectorRef {
        DataTokenValidationRulesList_Result_SelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTokenValidationRulesList_Result_SelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nIgnore operations that were otherwise included by `include`."]
    pub fn exclude(&self) -> terrars::ListRef<DataTokenValidationRulesList_Result_Selector_ExcludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `include` after provisioning.\nSelect all matching operations."]
    pub fn include(&self) -> terrars::ListRef<DataTokenValidationRulesList_Result_Selector_IncludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTokenValidationRulesList_Result {}

impl DataTokenValidationRulesList_Result { }

impl terrars::ToListMappable for DataTokenValidationRulesList_Result {
    type O = terrars::BlockAssignable<DataTokenValidationRulesList_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTokenValidationRulesList_Result {}

impl BuildDataTokenValidationRulesList_Result {
    pub fn build(self) -> DataTokenValidationRulesList_Result {
        DataTokenValidationRulesList_Result {}
    }
}

#[allow(dead_code)]
pub struct DataTokenValidationRulesList_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTokenValidationRulesList_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataTokenValidationRulesList_ResultRef {
        DataTokenValidationRulesList_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTokenValidationRulesList_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAction to take on requests that match operations included in `selector` and fail `expression`.\nAvailable values: \"log\", \"block\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description that gives more details than `title`."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nToggle rule on or off."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nRule expression. Requests that fail to match this expression will be subject to `action`.\n\nFor details on expressions, see the [Cloudflare Docs](https://developers.cloudflare.com/api-shield/security/jwt-validation/)."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc =
        "Get a reference to the value of field `selector` after provisioning.\nSelect operations covered by this rule.\n\nFor details on selectors, see the [Cloudflare Docs](https://developers.cloudflare.com/api-shield/security/jwt-validation/)."]
    pub fn selector(&self) -> DataTokenValidationRulesList_Result_SelectorRef {
        DataTokenValidationRulesList_Result_SelectorRef::new(
            self.shared().clone(),
            format!("{}.selector", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\nA human-readable name for the rule."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

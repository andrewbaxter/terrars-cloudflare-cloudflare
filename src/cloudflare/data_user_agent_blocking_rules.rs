#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataUserAgentBlockingRulesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paused: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataUserAgentBlockingRules_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataUserAgentBlockingRulesData_>,
}

#[derive(Clone)]
pub struct DataUserAgentBlockingRules(std::rc::Rc<DataUserAgentBlockingRules_>);

impl DataUserAgentBlockingRules {
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

    #[doc = "Set the field `description`.\nA string to search for in the description of existing rules."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `paused`.\nWhen true, indicates that the rule is currently paused."]
    pub fn set_paused(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().paused = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_agent`.\nA string to search for in the user agent values of existing rules."]
    pub fn set_user_agent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nDefines an identifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA string to search for in the description of existing rules."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paused` after provisioning.\nWhen true, indicates that the rule is currently paused."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataUserAgentBlockingRules_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_agent` after provisioning.\nA string to search for in the user agent values of existing rules."]
    pub fn user_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nDefines an identifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataUserAgentBlockingRules {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataUserAgentBlockingRules { }

impl terrars::ToListMappable for DataUserAgentBlockingRules {
    type O = terrars::ListRef<DataUserAgentBlockingRulesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataUserAgentBlockingRules_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_user_agent_blocking_rules".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataUserAgentBlockingRules {
    pub tf_id: std::string::String,
}

impl BuildDataUserAgentBlockingRules {
    pub fn build(self, stack: &mut terrars::Stack) -> DataUserAgentBlockingRules {
        let out = DataUserAgentBlockingRules(std::rc::Rc::new(DataUserAgentBlockingRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataUserAgentBlockingRulesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                description: core::default::Default::default(),
                max_items: core::default::Default::default(),
                paused: core::default::Default::default(),
                user_agent: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataUserAgentBlockingRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserAgentBlockingRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataUserAgentBlockingRulesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA string to search for in the description of existing rules."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paused` after provisioning.\nWhen true, indicates that the rule is currently paused."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataUserAgentBlockingRules_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_agent` after provisioning.\nA string to search for in the user agent values of existing rules."]
    pub fn user_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nDefines an identifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataUserAgentBlockingRules_Result_Configuration {}

impl DataUserAgentBlockingRules_Result_Configuration { }

impl terrars::ToListMappable for DataUserAgentBlockingRules_Result_Configuration {
    type O = terrars::BlockAssignable<DataUserAgentBlockingRules_Result_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserAgentBlockingRules_Result_Configuration {}

impl BuildDataUserAgentBlockingRules_Result_Configuration {
    pub fn build(self) -> DataUserAgentBlockingRules_Result_Configuration {
        DataUserAgentBlockingRules_Result_Configuration {}
    }
}

#[allow(dead_code)]
pub struct DataUserAgentBlockingRules_Result_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserAgentBlockingRules_Result_ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataUserAgentBlockingRules_Result_ConfigurationRef {
        DataUserAgentBlockingRules_Result_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserAgentBlockingRules_Result_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nThe configuration target for this rule. You must set the target to `ua` for User Agent Blocking rules."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe exact user agent string to match. This value will be compared to the received `User-Agent` HTTP header value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataUserAgentBlockingRules_Result {}

impl DataUserAgentBlockingRules_Result { }

impl terrars::ToListMappable for DataUserAgentBlockingRules_Result {
    type O = terrars::BlockAssignable<DataUserAgentBlockingRules_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserAgentBlockingRules_Result {}

impl BuildDataUserAgentBlockingRules_Result {
    pub fn build(self) -> DataUserAgentBlockingRules_Result {
        DataUserAgentBlockingRules_Result {}
    }
}

#[allow(dead_code)]
pub struct DataUserAgentBlockingRules_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserAgentBlockingRules_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataUserAgentBlockingRules_ResultRef {
        DataUserAgentBlockingRules_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserAgentBlockingRules_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `configuration` after provisioning.\nThe configuration object for the current rule."]
    pub fn configuration(&self) -> DataUserAgentBlockingRules_Result_ConfigurationRef {
        DataUserAgentBlockingRules_Result_ConfigurationRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative summary of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the User Agent Blocking rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe action to apply to a matched request.\nAvailable values: \"block\", \"challenge\", \"js_challenge\", \"managed_challenge\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `paused` after provisioning.\nWhen true, indicates that the rule is currently paused."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.base))
    }
}

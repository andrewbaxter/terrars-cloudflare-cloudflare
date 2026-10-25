#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataUserAgentBlockingRuleData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataUserAgentBlockingRule_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ua_rule_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataUserAgentBlockingRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataUserAgentBlockingRuleData_>,
}

#[derive(Clone)]
pub struct DataUserAgentBlockingRule(std::rc::Rc<DataUserAgentBlockingRule_>);

impl DataUserAgentBlockingRule {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataUserAgentBlockingRule_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ua_rule_id`.\nThe unique identifier of the User Agent Blocking rule."]
    pub fn set_ua_rule_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ua_rule_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nDefines an identifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `configuration` after provisioning.\nThe configuration object for the current rule."]
    pub fn configuration(&self) -> DataUserAgentBlockingRule_ConfigurationRef {
        DataUserAgentBlockingRule_ConfigurationRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative summary of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataUserAgentBlockingRule_FilterRef {
        DataUserAgentBlockingRule_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the User Agent Blocking rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe action to apply to a matched request.\nAvailable values: \"block\", \"challenge\", \"js_challenge\", \"managed_challenge\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paused` after provisioning.\nWhen true, indicates that the rule is currently paused."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ua_rule_id` after provisioning.\nThe unique identifier of the User Agent Blocking rule."]
    pub fn ua_rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ua_rule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nDefines an identifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataUserAgentBlockingRule {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataUserAgentBlockingRule { }

impl terrars::ToListMappable for DataUserAgentBlockingRule {
    type O = terrars::ListRef<DataUserAgentBlockingRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataUserAgentBlockingRule_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_user_agent_blocking_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataUserAgentBlockingRule {
    pub tf_id: std::string::String,
}

impl BuildDataUserAgentBlockingRule {
    pub fn build(self, stack: &mut terrars::Stack) -> DataUserAgentBlockingRule {
        let out = DataUserAgentBlockingRule(std::rc::Rc::new(DataUserAgentBlockingRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataUserAgentBlockingRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                ua_rule_id: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataUserAgentBlockingRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserAgentBlockingRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataUserAgentBlockingRuleRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `configuration` after provisioning.\nThe configuration object for the current rule."]
    pub fn configuration(&self) -> DataUserAgentBlockingRule_ConfigurationRef {
        DataUserAgentBlockingRule_ConfigurationRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative summary of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataUserAgentBlockingRule_FilterRef {
        DataUserAgentBlockingRule_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the User Agent Blocking rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe action to apply to a matched request.\nAvailable values: \"block\", \"challenge\", \"js_challenge\", \"managed_challenge\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paused` after provisioning.\nWhen true, indicates that the rule is currently paused."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ua_rule_id` after provisioning.\nThe unique identifier of the User Agent Blocking rule."]
    pub fn ua_rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ua_rule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nDefines an identifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataUserAgentBlockingRule_Configuration {}

impl DataUserAgentBlockingRule_Configuration { }

impl terrars::ToListMappable for DataUserAgentBlockingRule_Configuration {
    type O = terrars::BlockAssignable<DataUserAgentBlockingRule_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserAgentBlockingRule_Configuration {}

impl BuildDataUserAgentBlockingRule_Configuration {
    pub fn build(self) -> DataUserAgentBlockingRule_Configuration {
        DataUserAgentBlockingRule_Configuration {}
    }
}

#[allow(dead_code)]
pub struct DataUserAgentBlockingRule_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserAgentBlockingRule_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataUserAgentBlockingRule_ConfigurationRef {
        DataUserAgentBlockingRule_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserAgentBlockingRule_ConfigurationRef {
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
pub struct DataUserAgentBlockingRule_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paused: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_agent: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataUserAgentBlockingRule_Filter {
    #[doc = "Set the field `description`.\nA string to search for in the description of existing rules."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `paused`.\nWhen true, indicates that the rule is currently paused."]
    pub fn set_paused(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.paused = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_agent`.\nA string to search for in the user agent values of existing rules."]
    pub fn set_user_agent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_agent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataUserAgentBlockingRule_Filter {
    type O = terrars::BlockAssignable<DataUserAgentBlockingRule_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserAgentBlockingRule_Filter {}

impl BuildDataUserAgentBlockingRule_Filter {
    pub fn build(self) -> DataUserAgentBlockingRule_Filter {
        DataUserAgentBlockingRule_Filter {
            description: core::default::Default::default(),
            paused: core::default::Default::default(),
            user_agent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataUserAgentBlockingRule_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserAgentBlockingRule_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataUserAgentBlockingRule_FilterRef {
        DataUserAgentBlockingRule_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserAgentBlockingRule_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA string to search for in the description of existing rules."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `paused` after provisioning.\nWhen true, indicates that the rule is currently paused."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_agent` after provisioning.\nA string to search for in the user agent values of existing rules."]
    pub fn user_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_agent", self.base))
    }
}

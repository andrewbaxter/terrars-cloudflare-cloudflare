#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataEmailRoutingRuleData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataEmailRoutingRule_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataEmailRoutingRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEmailRoutingRuleData_>,
}

#[derive(Clone)]
pub struct DataEmailRoutingRule(std::rc::Rc<DataEmailRoutingRule_>);

impl DataEmailRoutingRule {
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
    pub fn set_filter(self, v: impl std::convert::Into<DataEmailRoutingRule_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_identifier`.\nRouting rule identifier."]
    pub fn set_rule_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rule_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\nList actions patterns."]
    pub fn actions(&self) -> terrars::ListRef<DataEmailRoutingRule_ActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nRouting rule status."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataEmailRoutingRule_FilterRef {
        DataEmailRoutingRule_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nRouting rule identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `matchers` after provisioning.\nMatching patterns to forward to your actions."]
    pub fn matchers(&self) -> terrars::ListRef<DataEmailRoutingRule_MatchersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matchers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRouting rule name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\nPriority of the routing rule."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_identifier` after provisioning.\nRouting rule identifier."]
    pub fn rule_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nWho manages the rule. `api` covers dashboard, generic API, and Terraform;\n`wrangler` means the rule is managed by a Worker's wrangler.jsonc. Defaults\nto `api` when omitted on write.\nAvailable values: \"api\", \"wrangler\"."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nRouting rule tag. (Deprecated, replaced by routing rule identifier)"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataEmailRoutingRule {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEmailRoutingRule { }

impl terrars::ToListMappable for DataEmailRoutingRule {
    type O = terrars::ListRef<DataEmailRoutingRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEmailRoutingRule_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_email_routing_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmailRoutingRule {
    pub tf_id: std::string::String,
}

impl BuildDataEmailRoutingRule {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEmailRoutingRule {
        let out = DataEmailRoutingRule(std::rc::Rc::new(DataEmailRoutingRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEmailRoutingRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                rule_identifier: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmailRoutingRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailRoutingRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEmailRoutingRuleRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\nList actions patterns."]
    pub fn actions(&self) -> terrars::ListRef<DataEmailRoutingRule_ActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nRouting rule status."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataEmailRoutingRule_FilterRef {
        DataEmailRoutingRule_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nRouting rule identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `matchers` after provisioning.\nMatching patterns to forward to your actions."]
    pub fn matchers(&self) -> terrars::ListRef<DataEmailRoutingRule_MatchersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matchers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRouting rule name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\nPriority of the routing rule."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_identifier` after provisioning.\nRouting rule identifier."]
    pub fn rule_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nWho manages the rule. `api` covers dashboard, generic API, and Terraform;\n`wrangler` means the rule is managed by a Worker's wrangler.jsonc. Defaults\nto `api` when omitted on write.\nAvailable values: \"api\", \"wrangler\"."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nRouting rule tag. (Deprecated, replaced by routing rule identifier)"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEmailRoutingRule_Actions {}

impl DataEmailRoutingRule_Actions { }

impl terrars::ToListMappable for DataEmailRoutingRule_Actions {
    type O = terrars::BlockAssignable<DataEmailRoutingRule_Actions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailRoutingRule_Actions {}

impl BuildDataEmailRoutingRule_Actions {
    pub fn build(self) -> DataEmailRoutingRule_Actions {
        DataEmailRoutingRule_Actions {}
    }
}

#[allow(dead_code)]
pub struct DataEmailRoutingRule_ActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailRoutingRule_ActionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEmailRoutingRule_ActionsRef {
        DataEmailRoutingRule_ActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailRoutingRule_ActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of supported action.\nAvailable values: \"drop\", \"forward\", \"worker\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nList of values for the action. Currently limited to a single value."]
    pub fn value(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEmailRoutingRule_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEmailRoutingRule_Filter {
    #[doc = "Set the field `enabled`.\nFilter by enabled routing rules."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEmailRoutingRule_Filter {
    type O = terrars::BlockAssignable<DataEmailRoutingRule_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailRoutingRule_Filter {}

impl BuildDataEmailRoutingRule_Filter {
    pub fn build(self) -> DataEmailRoutingRule_Filter {
        DataEmailRoutingRule_Filter { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEmailRoutingRule_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailRoutingRule_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEmailRoutingRule_FilterRef {
        DataEmailRoutingRule_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailRoutingRule_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nFilter by enabled routing rules."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEmailRoutingRule_Matchers {}

impl DataEmailRoutingRule_Matchers { }

impl terrars::ToListMappable for DataEmailRoutingRule_Matchers {
    type O = terrars::BlockAssignable<DataEmailRoutingRule_Matchers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailRoutingRule_Matchers {}

impl BuildDataEmailRoutingRule_Matchers {
    pub fn build(self) -> DataEmailRoutingRule_Matchers {
        DataEmailRoutingRule_Matchers {}
    }
}

#[allow(dead_code)]
pub struct DataEmailRoutingRule_MatchersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailRoutingRule_MatchersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEmailRoutingRule_MatchersRef {
        DataEmailRoutingRule_MatchersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailRoutingRule_MatchersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `field` after provisioning.\nField for type matcher.\nAvailable values: \"to\"."]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of matcher.\nAvailable values: \"all\", \"literal\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue for matcher."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

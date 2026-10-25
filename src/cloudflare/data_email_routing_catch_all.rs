#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataEmailRoutingCatchAllData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataEmailRoutingCatchAll_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEmailRoutingCatchAllData_>,
}

#[derive(Clone)]
pub struct DataEmailRoutingCatchAll(std::rc::Rc<DataEmailRoutingCatchAll_>);

impl DataEmailRoutingCatchAll {
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

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nList actions for the catch-all routing rule."]
    pub fn actions(&self) -> terrars::ListRef<DataEmailRoutingCatchAll_ActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nRouting rule status."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `matchers` after provisioning.\nList of matchers for the catch-all routing rule."]
    pub fn matchers(&self) -> terrars::ListRef<DataEmailRoutingCatchAll_MatchersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matchers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRouting rule name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

impl terrars::Referable for DataEmailRoutingCatchAll {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEmailRoutingCatchAll { }

impl terrars::ToListMappable for DataEmailRoutingCatchAll {
    type O = terrars::ListRef<DataEmailRoutingCatchAllRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEmailRoutingCatchAll_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_email_routing_catch_all".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmailRoutingCatchAll {
    pub tf_id: std::string::String,
}

impl BuildDataEmailRoutingCatchAll {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEmailRoutingCatchAll {
        let out = DataEmailRoutingCatchAll(std::rc::Rc::new(DataEmailRoutingCatchAll_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEmailRoutingCatchAllData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmailRoutingCatchAllRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailRoutingCatchAllRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEmailRoutingCatchAllRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nList actions for the catch-all routing rule."]
    pub fn actions(&self) -> terrars::ListRef<DataEmailRoutingCatchAll_ActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nRouting rule status."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `matchers` after provisioning.\nList of matchers for the catch-all routing rule."]
    pub fn matchers(&self) -> terrars::ListRef<DataEmailRoutingCatchAll_MatchersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matchers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRouting rule name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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
pub struct DataEmailRoutingCatchAll_Actions {}

impl DataEmailRoutingCatchAll_Actions { }

impl terrars::ToListMappable for DataEmailRoutingCatchAll_Actions {
    type O = terrars::BlockAssignable<DataEmailRoutingCatchAll_Actions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailRoutingCatchAll_Actions {}

impl BuildDataEmailRoutingCatchAll_Actions {
    pub fn build(self) -> DataEmailRoutingCatchAll_Actions {
        DataEmailRoutingCatchAll_Actions {}
    }
}

#[allow(dead_code)]
pub struct DataEmailRoutingCatchAll_ActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailRoutingCatchAll_ActionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEmailRoutingCatchAll_ActionsRef {
        DataEmailRoutingCatchAll_ActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailRoutingCatchAll_ActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of action for catch-all rule.\nAvailable values: \"drop\", \"forward\", \"worker\"."]
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
pub struct DataEmailRoutingCatchAll_Matchers {}

impl DataEmailRoutingCatchAll_Matchers { }

impl terrars::ToListMappable for DataEmailRoutingCatchAll_Matchers {
    type O = terrars::BlockAssignable<DataEmailRoutingCatchAll_Matchers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailRoutingCatchAll_Matchers {}

impl BuildDataEmailRoutingCatchAll_Matchers {
    pub fn build(self) -> DataEmailRoutingCatchAll_Matchers {
        DataEmailRoutingCatchAll_Matchers {}
    }
}

#[allow(dead_code)]
pub struct DataEmailRoutingCatchAll_MatchersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailRoutingCatchAll_MatchersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEmailRoutingCatchAll_MatchersRef {
        DataEmailRoutingCatchAll_MatchersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailRoutingCatchAll_MatchersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of matcher. Default is 'all'.\nAvailable values: \"all\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

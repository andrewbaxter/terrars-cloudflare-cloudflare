#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustRiskBehaviorData_ {
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
}

struct DataZeroTrustRiskBehavior_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustRiskBehaviorData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustRiskBehavior(std::rc::Rc<DataZeroTrustRiskBehavior_>);

impl DataZeroTrustRiskBehavior {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `behaviors` after provisioning.\n"]
    pub fn behaviors(&self) -> terrars::RecRef<DataZeroTrustRiskBehavior_BehaviorsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.behaviors", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustRiskBehavior {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustRiskBehavior { }

impl terrars::ToListMappable for DataZeroTrustRiskBehavior {
    type O = terrars::ListRef<DataZeroTrustRiskBehaviorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustRiskBehavior_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_risk_behavior".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustRiskBehavior {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustRiskBehavior {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustRiskBehavior {
        let out = DataZeroTrustRiskBehavior(std::rc::Rc::new(DataZeroTrustRiskBehavior_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustRiskBehaviorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustRiskBehaviorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustRiskBehaviorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustRiskBehaviorRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `behaviors` after provisioning.\n"]
    pub fn behaviors(&self) -> terrars::RecRef<DataZeroTrustRiskBehavior_BehaviorsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.behaviors", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustRiskBehavior_Behaviors {}

impl DataZeroTrustRiskBehavior_Behaviors { }

impl terrars::ToListMappable for DataZeroTrustRiskBehavior_Behaviors {
    type O = terrars::BlockAssignable<DataZeroTrustRiskBehavior_Behaviors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustRiskBehavior_Behaviors {}

impl BuildDataZeroTrustRiskBehavior_Behaviors {
    pub fn build(self) -> DataZeroTrustRiskBehavior_Behaviors {
        DataZeroTrustRiskBehavior_Behaviors {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustRiskBehavior_BehaviorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustRiskBehavior_BehaviorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustRiskBehavior_BehaviorsRef {
        DataZeroTrustRiskBehavior_BehaviorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustRiskBehavior_BehaviorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `risk_level` after provisioning.\nAvailable values: \"low\", \"medium\", \"high\"."]
    pub fn risk_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.risk_level", self.base))
    }
}

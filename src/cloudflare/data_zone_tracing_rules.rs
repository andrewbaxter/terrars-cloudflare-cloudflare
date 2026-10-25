#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZoneTracingRulesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct DataZoneTracingRules_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZoneTracingRulesData_>,
}

#[derive(Clone)]
pub struct DataZoneTracingRules(std::rc::Rc<DataZoneTracingRules_>);

impl DataZoneTracingRules {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\nSpecify the zone ID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nTrace rules in evaluation order."]
    pub fn rules(&self) -> terrars::ListRef<DataZoneTracingRules_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nSpecify the zone ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataZoneTracingRules {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZoneTracingRules { }

impl terrars::ToListMappable for DataZoneTracingRules {
    type O = terrars::ListRef<DataZoneTracingRulesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZoneTracingRules_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zone_tracing_rules".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZoneTracingRules {
    pub tf_id: std::string::String,
    #[doc = "Specify the zone ID."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZoneTracingRules {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZoneTracingRules {
        let out = DataZoneTracingRules(std::rc::Rc::new(DataZoneTracingRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZoneTracingRulesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZoneTracingRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneTracingRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZoneTracingRulesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nSpecify the zone ID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nTrace rules in evaluation order."]
    pub fn rules(&self) -> terrars::ListRef<DataZoneTracingRules_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nSpecify the zone ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZoneTracingRules_Rules_ActionParameters {}

impl DataZoneTracingRules_Rules_ActionParameters { }

impl terrars::ToListMappable for DataZoneTracingRules_Rules_ActionParameters {
    type O = terrars::BlockAssignable<DataZoneTracingRules_Rules_ActionParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZoneTracingRules_Rules_ActionParameters {}

impl BuildDataZoneTracingRules_Rules_ActionParameters {
    pub fn build(self) -> DataZoneTracingRules_Rules_ActionParameters {
        DataZoneTracingRules_Rules_ActionParameters {}
    }
}

#[allow(dead_code)]
pub struct DataZoneTracingRules_Rules_ActionParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneTracingRules_Rules_ActionParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZoneTracingRules_Rules_ActionParametersRef {
        DataZoneTracingRules_Rules_ActionParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZoneTracingRules_Rules_ActionParametersRef {
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
pub struct DataZoneTracingRules_Rules {}

impl DataZoneTracingRules_Rules { }

impl terrars::ToListMappable for DataZoneTracingRules_Rules {
    type O = terrars::BlockAssignable<DataZoneTracingRules_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZoneTracingRules_Rules {}

impl BuildDataZoneTracingRules_Rules {
    pub fn build(self) -> DataZoneTracingRules_Rules {
        DataZoneTracingRules_Rules {}
    }
}

#[allow(dead_code)]
pub struct DataZoneTracingRules_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneTracingRules_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZoneTracingRules_RulesRef {
        DataZoneTracingRules_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZoneTracingRules_RulesRef {
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
    pub fn action_parameters(&self) -> DataZoneTracingRules_Rules_ActionParametersRef {
        DataZoneTracingRules_Rules_ActionParametersRef::new(
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

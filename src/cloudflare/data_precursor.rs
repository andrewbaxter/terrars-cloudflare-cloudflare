#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataPrecursorData_ {
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

struct DataPrecursor_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPrecursorData_>,
}

#[derive(Clone)]
pub struct DataPrecursor(std::rc::Rc<DataPrecursor_>);

impl DataPrecursor {
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
        "Get a reference to the value of field `default_mode` after provisioning.\nThe zone-level Precursor enforcement mode applied to requests that do\nnot match a more specific enforcement rule.\nAvailable values: \"off\", \"min-friction\", \"max-security\"."]
    pub fn default_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enforcement_rules` after provisioning.\nThe ordered list of enforcement rules for the zone."]
    pub fn enforcement_rules(&self) -> terrars::ListRef<DataPrecursor_EnforcementRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enforcement_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataPrecursor {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPrecursor { }

impl terrars::ToListMappable for DataPrecursor {
    type O = terrars::ListRef<DataPrecursorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPrecursor_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_precursor".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPrecursor {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildDataPrecursor {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPrecursor {
        let out = DataPrecursor(std::rc::Rc::new(DataPrecursor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPrecursorData_ {
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

pub struct DataPrecursorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrecursorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPrecursorRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `default_mode` after provisioning.\nThe zone-level Precursor enforcement mode applied to requests that do\nnot match a more specific enforcement rule.\nAvailable values: \"off\", \"min-friction\", \"max-security\"."]
    pub fn default_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enforcement_rules` after provisioning.\nThe ordered list of enforcement rules for the zone."]
    pub fn enforcement_rules(&self) -> terrars::ListRef<DataPrecursor_EnforcementRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enforcement_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataPrecursor_EnforcementRules {}

impl DataPrecursor_EnforcementRules { }

impl terrars::ToListMappable for DataPrecursor_EnforcementRules {
    type O = terrars::BlockAssignable<DataPrecursor_EnforcementRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrecursor_EnforcementRules {}

impl BuildDataPrecursor_EnforcementRules {
    pub fn build(self) -> DataPrecursor_EnforcementRules {
        DataPrecursor_EnforcementRules {}
    }
}

#[allow(dead_code)]
pub struct DataPrecursor_EnforcementRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrecursor_EnforcementRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPrecursor_EnforcementRulesRef {
        DataPrecursor_EnforcementRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrecursor_EnforcementRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the rule is active."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nThe filter expression that determines which requests the rule matches."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe read-only identifier that Cloudflare assigns to the rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe override mode Precursor applies to requests matching an enforcement\nrule. Unlike `default_mode`, this cannot be `off`.\nAvailable values: \"min-friction\", \"max-security\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

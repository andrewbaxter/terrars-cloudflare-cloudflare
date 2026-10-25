#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustCasbPoliciesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
}

struct DataZeroTrustCasbPolicies_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustCasbPoliciesData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustCasbPolicies(std::rc::Rc<DataZeroTrustCasbPolicies_>);

impl DataZeroTrustCasbPolicies {
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

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustCasbPolicies_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustCasbPolicies {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustCasbPolicies { }

impl terrars::ToListMappable for DataZeroTrustCasbPolicies {
    type O = terrars::ListRef<DataZeroTrustCasbPoliciesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustCasbPolicies_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_casb_policies".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustCasbPolicies {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustCasbPolicies {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustCasbPolicies {
        let out = DataZeroTrustCasbPolicies(std::rc::Rc::new(DataZeroTrustCasbPolicies_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustCasbPoliciesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                max_items: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustCasbPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbPoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustCasbPoliciesRef {
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

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustCasbPolicies_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustCasbPolicies_Result_Actions_RemediationTypes {}

impl DataZeroTrustCasbPolicies_Result_Actions_RemediationTypes { }

impl terrars::ToListMappable for DataZeroTrustCasbPolicies_Result_Actions_RemediationTypes {
    type O = terrars::BlockAssignable<DataZeroTrustCasbPolicies_Result_Actions_RemediationTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustCasbPolicies_Result_Actions_RemediationTypes {}

impl BuildDataZeroTrustCasbPolicies_Result_Actions_RemediationTypes {
    pub fn build(self) -> DataZeroTrustCasbPolicies_Result_Actions_RemediationTypes {
        DataZeroTrustCasbPolicies_Result_Actions_RemediationTypes {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustCasbPolicies_Result_Actions_RemediationTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbPolicies_Result_Actions_RemediationTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustCasbPolicies_Result_Actions_RemediationTypesRef {
        DataZeroTrustCasbPolicies_Result_Actions_RemediationTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustCasbPolicies_Result_Actions_RemediationTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name/label of the remediation type."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `remediation_type` after provisioning.\nThe system name of the remediation type."]
    pub fn remediation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remediation_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `remediation_type_id` after provisioning.\nUnique identifier for the remediation type."]
    pub fn remediation_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remediation_type_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustCasbPolicies_Result_Actions_WebhookConfigs {}

impl DataZeroTrustCasbPolicies_Result_Actions_WebhookConfigs { }

impl terrars::ToListMappable for DataZeroTrustCasbPolicies_Result_Actions_WebhookConfigs {
    type O = terrars::BlockAssignable<DataZeroTrustCasbPolicies_Result_Actions_WebhookConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustCasbPolicies_Result_Actions_WebhookConfigs {}

impl BuildDataZeroTrustCasbPolicies_Result_Actions_WebhookConfigs {
    pub fn build(self) -> DataZeroTrustCasbPolicies_Result_Actions_WebhookConfigs {
        DataZeroTrustCasbPolicies_Result_Actions_WebhookConfigs {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustCasbPolicies_Result_Actions_WebhookConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbPolicies_Result_Actions_WebhookConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustCasbPolicies_Result_Actions_WebhookConfigsRef {
        DataZeroTrustCasbPolicies_Result_Actions_WebhookConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustCasbPolicies_Result_Actions_WebhookConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name/label of the webhook configuration."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook_config_id` after provisioning.\nUnique identifier for the webhook configuration."]
    pub fn webhook_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_config_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustCasbPolicies_Result_Actions {}

impl DataZeroTrustCasbPolicies_Result_Actions { }

impl terrars::ToListMappable for DataZeroTrustCasbPolicies_Result_Actions {
    type O = terrars::BlockAssignable<DataZeroTrustCasbPolicies_Result_Actions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustCasbPolicies_Result_Actions {}

impl BuildDataZeroTrustCasbPolicies_Result_Actions {
    pub fn build(self) -> DataZeroTrustCasbPolicies_Result_Actions {
        DataZeroTrustCasbPolicies_Result_Actions {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustCasbPolicies_Result_ActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbPolicies_Result_ActionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustCasbPolicies_Result_ActionsRef {
        DataZeroTrustCasbPolicies_Result_ActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustCasbPolicies_Result_ActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `remediation_types` after provisioning.\nList of remediation types that will be executed."]
    pub fn remediation_types(&self) -> terrars::ListRef<DataZeroTrustCasbPolicies_Result_Actions_RemediationTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remediation_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook_configs` after provisioning.\nList of webhook configurations that will be triggered."]
    pub fn webhook_configs(&self) -> terrars::ListRef<DataZeroTrustCasbPolicies_Result_Actions_WebhookConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webhook_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustCasbPolicies_Result {}

impl DataZeroTrustCasbPolicies_Result { }

impl terrars::ToListMappable for DataZeroTrustCasbPolicies_Result {
    type O = terrars::BlockAssignable<DataZeroTrustCasbPolicies_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustCasbPolicies_Result {}

impl BuildDataZeroTrustCasbPolicies_Result {
    pub fn build(self) -> DataZeroTrustCasbPolicies_Result {
        DataZeroTrustCasbPolicies_Result {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustCasbPolicies_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbPolicies_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustCasbPolicies_ResultRef {
        DataZeroTrustCasbPolicies_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustCasbPolicies_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nThe actions configured for this policy."]
    pub fn actions(&self) -> DataZeroTrustCasbPolicies_Result_ActionsRef {
        DataZeroTrustCasbPolicies_Result_ActionsRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `applies_to_all_integrations` after provisioning.\nWhen true, the policy applies to all integrations for the account. When false, it applies only to the specified integration_ids."]
    pub fn applies_to_all_integrations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.applies_to_all_integrations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the policy was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-set description of what this policy does. Limited to 1000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled_at` after provisioning.\nTimestamp when the policy was disabled. Omitted from the response when the policy\nis enabled."]
    pub fn disabled_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name for the policy configuration. Limited to 255 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the policy is enabled. Derived from disabled_at (enabled when disabled_at is unset)."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `finding_type_id` after provisioning.\nThe finding type this policy is associated with. Immutable after creation; attempts to change it via update are rejected."]
    pub fn finding_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finding_type_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the policy configuration."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `integration_ids` after provisioning.\nThe integrations this policy applies to."]
    pub fn integration_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `last_triggered_at` after provisioning.\nTimestamp of the most recent successful policy invocation. Omitted\nfrom the response when the policy has never been successfully\ntriggered. Only populated on GET responses; absent on responses from\ncreate/update endpoints."]
    pub fn last_triggered_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_triggered_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the policy was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

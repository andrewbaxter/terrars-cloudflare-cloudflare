#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustCasbPolicyData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    policy_id: terrars::PrimField<std::string::String>,
}

struct DataZeroTrustCasbPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustCasbPolicyData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustCasbPolicy(std::rc::Rc<DataZeroTrustCasbPolicy_>);

impl DataZeroTrustCasbPolicy {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nThe actions configured for this policy."]
    pub fn actions(&self) -> DataZeroTrustCasbPolicy_ActionsRef {
        DataZeroTrustCasbPolicy_ActionsRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `applies_to_all_integrations` after provisioning.\nWhen true, the policy applies to all integrations for the account. When false, it applies only to the specified integration_ids."]
    pub fn applies_to_all_integrations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.applies_to_all_integrations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the policy was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-set description of what this policy does. Limited to 1000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled_at` after provisioning.\nTimestamp when the policy was disabled. Omitted from the response when the policy\nis enabled."]
    pub fn disabled_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name for the policy configuration. Limited to 255 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the policy is enabled. Derived from disabled_at (enabled when disabled_at is unset)."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `finding_type_id` after provisioning.\nThe finding type this policy is associated with. Immutable after creation; attempts to change it via update are rejected."]
    pub fn finding_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finding_type_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `integration_ids` after provisioning.\nThe integrations this policy applies to."]
    pub fn integration_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_triggered_at` after provisioning.\nTimestamp of the most recent successful policy invocation. Omitted\nfrom the response when the policy has never been successfully\ntriggered. Only populated on GET responses; absent on responses from\ncreate/update endpoints."]
    pub fn last_triggered_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_triggered_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the policy was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustCasbPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustCasbPolicy { }

impl terrars::ToListMappable for DataZeroTrustCasbPolicy {
    type O = terrars::ListRef<DataZeroTrustCasbPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustCasbPolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_casb_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustCasbPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub policy_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustCasbPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustCasbPolicy {
        let out = DataZeroTrustCasbPolicy(std::rc::Rc::new(DataZeroTrustCasbPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustCasbPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                policy_id: self.policy_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustCasbPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustCasbPolicyRef {
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

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nThe actions configured for this policy."]
    pub fn actions(&self) -> DataZeroTrustCasbPolicy_ActionsRef {
        DataZeroTrustCasbPolicy_ActionsRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `applies_to_all_integrations` after provisioning.\nWhen true, the policy applies to all integrations for the account. When false, it applies only to the specified integration_ids."]
    pub fn applies_to_all_integrations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.applies_to_all_integrations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the policy was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-set description of what this policy does. Limited to 1000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled_at` after provisioning.\nTimestamp when the policy was disabled. Omitted from the response when the policy\nis enabled."]
    pub fn disabled_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name for the policy configuration. Limited to 255 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the policy is enabled. Derived from disabled_at (enabled when disabled_at is unset)."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `finding_type_id` after provisioning.\nThe finding type this policy is associated with. Immutable after creation; attempts to change it via update are rejected."]
    pub fn finding_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finding_type_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `integration_ids` after provisioning.\nThe integrations this policy applies to."]
    pub fn integration_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_triggered_at` after provisioning.\nTimestamp of the most recent successful policy invocation. Omitted\nfrom the response when the policy has never been successfully\ntriggered. Only populated on GET responses; absent on responses from\ncreate/update endpoints."]
    pub fn last_triggered_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_triggered_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the policy was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustCasbPolicy_Actions_RemediationTypes {}

impl DataZeroTrustCasbPolicy_Actions_RemediationTypes { }

impl terrars::ToListMappable for DataZeroTrustCasbPolicy_Actions_RemediationTypes {
    type O = terrars::BlockAssignable<DataZeroTrustCasbPolicy_Actions_RemediationTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustCasbPolicy_Actions_RemediationTypes {}

impl BuildDataZeroTrustCasbPolicy_Actions_RemediationTypes {
    pub fn build(self) -> DataZeroTrustCasbPolicy_Actions_RemediationTypes {
        DataZeroTrustCasbPolicy_Actions_RemediationTypes {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustCasbPolicy_Actions_RemediationTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbPolicy_Actions_RemediationTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustCasbPolicy_Actions_RemediationTypesRef {
        DataZeroTrustCasbPolicy_Actions_RemediationTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustCasbPolicy_Actions_RemediationTypesRef {
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
pub struct DataZeroTrustCasbPolicy_Actions_WebhookConfigs {}

impl DataZeroTrustCasbPolicy_Actions_WebhookConfigs { }

impl terrars::ToListMappable for DataZeroTrustCasbPolicy_Actions_WebhookConfigs {
    type O = terrars::BlockAssignable<DataZeroTrustCasbPolicy_Actions_WebhookConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustCasbPolicy_Actions_WebhookConfigs {}

impl BuildDataZeroTrustCasbPolicy_Actions_WebhookConfigs {
    pub fn build(self) -> DataZeroTrustCasbPolicy_Actions_WebhookConfigs {
        DataZeroTrustCasbPolicy_Actions_WebhookConfigs {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustCasbPolicy_Actions_WebhookConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbPolicy_Actions_WebhookConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustCasbPolicy_Actions_WebhookConfigsRef {
        DataZeroTrustCasbPolicy_Actions_WebhookConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustCasbPolicy_Actions_WebhookConfigsRef {
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
pub struct DataZeroTrustCasbPolicy_Actions {}

impl DataZeroTrustCasbPolicy_Actions { }

impl terrars::ToListMappable for DataZeroTrustCasbPolicy_Actions {
    type O = terrars::BlockAssignable<DataZeroTrustCasbPolicy_Actions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustCasbPolicy_Actions {}

impl BuildDataZeroTrustCasbPolicy_Actions {
    pub fn build(self) -> DataZeroTrustCasbPolicy_Actions {
        DataZeroTrustCasbPolicy_Actions {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustCasbPolicy_ActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbPolicy_ActionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustCasbPolicy_ActionsRef {
        DataZeroTrustCasbPolicy_ActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustCasbPolicy_ActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `remediation_types` after provisioning.\nList of remediation types that will be executed."]
    pub fn remediation_types(&self) -> terrars::ListRef<DataZeroTrustCasbPolicy_Actions_RemediationTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remediation_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook_configs` after provisioning.\nList of webhook configurations that will be triggered."]
    pub fn webhook_configs(&self) -> terrars::ListRef<DataZeroTrustCasbPolicy_Actions_WebhookConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webhook_configs", self.base))
    }
}

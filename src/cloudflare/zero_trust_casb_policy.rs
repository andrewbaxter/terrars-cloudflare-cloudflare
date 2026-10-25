#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustCasbPolicyData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    account_id: terrars::PrimField<std::string::String>,
    actions: ZeroTrustCasbPolicy_Actions,
    applies_to_all_integrations: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    enabled: terrars::PrimField<bool>,
    finding_type_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

struct ZeroTrustCasbPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustCasbPolicyData_>,
}

#[derive(Clone)]
pub struct ZeroTrustCasbPolicy(std::rc::Rc<ZeroTrustCasbPolicy_>);

impl ZeroTrustCasbPolicy {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Set the field `description`.\nOptional description of what this policy does."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `integration_ids`.\nThe integrations this policy applies to. Required when applies_to_all_integrations is false."]
    pub fn set_integration_ids(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().integration_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nActions to execute when this policy is triggered, grouped by action type.\nA policy must contain at least one action across all groups and may include\nat most one remediation."]
    pub fn actions(&self) -> ZeroTrustCasbPolicy_ActionsRef {
        ZeroTrustCasbPolicy_ActionsRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `applies_to_all_integrations` after provisioning.\nWhen true, the policy applies to all integrations for the account. When false, integration_ids must be provided."]
    pub fn applies_to_all_integrations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.applies_to_all_integrations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the policy was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional description of what this policy does."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled_at` after provisioning.\nTimestamp when the policy was disabled. Omitted from the response when the policy\nis enabled."]
    pub fn disabled_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name for the policy configuration."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nBoolean specifying if the policy is enabled or disabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `finding_type_id` after provisioning.\nThe finding type this policy is associated with. All remediation actions must match this finding type."]
    pub fn finding_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finding_type_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the policy configuration."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `integration_ids` after provisioning.\nThe integrations this policy applies to. Required when applies_to_all_integrations is false."]
    pub fn integration_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_triggered_at` after provisioning.\nTimestamp of the most recent successful policy invocation. Omitted\nfrom the response when the policy has never been successfully\ntriggered. Only populated on GET responses; absent on responses from\ncreate/update endpoints."]
    pub fn last_triggered_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_triggered_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the policy was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustCasbPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustCasbPolicy { }

impl terrars::ToListMappable for ZeroTrustCasbPolicy {
    type O = terrars::ListRef<ZeroTrustCasbPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustCasbPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_casb_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustCasbPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Actions to execute when this policy is triggered, grouped by action type.\nA policy must contain at least one action across all groups and may include\nat most one remediation."]
    pub actions: ZeroTrustCasbPolicy_Actions,
    #[doc =
        "When true, the policy applies to all integrations for the account. When false, integration_ids must be provided."]
    pub applies_to_all_integrations: terrars::PrimField<bool>,
    #[doc = "Display name for the policy configuration."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "Boolean specifying if the policy is enabled or disabled."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "The finding type this policy is associated with. All remediation actions must match this finding type."]
    pub finding_type_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustCasbPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustCasbPolicy {
        let out = ZeroTrustCasbPolicy(std::rc::Rc::new(ZeroTrustCasbPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustCasbPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                actions: self.actions,
                applies_to_all_integrations: self.applies_to_all_integrations,
                description: core::default::Default::default(),
                display_name: self.display_name,
                enabled: self.enabled,
                finding_type_id: self.finding_type_id,
                integration_ids: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustCasbPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustCasbPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nActions to execute when this policy is triggered, grouped by action type.\nA policy must contain at least one action across all groups and may include\nat most one remediation."]
    pub fn actions(&self) -> ZeroTrustCasbPolicy_ActionsRef {
        ZeroTrustCasbPolicy_ActionsRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `applies_to_all_integrations` after provisioning.\nWhen true, the policy applies to all integrations for the account. When false, integration_ids must be provided."]
    pub fn applies_to_all_integrations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.applies_to_all_integrations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the policy was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional description of what this policy does."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled_at` after provisioning.\nTimestamp when the policy was disabled. Omitted from the response when the policy\nis enabled."]
    pub fn disabled_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name for the policy configuration."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nBoolean specifying if the policy is enabled or disabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `finding_type_id` after provisioning.\nThe finding type this policy is associated with. All remediation actions must match this finding type."]
    pub fn finding_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finding_type_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the policy configuration."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `integration_ids` after provisioning.\nThe integrations this policy applies to. Required when applies_to_all_integrations is false."]
    pub fn integration_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_triggered_at` after provisioning.\nTimestamp of the most recent successful policy invocation. Omitted\nfrom the response when the policy has never been successfully\ntriggered. Only populated on GET responses; absent on responses from\ncreate/update endpoints."]
    pub fn last_triggered_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_triggered_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the policy was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbPolicy_Actions_RemediationTypes {
    remediation_type_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustCasbPolicy_Actions_RemediationTypes { }

impl terrars::ToListMappable for ZeroTrustCasbPolicy_Actions_RemediationTypes {
    type O = terrars::BlockAssignable<ZeroTrustCasbPolicy_Actions_RemediationTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbPolicy_Actions_RemediationTypes {
    #[doc = "The ID of the remediation type to execute."]
    pub remediation_type_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustCasbPolicy_Actions_RemediationTypes {
    pub fn build(self) -> ZeroTrustCasbPolicy_Actions_RemediationTypes {
        ZeroTrustCasbPolicy_Actions_RemediationTypes { remediation_type_id: self.remediation_type_id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbPolicy_Actions_RemediationTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbPolicy_Actions_RemediationTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustCasbPolicy_Actions_RemediationTypesRef {
        ZeroTrustCasbPolicy_Actions_RemediationTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbPolicy_Actions_RemediationTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `remediation_type_id` after provisioning.\nThe ID of the remediation type to execute."]
    pub fn remediation_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remediation_type_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbPolicy_Actions_WebhookConfigs {
    webhook_config_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustCasbPolicy_Actions_WebhookConfigs { }

impl terrars::ToListMappable for ZeroTrustCasbPolicy_Actions_WebhookConfigs {
    type O = terrars::BlockAssignable<ZeroTrustCasbPolicy_Actions_WebhookConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbPolicy_Actions_WebhookConfigs {
    #[doc = "The ID of the webhook configuration to use."]
    pub webhook_config_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustCasbPolicy_Actions_WebhookConfigs {
    pub fn build(self) -> ZeroTrustCasbPolicy_Actions_WebhookConfigs {
        ZeroTrustCasbPolicy_Actions_WebhookConfigs { webhook_config_id: self.webhook_config_id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbPolicy_Actions_WebhookConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbPolicy_Actions_WebhookConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustCasbPolicy_Actions_WebhookConfigsRef {
        ZeroTrustCasbPolicy_Actions_WebhookConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbPolicy_Actions_WebhookConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `webhook_config_id` after provisioning.\nThe ID of the webhook configuration to use."]
    pub fn webhook_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_config_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbPolicy_Actions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remediation_types: std::option::Option<std::vec::Vec<ZeroTrustCasbPolicy_Actions_RemediationTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook_configs: std::option::Option<std::vec::Vec<ZeroTrustCasbPolicy_Actions_WebhookConfigs>>,
}

impl ZeroTrustCasbPolicy_Actions {
    #[doc = "Set the field `remediation_types`.\nRemediation actions to execute (at most one)."]
    pub fn set_remediation_types(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustCasbPolicy_Actions_RemediationTypes>>,
    ) -> Self {
        self.remediation_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `webhook_configs`.\nWebhook actions to execute."]
    pub fn set_webhook_configs(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustCasbPolicy_Actions_WebhookConfigs>>,
    ) -> Self {
        self.webhook_configs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustCasbPolicy_Actions {
    type O = terrars::BlockAssignable<ZeroTrustCasbPolicy_Actions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbPolicy_Actions {}

impl BuildZeroTrustCasbPolicy_Actions {
    pub fn build(self) -> ZeroTrustCasbPolicy_Actions {
        ZeroTrustCasbPolicy_Actions {
            remediation_types: core::default::Default::default(),
            webhook_configs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbPolicy_ActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbPolicy_ActionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustCasbPolicy_ActionsRef {
        ZeroTrustCasbPolicy_ActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbPolicy_ActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `remediation_types` after provisioning.\nRemediation actions to execute (at most one)."]
    pub fn remediation_types(&self) -> terrars::ListRef<ZeroTrustCasbPolicy_Actions_RemediationTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remediation_types", self.base))
    }

    #[doc = "Get a reference to the value of field `webhook_configs` after provisioning.\nWebhook actions to execute."]
    pub fn webhook_configs(&self) -> terrars::ListRef<ZeroTrustCasbPolicy_Actions_WebhookConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webhook_configs", self.base))
    }
}

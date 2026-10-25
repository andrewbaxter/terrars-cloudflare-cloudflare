#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustGatewayPolicyData_ {
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
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_posture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration: std::option::Option<ZeroTrustGatewayPolicy_Expiration>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    precedence: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_settings: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<ZeroTrustGatewayPolicy_Schedule>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ZeroTrustGatewayPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustGatewayPolicyData_>,
}

#[derive(Clone)]
pub struct ZeroTrustGatewayPolicy(std::rc::Rc<ZeroTrustGatewayPolicy_>);

impl ZeroTrustGatewayPolicy {
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

    #[doc = "Set the field `description`.\nSpecify the rule description."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `device_posture`.\nSpecify the wirefilter expression used for device posture check. The API automatically formats and sanitizes expressions before storing them. To prevent Terraform state drift, use the formatted expression returned in the API response."]
    pub fn set_device_posture(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().device_posture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nSpecify whether the rule is enabled."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expiration`.\nDefines the expiration time stamp and default duration of a DNS policy. Takes precedence over the policy's `schedule` configuration, if any. This  does not apply to HTTP or network policies. Settable only for `dns` rules."]
    pub fn set_expiration(self, v: impl std::convert::Into<ZeroTrustGatewayPolicy_Expiration>) -> Self {
        self.0.data.borrow_mut().expiration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filters`.\nSpecify the protocol or layer to evaluate the traffic, identity, and device posture expressions. Can only contain a single value."]
    pub fn set_filters(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().filters = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity`.\nSpecify the wirefilter expression used for identity matching. The API automatically formats and sanitizes expressions before storing them. To prevent Terraform state drift, use the formatted expression returned in the API response."]
    pub fn set_identity(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().identity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `precedence`.\nSet the order of your rules. Lower values indicate higher precedence. At each processing phase, evaluate applicable rules in ascending order of this value. Refer to [Order of enforcement](http://developers.cloudflare.com/learning-paths/secure-internet-traffic/understand-policies/order-of-enforcement/#manage-precedence-with-terraform) to manage precedence via Terraform."]
    pub fn set_precedence(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().precedence = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rule_settings`.\nDefines settings for this rule. Settings apply only to specific rule types and must use compatible selectors. If Terraform detects drift, confirm the setting supports your rule type and check whether the API modifies the value. Use API-returned values in your configuration to prevent drift."]
    pub fn set_rule_settings(self, v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings>) -> Self {
        self.0.data.borrow_mut().rule_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schedule`.\nDefines the schedule for activating DNS policies. Settable only for `dns` and `dns_resolver` rules."]
    pub fn set_schedule(self, v: impl std::convert::Into<ZeroTrustGatewayPolicy_Schedule>) -> Self {
        self.0.data.borrow_mut().schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `traffic`.\nSpecify the wirefilter expression used for traffic matching. The API automatically formats and sanitizes expressions before storing them. To prevent Terraform state drift, use the formatted expression returned in the API response."]
    pub fn set_traffic(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().traffic = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nSpecify the action to perform when the associated traffic, identity, and device posture expressions either absent or evaluate to `true`.\nAvailable values: \"on\", \"off\", \"allow\", \"block\", \"scan\", \"noscan\", \"safesearch\", \"ytrestricted\", \"isolate\", \"noisolate\", \"override\", \"l4_override\", \"egress\", \"resolve\", \"quarantine\", \"redirect\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deleted_at` after provisioning.\nIndicate the date of deletion, if any."]
    pub fn deleted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nSpecify the rule description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `device_posture` after provisioning.\nSpecify the wirefilter expression used for device posture check. The API automatically formats and sanitizes expressions before storing them. To prevent Terraform state drift, use the formatted expression returned in the API response."]
    pub fn device_posture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_posture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nSpecify whether the rule is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expiration` after provisioning.\nDefines the expiration time stamp and default duration of a DNS policy. Takes precedence over the policy's `schedule` configuration, if any. This  does not apply to HTTP or network policies. Settable only for `dns` rules."]
    pub fn expiration(&self) -> ZeroTrustGatewayPolicy_ExpirationRef {
        ZeroTrustGatewayPolicy_ExpirationRef::new(
            self.shared().clone(),
            format!("{}.expiration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `filters` after provisioning.\nSpecify the protocol or layer to evaluate the traffic, identity, and device posture expressions. Can only contain a single value."]
    pub fn filters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentify the API resource with a UUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identity` after provisioning.\nSpecify the wirefilter expression used for identity matching. The API automatically formats and sanitizes expressions before storing them. To prevent Terraform state drift, use the formatted expression returned in the API response."]
    pub fn identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecify the rule name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `precedence` after provisioning.\nSet the order of your rules. Lower values indicate higher precedence. At each processing phase, evaluate applicable rules in ascending order of this value. Refer to [Order of enforcement](http://developers.cloudflare.com/learning-paths/secure-internet-traffic/understand-policies/order-of-enforcement/#manage-precedence-with-terraform) to manage precedence via Terraform."]
    pub fn precedence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nIndicate that this rule is shared via the Orgs API and read only."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_settings` after provisioning.\nDefines settings for this rule. Settings apply only to specific rule types and must use compatible selectors. If Terraform detects drift, confirm the setting supports your rule type and check whether the API modifies the value. Use API-returned values in your configuration to prevent drift."]
    pub fn rule_settings(&self) -> ZeroTrustGatewayPolicy_RuleSettingsRef {
        ZeroTrustGatewayPolicy_RuleSettingsRef::new(
            self.shared().clone(),
            format!("{}.rule_settings", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nDefines the schedule for activating DNS policies. Settable only for `dns` and `dns_resolver` rules."]
    pub fn schedule(&self) -> ZeroTrustGatewayPolicy_ScheduleRef {
        ZeroTrustGatewayPolicy_ScheduleRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sharable` after provisioning.\nIndicate that this rule is sharable via the Orgs API."]
    pub fn sharable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sharable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_account` after provisioning.\nProvide the account tag of the account that created the rule."]
    pub fn source_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic` after provisioning.\nSpecify the wirefilter expression used for traffic matching. The API automatically formats and sanitizes expressions before storing them. To prevent Terraform state drift, use the formatted expression returned in the API response."]
    pub fn traffic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nIndicate the version number of the rule(read-only)."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `warning_status` after provisioning.\nIndicate a warning for a misconfigured rule, if any."]
    pub fn warning_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warning_status", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustGatewayPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustGatewayPolicy { }

impl terrars::ToListMappable for ZeroTrustGatewayPolicy {
    type O = terrars::ListRef<ZeroTrustGatewayPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustGatewayPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_gateway_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustGatewayPolicy {
    pub tf_id: std::string::String,
    #[doc = "Specify the Cloudflare account identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Specify the action to perform when the associated traffic, identity, and device posture expressions either absent or evaluate to `true`.\nAvailable values: \"on\", \"off\", \"allow\", \"block\", \"scan\", \"noscan\", \"safesearch\", \"ytrestricted\", \"isolate\", \"noisolate\", \"override\", \"l4_override\", \"egress\", \"resolve\", \"quarantine\", \"redirect\"."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = "Specify the rule name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustGatewayPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustGatewayPolicy {
        let out = ZeroTrustGatewayPolicy(std::rc::Rc::new(ZeroTrustGatewayPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustGatewayPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                action: self.action,
                description: core::default::Default::default(),
                device_posture: core::default::Default::default(),
                enabled: core::default::Default::default(),
                expiration: core::default::Default::default(),
                filters: core::default::Default::default(),
                identity: core::default::Default::default(),
                name: self.name,
                precedence: core::default::Default::default(),
                rule_settings: core::default::Default::default(),
                schedule: core::default::Default::default(),
                traffic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustGatewayPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustGatewayPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nSpecify the action to perform when the associated traffic, identity, and device posture expressions either absent or evaluate to `true`.\nAvailable values: \"on\", \"off\", \"allow\", \"block\", \"scan\", \"noscan\", \"safesearch\", \"ytrestricted\", \"isolate\", \"noisolate\", \"override\", \"l4_override\", \"egress\", \"resolve\", \"quarantine\", \"redirect\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deleted_at` after provisioning.\nIndicate the date of deletion, if any."]
    pub fn deleted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nSpecify the rule description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `device_posture` after provisioning.\nSpecify the wirefilter expression used for device posture check. The API automatically formats and sanitizes expressions before storing them. To prevent Terraform state drift, use the formatted expression returned in the API response."]
    pub fn device_posture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_posture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nSpecify whether the rule is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expiration` after provisioning.\nDefines the expiration time stamp and default duration of a DNS policy. Takes precedence over the policy's `schedule` configuration, if any. This  does not apply to HTTP or network policies. Settable only for `dns` rules."]
    pub fn expiration(&self) -> ZeroTrustGatewayPolicy_ExpirationRef {
        ZeroTrustGatewayPolicy_ExpirationRef::new(
            self.shared().clone(),
            format!("{}.expiration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `filters` after provisioning.\nSpecify the protocol or layer to evaluate the traffic, identity, and device posture expressions. Can only contain a single value."]
    pub fn filters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentify the API resource with a UUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identity` after provisioning.\nSpecify the wirefilter expression used for identity matching. The API automatically formats and sanitizes expressions before storing them. To prevent Terraform state drift, use the formatted expression returned in the API response."]
    pub fn identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecify the rule name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `precedence` after provisioning.\nSet the order of your rules. Lower values indicate higher precedence. At each processing phase, evaluate applicable rules in ascending order of this value. Refer to [Order of enforcement](http://developers.cloudflare.com/learning-paths/secure-internet-traffic/understand-policies/order-of-enforcement/#manage-precedence-with-terraform) to manage precedence via Terraform."]
    pub fn precedence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nIndicate that this rule is shared via the Orgs API and read only."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_settings` after provisioning.\nDefines settings for this rule. Settings apply only to specific rule types and must use compatible selectors. If Terraform detects drift, confirm the setting supports your rule type and check whether the API modifies the value. Use API-returned values in your configuration to prevent drift."]
    pub fn rule_settings(&self) -> ZeroTrustGatewayPolicy_RuleSettingsRef {
        ZeroTrustGatewayPolicy_RuleSettingsRef::new(
            self.shared().clone(),
            format!("{}.rule_settings", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nDefines the schedule for activating DNS policies. Settable only for `dns` and `dns_resolver` rules."]
    pub fn schedule(&self) -> ZeroTrustGatewayPolicy_ScheduleRef {
        ZeroTrustGatewayPolicy_ScheduleRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sharable` after provisioning.\nIndicate that this rule is sharable via the Orgs API."]
    pub fn sharable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sharable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_account` after provisioning.\nProvide the account tag of the account that created the rule."]
    pub fn source_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic` after provisioning.\nSpecify the wirefilter expression used for traffic matching. The API automatically formats and sanitizes expressions before storing them. To prevent Terraform state drift, use the formatted expression returned in the API response."]
    pub fn traffic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nIndicate the version number of the rule(read-only)."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `warning_status` after provisioning.\nIndicate a warning for a misconfigured rule, if any."]
    pub fn warning_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warning_status", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_Expiration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<f64>>,
    expires_at: terrars::PrimField<std::string::String>,
}

impl ZeroTrustGatewayPolicy_Expiration {
    #[doc =
        "Set the field `duration`.\nDefines the default duration a policy active in minutes. Must set in order to use the `reset_expiration` endpoint on this rule."]
    pub fn set_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_Expiration {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_Expiration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_Expiration {
    #[doc =
        "Show the timestamp when the policy expires and stops applying.  The value must follow RFC 3339 and include a UTC offset.  The system accepts non-zero offsets but converts them to the equivalent UTC+00:00  value and returns timestamps with a trailing Z. Expiration policies ignore client  timezones and expire globally at the specified expires_at time."]
    pub expires_at: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustGatewayPolicy_Expiration {
    pub fn build(self) -> ZeroTrustGatewayPolicy_Expiration {
        ZeroTrustGatewayPolicy_Expiration {
            duration: core::default::Default::default(),
            expires_at: self.expires_at,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_ExpirationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_ExpirationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustGatewayPolicy_ExpirationRef {
        ZeroTrustGatewayPolicy_ExpirationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_ExpirationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nDefines the default duration a policy active in minutes. Must set in order to use the `reset_expiration` endpoint on this rule."]
    pub fn duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expired` after provisioning.\nIndicates whether the policy is expired."]
    pub fn expired(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expired", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expires_at` after provisioning.\nShow the timestamp when the policy expires and stops applying.  The value must follow RFC 3339 and include a UTC offset.  The system accepts non-zero offsets but converts them to the equivalent UTC+00:00  value and returns timestamps with a trailing Z. Expiration policies ignore client  timezones and expire globally at the specified expires_at time."]
    pub fn expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_AuditSsh {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command_logging: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_AuditSsh {
    #[doc = "Set the field `command_logging`.\nEnable SSH command logging."]
    pub fn set_command_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.command_logging = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_AuditSsh {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_AuditSsh>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_AuditSsh {}

impl BuildZeroTrustGatewayPolicy_RuleSettings_AuditSsh {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_AuditSsh {
        ZeroTrustGatewayPolicy_RuleSettings_AuditSsh { command_logging: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_AuditSshRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_AuditSshRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_AuditSshRef {
        ZeroTrustGatewayPolicy_RuleSettings_AuditSshRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_AuditSshRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `command_logging` after provisioning.\nEnable SSH command logging."]
    pub fn command_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.command_logging", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dcp: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dd: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dk: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    download: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dp: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    du: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keyboard: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paste: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    printing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wm_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls {
    #[doc =
        "Set the field `copy`.\nConfigure copy behavior. If set to remote_only, users cannot copy isolated content from the remote browser to the local clipboard. If this field is absent, copying remains enabled. Applies only when version == \"v2\".\nAvailable values: \"enabled\", \"disabled\", \"remote_only\"."]
    pub fn set_copy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.copy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dcp`.\nSet to false to enable copy-pasting. Only applies when `version == \"v1\"`."]
    pub fn set_dcp(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dcp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dd`.\nSet to false to enable downloading. Only applies when `version == \"v1\"`."]
    pub fn set_dd(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dd = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dk`.\nSet to false to enable keyboard usage. Only applies when `version == \"v1\"`."]
    pub fn set_dk(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dk = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `download`.\nConfigure download behavior. When set to remote_only, users can view downloads but cannot save them. If this field is absent, downloading remains enabled. Applies only when version == \"v2\".\nAvailable values: \"enabled\", \"disabled\", \"remote_only\"."]
    pub fn set_download(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.download = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dp`.\nSet to false to enable printing. Only applies when `version == \"v1\"`."]
    pub fn set_dp(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `du`.\nSet to false to enable uploading. Only applies when `version == \"v1\"`."]
    pub fn set_du(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.du = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `keyboard`.\nConfigure keyboard usage behavior. If this field is absent, keyboard usage remains enabled. Applies only when version == \"v2\".\nAvailable values: \"enabled\", \"disabled\"."]
    pub fn set_keyboard(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.keyboard = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `paste`.\nConfigure paste behavior. If set to remote_only, users cannot paste content from the local clipboard into isolated pages. If this field is absent, pasting remains enabled. Applies only when version == \"v2\".\nAvailable values: \"enabled\", \"disabled\", \"remote_only\"."]
    pub fn set_paste(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.paste = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `printing`.\nConfigure print behavior. Default, Printing is enabled. Applies only when version == \"v2\".\nAvailable values: \"enabled\", \"disabled\"."]
    pub fn set_printing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.printing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `upload`.\nConfigure upload behavior. If this field is absent, uploading remains enabled. Applies only when version == \"v2\".\nAvailable values: \"enabled\", \"disabled\"."]
    pub fn set_upload(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.upload = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nIndicate which version of the browser isolation controls should apply.\nAvailable values: \"v1\", \"v2\"."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wm_id`.\nSpecify the watermark ID (UUID) to apply to the isolated browser session. When present, enables watermark rendering in the isolated browser."]
    pub fn set_wm_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.wm_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls {}

impl BuildZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls {
        ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls {
            copy: core::default::Default::default(),
            dcp: core::default::Default::default(),
            dd: core::default::Default::default(),
            dk: core::default::Default::default(),
            download: core::default::Default::default(),
            dp: core::default::Default::default(),
            du: core::default::Default::default(),
            keyboard: core::default::Default::default(),
            paste: core::default::Default::default(),
            printing: core::default::Default::default(),
            upload: core::default::Default::default(),
            version: core::default::Default::default(),
            wm_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef {
        ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `copy` after provisioning.\nConfigure copy behavior. If set to remote_only, users cannot copy isolated content from the remote browser to the local clipboard. If this field is absent, copying remains enabled. Applies only when version == \"v2\".\nAvailable values: \"enabled\", \"disabled\", \"remote_only\"."]
    pub fn copy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dcp` after provisioning.\nSet to false to enable copy-pasting. Only applies when `version == \"v1\"`."]
    pub fn dcp(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dcp", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dd` after provisioning.\nSet to false to enable downloading. Only applies when `version == \"v1\"`."]
    pub fn dd(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dd", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dk` after provisioning.\nSet to false to enable keyboard usage. Only applies when `version == \"v1\"`."]
    pub fn dk(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dk", self.base))
    }

    #[doc =
        "Get a reference to the value of field `download` after provisioning.\nConfigure download behavior. When set to remote_only, users can view downloads but cannot save them. If this field is absent, downloading remains enabled. Applies only when version == \"v2\".\nAvailable values: \"enabled\", \"disabled\", \"remote_only\"."]
    pub fn download(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.download", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dp` after provisioning.\nSet to false to enable printing. Only applies when `version == \"v1\"`."]
    pub fn dp(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dp", self.base))
    }

    #[doc =
        "Get a reference to the value of field `du` after provisioning.\nSet to false to enable uploading. Only applies when `version == \"v1\"`."]
    pub fn du(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.du", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keyboard` after provisioning.\nConfigure keyboard usage behavior. If this field is absent, keyboard usage remains enabled. Applies only when version == \"v2\".\nAvailable values: \"enabled\", \"disabled\"."]
    pub fn keyboard(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyboard", self.base))
    }

    #[doc =
        "Get a reference to the value of field `paste` after provisioning.\nConfigure paste behavior. If set to remote_only, users cannot paste content from the local clipboard into isolated pages. If this field is absent, pasting remains enabled. Applies only when version == \"v2\".\nAvailable values: \"enabled\", \"disabled\", \"remote_only\"."]
    pub fn paste(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paste", self.base))
    }

    #[doc =
        "Get a reference to the value of field `printing` after provisioning.\nConfigure print behavior. Default, Printing is enabled. Applies only when version == \"v2\".\nAvailable values: \"enabled\", \"disabled\"."]
    pub fn printing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.printing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `upload` after provisioning.\nConfigure upload behavior. If this field is absent, uploading remains enabled. Applies only when version == \"v2\".\nAvailable values: \"enabled\", \"disabled\"."]
    pub fn upload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nIndicate which version of the browser isolation controls should apply.\nAvailable values: \"v1\", \"v2\"."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wm_id` after provisioning.\nSpecify the watermark ID (UUID) to apply to the isolated browser session. When present, enables watermark rendering in the isolated browser."]
    pub fn wm_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wm_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_BlockPage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_context: std::option::Option<terrars::PrimField<bool>>,
    target_uri: terrars::PrimField<std::string::String>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_BlockPage {
    #[doc = "Set the field `include_context`.\nSpecify whether to pass the context information as query parameters."]
    pub fn set_include_context(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_context = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_BlockPage {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_BlockPage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_BlockPage {
    #[doc = "Specify the URI to which the user is redirected."]
    pub target_uri: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustGatewayPolicy_RuleSettings_BlockPage {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_BlockPage {
        ZeroTrustGatewayPolicy_RuleSettings_BlockPage {
            include_context: core::default::Default::default(),
            target_uri: self.target_uri,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_BlockPageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_BlockPageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_BlockPageRef {
        ZeroTrustGatewayPolicy_RuleSettings_BlockPageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_BlockPageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `include_context` after provisioning.\nSpecify whether to pass the context information as query parameters."]
    pub fn include_context(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_context", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_uri` after provisioning.\nSpecify the URI to which the user is redirected."]
    pub fn target_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_CheckSession {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_CheckSession {
    #[doc =
        "Set the field `duration`.\nSets the required session freshness threshold. The API returns a normalized version of this value."]
    pub fn set_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforce`.\nEnable session enforcement."]
    pub fn set_enforce(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforce = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_CheckSession {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_CheckSession>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_CheckSession {}

impl BuildZeroTrustGatewayPolicy_RuleSettings_CheckSession {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_CheckSession {
        ZeroTrustGatewayPolicy_RuleSettings_CheckSession {
            duration: core::default::Default::default(),
            enforce: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef {
        ZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nSets the required session freshness threshold. The API returns a normalized version of this value."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce` after provisioning.\nEnable session enforcement."]
    pub fn enforce(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 {
    ip: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_through_private_network: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 {
    #[doc =
        "Set the field `port`.\nSpecify a port number to use for the upstream resolver. Defaults to 53 if unspecified."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `route_through_private_network`.\nIndicate whether to connect to this resolver over a private network. Must set when vnet_id set."]
    pub fn set_route_through_private_network(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.route_through_private_network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vnet_id`.\nSpecify an optional virtual network for this resolver. Uses default virtual network id if omitted."]
    pub fn set_vnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vnet_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 {
    #[doc = "Specify the IPv4 address of the upstream resolver."]
    pub ip: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 {
        ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 {
            ip: self.ip,
            port: core::default::Default::default(),
            route_through_private_network: core::default::Default::default(),
            vnet_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4Ref {
        ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nSpecify the IPv4 address of the upstream resolver."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nSpecify a port number to use for the upstream resolver. Defaults to 53 if unspecified."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `route_through_private_network` after provisioning.\nIndicate whether to connect to this resolver over a private network. Must set when vnet_id set."]
    pub fn route_through_private_network(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_through_private_network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vnet_id` after provisioning.\nSpecify an optional virtual network for this resolver. Uses default virtual network id if omitted."]
    pub fn vnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 {
    ip: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_through_private_network: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 {
    #[doc =
        "Set the field `port`.\nSpecify a port number to use for the upstream resolver. Defaults to 53 if unspecified."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `route_through_private_network`.\nIndicate whether to connect to this resolver over a private network. Must set when vnet_id set."]
    pub fn set_route_through_private_network(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.route_through_private_network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vnet_id`.\nSpecify an optional virtual network for this resolver. Uses default virtual network id if omitted."]
    pub fn set_vnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vnet_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 {
    #[doc = "Specify the IPv6 address of the upstream resolver."]
    pub ip: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 {
        ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 {
            ip: self.ip,
            port: core::default::Default::default(),
            route_through_private_network: core::default::Default::default(),
            vnet_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6Ref {
        ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nSpecify the IPv6 address of the upstream resolver."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nSpecify a port number to use for the upstream resolver. Defaults to 53 if unspecified."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `route_through_private_network` after provisioning.\nIndicate whether to connect to this resolver over a private network. Must set when vnet_id set."]
    pub fn route_through_private_network(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_through_private_network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vnet_id` after provisioning.\nSpecify an optional virtual network for this resolver. Uses default virtual network id if omitted."]
    pub fn vnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4: std::option::Option<std::vec::Vec<ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6: std::option::Option<std::vec::Vec<ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers {
    #[doc = "Set the field `ipv4`.\n"]
    pub fn set_ipv4(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4>>,
    ) -> Self {
        self.ipv4 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6`.\n"]
    pub fn set_ipv6(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6>>,
    ) -> Self {
        self.ipv6 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_DnsResolvers {}

impl BuildZeroTrustGatewayPolicy_RuleSettings_DnsResolvers {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers {
        ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers {
            ipv4: core::default::Default::default(),
            ipv6: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef {
        ZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv4` after provisioning.\n"]
    pub fn ipv4(&self) -> terrars::ListRef<ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv4", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> terrars::ListRef<ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_Egress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_fallback: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_Egress {
    #[doc = "Set the field `ipv4`.\nSpecify the IPv4 address to use for egress."]
    pub fn set_ipv4(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv4 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv4_fallback`.\nSpecify the fallback IPv4 address to use for egress when the primary IPv4 fails. Set '0.0.0.0' to indicate local egress via WARP IPs."]
    pub fn set_ipv4_fallback(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv4_fallback = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6`.\nSpecify the IPv6 range to use for egress."]
    pub fn set_ipv6(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_Egress {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_Egress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_Egress {}

impl BuildZeroTrustGatewayPolicy_RuleSettings_Egress {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_Egress {
        ZeroTrustGatewayPolicy_RuleSettings_Egress {
            ipv4: core::default::Default::default(),
            ipv4_fallback: core::default::Default::default(),
            ipv6: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_EgressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_EgressRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustGatewayPolicy_RuleSettings_EgressRef {
        ZeroTrustGatewayPolicy_RuleSettings_EgressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_EgressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ipv4` after provisioning.\nSpecify the IPv4 address to use for egress."]
    pub fn ipv4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_fallback` after provisioning.\nSpecify the fallback IPv4 address to use for egress when the primary IPv4 fails. Set '0.0.0.0' to indicate local egress via WARP IPs."]
    pub fn ipv4_fallback(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_fallback", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv6` after provisioning.\nSpecify the IPv6 range to use for egress."]
    pub fn ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_ForensicCopy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_ForensicCopy {
    #[doc = "Set the field `enabled`.\nEnable sending the copy to storage."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_ForensicCopy {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_ForensicCopy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_ForensicCopy {}

impl BuildZeroTrustGatewayPolicy_RuleSettings_ForensicCopy {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_ForensicCopy {
        ZeroTrustGatewayPolicy_RuleSettings_ForensicCopy { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef {
        ZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnable sending the copy to storage."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_L4override {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_L4override {
    #[doc = "Set the field `ip`.\nDefines the IPv4 or IPv6 address."]
    pub fn set_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nDefines a port number to use for TCP/UDP overrides."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_L4override {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_L4override>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_L4override {}

impl BuildZeroTrustGatewayPolicy_RuleSettings_L4override {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_L4override {
        ZeroTrustGatewayPolicy_RuleSettings_L4override {
            ip: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_L4overrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_L4overrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_L4overrideRef {
        ZeroTrustGatewayPolicy_RuleSettings_L4overrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_L4overrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\nDefines the IPv4 or IPv6 address."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nDefines a port number to use for TCP/UDP overrides."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_NotificationSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_context: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    msg: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    support_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_NotificationSettings {
    #[doc = "Set the field `enabled`.\nEnable notification."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_context`.\nIndicates whether to pass the context information as query parameters."]
    pub fn set_include_context(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `msg`.\nCustomize the message shown in the notification."]
    pub fn set_msg(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.msg = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `support_url`.\nDefines an optional URL to direct users to additional information. If unset, the notification opens a block page."]
    pub fn set_support_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.support_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_NotificationSettings {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_NotificationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_NotificationSettings {}

impl BuildZeroTrustGatewayPolicy_RuleSettings_NotificationSettings {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_NotificationSettings {
        ZeroTrustGatewayPolicy_RuleSettings_NotificationSettings {
            enabled: core::default::Default::default(),
            include_context: core::default::Default::default(),
            msg: core::default::Default::default(),
            support_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef {
        ZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnable notification."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_context` after provisioning.\nIndicates whether to pass the context information as query parameters."]
    pub fn include_context(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_context", self.base))
    }

    #[doc =
        "Get a reference to the value of field `msg` after provisioning.\nCustomize the message shown in the notification."]
    pub fn msg(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.msg", self.base))
    }

    #[doc =
        "Get a reference to the value of field `support_url` after provisioning.\nDefines an optional URL to direct users to additional information. If unset, the notification opens a block page."]
    pub fn support_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_url", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_PayloadLog {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_PayloadLog {
    #[doc = "Set the field `enabled`.\nEnable DLP payload logging for this rule."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_PayloadLog {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_PayloadLog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_PayloadLog {}

impl BuildZeroTrustGatewayPolicy_RuleSettings_PayloadLog {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_PayloadLog {
        ZeroTrustGatewayPolicy_RuleSettings_PayloadLog { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef {
        ZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnable DLP payload logging for this rule."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_Quarantine {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_Quarantine {
    #[doc = "Set the field `file_types`.\nSpecify the types of files to sandbox."]
    pub fn set_file_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_Quarantine {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_Quarantine>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_Quarantine {}

impl BuildZeroTrustGatewayPolicy_RuleSettings_Quarantine {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_Quarantine {
        ZeroTrustGatewayPolicy_RuleSettings_Quarantine { file_types: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_QuarantineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_QuarantineRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_QuarantineRef {
        ZeroTrustGatewayPolicy_RuleSettings_QuarantineRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_QuarantineRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `file_types` after provisioning.\nSpecify the types of files to sandbox."]
    pub fn file_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_types", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_Redirect {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_context: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_and_query: std::option::Option<terrars::PrimField<bool>>,
    target_uri: terrars::PrimField<std::string::String>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_Redirect {
    #[doc = "Set the field `include_context`.\nSpecify whether to pass the context information as query parameters."]
    pub fn set_include_context(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_context = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preserve_path_and_query`.\nSpecify whether to append the path and query parameters from the original request to target_uri."]
    pub fn set_preserve_path_and_query(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_path_and_query = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_Redirect {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_Redirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_Redirect {
    #[doc = "Specify the URI to which the user is redirected."]
    pub target_uri: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustGatewayPolicy_RuleSettings_Redirect {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_Redirect {
        ZeroTrustGatewayPolicy_RuleSettings_Redirect {
            include_context: core::default::Default::default(),
            preserve_path_and_query: core::default::Default::default(),
            target_uri: self.target_uri,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_RedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_RedirectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_RedirectRef {
        ZeroTrustGatewayPolicy_RuleSettings_RedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_RedirectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `include_context` after provisioning.\nSpecify whether to pass the context information as query parameters."]
    pub fn include_context(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_context", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preserve_path_and_query` after provisioning.\nSpecify whether to append the path and query parameters from the original request to target_uri."]
    pub fn preserve_path_and_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_and_query", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_uri` after provisioning.\nSpecify the URI to which the user is redirected."]
    pub fn target_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fallback: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally {
    #[doc =
        "Set the field `fallback`.\nSpecify the fallback behavior to apply when the internal DNS response code differs from 'NOERROR' or when the response data contains only CNAME records for 'A' or 'AAAA' queries.\nAvailable values: \"none\", \"public_dns\"."]
    pub fn set_fallback(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fallback = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view_id`.\nSpecify the internal DNS view identifier to pass to the internal DNS service."]
    pub fn set_view_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.view_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally {}

impl BuildZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally {
        ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally {
            fallback: core::default::Default::default(),
            view_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef {
        ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fallback` after provisioning.\nSpecify the fallback behavior to apply when the internal DNS response code differs from 'NOERROR' or when the response data contains only CNAME records for 'A' or 'AAAA' queries.\nAvailable values: \"none\", \"public_dns\"."]
    pub fn fallback(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback", self.base))
    }

    #[doc =
        "Get a reference to the value of field `view_id` after provisioning.\nSpecify the internal DNS view identifier to pass to the internal DNS service."]
    pub fn view_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_UntrustedCert {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustGatewayPolicy_RuleSettings_UntrustedCert {
    #[doc =
        "Set the field `action`.\nDefines the action performed when an untrusted certificate seen. The default action an error with HTTP code 526.\nAvailable values: \"pass_through\", \"block\", \"error\"."]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings_UntrustedCert {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings_UntrustedCert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings_UntrustedCert {}

impl BuildZeroTrustGatewayPolicy_RuleSettings_UntrustedCert {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings_UntrustedCert {
        ZeroTrustGatewayPolicy_RuleSettings_UntrustedCert { action: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef {
        ZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nDefines the action performed when an untrusted certificate seen. The default action an error with HTTP code 526.\nAvailable values: \"pass_through\", \"block\", \"error\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_RuleSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_headers: std::option::Option<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_child_bypass: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_ssh: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_AuditSsh>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    biso_admin_controls: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_page: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_BlockPage>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_page_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bypass_parent_rule: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_session: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_CheckSession>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_resolvers: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_Egress>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forensic_copy: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_ForensicCopy>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_cname_category_matches: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure_disable_dnssec_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_categories: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_indicator_feeds: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    l4override: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_L4override>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_settings: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_NotificationSettings>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_ips: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload_log: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_PayloadLog>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quarantine: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_Quarantine>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_Redirect>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolve_dns_internally: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolve_dns_through_cloudflare: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_headers: std::option::Option<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    untrusted_cert: std::option::Option<ZeroTrustGatewayPolicy_RuleSettings_UntrustedCert>,
}

impl ZeroTrustGatewayPolicy_RuleSettings {
    #[doc =
        "Set the field `add_headers`.\nAdd custom headers to allowed requests as key-value pairs. Use header names as keys that map to arrays of header values. Settable only for `http` rules with the action set to `allow`."]
    pub fn set_add_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.add_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_child_bypass`.\nSet to enable MSP children to bypass this rule. Only parent MSP accounts can set this. this rule. Settable for all types of rules."]
    pub fn set_allow_child_bypass(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_child_bypass = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `audit_ssh`.\nDefine the settings for the Audit SSH action. Settable only for `l4` rules with `audit_ssh` action."]
    pub fn set_audit_ssh(mut self, v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_AuditSsh>) -> Self {
        self.audit_ssh = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `biso_admin_controls`.\nConfigure browser isolation behavior. Settable only for `http` rules with the action set to `isolate`."]
    pub fn set_biso_admin_controls(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls>,
    ) -> Self {
        self.biso_admin_controls = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `block_page`.\nConfigure custom block page settings. If missing or null, use the account settings. Settable only for `http` rules with the action set to `block`."]
    pub fn set_block_page(mut self, v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_BlockPage>) -> Self {
        self.block_page = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `block_page_enabled`.\nEnable the custom block page. Settable only for `dns` rules with action `block`."]
    pub fn set_block_page_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.block_page_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `block_reason`.\nExplain why the rule blocks the request. The custom block page shows this text (if enabled). Settable only for `dns`, `l4`, and `http` rules when the action set to `block`."]
    pub fn set_block_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.block_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bypass_parent_rule`.\nSet to enable MSP accounts to bypass their parent's rules. Only MSP child accounts can set this. Settable for all types of rules."]
    pub fn set_bypass_parent_rule(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bypass_parent_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `check_session`.\nConfigure session check behavior. Settable only for `l4` and `http` rules with the action set to `allow`."]
    pub fn set_check_session(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_CheckSession>,
    ) -> Self {
        self.check_session = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete_headers`.\nRemove headers from allowed requests by name. A maximum of 20 header operations (add + set + delete) is allowed per policy. Each header name may not exceed 256 bytes. Settable only for `http` rules with the action set to `allow`."]
    pub fn set_delete_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.delete_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dns_resolvers`.\nConfigure custom resolvers to route queries that match the resolver policy. Unused with 'resolve_dns_through_cloudflare' or 'resolve_dns_internally' settings. DNS queries get routed to the address closest to their origin. Only valid when a rule's action set to 'resolve'. Settable only for `dns_resolver` rules."]
    pub fn set_dns_resolvers(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_DnsResolvers>,
    ) -> Self {
        self.dns_resolvers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `egress`.\nConfigure how Gateway Proxy traffic egresses. You can enable this setting for rules with Egress actions and filters, or omit it to indicate local egress via WARP IPs. Settable only for `egress` rules."]
    pub fn set_egress(mut self, v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_Egress>) -> Self {
        self.egress = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `forensic_copy`.\nConfigure whether a copy of the HTTP request will be sent to storage when the rule matches."]
    pub fn set_forensic_copy(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_ForensicCopy>,
    ) -> Self {
        self.forensic_copy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_cname_category_matches`.\nIgnore category matches at CNAME domains in a response. When off, evaluate categories in this rule against all CNAME domain categories in the response. Settable only for `dns` and `dns_resolver` rules."]
    pub fn set_ignore_cname_category_matches(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_cname_category_matches = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `insecure_disable_dnssec_validation`.\nSpecify whether to disable DNSSEC validation (for Allow actions) [INSECURE]. Settable only for `dns` rules."]
    pub fn set_insecure_disable_dnssec_validation(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.insecure_disable_dnssec_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_categories`.\nEnable IPs in DNS resolver category blocks. The system blocks only domain name categories unless you enable this setting. Settable only for `dns` and `dns_resolver` rules."]
    pub fn set_ip_categories(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ip_categories = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_indicator_feeds`.\nIndicates whether to include IPs in DNS resolver indicator feed blocks. Default, indicator feeds block only domain names. Settable only for `dns` and `dns_resolver` rules."]
    pub fn set_ip_indicator_feeds(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ip_indicator_feeds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `l4override`.\nSend matching traffic to the supplied destination IP address and port. Settable only for `l4` rules with the action set to `l4_override`."]
    pub fn set_l4override(mut self, v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_L4override>) -> Self {
        self.l4override = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `notification_settings`.\nConfigure a notification to display on the user's device when this rule matched. Settable for all types of rules with the action set to `block`."]
    pub fn set_notification_settings(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_NotificationSettings>,
    ) -> Self {
        self.notification_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `override_host`.\nDefines a hostname for override, for the matching DNS queries. Settable only for `dns` rules with the action set to `override`."]
    pub fn set_override_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.override_host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `override_ips`.\nDefines a an IP or set of IPs for overriding matched DNS queries. Settable only for `dns` rules with the action set to `override`."]
    pub fn set_override_ips(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.override_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payload_log`.\nConfigure DLP payload logging. Settable only for `http` rules."]
    pub fn set_payload_log(mut self, v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_PayloadLog>) -> Self {
        self.payload_log = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `quarantine`.\nConfigure settings that apply to quarantine rules. Settable only for `http` rules."]
    pub fn set_quarantine(mut self, v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_Quarantine>) -> Self {
        self.quarantine = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redirect`.\nApply settings to redirect rules. Settable only for `http` rules with the action set to `redirect`."]
    pub fn set_redirect(mut self, v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_Redirect>) -> Self {
        self.redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resolve_dns_internally`.\nConfigure to forward the query to the internal DNS service, passing the specified 'view_id' as input. Not used when 'dns_resolvers' is specified or 'resolve_dns_through_cloudflare' is set. Only valid when a rule's action set to 'resolve'. Settable only for `dns_resolver` rules."]
    pub fn set_resolve_dns_internally(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally>,
    ) -> Self {
        self.resolve_dns_internally = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resolve_dns_through_cloudflare`.\nEnable to send queries that match the policy to Cloudflare's default 1.1.1.1 DNS resolver. Cannot set when 'dns_resolvers' specified or 'resolve_dns_internally' is set. Only valid when a rule's action set to 'resolve'. Settable only for `dns_resolver` rules."]
    pub fn set_resolve_dns_through_cloudflare(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.resolve_dns_through_cloudflare = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `set_headers`.\nReplace existing headers on allowed requests with the specified key-value pairs. If a header does not exist, it is added. Header values may contain `@{selector.name}` variable references that are interpolated at the edge. Use `@@{` to escape a literal `@{`. A maximum of 20 header operations (add + set + delete) is allowed per policy. Each header name may not exceed 256 bytes and each header value may not exceed 4 KB. Settable only for `http` rules with the action set to `allow`."]
    pub fn set_set_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.set_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `untrusted_cert`.\nConfigure behavior when an upstream certificate is invalid or an SSL error occurs. Settable only for `http` rules with the action set to `allow`."]
    pub fn set_untrusted_cert(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewayPolicy_RuleSettings_UntrustedCert>,
    ) -> Self {
        self.untrusted_cert = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_RuleSettings {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_RuleSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_RuleSettings {}

impl BuildZeroTrustGatewayPolicy_RuleSettings {
    pub fn build(self) -> ZeroTrustGatewayPolicy_RuleSettings {
        ZeroTrustGatewayPolicy_RuleSettings {
            add_headers: core::default::Default::default(),
            allow_child_bypass: core::default::Default::default(),
            audit_ssh: core::default::Default::default(),
            biso_admin_controls: core::default::Default::default(),
            block_page: core::default::Default::default(),
            block_page_enabled: core::default::Default::default(),
            block_reason: core::default::Default::default(),
            bypass_parent_rule: core::default::Default::default(),
            check_session: core::default::Default::default(),
            delete_headers: core::default::Default::default(),
            dns_resolvers: core::default::Default::default(),
            egress: core::default::Default::default(),
            forensic_copy: core::default::Default::default(),
            ignore_cname_category_matches: core::default::Default::default(),
            insecure_disable_dnssec_validation: core::default::Default::default(),
            ip_categories: core::default::Default::default(),
            ip_indicator_feeds: core::default::Default::default(),
            l4override: core::default::Default::default(),
            notification_settings: core::default::Default::default(),
            override_host: core::default::Default::default(),
            override_ips: core::default::Default::default(),
            payload_log: core::default::Default::default(),
            quarantine: core::default::Default::default(),
            redirect: core::default::Default::default(),
            resolve_dns_internally: core::default::Default::default(),
            resolve_dns_through_cloudflare: core::default::Default::default(),
            set_headers: core::default::Default::default(),
            untrusted_cert: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_RuleSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_RuleSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustGatewayPolicy_RuleSettingsRef {
        ZeroTrustGatewayPolicy_RuleSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_RuleSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `add_headers` after provisioning.\nAdd custom headers to allowed requests as key-value pairs. Use header names as keys that map to arrays of header values. Settable only for `http` rules with the action set to `allow`."]
    pub fn add_headers(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.add_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_child_bypass` after provisioning.\nSet to enable MSP children to bypass this rule. Only parent MSP accounts can set this. this rule. Settable for all types of rules."]
    pub fn allow_child_bypass(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_child_bypass", self.base))
    }

    #[doc =
        "Get a reference to the value of field `audit_ssh` after provisioning.\nDefine the settings for the Audit SSH action. Settable only for `l4` rules with `audit_ssh` action."]
    pub fn audit_ssh(&self) -> ZeroTrustGatewayPolicy_RuleSettings_AuditSshRef {
        ZeroTrustGatewayPolicy_RuleSettings_AuditSshRef::new(
            self.shared().clone(),
            format!("{}.audit_ssh", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `biso_admin_controls` after provisioning.\nConfigure browser isolation behavior. Settable only for `http` rules with the action set to `isolate`."]
    pub fn biso_admin_controls(&self) -> ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef {
        ZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef::new(
            self.shared().clone(),
            format!("{}.biso_admin_controls", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `block_page` after provisioning.\nConfigure custom block page settings. If missing or null, use the account settings. Settable only for `http` rules with the action set to `block`."]
    pub fn block_page(&self) -> ZeroTrustGatewayPolicy_RuleSettings_BlockPageRef {
        ZeroTrustGatewayPolicy_RuleSettings_BlockPageRef::new(
            self.shared().clone(),
            format!("{}.block_page", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `block_page_enabled` after provisioning.\nEnable the custom block page. Settable only for `dns` rules with action `block`."]
    pub fn block_page_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_page_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `block_reason` after provisioning.\nExplain why the rule blocks the request. The custom block page shows this text (if enabled). Settable only for `dns`, `l4`, and `http` rules when the action set to `block`."]
    pub fn block_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_reason", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bypass_parent_rule` after provisioning.\nSet to enable MSP accounts to bypass their parent's rules. Only MSP child accounts can set this. Settable for all types of rules."]
    pub fn bypass_parent_rule(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bypass_parent_rule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `check_session` after provisioning.\nConfigure session check behavior. Settable only for `l4` and `http` rules with the action set to `allow`."]
    pub fn check_session(&self) -> ZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef {
        ZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef::new(
            self.shared().clone(),
            format!("{}.check_session", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `delete_headers` after provisioning.\nRemove headers from allowed requests by name. A maximum of 20 header operations (add + set + delete) is allowed per policy. Each header name may not exceed 256 bytes. Settable only for `http` rules with the action set to `allow`."]
    pub fn delete_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dns_resolvers` after provisioning.\nConfigure custom resolvers to route queries that match the resolver policy. Unused with 'resolve_dns_through_cloudflare' or 'resolve_dns_internally' settings. DNS queries get routed to the address closest to their origin. Only valid when a rule's action set to 'resolve'. Settable only for `dns_resolver` rules."]
    pub fn dns_resolvers(&self) -> ZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef {
        ZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef::new(
            self.shared().clone(),
            format!("{}.dns_resolvers", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `egress` after provisioning.\nConfigure how Gateway Proxy traffic egresses. You can enable this setting for rules with Egress actions and filters, or omit it to indicate local egress via WARP IPs. Settable only for `egress` rules."]
    pub fn egress(&self) -> ZeroTrustGatewayPolicy_RuleSettings_EgressRef {
        ZeroTrustGatewayPolicy_RuleSettings_EgressRef::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc =
        "Get a reference to the value of field `forensic_copy` after provisioning.\nConfigure whether a copy of the HTTP request will be sent to storage when the rule matches."]
    pub fn forensic_copy(&self) -> ZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef {
        ZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef::new(
            self.shared().clone(),
            format!("{}.forensic_copy", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `ignore_cname_category_matches` after provisioning.\nIgnore category matches at CNAME domains in a response. When off, evaluate categories in this rule against all CNAME domain categories in the response. Settable only for `dns` and `dns_resolver` rules."]
    pub fn ignore_cname_category_matches(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_cname_category_matches", self.base))
    }

    #[doc =
        "Get a reference to the value of field `insecure_disable_dnssec_validation` after provisioning.\nSpecify whether to disable DNSSEC validation (for Allow actions) [INSECURE]. Settable only for `dns` rules."]
    pub fn insecure_disable_dnssec_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insecure_disable_dnssec_validation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_categories` after provisioning.\nEnable IPs in DNS resolver category blocks. The system blocks only domain name categories unless you enable this setting. Settable only for `dns` and `dns_resolver` rules."]
    pub fn ip_categories(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_categories", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_indicator_feeds` after provisioning.\nIndicates whether to include IPs in DNS resolver indicator feed blocks. Default, indicator feeds block only domain names. Settable only for `dns` and `dns_resolver` rules."]
    pub fn ip_indicator_feeds(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_indicator_feeds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `l4override` after provisioning.\nSend matching traffic to the supplied destination IP address and port. Settable only for `l4` rules with the action set to `l4_override`."]
    pub fn l4override(&self) -> ZeroTrustGatewayPolicy_RuleSettings_L4overrideRef {
        ZeroTrustGatewayPolicy_RuleSettings_L4overrideRef::new(
            self.shared().clone(),
            format!("{}.l4override", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `notification_settings` after provisioning.\nConfigure a notification to display on the user's device when this rule matched. Settable for all types of rules with the action set to `block`."]
    pub fn notification_settings(&self) -> ZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef {
        ZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef::new(
            self.shared().clone(),
            format!("{}.notification_settings", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `override_host` after provisioning.\nDefines a hostname for override, for the matching DNS queries. Settable only for `dns` rules with the action set to `override`."]
    pub fn override_host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `override_ips` after provisioning.\nDefines a an IP or set of IPs for overriding matched DNS queries. Settable only for `dns` rules with the action set to `override`."]
    pub fn override_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override_ips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `payload_log` after provisioning.\nConfigure DLP payload logging. Settable only for `http` rules."]
    pub fn payload_log(&self) -> ZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef {
        ZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef::new(
            self.shared().clone(),
            format!("{}.payload_log", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `quarantine` after provisioning.\nConfigure settings that apply to quarantine rules. Settable only for `http` rules."]
    pub fn quarantine(&self) -> ZeroTrustGatewayPolicy_RuleSettings_QuarantineRef {
        ZeroTrustGatewayPolicy_RuleSettings_QuarantineRef::new(
            self.shared().clone(),
            format!("{}.quarantine", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `redirect` after provisioning.\nApply settings to redirect rules. Settable only for `http` rules with the action set to `redirect`."]
    pub fn redirect(&self) -> ZeroTrustGatewayPolicy_RuleSettings_RedirectRef {
        ZeroTrustGatewayPolicy_RuleSettings_RedirectRef::new(
            self.shared().clone(),
            format!("{}.redirect", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `resolve_dns_internally` after provisioning.\nConfigure to forward the query to the internal DNS service, passing the specified 'view_id' as input. Not used when 'dns_resolvers' is specified or 'resolve_dns_through_cloudflare' is set. Only valid when a rule's action set to 'resolve'. Settable only for `dns_resolver` rules."]
    pub fn resolve_dns_internally(&self) -> ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef {
        ZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef::new(
            self.shared().clone(),
            format!("{}.resolve_dns_internally", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `resolve_dns_through_cloudflare` after provisioning.\nEnable to send queries that match the policy to Cloudflare's default 1.1.1.1 DNS resolver. Cannot set when 'dns_resolvers' specified or 'resolve_dns_internally' is set. Only valid when a rule's action set to 'resolve'. Settable only for `dns_resolver` rules."]
    pub fn resolve_dns_through_cloudflare(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolve_dns_through_cloudflare", self.base))
    }

    #[doc =
        "Get a reference to the value of field `set_headers` after provisioning.\nReplace existing headers on allowed requests with the specified key-value pairs. If a header does not exist, it is added. Header values may contain `@{selector.name}` variable references that are interpolated at the edge. Use `@@{` to escape a literal `@{`. A maximum of 20 header operations (add + set + delete) is allowed per policy. Each header name may not exceed 256 bytes and each header value may not exceed 4 KB. Settable only for `http` rules with the action set to `allow`."]
    pub fn set_headers(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.set_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `untrusted_cert` after provisioning.\nConfigure behavior when an upstream certificate is invalid or an SSL error occurs. Settable only for `http` rules with the action set to `allow`."]
    pub fn untrusted_cert(&self) -> ZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef {
        ZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef::new(
            self.shared().clone(),
            format!("{}.untrusted_cert", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayPolicy_Schedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mon: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sat: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sun: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tue: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wed: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustGatewayPolicy_Schedule {
    #[doc =
        "Set the field `fri`.\nSpecify the time intervals when the rule is active on Fridays, in the increasing order from 00:00-24:00.  If this parameter omitted, the rule is deactivated on Fridays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn set_fri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mon`.\nSpecify the time intervals when the rule is active on Mondays, in the increasing order from 00:00-24:00(capped at maximum of 6 time splits). If this parameter omitted, the rule is deactivated on Mondays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn set_mon(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mon = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sat`.\nSpecify the time intervals when the rule is active on Saturdays, in the increasing order from 00:00-24:00.  If this parameter omitted, the rule is deactivated on Saturdays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn set_sat(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sat = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sun`.\nSpecify the time intervals when the rule is active on Sundays, in the increasing order from 00:00-24:00. If this parameter omitted, the rule is deactivated on Sundays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn set_sun(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sun = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `thu`.\nSpecify the time intervals when the rule is active on Thursdays, in the increasing order from 00:00-24:00. If this parameter omitted, the rule is deactivated on Thursdays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn set_thu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.thu = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_zone`.\nSpecify the time zone for rule evaluation. When a [valid time zone city name](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones#List) is provided, Gateway always uses the current time for that time zone. When this parameter is omitted, Gateway uses the time zone determined from the user's IP address. Colo time zone is used when the user's IP address does not resolve to a location."]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tue`.\nSpecify the time intervals when the rule is active on Tuesdays, in the increasing order from 00:00-24:00. If this parameter omitted, the rule is deactivated on Tuesdays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn set_tue(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tue = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wed`.\nSpecify the time intervals when the rule is active on Wednesdays, in the increasing order from 00:00-24:00. If this parameter omitted, the rule is deactivated on Wednesdays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn set_wed(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.wed = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayPolicy_Schedule {
    type O = terrars::BlockAssignable<ZeroTrustGatewayPolicy_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayPolicy_Schedule {}

impl BuildZeroTrustGatewayPolicy_Schedule {
    pub fn build(self) -> ZeroTrustGatewayPolicy_Schedule {
        ZeroTrustGatewayPolicy_Schedule {
            fri: core::default::Default::default(),
            mon: core::default::Default::default(),
            sat: core::default::Default::default(),
            sun: core::default::Default::default(),
            thu: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            tue: core::default::Default::default(),
            wed: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicy_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayPolicy_ScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustGatewayPolicy_ScheduleRef {
        ZeroTrustGatewayPolicy_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayPolicy_ScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fri` after provisioning.\nSpecify the time intervals when the rule is active on Fridays, in the increasing order from 00:00-24:00.  If this parameter omitted, the rule is deactivated on Fridays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn fri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mon` after provisioning.\nSpecify the time intervals when the rule is active on Mondays, in the increasing order from 00:00-24:00(capped at maximum of 6 time splits). If this parameter omitted, the rule is deactivated on Mondays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn mon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mon", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sat` after provisioning.\nSpecify the time intervals when the rule is active on Saturdays, in the increasing order from 00:00-24:00.  If this parameter omitted, the rule is deactivated on Saturdays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn sat(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sat", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sun` after provisioning.\nSpecify the time intervals when the rule is active on Sundays, in the increasing order from 00:00-24:00. If this parameter omitted, the rule is deactivated on Sundays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn sun(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sun", self.base))
    }

    #[doc =
        "Get a reference to the value of field `thu` after provisioning.\nSpecify the time intervals when the rule is active on Thursdays, in the increasing order from 00:00-24:00. If this parameter omitted, the rule is deactivated on Thursdays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn thu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nSpecify the time zone for rule evaluation. When a [valid time zone city name](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones#List) is provided, Gateway always uses the current time for that time zone. When this parameter is omitted, Gateway uses the time zone determined from the user's IP address. Colo time zone is used when the user's IP address does not resolve to a location."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tue` after provisioning.\nSpecify the time intervals when the rule is active on Tuesdays, in the increasing order from 00:00-24:00. If this parameter omitted, the rule is deactivated on Tuesdays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn tue(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tue", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wed` after provisioning.\nSpecify the time intervals when the rule is active on Wednesdays, in the increasing order from 00:00-24:00. If this parameter omitted, the rule is deactivated on Wednesdays. API returns a formatted version of this string, which may cause Terraform drift if a unformatted value is used."]
    pub fn wed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wed", self.base))
    }
}

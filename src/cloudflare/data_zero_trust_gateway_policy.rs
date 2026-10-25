#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustGatewayPolicyData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataZeroTrustGatewayPolicy_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustGatewayPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustGatewayPolicyData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustGatewayPolicy(std::rc::Rc<DataZeroTrustGatewayPolicy_>);

impl DataZeroTrustGatewayPolicy {
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

    #[doc = "Set the field `account_id`.\nSpecify the Cloudflare account identifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataZeroTrustGatewayPolicy_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_id`.\nIdentify the API resource with a UUID."]
    pub fn set_rule_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rule_id = std::option::Option::Some(v.into());
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
    pub fn expiration(&self) -> DataZeroTrustGatewayPolicy_ExpirationRef {
        DataZeroTrustGatewayPolicy_ExpirationRef::new(
            self.shared().clone(),
            format!("{}.expiration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustGatewayPolicy_FilterRef {
        DataZeroTrustGatewayPolicy_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
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
        "Get a reference to the value of field `rule_id` after provisioning.\nIdentify the API resource with a UUID."]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_settings` after provisioning.\nDefines settings for this rule. Settings apply only to specific rule types and must use compatible selectors. If Terraform detects drift, confirm the setting supports your rule type and check whether the API modifies the value. Use API-returned values in your configuration to prevent drift."]
    pub fn rule_settings(&self) -> DataZeroTrustGatewayPolicy_RuleSettingsRef {
        DataZeroTrustGatewayPolicy_RuleSettingsRef::new(
            self.shared().clone(),
            format!("{}.rule_settings", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nDefines the schedule for activating DNS policies. Settable only for `dns` and `dns_resolver` rules."]
    pub fn schedule(&self) -> DataZeroTrustGatewayPolicy_ScheduleRef {
        DataZeroTrustGatewayPolicy_ScheduleRef::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
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

impl terrars::Referable for DataZeroTrustGatewayPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustGatewayPolicy { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy {
    type O = terrars::ListRef<DataZeroTrustGatewayPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustGatewayPolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_gateway_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustGatewayPolicy {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustGatewayPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustGatewayPolicy {
        let out = DataZeroTrustGatewayPolicy(std::rc::Rc::new(DataZeroTrustGatewayPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustGatewayPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                rule_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustGatewayPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustGatewayPolicyRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
    pub fn expiration(&self) -> DataZeroTrustGatewayPolicy_ExpirationRef {
        DataZeroTrustGatewayPolicy_ExpirationRef::new(
            self.shared().clone(),
            format!("{}.expiration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustGatewayPolicy_FilterRef {
        DataZeroTrustGatewayPolicy_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
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
        "Get a reference to the value of field `rule_id` after provisioning.\nIdentify the API resource with a UUID."]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_settings` after provisioning.\nDefines settings for this rule. Settings apply only to specific rule types and must use compatible selectors. If Terraform detects drift, confirm the setting supports your rule type and check whether the API modifies the value. Use API-returned values in your configuration to prevent drift."]
    pub fn rule_settings(&self) -> DataZeroTrustGatewayPolicy_RuleSettingsRef {
        DataZeroTrustGatewayPolicy_RuleSettingsRef::new(
            self.shared().clone(),
            format!("{}.rule_settings", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nDefines the schedule for activating DNS policies. Settable only for `dns` and `dns_resolver` rules."]
    pub fn schedule(&self) -> DataZeroTrustGatewayPolicy_ScheduleRef {
        DataZeroTrustGatewayPolicy_ScheduleRef::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
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
pub struct DataZeroTrustGatewayPolicy_Expiration {}

impl DataZeroTrustGatewayPolicy_Expiration { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_Expiration {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_Expiration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_Expiration {}

impl BuildDataZeroTrustGatewayPolicy_Expiration {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_Expiration {
        DataZeroTrustGatewayPolicy_Expiration {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_ExpirationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_ExpirationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustGatewayPolicy_ExpirationRef {
        DataZeroTrustGatewayPolicy_ExpirationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_ExpirationRef {
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
pub struct DataZeroTrustGatewayPolicy_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataZeroTrustGatewayPolicy_Filter {
    #[doc =
        "Set the field `direction`.\nSort direction. When `order_by` is omitted, this controls the direction\nof the existing precedence ordering. Shared rules remain first in either\ndirection. Accepted values are `asc` and `desc`.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter`.\nFilter the returned rules by one or more `field:value` pairs. Repeat the\nparameter to combine filters with logical AND.\n\nSupported fields are `name`, `id`, `action`, `enabled`, `source_account`,\n`is_shared`, `filters`, and `expression` (max 1024 bytes). The `source_account`\nvalue is matched as a normalized UUID substring. The `filters` value must\nbe one of the rule filter names and matches a member of the rule's `filters`\narray. The `expression` filter performs a case-insensitive literal\nsubstring match across traffic, identity, and device posture expressions."]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order_by`.\nField to sort the returned rules by. Supported values are `name`,\n`created_at`, `updated_at`, and `precedence`.\nAvailable values: \"name\", \"created_at\", \"updated_at\", \"precedence\"."]
    pub fn set_order_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\nCase-insensitive substring search across rule name and description."]
    pub fn set_search(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.search = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_Filter {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_Filter {}

impl BuildDataZeroTrustGatewayPolicy_Filter {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_Filter {
        DataZeroTrustGatewayPolicy_Filter {
            direction: core::default::Default::default(),
            filter: core::default::Default::default(),
            order_by: core::default::Default::default(),
            search: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustGatewayPolicy_FilterRef {
        DataZeroTrustGatewayPolicy_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nSort direction. When `order_by` is omitted, this controls the direction\nof the existing precedence ordering. Shared rules remain first in either\ndirection. Accepted values are `asc` and `desc`.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter the returned rules by one or more `field:value` pairs. Repeat the\nparameter to combine filters with logical AND.\n\nSupported fields are `name`, `id`, `action`, `enabled`, `source_account`,\n`is_shared`, `filters`, and `expression` (max 1024 bytes). The `source_account`\nvalue is matched as a normalized UUID substring. The `filters` value must\nbe one of the rule filter names and matches a member of the rule's `filters`\narray. The `expression` filter performs a case-insensitive literal\nsubstring match across traffic, identity, and device posture expressions."]
    pub fn filter(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nField to sort the returned rules by. Supported values are `name`,\n`created_at`, `updated_at`, and `precedence`.\nAvailable values: \"name\", \"created_at\", \"updated_at\", \"precedence\"."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nCase-insensitive substring search across rule name and description."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_AuditSsh {}

impl DataZeroTrustGatewayPolicy_RuleSettings_AuditSsh { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_AuditSsh {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_AuditSsh>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_AuditSsh {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_AuditSsh {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_AuditSsh {
        DataZeroTrustGatewayPolicy_RuleSettings_AuditSsh {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_AuditSshRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_AuditSshRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_AuditSshRef {
        DataZeroTrustGatewayPolicy_RuleSettings_AuditSshRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_AuditSshRef {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls {}

impl DataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls {
        DataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControls {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef {
        DataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_BlockPage {}

impl DataZeroTrustGatewayPolicy_RuleSettings_BlockPage { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_BlockPage {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_BlockPage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_BlockPage {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_BlockPage {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_BlockPage {
        DataZeroTrustGatewayPolicy_RuleSettings_BlockPage {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_BlockPageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_BlockPageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_BlockPageRef {
        DataZeroTrustGatewayPolicy_RuleSettings_BlockPageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_BlockPageRef {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_CheckSession {}

impl DataZeroTrustGatewayPolicy_RuleSettings_CheckSession { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_CheckSession {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_CheckSession>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_CheckSession {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_CheckSession {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_CheckSession {
        DataZeroTrustGatewayPolicy_RuleSettings_CheckSession {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef {
        DataZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 {}

impl DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 {
        DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4 {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4Ref {
        DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4Ref {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 {}

impl DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 {
        DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6 {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6Ref {
        DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6Ref {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers {}

impl DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers {
        DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef {
        DataZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv4` after provisioning.\n"]
    pub fn ipv4(&self) -> terrars::ListRef<DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv4Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv4", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> terrars::ListRef<DataZeroTrustGatewayPolicy_RuleSettings_DnsResolvers_Ipv6Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_Egress {}

impl DataZeroTrustGatewayPolicy_RuleSettings_Egress { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_Egress {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_Egress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_Egress {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_Egress {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_Egress {
        DataZeroTrustGatewayPolicy_RuleSettings_Egress {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_EgressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_EgressRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_EgressRef {
        DataZeroTrustGatewayPolicy_RuleSettings_EgressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_EgressRef {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_ForensicCopy {}

impl DataZeroTrustGatewayPolicy_RuleSettings_ForensicCopy { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_ForensicCopy {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_ForensicCopy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_ForensicCopy {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_ForensicCopy {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_ForensicCopy {
        DataZeroTrustGatewayPolicy_RuleSettings_ForensicCopy {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef {
        DataZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_L4override {}

impl DataZeroTrustGatewayPolicy_RuleSettings_L4override { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_L4override {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_L4override>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_L4override {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_L4override {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_L4override {
        DataZeroTrustGatewayPolicy_RuleSettings_L4override {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_L4overrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_L4overrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_L4overrideRef {
        DataZeroTrustGatewayPolicy_RuleSettings_L4overrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_L4overrideRef {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_NotificationSettings {}

impl DataZeroTrustGatewayPolicy_RuleSettings_NotificationSettings { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_NotificationSettings {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_NotificationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_NotificationSettings {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_NotificationSettings {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_NotificationSettings {
        DataZeroTrustGatewayPolicy_RuleSettings_NotificationSettings {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef {
        DataZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_PayloadLog {}

impl DataZeroTrustGatewayPolicy_RuleSettings_PayloadLog { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_PayloadLog {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_PayloadLog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_PayloadLog {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_PayloadLog {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_PayloadLog {
        DataZeroTrustGatewayPolicy_RuleSettings_PayloadLog {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef {
        DataZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_Quarantine {}

impl DataZeroTrustGatewayPolicy_RuleSettings_Quarantine { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_Quarantine {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_Quarantine>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_Quarantine {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_Quarantine {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_Quarantine {
        DataZeroTrustGatewayPolicy_RuleSettings_Quarantine {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_QuarantineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_QuarantineRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_QuarantineRef {
        DataZeroTrustGatewayPolicy_RuleSettings_QuarantineRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_QuarantineRef {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_Redirect {}

impl DataZeroTrustGatewayPolicy_RuleSettings_Redirect { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_Redirect {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_Redirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_Redirect {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_Redirect {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_Redirect {
        DataZeroTrustGatewayPolicy_RuleSettings_Redirect {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_RedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_RedirectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_RedirectRef {
        DataZeroTrustGatewayPolicy_RuleSettings_RedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_RedirectRef {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally {}

impl DataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally {
        DataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternally {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef {
        DataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings_UntrustedCert {}

impl DataZeroTrustGatewayPolicy_RuleSettings_UntrustedCert { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings_UntrustedCert {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings_UntrustedCert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings_UntrustedCert {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings_UntrustedCert {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings_UntrustedCert {
        DataZeroTrustGatewayPolicy_RuleSettings_UntrustedCert {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef {
        DataZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef {
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
pub struct DataZeroTrustGatewayPolicy_RuleSettings {}

impl DataZeroTrustGatewayPolicy_RuleSettings { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_RuleSettings {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_RuleSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_RuleSettings {}

impl BuildDataZeroTrustGatewayPolicy_RuleSettings {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_RuleSettings {
        DataZeroTrustGatewayPolicy_RuleSettings {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_RuleSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_RuleSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustGatewayPolicy_RuleSettingsRef {
        DataZeroTrustGatewayPolicy_RuleSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_RuleSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `add_headers` after provisioning.\nAdd custom headers to allowed requests as key-value pairs. Use header names as keys that map to arrays of header values. Header values may contain `@{selector.name}` variable references that are interpolated at the edge. Use `@@{` to escape a literal `@{`. A maximum of 20 header operations (add + set + delete) is allowed per policy. Each header name may not exceed 256 bytes and each header value may not exceed 4 KB. Settable only for `http` rules with the action set to `allow`."]
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
    pub fn audit_ssh(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_AuditSshRef {
        DataZeroTrustGatewayPolicy_RuleSettings_AuditSshRef::new(
            self.shared().clone(),
            format!("{}.audit_ssh", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `biso_admin_controls` after provisioning.\nConfigure browser isolation behavior. Settable only for `http` rules with the action set to `isolate`."]
    pub fn biso_admin_controls(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef {
        DataZeroTrustGatewayPolicy_RuleSettings_BisoAdminControlsRef::new(
            self.shared().clone(),
            format!("{}.biso_admin_controls", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `block_page` after provisioning.\nConfigure custom block page settings. If missing or null, use the account settings. Settable only for `http` rules with the action set to `block`."]
    pub fn block_page(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_BlockPageRef {
        DataZeroTrustGatewayPolicy_RuleSettings_BlockPageRef::new(
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
    pub fn check_session(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef {
        DataZeroTrustGatewayPolicy_RuleSettings_CheckSessionRef::new(
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
    pub fn dns_resolvers(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef {
        DataZeroTrustGatewayPolicy_RuleSettings_DnsResolversRef::new(
            self.shared().clone(),
            format!("{}.dns_resolvers", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `egress` after provisioning.\nConfigure how Gateway Proxy traffic egresses. You can enable this setting for rules with Egress actions and filters, or omit it to indicate local egress via WARP IPs. Settable only for `egress` rules."]
    pub fn egress(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_EgressRef {
        DataZeroTrustGatewayPolicy_RuleSettings_EgressRef::new(
            self.shared().clone(),
            format!("{}.egress", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `forensic_copy` after provisioning.\nConfigure whether a copy of the HTTP request will be sent to storage when the rule matches."]
    pub fn forensic_copy(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef {
        DataZeroTrustGatewayPolicy_RuleSettings_ForensicCopyRef::new(
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
    pub fn l4override(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_L4overrideRef {
        DataZeroTrustGatewayPolicy_RuleSettings_L4overrideRef::new(
            self.shared().clone(),
            format!("{}.l4override", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `notification_settings` after provisioning.\nConfigure a notification to display on the user's device when this rule matched. Settable for all types of rules with the action set to `block`."]
    pub fn notification_settings(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef {
        DataZeroTrustGatewayPolicy_RuleSettings_NotificationSettingsRef::new(
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
    pub fn payload_log(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef {
        DataZeroTrustGatewayPolicy_RuleSettings_PayloadLogRef::new(
            self.shared().clone(),
            format!("{}.payload_log", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `quarantine` after provisioning.\nConfigure settings that apply to quarantine rules. Settable only for `http` rules."]
    pub fn quarantine(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_QuarantineRef {
        DataZeroTrustGatewayPolicy_RuleSettings_QuarantineRef::new(
            self.shared().clone(),
            format!("{}.quarantine", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `redirect` after provisioning.\nApply settings to redirect rules. Settable only for `http` rules with the action set to `redirect`."]
    pub fn redirect(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_RedirectRef {
        DataZeroTrustGatewayPolicy_RuleSettings_RedirectRef::new(
            self.shared().clone(),
            format!("{}.redirect", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `resolve_dns_internally` after provisioning.\nConfigure to forward the query to the internal DNS service, passing the specified 'view_id' as input. Not used when 'dns_resolvers' is specified or 'resolve_dns_through_cloudflare' is set. Only valid when a rule's action set to 'resolve'. Settable only for `dns_resolver` rules."]
    pub fn resolve_dns_internally(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef {
        DataZeroTrustGatewayPolicy_RuleSettings_ResolveDnsInternallyRef::new(
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
    pub fn untrusted_cert(&self) -> DataZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef {
        DataZeroTrustGatewayPolicy_RuleSettings_UntrustedCertRef::new(
            self.shared().clone(),
            format!("{}.untrusted_cert", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustGatewayPolicy_Schedule {}

impl DataZeroTrustGatewayPolicy_Schedule { }

impl terrars::ToListMappable for DataZeroTrustGatewayPolicy_Schedule {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayPolicy_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayPolicy_Schedule {}

impl BuildDataZeroTrustGatewayPolicy_Schedule {
    pub fn build(self) -> DataZeroTrustGatewayPolicy_Schedule {
        DataZeroTrustGatewayPolicy_Schedule {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayPolicy_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayPolicy_ScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustGatewayPolicy_ScheduleRef {
        DataZeroTrustGatewayPolicy_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayPolicy_ScheduleRef {
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

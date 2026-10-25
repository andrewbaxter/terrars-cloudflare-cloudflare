#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDevicePostureRuleData_ {
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
    rule_id: terrars::PrimField<std::string::String>,
}

struct DataZeroTrustDevicePostureRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDevicePostureRuleData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDevicePostureRule(std::rc::Rc<DataZeroTrustDevicePostureRule_>);

impl DataZeroTrustDevicePostureRule {
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

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the device posture rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the rule is enabled. This is a computed, read-only value. It is false for deprecated Kolide posture rules that still use the issue_count input, and true otherwise."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expiration` after provisioning.\nSets the expiration time for a posture check result. If empty, the result remains valid until it is overwritten by new data from the WARP client."]
    pub fn expiration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAPI UUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\nThe value to be checked against."]
    pub fn input(&self) -> DataZeroTrustDevicePostureRule_InputRef {
        DataZeroTrustDevicePostureRule_InputRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nThe conditions that the client must match to run the rule."]
    pub fn match_(&self) -> terrars::ListRef<DataZeroTrustDevicePostureRule_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the device posture rule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nAPI UUID."]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nPolling frequency for the WARP client posture check. Default: `5m` (poll every five minutes). Minimum: `1m`."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of device posture rule.\nAvailable values: \"file\", \"application\", \"tanium\", \"gateway\", \"warp\", \"disk_encryption\", \"serial_number\", \"sentinelone\", \"carbonblack\", \"firewall\", \"os_version\", \"domain_joined\", \"client_certificate\", \"client_certificate_v2\", \"antivirus\", \"unique_client_id\", \"kolide\", \"tanium_s2s\", \"crowdstrike_s2s\", \"intune\", \"workspace_one\", \"sentinelone_s2s\", \"custom_s2s\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustDevicePostureRule {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDevicePostureRule { }

impl terrars::ToListMappable for DataZeroTrustDevicePostureRule {
    type O = terrars::ListRef<DataZeroTrustDevicePostureRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDevicePostureRule_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_device_posture_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDevicePostureRule {
    pub tf_id: std::string::String,
    #[doc = "API UUID."]
    pub rule_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustDevicePostureRule {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDevicePostureRule {
        let out = DataZeroTrustDevicePostureRule(std::rc::Rc::new(DataZeroTrustDevicePostureRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDevicePostureRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                rule_id: self.rule_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustDevicePostureRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDevicePostureRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDevicePostureRuleRef {
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
        "Get a reference to the value of field `description` after provisioning.\nThe description of the device posture rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the rule is enabled. This is a computed, read-only value. It is false for deprecated Kolide posture rules that still use the issue_count input, and true otherwise."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expiration` after provisioning.\nSets the expiration time for a posture check result. If empty, the result remains valid until it is overwritten by new data from the WARP client."]
    pub fn expiration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAPI UUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\nThe value to be checked against."]
    pub fn input(&self) -> DataZeroTrustDevicePostureRule_InputRef {
        DataZeroTrustDevicePostureRule_InputRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nThe conditions that the client must match to run the rule."]
    pub fn match_(&self) -> terrars::ListRef<DataZeroTrustDevicePostureRule_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the device posture rule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nAPI UUID."]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nPolling frequency for the WARP client posture check. Default: `5m` (poll every five minutes). Minimum: `1m`."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of device posture rule.\nAvailable values: \"file\", \"application\", \"tanium\", \"gateway\", \"warp\", \"disk_encryption\", \"serial_number\", \"sentinelone\", \"carbonblack\", \"firewall\", \"os_version\", \"domain_joined\", \"client_certificate\", \"client_certificate_v2\", \"antivirus\", \"unique_client_id\", \"kolide\", \"tanium_s2s\", \"crowdstrike_s2s\", \"intune\", \"workspace_one\", \"sentinelone_s2s\", \"custom_s2s\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDevicePostureRule_Input_Locations {}

impl DataZeroTrustDevicePostureRule_Input_Locations { }

impl terrars::ToListMappable for DataZeroTrustDevicePostureRule_Input_Locations {
    type O = terrars::BlockAssignable<DataZeroTrustDevicePostureRule_Input_Locations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDevicePostureRule_Input_Locations {}

impl BuildDataZeroTrustDevicePostureRule_Input_Locations {
    pub fn build(self) -> DataZeroTrustDevicePostureRule_Input_Locations {
        DataZeroTrustDevicePostureRule_Input_Locations {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDevicePostureRule_Input_LocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDevicePostureRule_Input_LocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDevicePostureRule_Input_LocationsRef {
        DataZeroTrustDevicePostureRule_Input_LocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDevicePostureRule_Input_LocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `paths` after provisioning.\nList of paths to check for client certificate on linux."]
    pub fn paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_stores` after provisioning.\nList of trust stores to check for client certificate."]
    pub fn trust_stores(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_stores", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDevicePostureRule_Input {}

impl DataZeroTrustDevicePostureRule_Input { }

impl terrars::ToListMappable for DataZeroTrustDevicePostureRule_Input {
    type O = terrars::BlockAssignable<DataZeroTrustDevicePostureRule_Input>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDevicePostureRule_Input {}

impl BuildDataZeroTrustDevicePostureRule_Input {
    pub fn build(self) -> DataZeroTrustDevicePostureRule_Input {
        DataZeroTrustDevicePostureRule_Input {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDevicePostureRule_InputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDevicePostureRule_InputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDevicePostureRule_InputRef {
        DataZeroTrustDevicePostureRule_InputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDevicePostureRule_InputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_threats` after provisioning.\nThe Number of active threats."]
    pub fn active_threats(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_threats", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auth_state` after provisioning.\nThe set of Kolide device authentication states that pass the posture check. Device must match one of the specified states."]
    pub fn auth_state(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `certificate_id` after provisioning.\nUUID of Cloudflare managed certificate."]
    pub fn certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `check_disks` after provisioning.\nList of volume names to be checked for encryption."]
    pub fn check_disks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_disks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `check_private_key` after provisioning.\nConfirm the certificate was not imported from another device. We recommend keeping this enabled unless the certificate was deployed without a private key."]
    pub fn check_private_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.check_private_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cn` after provisioning.\nCommon Name that is protected by the certificate."]
    pub fn cn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `compliance_status` after provisioning.\nCompliance Status.\nAvailable values: \"compliant\", \"noncompliant\", \"unknown\", \"notapplicable\", \"ingraceperiod\", \"error\"."]
    pub fn compliance_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compliance_status", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\nPosture Integration ID."]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `count_operator` after provisioning.\nCount Operator.\nAvailable values: \"<\", \"<=\", \">\", \">=\", \"==\"."]
    pub fn count_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nDomain."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `eid_last_seen` after provisioning.\nFor more details on eid last seen, refer to the Tanium documentation."]
    pub fn eid_last_seen(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eid_last_seen", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `exists` after provisioning.\nWhether or not file exists."]
    pub fn exists(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exists", self.base))
    }

    #[doc =
        "Get a reference to the value of field `extended_key_usage` after provisioning.\nList of values indicating purposes for which the certificate public key can be used."]
    pub fn extended_key_usage(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nList ID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `infected` after provisioning.\nWhether device is infected."]
    pub fn infected(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.infected", self.base))
    }

    #[doc = "Get a reference to the value of field `is_active` after provisioning.\nWhether device is active."]
    pub fn is_active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_active", self.base))
    }

    #[doc = "Get a reference to the value of field `issue_count` after provisioning.\nThe Number of Issues."]
    pub fn issue_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issue_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `last_seen` after provisioning.\nFor more details on last seen, please refer to the Crowdstrike documentation."]
    pub fn last_seen(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_seen", self.base))
    }

    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> DataZeroTrustDevicePostureRule_Input_LocationsRef {
        DataZeroTrustDevicePostureRule_Input_LocationsRef::new(
            self.shared().clone(),
            format!("{}.locations", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `network_status` after provisioning.\nNetwork status of device.\nAvailable values: \"connected\", \"disconnected\", \"disconnecting\", \"connecting\"."]
    pub fn network_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operating_system` after provisioning.\nOperating system.\nAvailable values: \"windows\", \"linux\", \"mac\", \"android\", \"ios\", \"chromeos\"."]
    pub fn operating_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operating_system", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operational_state` after provisioning.\nAgent operational state.\nAvailable values: \"na\", \"partially_disabled\", \"auto_fully_disabled\", \"fully_disabled\", \"auto_partially_disabled\", \"disabled_error\", \"db_corruption\"."]
    pub fn operational_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operational_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nOperator.\nAvailable values: \"<\", \"<=\", \">\", \">=\", \"==\"."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `os` after provisioning.\nOs Version."]
    pub fn os(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os", self.base))
    }

    #[doc =
        "Get a reference to the value of field `os_distro_name` after provisioning.\nOperating System Distribution Name (linux only)."]
    pub fn os_distro_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_distro_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `os_distro_revision` after provisioning.\nVersion of OS Distribution (linux only)."]
    pub fn os_distro_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_distro_revision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `os_version_extra` after provisioning.\nAdditional operating system version details. For Windows, the UBR (Update Build Revision). For Mac or iOS, the Product Version Extra. For Linux, the distribution name and version."]
    pub fn os_version_extra(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_version_extra", self.base))
    }

    #[doc = "Get a reference to the value of field `overall` after provisioning.\nOverall."]
    pub fn overall(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overall", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nFile path."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_all` after provisioning.\nWhether to check all disks for encryption."]
    pub fn require_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `risk_level` after provisioning.\nFor more details on risk level, refer to the Tanium documentation.\nAvailable values: \"low\", \"medium\", \"high\", \"critical\"."]
    pub fn risk_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.risk_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `score` after provisioning.\nA value between 0-100 assigned to devices set by the 3rd party posture provider."]
    pub fn score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }

    #[doc =
        "Get a reference to the value of field `score_operator` after provisioning.\nScore Operator.\nAvailable values: \"<\", \"<=\", \">\", \">=\", \"==\"."]
    pub fn score_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `sensor_config` after provisioning.\nSensorConfig."]
    pub fn sensor_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensor_config", self.base))
    }

    #[doc = "Get a reference to the value of field `sha256` after provisioning.\nSHA-256."]
    pub fn sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nFor more details on state, please refer to the Crowdstrike documentation.\nAvailable values: \"online\", \"offline\", \"unknown\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subject_alternative_names` after provisioning.\nList of certificate Subject Alternative Names."]
    pub fn subject_alternative_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `thumbprint` after provisioning.\nSigning certificate thumbprint."]
    pub fn thumbprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thumbprint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `total_score` after provisioning.\nFor more details on total score, refer to the Tanium documentation."]
    pub fn total_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_score", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_window_days` after provisioning.\nNumber of days that the antivirus should be updated within."]
    pub fn update_window_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_window_days", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of OS."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version_operator` after provisioning.\nVersion Operator.\nAvailable values: \"<\", \"<=\", \">\", \">=\", \"==\"."]
    pub fn version_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_operator", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDevicePostureRule_Match {}

impl DataZeroTrustDevicePostureRule_Match { }

impl terrars::ToListMappable for DataZeroTrustDevicePostureRule_Match {
    type O = terrars::BlockAssignable<DataZeroTrustDevicePostureRule_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDevicePostureRule_Match {}

impl BuildDataZeroTrustDevicePostureRule_Match {
    pub fn build(self) -> DataZeroTrustDevicePostureRule_Match {
        DataZeroTrustDevicePostureRule_Match {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDevicePostureRule_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDevicePostureRule_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDevicePostureRule_MatchRef {
        DataZeroTrustDevicePostureRule_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDevicePostureRule_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `platform` after provisioning.\nAvailable values: \"windows\", \"mac\", \"linux\", \"android\", \"ios\", \"chromeos\"."]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.base))
    }
}

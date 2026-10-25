#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustDevicePostureRuleData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<ZeroTrustDevicePostureRule_Input>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<ZeroTrustDevicePostureRule_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

struct ZeroTrustDevicePostureRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustDevicePostureRuleData_>,
}

#[derive(Clone)]
pub struct ZeroTrustDevicePostureRule(std::rc::Rc<ZeroTrustDevicePostureRule_>);

impl ZeroTrustDevicePostureRule {
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

    #[doc = "Set the field `description`.\nThe description of the device posture rule."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expiration`.\nSets the expiration time for a posture check result. If empty, the result remains valid until it is overwritten by new data from the WARP client."]
    pub fn set_expiration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expiration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input`.\nThe value to be checked against."]
    pub fn set_input(self, v: impl std::convert::Into<ZeroTrustDevicePostureRule_Input>) -> Self {
        self.0.data.borrow_mut().input = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\nThe conditions that the client must match to run the rule."]
    pub fn set_match(self, v: impl std::convert::Into<std::vec::Vec<ZeroTrustDevicePostureRule_Match>>) -> Self {
        self.0.data.borrow_mut().match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of the device posture rule."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schedule`.\nPolling frequency for the WARP client posture check. Default: `5m` (poll every five minutes). Minimum: `1m`."]
    pub fn set_schedule(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().schedule = std::option::Option::Some(v.into());
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
    pub fn input(&self) -> ZeroTrustDevicePostureRule_InputRef {
        ZeroTrustDevicePostureRule_InputRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nThe conditions that the client must match to run the rule."]
    pub fn match_(&self) -> terrars::ListRef<ZeroTrustDevicePostureRule_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the device posture rule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

impl terrars::Referable for ZeroTrustDevicePostureRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustDevicePostureRule { }

impl terrars::ToListMappable for ZeroTrustDevicePostureRule {
    type O = terrars::ListRef<ZeroTrustDevicePostureRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustDevicePostureRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_device_posture_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustDevicePostureRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of device posture rule.\nAvailable values: \"file\", \"application\", \"tanium\", \"gateway\", \"warp\", \"disk_encryption\", \"serial_number\", \"sentinelone\", \"carbonblack\", \"firewall\", \"os_version\", \"domain_joined\", \"client_certificate\", \"client_certificate_v2\", \"antivirus\", \"unique_client_id\", \"kolide\", \"tanium_s2s\", \"crowdstrike_s2s\", \"intune\", \"workspace_one\", \"sentinelone_s2s\", \"custom_s2s\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDevicePostureRule {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustDevicePostureRule {
        let out = ZeroTrustDevicePostureRule(std::rc::Rc::new(ZeroTrustDevicePostureRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustDevicePostureRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                description: core::default::Default::default(),
                expiration: core::default::Default::default(),
                input: core::default::Default::default(),
                match_: core::default::Default::default(),
                name: core::default::Default::default(),
                schedule: core::default::Default::default(),
                type_: self.type_,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustDevicePostureRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDevicePostureRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustDevicePostureRuleRef {
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
    pub fn input(&self) -> ZeroTrustDevicePostureRule_InputRef {
        ZeroTrustDevicePostureRule_InputRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nThe conditions that the client must match to run the rule."]
    pub fn match_(&self) -> terrars::ListRef<ZeroTrustDevicePostureRule_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the device posture rule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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
pub struct ZeroTrustDevicePostureRule_Input_Locations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paths: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_stores: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ZeroTrustDevicePostureRule_Input_Locations {
    #[doc = "Set the field `paths`.\nList of paths to check for client certificate on linux."]
    pub fn set_paths(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.paths = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_stores`.\nList of trust stores to check for client certificate."]
    pub fn set_trust_stores(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.trust_stores = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDevicePostureRule_Input_Locations {
    type O = terrars::BlockAssignable<ZeroTrustDevicePostureRule_Input_Locations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDevicePostureRule_Input_Locations {}

impl BuildZeroTrustDevicePostureRule_Input_Locations {
    pub fn build(self) -> ZeroTrustDevicePostureRule_Input_Locations {
        ZeroTrustDevicePostureRule_Input_Locations {
            paths: core::default::Default::default(),
            trust_stores: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDevicePostureRule_Input_LocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDevicePostureRule_Input_LocationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDevicePostureRule_Input_LocationsRef {
        ZeroTrustDevicePostureRule_Input_LocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDevicePostureRule_Input_LocationsRef {
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
pub struct ZeroTrustDevicePostureRule_Input {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_threats: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_state: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_disks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_private_key: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eid_last_seen: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exists: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_key_usage: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infected: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_active: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issue_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_seen: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locations: std::option::Option<ZeroTrustDevicePostureRule_Input_Locations>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_system: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operational_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_distro_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_distro_revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_version_extra: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overall: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    risk_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensor_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha256: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thumbprint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_score: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_window_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_operator: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustDevicePostureRule_Input {
    #[doc = "Set the field `active_threats`.\nThe Number of active threats."]
    pub fn set_active_threats(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.active_threats = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auth_state`.\nThe set of Kolide device authentication states that pass the posture check. Device must match one of the specified states."]
    pub fn set_auth_state(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.auth_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_id`.\nUUID of Cloudflare managed certificate."]
    pub fn set_certificate_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `check_disks`.\nList of volume names to be checked for encryption."]
    pub fn set_check_disks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.check_disks = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `check_private_key`.\nConfirm the certificate was not imported from another device. We recommend keeping this enabled unless the certificate was deployed without a private key."]
    pub fn set_check_private_key(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.check_private_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cn`.\nCommon Name that is protected by the certificate."]
    pub fn set_cn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `compliance_status`.\nCompliance Status.\nAvailable values: \"compliant\", \"noncompliant\", \"unknown\", \"notapplicable\", \"ingraceperiod\", \"error\"."]
    pub fn set_compliance_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compliance_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_id`.\nPosture Integration ID."]
    pub fn set_connection_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `count_operator`.\nCount Operator.\nAvailable values: \"<\", \"<=\", \">\", \">=\", \"==\"."]
    pub fn set_count_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.count_operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\nDomain."]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `eid_last_seen`.\nFor more details on eid last seen, refer to the Tanium documentation."]
    pub fn set_eid_last_seen(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eid_last_seen = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nEnabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exists`.\nWhether or not file exists."]
    pub fn set_exists(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exists = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `extended_key_usage`.\nList of values indicating purposes for which the certificate public key can be used."]
    pub fn set_extended_key_usage(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.extended_key_usage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nList ID."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `infected`.\nWhether device is infected."]
    pub fn set_infected(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.infected = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_active`.\nWhether device is active."]
    pub fn set_is_active(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_active = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issue_count`.\nThe Number of Issues."]
    pub fn set_issue_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issue_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_seen`.\nFor more details on last seen, please refer to the Crowdstrike documentation."]
    pub fn set_last_seen(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_seen = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(mut self, v: impl std::convert::Into<ZeroTrustDevicePostureRule_Input_Locations>) -> Self {
        self.locations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network_status`.\nNetwork status of device.\nAvailable values: \"connected\", \"disconnected\", \"disconnecting\", \"connecting\"."]
    pub fn set_network_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_status = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `operating_system`.\nOperating system.\nAvailable values: \"windows\", \"linux\", \"mac\", \"android\", \"ios\", \"chromeos\"."]
    pub fn set_operating_system(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operating_system = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `operational_state`.\nAgent operational state.\nAvailable values: \"na\", \"partially_disabled\", \"auto_fully_disabled\", \"fully_disabled\", \"auto_partially_disabled\", \"disabled_error\", \"db_corruption\"."]
    pub fn set_operational_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operational_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\nOperator.\nAvailable values: \"<\", \"<=\", \">\", \">=\", \"==\"."]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `os`.\nOs Version."]
    pub fn set_os(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.os = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `os_distro_name`.\nOperating System Distribution Name (linux only)."]
    pub fn set_os_distro_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.os_distro_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `os_distro_revision`.\nVersion of OS Distribution (linux only)."]
    pub fn set_os_distro_revision(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.os_distro_revision = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `os_version_extra`.\nAdditional operating system version details. For Windows, the UBR (Update Build Revision). For Mac or iOS, the Product Version Extra. For Linux, the distribution name and version."]
    pub fn set_os_version_extra(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.os_version_extra = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `overall`.\nOverall."]
    pub fn set_overall(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.overall = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\nFile path."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_all`.\nWhether to check all disks for encryption."]
    pub fn set_require_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_all = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `risk_level`.\nFor more details on risk level, refer to the Tanium documentation.\nAvailable values: \"low\", \"medium\", \"high\", \"critical\"."]
    pub fn set_risk_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.risk_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `score`.\nA value between 0-100 assigned to devices set by the 3rd party posture provider."]
    pub fn set_score(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.score = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `score_operator`.\nScore Operator.\nAvailable values: \"<\", \"<=\", \">\", \">=\", \"==\"."]
    pub fn set_score_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.score_operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensor_config`.\nSensorConfig."]
    pub fn set_sensor_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sensor_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sha256`.\nSHA-256."]
    pub fn set_sha256(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha256 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `state`.\nFor more details on state, please refer to the Crowdstrike documentation.\nAvailable values: \"online\", \"offline\", \"unknown\"."]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_alternative_names`.\nList of certificate Subject Alternative Names."]
    pub fn set_subject_alternative_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subject_alternative_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thumbprint`.\nSigning certificate thumbprint."]
    pub fn set_thumbprint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.thumbprint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_score`.\nFor more details on total score, refer to the Tanium documentation."]
    pub fn set_total_score(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_score = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_window_days`.\nNumber of days that the antivirus should be updated within."]
    pub fn set_update_window_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.update_window_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nVersion of OS."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version_operator`.\nVersion Operator.\nAvailable values: \"<\", \"<=\", \">\", \">=\", \"==\"."]
    pub fn set_version_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_operator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDevicePostureRule_Input {
    type O = terrars::BlockAssignable<ZeroTrustDevicePostureRule_Input>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDevicePostureRule_Input {}

impl BuildZeroTrustDevicePostureRule_Input {
    pub fn build(self) -> ZeroTrustDevicePostureRule_Input {
        ZeroTrustDevicePostureRule_Input {
            active_threats: core::default::Default::default(),
            auth_state: core::default::Default::default(),
            certificate_id: core::default::Default::default(),
            check_disks: core::default::Default::default(),
            check_private_key: core::default::Default::default(),
            cn: core::default::Default::default(),
            compliance_status: core::default::Default::default(),
            connection_id: core::default::Default::default(),
            count_operator: core::default::Default::default(),
            domain: core::default::Default::default(),
            eid_last_seen: core::default::Default::default(),
            enabled: core::default::Default::default(),
            exists: core::default::Default::default(),
            extended_key_usage: core::default::Default::default(),
            id: core::default::Default::default(),
            infected: core::default::Default::default(),
            is_active: core::default::Default::default(),
            issue_count: core::default::Default::default(),
            last_seen: core::default::Default::default(),
            locations: core::default::Default::default(),
            network_status: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            operational_state: core::default::Default::default(),
            operator: core::default::Default::default(),
            os: core::default::Default::default(),
            os_distro_name: core::default::Default::default(),
            os_distro_revision: core::default::Default::default(),
            os_version_extra: core::default::Default::default(),
            overall: core::default::Default::default(),
            path: core::default::Default::default(),
            require_all: core::default::Default::default(),
            risk_level: core::default::Default::default(),
            score: core::default::Default::default(),
            score_operator: core::default::Default::default(),
            sensor_config: core::default::Default::default(),
            sha256: core::default::Default::default(),
            state: core::default::Default::default(),
            subject_alternative_names: core::default::Default::default(),
            thumbprint: core::default::Default::default(),
            total_score: core::default::Default::default(),
            update_window_days: core::default::Default::default(),
            version: core::default::Default::default(),
            version_operator: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDevicePostureRule_InputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDevicePostureRule_InputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDevicePostureRule_InputRef {
        ZeroTrustDevicePostureRule_InputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDevicePostureRule_InputRef {
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
    pub fn locations(&self) -> ZeroTrustDevicePostureRule_Input_LocationsRef {
        ZeroTrustDevicePostureRule_Input_LocationsRef::new(self.shared().clone(), format!("{}.locations", self.base))
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
pub struct ZeroTrustDevicePostureRule_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustDevicePostureRule_Match {
    #[doc =
        "Set the field `platform`.\nAvailable values: \"windows\", \"mac\", \"linux\", \"android\", \"ios\", \"chromeos\"."]
    pub fn set_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.platform = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDevicePostureRule_Match {
    type O = terrars::BlockAssignable<ZeroTrustDevicePostureRule_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDevicePostureRule_Match {}

impl BuildZeroTrustDevicePostureRule_Match {
    pub fn build(self) -> ZeroTrustDevicePostureRule_Match {
        ZeroTrustDevicePostureRule_Match { platform: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDevicePostureRule_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDevicePostureRule_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDevicePostureRule_MatchRef {
        ZeroTrustDevicePostureRule_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDevicePostureRule_MatchRef {
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

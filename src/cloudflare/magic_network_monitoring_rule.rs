#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct MagicNetworkMonitoringRuleData_ {
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
    automatic_advertisement: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bandwidth_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    packet_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_match: std::option::Option<terrars::PrimField<std::string::String>>,
    prefixes: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zscore_sensitivity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zscore_target: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct MagicNetworkMonitoringRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MagicNetworkMonitoringRuleData_>,
}

#[derive(Clone)]
pub struct MagicNetworkMonitoringRule(std::rc::Rc<MagicNetworkMonitoringRule_>);

impl MagicNetworkMonitoringRule {
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

    #[doc =
        "Set the field `bandwidth_threshold`.\nThe number of bits per second for the rule. When this value is exceeded for the set duration, an alert notification is sent. Minimum of 1 and no maximum."]
    pub fn set_bandwidth_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().bandwidth_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `duration`.\nThe amount of time that the rule threshold must be exceeded to send an alert notification. The final value must be equivalent to one of the following 8 values [\"1m\",\"5m\",\"10m\",\"15m\",\"20m\",\"30m\",\"45m\",\"60m\"].\nAvailable values: \"1m\", \"5m\", \"10m\", \"15m\", \"20m\", \"30m\", \"45m\", \"60m\"."]
    pub fn set_duration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `packet_threshold`.\nThe number of packets per second for the rule. When this value is exceeded for the set duration, an alert notification is sent. Minimum of 1 and no maximum."]
    pub fn set_packet_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().packet_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix_match`.\nPrefix match type to be applied for a prefix auto advertisement when using an advanced_ddos rule.\nAvailable values: \"exact\", \"subnet\", \"supernet\"."]
    pub fn set_prefix_match(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().prefix_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zscore_sensitivity`.\nLevel of sensitivity set for zscore rules.\nAvailable values: \"low\", \"medium\", \"high\"."]
    pub fn set_zscore_sensitivity(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zscore_sensitivity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zscore_target`.\nTarget of the zscore rule analysis.\nAvailable values: \"bits\", \"packets\"."]
    pub fn set_zscore_target(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zscore_target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `automatic_advertisement` after provisioning.\nToggle on if you would like Cloudflare to automatically advertise the IP Prefixes within the rule via Magic Transit when the rule is triggered. Only available for users of Magic Transit."]
    pub fn automatic_advertisement(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_advertisement", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bandwidth_threshold` after provisioning.\nThe number of bits per second for the rule. When this value is exceeded for the set duration, an alert notification is sent. Minimum of 1 and no maximum."]
    pub fn bandwidth_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bandwidth_threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nThe amount of time that the rule threshold must be exceeded to send an alert notification. The final value must be equivalent to one of the following 8 values [\"1m\",\"5m\",\"10m\",\"15m\",\"20m\",\"30m\",\"45m\",\"60m\"].\nAvailable values: \"1m\", \"5m\", \"10m\", \"15m\", \"20m\", \"30m\", \"45m\", \"60m\"."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe id of the rule. Must be unique."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the rule. Must be unique. Supports characters A-Z, a-z, 0-9, underscore (_), dash (-), period (.), and tilde (~). You can’t have a space in the rule name. Max 256 characters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `packet_threshold` after provisioning.\nThe number of packets per second for the rule. When this value is exceeded for the set duration, an alert notification is sent. Minimum of 1 and no maximum."]
    pub fn packet_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.packet_threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `prefix_match` after provisioning.\nPrefix match type to be applied for a prefix auto advertisement when using an advanced_ddos rule.\nAvailable values: \"exact\", \"subnet\", \"supernet\"."]
    pub fn prefix_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prefixes` after provisioning.\n"]
    pub fn prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefixes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nMNM rule type.\nAvailable values: \"threshold\", \"zscore\", \"advanced_ddos\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zscore_sensitivity` after provisioning.\nLevel of sensitivity set for zscore rules.\nAvailable values: \"low\", \"medium\", \"high\"."]
    pub fn zscore_sensitivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zscore_sensitivity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zscore_target` after provisioning.\nTarget of the zscore rule analysis.\nAvailable values: \"bits\", \"packets\"."]
    pub fn zscore_target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zscore_target", self.extract_ref()))
    }
}

impl terrars::Referable for MagicNetworkMonitoringRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MagicNetworkMonitoringRule { }

impl terrars::ToListMappable for MagicNetworkMonitoringRule {
    type O = terrars::ListRef<MagicNetworkMonitoringRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MagicNetworkMonitoringRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_magic_network_monitoring_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMagicNetworkMonitoringRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Toggle on if you would like Cloudflare to automatically advertise the IP Prefixes within the rule via Magic Transit when the rule is triggered. Only available for users of Magic Transit."]
    pub automatic_advertisement: terrars::PrimField<bool>,
    #[doc =
        "The name of the rule. Must be unique. Supports characters A-Z, a-z, 0-9, underscore (_), dash (-), period (.), and tilde (~). You can’t have a space in the rule name. Max 256 characters."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub prefixes: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "MNM rule type.\nAvailable values: \"threshold\", \"zscore\", \"advanced_ddos\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildMagicNetworkMonitoringRule {
    pub fn build(self, stack: &mut terrars::Stack) -> MagicNetworkMonitoringRule {
        let out = MagicNetworkMonitoringRule(std::rc::Rc::new(MagicNetworkMonitoringRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MagicNetworkMonitoringRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                automatic_advertisement: self.automatic_advertisement,
                bandwidth_threshold: core::default::Default::default(),
                duration: core::default::Default::default(),
                name: self.name,
                packet_threshold: core::default::Default::default(),
                prefix_match: core::default::Default::default(),
                prefixes: self.prefixes,
                type_: self.type_,
                zscore_sensitivity: core::default::Default::default(),
                zscore_target: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MagicNetworkMonitoringRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicNetworkMonitoringRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MagicNetworkMonitoringRuleRef {
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
        "Get a reference to the value of field `automatic_advertisement` after provisioning.\nToggle on if you would like Cloudflare to automatically advertise the IP Prefixes within the rule via Magic Transit when the rule is triggered. Only available for users of Magic Transit."]
    pub fn automatic_advertisement(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_advertisement", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bandwidth_threshold` after provisioning.\nThe number of bits per second for the rule. When this value is exceeded for the set duration, an alert notification is sent. Minimum of 1 and no maximum."]
    pub fn bandwidth_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bandwidth_threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nThe amount of time that the rule threshold must be exceeded to send an alert notification. The final value must be equivalent to one of the following 8 values [\"1m\",\"5m\",\"10m\",\"15m\",\"20m\",\"30m\",\"45m\",\"60m\"].\nAvailable values: \"1m\", \"5m\", \"10m\", \"15m\", \"20m\", \"30m\", \"45m\", \"60m\"."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe id of the rule. Must be unique."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the rule. Must be unique. Supports characters A-Z, a-z, 0-9, underscore (_), dash (-), period (.), and tilde (~). You can’t have a space in the rule name. Max 256 characters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `packet_threshold` after provisioning.\nThe number of packets per second for the rule. When this value is exceeded for the set duration, an alert notification is sent. Minimum of 1 and no maximum."]
    pub fn packet_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.packet_threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `prefix_match` after provisioning.\nPrefix match type to be applied for a prefix auto advertisement when using an advanced_ddos rule.\nAvailable values: \"exact\", \"subnet\", \"supernet\"."]
    pub fn prefix_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prefixes` after provisioning.\n"]
    pub fn prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefixes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nMNM rule type.\nAvailable values: \"threshold\", \"zscore\", \"advanced_ddos\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zscore_sensitivity` after provisioning.\nLevel of sensitivity set for zscore rules.\nAvailable values: \"low\", \"medium\", \"high\"."]
    pub fn zscore_sensitivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zscore_sensitivity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zscore_target` after provisioning.\nTarget of the zscore rule analysis.\nAvailable values: \"bits\", \"packets\"."]
    pub fn zscore_target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zscore_target", self.extract_ref()))
    }
}

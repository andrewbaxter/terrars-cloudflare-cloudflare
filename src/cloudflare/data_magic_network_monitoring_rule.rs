#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataMagicNetworkMonitoringRuleData_ {
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

struct DataMagicNetworkMonitoringRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMagicNetworkMonitoringRuleData_>,
}

#[derive(Clone)]
pub struct DataMagicNetworkMonitoringRule(std::rc::Rc<DataMagicNetworkMonitoringRule_>);

impl DataMagicNetworkMonitoringRule {
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

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nThe id of the rule. Must be unique."]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
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

impl terrars::Referable for DataMagicNetworkMonitoringRule {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMagicNetworkMonitoringRule { }

impl terrars::ToListMappable for DataMagicNetworkMonitoringRule {
    type O = terrars::ListRef<DataMagicNetworkMonitoringRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMagicNetworkMonitoringRule_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_magic_network_monitoring_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMagicNetworkMonitoringRule {
    pub tf_id: std::string::String,
    #[doc = "The id of the rule. Must be unique."]
    pub rule_id: terrars::PrimField<std::string::String>,
}

impl BuildDataMagicNetworkMonitoringRule {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMagicNetworkMonitoringRule {
        let out = DataMagicNetworkMonitoringRule(std::rc::Rc::new(DataMagicNetworkMonitoringRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMagicNetworkMonitoringRuleData_ {
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

pub struct DataMagicNetworkMonitoringRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicNetworkMonitoringRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMagicNetworkMonitoringRuleRef {
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

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nThe id of the rule. Must be unique."]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
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

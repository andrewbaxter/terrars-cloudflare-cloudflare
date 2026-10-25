#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataMagicNetworkMonitoringConfigurationData_ {
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
}

struct DataMagicNetworkMonitoringConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMagicNetworkMonitoringConfigurationData_>,
}

#[derive(Clone)]
pub struct DataMagicNetworkMonitoringConfiguration(std::rc::Rc<DataMagicNetworkMonitoringConfiguration_>);

impl DataMagicNetworkMonitoringConfiguration {
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
        "Get a reference to the value of field `default_sampling` after provisioning.\nFallback sampling rate of flow messages being sent in packets per second. This should match the packet sampling rate configured on the router."]
    pub fn default_sampling(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_sampling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe account name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `router_ips` after provisioning.\n"]
    pub fn router_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.router_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warp_devices` after provisioning.\n"]
    pub fn warp_devices(&self) -> terrars::ListRef<DataMagicNetworkMonitoringConfiguration_WarpDevicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warp_devices", self.extract_ref()))
    }
}

impl terrars::Referable for DataMagicNetworkMonitoringConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMagicNetworkMonitoringConfiguration { }

impl terrars::ToListMappable for DataMagicNetworkMonitoringConfiguration {
    type O = terrars::ListRef<DataMagicNetworkMonitoringConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMagicNetworkMonitoringConfiguration_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_magic_network_monitoring_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMagicNetworkMonitoringConfiguration {
    pub tf_id: std::string::String,
}

impl BuildDataMagicNetworkMonitoringConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMagicNetworkMonitoringConfiguration {
        let out = DataMagicNetworkMonitoringConfiguration(std::rc::Rc::new(DataMagicNetworkMonitoringConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMagicNetworkMonitoringConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMagicNetworkMonitoringConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicNetworkMonitoringConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMagicNetworkMonitoringConfigurationRef {
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
        "Get a reference to the value of field `default_sampling` after provisioning.\nFallback sampling rate of flow messages being sent in packets per second. This should match the packet sampling rate configured on the router."]
    pub fn default_sampling(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_sampling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe account name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `router_ips` after provisioning.\n"]
    pub fn router_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.router_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warp_devices` after provisioning.\n"]
    pub fn warp_devices(&self) -> terrars::ListRef<DataMagicNetworkMonitoringConfiguration_WarpDevicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warp_devices", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMagicNetworkMonitoringConfiguration_WarpDevices {}

impl DataMagicNetworkMonitoringConfiguration_WarpDevices { }

impl terrars::ToListMappable for DataMagicNetworkMonitoringConfiguration_WarpDevices {
    type O = terrars::BlockAssignable<DataMagicNetworkMonitoringConfiguration_WarpDevices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicNetworkMonitoringConfiguration_WarpDevices {}

impl BuildDataMagicNetworkMonitoringConfiguration_WarpDevices {
    pub fn build(self) -> DataMagicNetworkMonitoringConfiguration_WarpDevices {
        DataMagicNetworkMonitoringConfiguration_WarpDevices {}
    }
}

#[allow(dead_code)]
pub struct DataMagicNetworkMonitoringConfiguration_WarpDevicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicNetworkMonitoringConfiguration_WarpDevicesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicNetworkMonitoringConfiguration_WarpDevicesRef {
        DataMagicNetworkMonitoringConfiguration_WarpDevicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicNetworkMonitoringConfiguration_WarpDevicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the warp device."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the warp device."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `router_ip` after provisioning.\nIPv4 CIDR of the router sourcing flow data associated with this warp device. Only /32 addresses are currently supported."]
    pub fn router_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router_ip", self.base))
    }
}

#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct MagicNetworkMonitoringConfigurationData_ {
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
    default_sampling: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    router_ips: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warp_devices: std::option::Option<std::vec::Vec<MagicNetworkMonitoringConfiguration_WarpDevices>>,
}

struct MagicNetworkMonitoringConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MagicNetworkMonitoringConfigurationData_>,
}

#[derive(Clone)]
pub struct MagicNetworkMonitoringConfiguration(std::rc::Rc<MagicNetworkMonitoringConfiguration_>);

impl MagicNetworkMonitoringConfiguration {
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
        "Set the field `default_sampling`.\nFallback sampling rate of flow messages being sent in packets per second. This should match the packet sampling rate configured on the router."]
    pub fn set_default_sampling(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_sampling = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `router_ips`.\n"]
    pub fn set_router_ips(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().router_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warp_devices`.\n"]
    pub fn set_warp_devices(
        self,
        v: impl std::convert::Into<std::vec::Vec<MagicNetworkMonitoringConfiguration_WarpDevices>>,
    ) -> Self {
        self.0.data.borrow_mut().warp_devices = std::option::Option::Some(v.into());
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
    pub fn warp_devices(&self) -> terrars::ListRef<MagicNetworkMonitoringConfiguration_WarpDevicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warp_devices", self.extract_ref()))
    }
}

impl terrars::Referable for MagicNetworkMonitoringConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MagicNetworkMonitoringConfiguration { }

impl terrars::ToListMappable for MagicNetworkMonitoringConfiguration {
    type O = terrars::ListRef<MagicNetworkMonitoringConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MagicNetworkMonitoringConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_magic_network_monitoring_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMagicNetworkMonitoringConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "The account name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMagicNetworkMonitoringConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> MagicNetworkMonitoringConfiguration {
        let out = MagicNetworkMonitoringConfiguration(std::rc::Rc::new(MagicNetworkMonitoringConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MagicNetworkMonitoringConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                default_sampling: core::default::Default::default(),
                name: self.name,
                router_ips: core::default::Default::default(),
                warp_devices: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MagicNetworkMonitoringConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicNetworkMonitoringConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MagicNetworkMonitoringConfigurationRef {
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
    pub fn warp_devices(&self) -> terrars::ListRef<MagicNetworkMonitoringConfiguration_WarpDevicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warp_devices", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MagicNetworkMonitoringConfiguration_WarpDevices {
    id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    router_ip: terrars::PrimField<std::string::String>,
}

impl MagicNetworkMonitoringConfiguration_WarpDevices { }

impl terrars::ToListMappable for MagicNetworkMonitoringConfiguration_WarpDevices {
    type O = terrars::BlockAssignable<MagicNetworkMonitoringConfiguration_WarpDevices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicNetworkMonitoringConfiguration_WarpDevices {
    #[doc = "Unique identifier for the warp device."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the warp device."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "IPv4 CIDR of the router sourcing flow data associated with this warp device. Only /32 addresses are currently supported."]
    pub router_ip: terrars::PrimField<std::string::String>,
}

impl BuildMagicNetworkMonitoringConfiguration_WarpDevices {
    pub fn build(self) -> MagicNetworkMonitoringConfiguration_WarpDevices {
        MagicNetworkMonitoringConfiguration_WarpDevices {
            id: self.id,
            name: self.name,
            router_ip: self.router_ip,
        }
    }
}

#[allow(dead_code)]
pub struct MagicNetworkMonitoringConfiguration_WarpDevicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicNetworkMonitoringConfiguration_WarpDevicesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MagicNetworkMonitoringConfiguration_WarpDevicesRef {
        MagicNetworkMonitoringConfiguration_WarpDevicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicNetworkMonitoringConfiguration_WarpDevicesRef {
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

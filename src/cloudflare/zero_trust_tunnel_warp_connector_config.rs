#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustTunnelWarpConnectorConfigData_ {
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
    config: std::option::Option<ZeroTrustTunnelWarpConnectorConfig_Config>,
    ha_mode: terrars::PrimField<std::string::String>,
    tunnel_id: terrars::PrimField<std::string::String>,
}

struct ZeroTrustTunnelWarpConnectorConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustTunnelWarpConnectorConfigData_>,
}

#[derive(Clone)]
pub struct ZeroTrustTunnelWarpConnectorConfig(std::rc::Rc<ZeroTrustTunnelWarpConnectorConfig_>);

impl ZeroTrustTunnelWarpConnectorConfig {
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
        "Set the field `config`.\nProvider-specific configuration. Required shape depends on ha_mode. For `aws`, must contain `fnr_id`. For `local`, must contain `vips`. For `none` and `disabled`, must be empty or omitted."]
    pub fn set_config(self, v: impl std::convert::Into<ZeroTrustTunnelWarpConnectorConfig_Config>) -> Self {
        self.0.data.borrow_mut().config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nProvider-specific configuration. Required shape depends on ha_mode. For `aws`, must contain `fnr_id`. For `local`, must contain `vips`. For `none` and `disabled`, must be empty or omitted."]
    pub fn config(&self) -> ZeroTrustTunnelWarpConnectorConfig_ConfigRef {
        ZeroTrustTunnelWarpConnectorConfig_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `configuration_version` after provisioning.\nMonotonically increasing configuration version, incremented on each PUT."]
    pub fn configuration_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp of when the resource was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ha_mode` after provisioning.\nHigh-availability mode for the Mesh node. `none` means HA is enabled but no provider is configured yet (newly created nodes default to this). `disabled` means HA is explicitly turned off. `aws` uses AWS ENI move for failover. `local` uses virtual IPs (VIPs) on the local interface.\nAvailable values: \"none\", \"disabled\", \"aws\", \"local\"."]
    pub fn ha_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ha_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the tunnel."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\nUUID of the tunnel."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp of the last update. Null if never updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustTunnelWarpConnectorConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustTunnelWarpConnectorConfig { }

impl terrars::ToListMappable for ZeroTrustTunnelWarpConnectorConfig {
    type O = terrars::ListRef<ZeroTrustTunnelWarpConnectorConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustTunnelWarpConnectorConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_tunnel_warp_connector_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustTunnelWarpConnectorConfig {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "High-availability mode for the Mesh node. `none` means HA is enabled but no provider is configured yet (newly created nodes default to this). `disabled` means HA is explicitly turned off. `aws` uses AWS ENI move for failover. `local` uses virtual IPs (VIPs) on the local interface.\nAvailable values: \"none\", \"disabled\", \"aws\", \"local\"."]
    pub ha_mode: terrars::PrimField<std::string::String>,
    #[doc = "UUID of the tunnel."]
    pub tunnel_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustTunnelWarpConnectorConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustTunnelWarpConnectorConfig {
        let out = ZeroTrustTunnelWarpConnectorConfig(std::rc::Rc::new(ZeroTrustTunnelWarpConnectorConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustTunnelWarpConnectorConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                config: core::default::Default::default(),
                ha_mode: self.ha_mode,
                tunnel_id: self.tunnel_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustTunnelWarpConnectorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustTunnelWarpConnectorConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustTunnelWarpConnectorConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nProvider-specific configuration. Required shape depends on ha_mode. For `aws`, must contain `fnr_id`. For `local`, must contain `vips`. For `none` and `disabled`, must be empty or omitted."]
    pub fn config(&self) -> ZeroTrustTunnelWarpConnectorConfig_ConfigRef {
        ZeroTrustTunnelWarpConnectorConfig_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `configuration_version` after provisioning.\nMonotonically increasing configuration version, incremented on each PUT."]
    pub fn configuration_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp of when the resource was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ha_mode` after provisioning.\nHigh-availability mode for the Mesh node. `none` means HA is enabled but no provider is configured yet (newly created nodes default to this). `disabled` means HA is explicitly turned off. `aws` uses AWS ENI move for failover. `local` uses virtual IPs (VIPs) on the local interface.\nAvailable values: \"none\", \"disabled\", \"aws\", \"local\"."]
    pub fn ha_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ha_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the tunnel."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\nUUID of the tunnel."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp of the last update. Null if never updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustTunnelWarpConnectorConfig_Config_Vips {
    address: terrars::PrimField<std::string::String>,
}

impl ZeroTrustTunnelWarpConnectorConfig_Config_Vips { }

impl terrars::ToListMappable for ZeroTrustTunnelWarpConnectorConfig_Config_Vips {
    type O = terrars::BlockAssignable<ZeroTrustTunnelWarpConnectorConfig_Config_Vips>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustTunnelWarpConnectorConfig_Config_Vips {
    #[doc = "Virtual IP address (IPv4 or IPv6)."]
    pub address: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustTunnelWarpConnectorConfig_Config_Vips {
    pub fn build(self) -> ZeroTrustTunnelWarpConnectorConfig_Config_Vips {
        ZeroTrustTunnelWarpConnectorConfig_Config_Vips { address: self.address }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustTunnelWarpConnectorConfig_Config_VipsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustTunnelWarpConnectorConfig_Config_VipsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustTunnelWarpConnectorConfig_Config_VipsRef {
        ZeroTrustTunnelWarpConnectorConfig_Config_VipsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustTunnelWarpConnectorConfig_Config_VipsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\nVirtual IP address (IPv4 or IPv6)."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious {
    address: terrars::PrimField<std::string::String>,
}

impl ZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious { }

impl terrars::ToListMappable for ZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious {
    type O = terrars::BlockAssignable<ZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious {
    #[doc = "Virtual IP address (IPv4 or IPv6)."]
    pub address: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious {
    pub fn build(self) -> ZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious {
        ZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious { address: self.address }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustTunnelWarpConnectorConfig_Config_VipsPreviousRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustTunnelWarpConnectorConfig_Config_VipsPreviousRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustTunnelWarpConnectorConfig_Config_VipsPreviousRef {
        ZeroTrustTunnelWarpConnectorConfig_Config_VipsPreviousRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustTunnelWarpConnectorConfig_Config_VipsPreviousRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\nVirtual IP address (IPv4 or IPv6)."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustTunnelWarpConnectorConfig_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fnr_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vips: std::option::Option<std::vec::Vec<ZeroTrustTunnelWarpConnectorConfig_Config_Vips>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vips_previous: std::option::Option<std::vec::Vec<ZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious>>,
}

impl ZeroTrustTunnelWarpConnectorConfig_Config {
    #[doc =
        "Set the field `fnr_id`.\nFloating Network Resource ID — the secondary ENI that is moved between nodes on failover."]
    pub fn set_fnr_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fnr_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vips`.\nVIPs to assign on the CloudflareWARP interface."]
    pub fn set_vips(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustTunnelWarpConnectorConfig_Config_Vips>>,
    ) -> Self {
        self.vips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vips_previous`.\nVIPs to clean up on demotion or version drift."]
    pub fn set_vips_previous(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious>>,
    ) -> Self {
        self.vips_previous = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustTunnelWarpConnectorConfig_Config {
    type O = terrars::BlockAssignable<ZeroTrustTunnelWarpConnectorConfig_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustTunnelWarpConnectorConfig_Config {}

impl BuildZeroTrustTunnelWarpConnectorConfig_Config {
    pub fn build(self) -> ZeroTrustTunnelWarpConnectorConfig_Config {
        ZeroTrustTunnelWarpConnectorConfig_Config {
            fnr_id: core::default::Default::default(),
            vips: core::default::Default::default(),
            vips_previous: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustTunnelWarpConnectorConfig_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustTunnelWarpConnectorConfig_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustTunnelWarpConnectorConfig_ConfigRef {
        ZeroTrustTunnelWarpConnectorConfig_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustTunnelWarpConnectorConfig_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fnr_id` after provisioning.\nFloating Network Resource ID — the secondary ENI that is moved between nodes on failover."]
    pub fn fnr_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fnr_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vips` after provisioning.\nVIPs to assign on the CloudflareWARP interface."]
    pub fn vips(&self) -> terrars::ListRef<ZeroTrustTunnelWarpConnectorConfig_Config_VipsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vips_previous` after provisioning.\nVIPs to clean up on demotion or version drift."]
    pub fn vips_previous(&self) -> terrars::ListRef<ZeroTrustTunnelWarpConnectorConfig_Config_VipsPreviousRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vips_previous", self.base))
    }
}

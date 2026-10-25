#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustTunnelWarpConnectorConfigData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    tunnel_id: terrars::PrimField<std::string::String>,
}

struct DataZeroTrustTunnelWarpConnectorConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustTunnelWarpConnectorConfigData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustTunnelWarpConnectorConfig(std::rc::Rc<DataZeroTrustTunnelWarpConnectorConfig_>);

impl DataZeroTrustTunnelWarpConnectorConfig {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nProvider-specific configuration. Present for `aws` and `local` modes."]
    pub fn config(&self) -> DataZeroTrustTunnelWarpConnectorConfig_ConfigRef {
        DataZeroTrustTunnelWarpConnectorConfig_ConfigRef::new(
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

impl terrars::Referable for DataZeroTrustTunnelWarpConnectorConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustTunnelWarpConnectorConfig { }

impl terrars::ToListMappable for DataZeroTrustTunnelWarpConnectorConfig {
    type O = terrars::ListRef<DataZeroTrustTunnelWarpConnectorConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustTunnelWarpConnectorConfig_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_tunnel_warp_connector_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustTunnelWarpConnectorConfig {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "UUID of the tunnel."]
    pub tunnel_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustTunnelWarpConnectorConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustTunnelWarpConnectorConfig {
        let out = DataZeroTrustTunnelWarpConnectorConfig(std::rc::Rc::new(DataZeroTrustTunnelWarpConnectorConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustTunnelWarpConnectorConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                tunnel_id: self.tunnel_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustTunnelWarpConnectorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelWarpConnectorConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustTunnelWarpConnectorConfigRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nProvider-specific configuration. Present for `aws` and `local` modes."]
    pub fn config(&self) -> DataZeroTrustTunnelWarpConnectorConfig_ConfigRef {
        DataZeroTrustTunnelWarpConnectorConfig_ConfigRef::new(
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
pub struct DataZeroTrustTunnelWarpConnectorConfig_Config_Vips {}

impl DataZeroTrustTunnelWarpConnectorConfig_Config_Vips { }

impl terrars::ToListMappable for DataZeroTrustTunnelWarpConnectorConfig_Config_Vips {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelWarpConnectorConfig_Config_Vips>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelWarpConnectorConfig_Config_Vips {}

impl BuildDataZeroTrustTunnelWarpConnectorConfig_Config_Vips {
    pub fn build(self) -> DataZeroTrustTunnelWarpConnectorConfig_Config_Vips {
        DataZeroTrustTunnelWarpConnectorConfig_Config_Vips {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelWarpConnectorConfig_Config_VipsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelWarpConnectorConfig_Config_VipsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustTunnelWarpConnectorConfig_Config_VipsRef {
        DataZeroTrustTunnelWarpConnectorConfig_Config_VipsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelWarpConnectorConfig_Config_VipsRef {
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
pub struct DataZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious {}

impl DataZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious { }

impl terrars::ToListMappable for DataZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious {}

impl BuildDataZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious {
    pub fn build(self) -> DataZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious {
        DataZeroTrustTunnelWarpConnectorConfig_Config_VipsPrevious {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelWarpConnectorConfig_Config_VipsPreviousRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelWarpConnectorConfig_Config_VipsPreviousRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustTunnelWarpConnectorConfig_Config_VipsPreviousRef {
        DataZeroTrustTunnelWarpConnectorConfig_Config_VipsPreviousRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelWarpConnectorConfig_Config_VipsPreviousRef {
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
pub struct DataZeroTrustTunnelWarpConnectorConfig_Config {}

impl DataZeroTrustTunnelWarpConnectorConfig_Config { }

impl terrars::ToListMappable for DataZeroTrustTunnelWarpConnectorConfig_Config {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelWarpConnectorConfig_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelWarpConnectorConfig_Config {}

impl BuildDataZeroTrustTunnelWarpConnectorConfig_Config {
    pub fn build(self) -> DataZeroTrustTunnelWarpConnectorConfig_Config {
        DataZeroTrustTunnelWarpConnectorConfig_Config {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelWarpConnectorConfig_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelWarpConnectorConfig_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustTunnelWarpConnectorConfig_ConfigRef {
        DataZeroTrustTunnelWarpConnectorConfig_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelWarpConnectorConfig_ConfigRef {
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
    pub fn vips(&self) -> terrars::ListRef<DataZeroTrustTunnelWarpConnectorConfig_Config_VipsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vips_previous` after provisioning.\nVIPs to clean up on demotion or version drift."]
    pub fn vips_previous(&self) -> terrars::ListRef<DataZeroTrustTunnelWarpConnectorConfig_Config_VipsPreviousRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vips_previous", self.base))
    }
}

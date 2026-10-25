#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustTunnelWarpConnectorData_ {
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
    filter: std::option::Option<DataZeroTrustTunnelWarpConnector_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustTunnelWarpConnector_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustTunnelWarpConnectorData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustTunnelWarpConnector(std::rc::Rc<DataZeroTrustTunnelWarpConnector_>);

impl DataZeroTrustTunnelWarpConnector {
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

    #[doc = "Set the field `account_id`.\nCloudflare account ID"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataZeroTrustTunnelWarpConnector_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel_id`.\nUUID of the tunnel."]
    pub fn set_tunnel_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tunnel_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_tag` after provisioning.\nCloudflare account ID"]
    pub fn account_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connections` after provisioning.\nThe Cloudflare Tunnel connections between your origin and Cloudflare's edge."]
    pub fn connections(&self) -> terrars::ListRef<DataZeroTrustTunnelWarpConnector_ConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `conns_active_at` after provisioning.\nTimestamp of when the tunnel established at least one connection to Cloudflare's edge. If `null`, the tunnel is inactive."]
    pub fn conns_active_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conns_active_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `conns_inactive_at` after provisioning.\nTimestamp of when the tunnel became inactive (no connections to Cloudflare's edge). If `null`, the tunnel is active."]
    pub fn conns_inactive_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conns_inactive_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp of when the resource was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deleted_at` after provisioning.\nTimestamp of when the resource was deleted. If `null`, the resource has not been deleted."]
    pub fn deleted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustTunnelWarpConnector_FilterRef {
        DataZeroTrustTunnelWarpConnector_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the tunnel."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nMetadata associated with the tunnel."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-friendly name for a tunnel."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of the tunnel. Valid values are `inactive` (tunnel has never been run), `degraded` (tunnel is active and able to serve traffic but in an unhealthy state), `healthy` (tunnel is active and able to serve traffic), or `down` (tunnel can not serve traffic as it has no connections to the Cloudflare Edge).\nAvailable values: \"inactive\", \"degraded\", \"healthy\", \"down\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tun_type` after provisioning.\nThe type of tunnel.\nAvailable values: \"cfd_tunnel\", \"warp_connector\", \"warp\", \"magic\", \"ip_sec\", \"gre\", \"cni\"."]
    pub fn tun_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tun_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\nUUID of the tunnel."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustTunnelWarpConnector {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustTunnelWarpConnector { }

impl terrars::ToListMappable for DataZeroTrustTunnelWarpConnector {
    type O = terrars::ListRef<DataZeroTrustTunnelWarpConnectorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustTunnelWarpConnector_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_tunnel_warp_connector".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustTunnelWarpConnector {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustTunnelWarpConnector {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustTunnelWarpConnector {
        let out = DataZeroTrustTunnelWarpConnector(std::rc::Rc::new(DataZeroTrustTunnelWarpConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustTunnelWarpConnectorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                tunnel_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustTunnelWarpConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelWarpConnectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustTunnelWarpConnectorRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_tag` after provisioning.\nCloudflare account ID"]
    pub fn account_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connections` after provisioning.\nThe Cloudflare Tunnel connections between your origin and Cloudflare's edge."]
    pub fn connections(&self) -> terrars::ListRef<DataZeroTrustTunnelWarpConnector_ConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `conns_active_at` after provisioning.\nTimestamp of when the tunnel established at least one connection to Cloudflare's edge. If `null`, the tunnel is inactive."]
    pub fn conns_active_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conns_active_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `conns_inactive_at` after provisioning.\nTimestamp of when the tunnel became inactive (no connections to Cloudflare's edge). If `null`, the tunnel is active."]
    pub fn conns_inactive_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conns_inactive_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp of when the resource was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deleted_at` after provisioning.\nTimestamp of when the resource was deleted. If `null`, the resource has not been deleted."]
    pub fn deleted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustTunnelWarpConnector_FilterRef {
        DataZeroTrustTunnelWarpConnector_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the tunnel."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nMetadata associated with the tunnel."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-friendly name for a tunnel."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of the tunnel. Valid values are `inactive` (tunnel has never been run), `degraded` (tunnel is active and able to serve traffic but in an unhealthy state), `healthy` (tunnel is active and able to serve traffic), or `down` (tunnel can not serve traffic as it has no connections to the Cloudflare Edge).\nAvailable values: \"inactive\", \"degraded\", \"healthy\", \"down\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tun_type` after provisioning.\nThe type of tunnel.\nAvailable values: \"cfd_tunnel\", \"warp_connector\", \"warp\", \"magic\", \"ip_sec\", \"gre\", \"cni\"."]
    pub fn tun_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tun_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\nUUID of the tunnel."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustTunnelWarpConnector_Connections {}

impl DataZeroTrustTunnelWarpConnector_Connections { }

impl terrars::ToListMappable for DataZeroTrustTunnelWarpConnector_Connections {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelWarpConnector_Connections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelWarpConnector_Connections {}

impl BuildDataZeroTrustTunnelWarpConnector_Connections {
    pub fn build(self) -> DataZeroTrustTunnelWarpConnector_Connections {
        DataZeroTrustTunnelWarpConnector_Connections {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelWarpConnector_ConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelWarpConnector_ConnectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustTunnelWarpConnector_ConnectionsRef {
        DataZeroTrustTunnelWarpConnector_ConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelWarpConnector_ConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nUUID of the Cloudflare Tunnel connector."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_version` after provisioning.\nThe cloudflared version used to establish this connection."]
    pub fn client_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `colo_name` after provisioning.\nThe Cloudflare data center used for this connection."]
    pub fn colo_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.colo_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the Cloudflare Tunnel connection."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_pending_reconnect` after provisioning.\nCloudflare continues to track connections for several minutes after they disconnect. This is an optimization to improve latency and reliability of reconnecting.  If `true`, the connection has disconnected but is still being tracked. If `false`, the connection is actively serving traffic."]
    pub fn is_pending_reconnect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_pending_reconnect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `opened_at` after provisioning.\nTimestamp of when the connection was established."]
    pub fn opened_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opened_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_ip` after provisioning.\nThe public IP address of the host running cloudflared."]
    pub fn origin_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uuid` after provisioning.\nUUID of the Cloudflare Tunnel connection."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustTunnelWarpConnector_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    existed_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_deleted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    was_active_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    was_inactive_at: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataZeroTrustTunnelWarpConnector_Filter {
    #[doc = "Set the field `exclude_prefix`.\n"]
    pub fn set_exclude_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exclude_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `existed_at`.\nIf provided, include only resources that were created (and not deleted) before this time. URL encoded."]
    pub fn set_existed_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.existed_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_prefix`.\n"]
    pub fn set_include_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.include_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_deleted`.\nIf `true`, only include deleted tunnels. If `false`, exclude deleted tunnels. If empty, all tunnels will be included."]
    pub fn set_is_deleted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_deleted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nA user-friendly name for the tunnel."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nThe status of the tunnel. Valid values are `inactive` (tunnel has never been run), `degraded` (tunnel is active and able to serve traffic but in an unhealthy state), `healthy` (tunnel is active and able to serve traffic), or `down` (tunnel can not serve traffic as it has no connections to the Cloudflare Edge).\nAvailable values: \"inactive\", \"degraded\", \"healthy\", \"down\"."]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\nUUID of the tunnel."]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `was_active_at`.\n"]
    pub fn set_was_active_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.was_active_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `was_inactive_at`.\n"]
    pub fn set_was_inactive_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.was_inactive_at = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZeroTrustTunnelWarpConnector_Filter {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelWarpConnector_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelWarpConnector_Filter {}

impl BuildDataZeroTrustTunnelWarpConnector_Filter {
    pub fn build(self) -> DataZeroTrustTunnelWarpConnector_Filter {
        DataZeroTrustTunnelWarpConnector_Filter {
            exclude_prefix: core::default::Default::default(),
            existed_at: core::default::Default::default(),
            include_prefix: core::default::Default::default(),
            is_deleted: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
            uuid: core::default::Default::default(),
            was_active_at: core::default::Default::default(),
            was_inactive_at: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelWarpConnector_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelWarpConnector_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustTunnelWarpConnector_FilterRef {
        DataZeroTrustTunnelWarpConnector_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelWarpConnector_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude_prefix` after provisioning.\n"]
    pub fn exclude_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `existed_at` after provisioning.\nIf provided, include only resources that were created (and not deleted) before this time. URL encoded."]
    pub fn existed_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.existed_at", self.base))
    }

    #[doc = "Get a reference to the value of field `include_prefix` after provisioning.\n"]
    pub fn include_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_deleted` after provisioning.\nIf `true`, only include deleted tunnels. If `false`, exclude deleted tunnels. If empty, all tunnels will be included."]
    pub fn is_deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_deleted", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-friendly name for the tunnel."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of the tunnel. Valid values are `inactive` (tunnel has never been run), `degraded` (tunnel is active and able to serve traffic but in an unhealthy state), `healthy` (tunnel is active and able to serve traffic), or `down` (tunnel can not serve traffic as it has no connections to the Cloudflare Edge).\nAvailable values: \"inactive\", \"degraded\", \"healthy\", \"down\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nUUID of the tunnel."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `was_active_at` after provisioning.\n"]
    pub fn was_active_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.was_active_at", self.base))
    }

    #[doc = "Get a reference to the value of field `was_inactive_at` after provisioning.\n"]
    pub fn was_inactive_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.was_inactive_at", self.base))
    }
}

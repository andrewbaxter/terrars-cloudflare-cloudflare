#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustTunnelCloudflaredData_ {
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
    config_src: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel_secret: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ZeroTrustTunnelCloudflared_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustTunnelCloudflaredData_>,
}

#[derive(Clone)]
pub struct ZeroTrustTunnelCloudflared(std::rc::Rc<ZeroTrustTunnelCloudflared_>);

impl ZeroTrustTunnelCloudflared {
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
        "Set the field `config_src`.\nIndicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel on the Zero Trust dashboard.\nAvailable values: \"local\", \"cloudflare\"."]
    pub fn set_config_src(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().config_src = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tunnel_secret`.\nSets the password required to run a locally-managed tunnel. Must be at least 32 bytes and encoded as a base64 string."]
    pub fn set_tunnel_secret(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tunnel_secret = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `config_src` after provisioning.\nIndicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel on the Zero Trust dashboard.\nAvailable values: \"local\", \"cloudflare\"."]
    pub fn config_src(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_src", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connections` after provisioning.\nThe Cloudflare Tunnel connections between your origin and Cloudflare's edge."]
    pub fn connections(&self) -> terrars::ListRef<ZeroTrustTunnelCloudflared_ConnectionsRef> {
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
        "Get a reference to the value of field `remote_config` after provisioning.\nIf `true`, the tunnel can be configured remotely from the Zero Trust dashboard. If `false`, the tunnel must be configured locally on the origin machine."]
    pub fn remote_config(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_config", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `tunnel_secret` after provisioning.\nSets the password required to run a locally-managed tunnel. Must be at least 32 bytes and encoded as a base64 string."]
    pub fn tunnel_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_secret", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustTunnelCloudflared {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustTunnelCloudflared { }

impl terrars::ToListMappable for ZeroTrustTunnelCloudflared {
    type O = terrars::ListRef<ZeroTrustTunnelCloudflaredRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustTunnelCloudflared_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_tunnel_cloudflared".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustTunnelCloudflared {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account ID"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "A user-friendly name for a tunnel."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustTunnelCloudflared {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustTunnelCloudflared {
        let out = ZeroTrustTunnelCloudflared(std::rc::Rc::new(ZeroTrustTunnelCloudflared_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustTunnelCloudflaredData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                config_src: core::default::Default::default(),
                name: self.name,
                tunnel_secret: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustTunnelCloudflaredRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustTunnelCloudflaredRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustTunnelCloudflaredRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `config_src` after provisioning.\nIndicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel on the Zero Trust dashboard.\nAvailable values: \"local\", \"cloudflare\"."]
    pub fn config_src(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_src", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connections` after provisioning.\nThe Cloudflare Tunnel connections between your origin and Cloudflare's edge."]
    pub fn connections(&self) -> terrars::ListRef<ZeroTrustTunnelCloudflared_ConnectionsRef> {
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
        "Get a reference to the value of field `remote_config` after provisioning.\nIf `true`, the tunnel can be configured remotely from the Zero Trust dashboard. If `false`, the tunnel must be configured locally on the origin machine."]
    pub fn remote_config(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_config", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `tunnel_secret` after provisioning.\nSets the password required to run a locally-managed tunnel. Must be at least 32 bytes and encoded as a base64 string."]
    pub fn tunnel_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_secret", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustTunnelCloudflared_Connections {}

impl ZeroTrustTunnelCloudflared_Connections { }

impl terrars::ToListMappable for ZeroTrustTunnelCloudflared_Connections {
    type O = terrars::BlockAssignable<ZeroTrustTunnelCloudflared_Connections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustTunnelCloudflared_Connections {}

impl BuildZeroTrustTunnelCloudflared_Connections {
    pub fn build(self) -> ZeroTrustTunnelCloudflared_Connections {
        ZeroTrustTunnelCloudflared_Connections {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustTunnelCloudflared_ConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustTunnelCloudflared_ConnectionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustTunnelCloudflared_ConnectionsRef {
        ZeroTrustTunnelCloudflared_ConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustTunnelCloudflared_ConnectionsRef {
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

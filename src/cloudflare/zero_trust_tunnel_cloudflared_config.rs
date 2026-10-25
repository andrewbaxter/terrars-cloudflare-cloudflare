#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustTunnelCloudflaredConfigData_ {
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
    config: std::option::Option<ZeroTrustTunnelCloudflaredConfig_Config>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    tunnel_id: terrars::PrimField<std::string::String>,
}

struct ZeroTrustTunnelCloudflaredConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustTunnelCloudflaredConfigData_>,
}

#[derive(Clone)]
pub struct ZeroTrustTunnelCloudflaredConfig(std::rc::Rc<ZeroTrustTunnelCloudflaredConfig_>);

impl ZeroTrustTunnelCloudflaredConfig {
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

    #[doc = "Set the field `config`.\nThe tunnel configuration and ingress rules."]
    pub fn set_config(self, v: impl std::convert::Into<ZeroTrustTunnelCloudflaredConfig_Config>) -> Self {
        self.0.data.borrow_mut().config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source`.\nIndicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel's configuration on the Zero Trust dashboard.\nAvailable values: \"local\", \"cloudflare\"."]
    pub fn set_source(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe tunnel configuration and ingress rules."]
    pub fn config(&self) -> ZeroTrustTunnelCloudflaredConfig_ConfigRef {
        ZeroTrustTunnelCloudflaredConfig_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the tunnel."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nIndicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel's configuration on the Zero Trust dashboard.\nAvailable values: \"local\", \"cloudflare\"."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\nUUID of the tunnel."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe version of the Tunnel Configuration."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustTunnelCloudflaredConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustTunnelCloudflaredConfig { }

impl terrars::ToListMappable for ZeroTrustTunnelCloudflaredConfig {
    type O = terrars::ListRef<ZeroTrustTunnelCloudflaredConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustTunnelCloudflaredConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_tunnel_cloudflared_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustTunnelCloudflaredConfig {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "UUID of the tunnel."]
    pub tunnel_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustTunnelCloudflaredConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustTunnelCloudflaredConfig {
        let out = ZeroTrustTunnelCloudflaredConfig(std::rc::Rc::new(ZeroTrustTunnelCloudflaredConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustTunnelCloudflaredConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                config: core::default::Default::default(),
                source: core::default::Default::default(),
                tunnel_id: self.tunnel_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustTunnelCloudflaredConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustTunnelCloudflaredConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustTunnelCloudflaredConfigRef {
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
        "Get a reference to the value of field `config` after provisioning.\nThe tunnel configuration and ingress rules."]
    pub fn config(&self) -> ZeroTrustTunnelCloudflaredConfig_ConfigRef {
        ZeroTrustTunnelCloudflaredConfig_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the tunnel."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nIndicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel's configuration on the Zero Trust dashboard.\nAvailable values: \"local\", \"cloudflare\"."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\nUUID of the tunnel."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe version of the Tunnel Configuration."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access {
    aud_tag: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
    team_name: terrars::PrimField<std::string::String>,
}

impl ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access {
    #[doc = "Set the field `required`.\nDeny traffic that has not fulfilled Access authorization."]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access {
    type O = terrars::BlockAssignable<ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access {
    #[doc =
        "Access applications that are allowed to reach this hostname for this Tunnel. Audience tags can be identified in the dashboard or via the List Access policies API."]
    pub aud_tag: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub team_name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access {
    pub fn build(self) -> ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access {
        ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access {
            aud_tag: self.aud_tag,
            required: core::default::Default::default(),
            team_name: self.team_name,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef {
        ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aud_tag` after provisioning.\nAccess applications that are allowed to reach this hostname for this Tunnel. Audience tags can be identified in the dashboard or via the List Access policies API."]
    pub fn aud_tag(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aud_tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required` after provisioning.\nDeny traffic that has not fulfilled Access authorization."]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `team_name` after provisioning.\n"]
    pub fn team_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.team_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access: std::option::Option<ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connect_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_chunked_encoding: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2_origin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_host_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_alive_connections: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_alive_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_sn_ito_host: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_happy_eyeballs: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_tls_verify: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_keep_alive: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_timeout: std::option::Option<terrars::PrimField<f64>>,
}

impl ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest {
    #[doc =
        "Set the field `access`.\nFor all L7 requests to this hostname, cloudflared will validate each request's Cf-Access-Jwt-Assertion request header."]
    pub fn set_access(
        mut self,
        v: impl std::convert::Into<ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access>,
    ) -> Self {
        self.access = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ca_pool`.\nPath to the certificate authority (CA) for the certificate of your origin. This option should be used only if your certificate is not signed by Cloudflare."]
    pub fn set_ca_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connect_timeout`.\nTimeout for establishing a new TCP connection to your origin server. This excludes the time taken to establish TLS, which is controlled by tlsTimeout."]
    pub fn set_connect_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connect_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_chunked_encoding`.\nDisables chunked transfer encoding. Useful if you are running a WSGI server."]
    pub fn set_disable_chunked_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_chunked_encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `http2_origin`.\nAttempt to connect to origin using HTTP2. Origin must be configured as https."]
    pub fn set_http2_origin(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.http2_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_host_header`.\nSets the HTTP Host header on requests sent to the local service."]
    pub fn set_http_host_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_host_header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `keep_alive_connections`.\nMaximum number of idle keepalive connections between Tunnel and your origin. This does not restrict the total number of concurrent connections."]
    pub fn set_keep_alive_connections(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.keep_alive_connections = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keep_alive_timeout`.\nTimeout after which an idle keepalive connection can be discarded."]
    pub fn set_keep_alive_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.keep_alive_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_sn_ito_host`.\nAuto configure the Hostname on the origin server certificate."]
    pub fn set_match_sn_ito_host(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.match_sn_ito_host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `no_happy_eyeballs`.\nDisable the “happy eyeballs” algorithm for IPv4/IPv6 fallback if your local network has misconfigured one of the protocols."]
    pub fn set_no_happy_eyeballs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_happy_eyeballs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `no_tls_verify`.\nDisables TLS verification of the certificate presented by your origin. Will allow any certificate from the origin to be accepted."]
    pub fn set_no_tls_verify(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_tls_verify = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_server_name`.\nHostname that cloudflared should expect from your origin server certificate."]
    pub fn set_origin_server_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.origin_server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_type`.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures what type of proxy will be started. Valid options are: \"\" for the regular proxy and \"socks\" for a SOCKS5 proxy."]
    pub fn set_proxy_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tcp_keep_alive`.\nThe timeout after which a TCP keepalive packet is sent on a connection between Tunnel and the origin server."]
    pub fn set_tcp_keep_alive(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tcp_keep_alive = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tls_timeout`.\nTimeout for completing a TLS handshake to your origin server, if you have chosen to connect Tunnel to an HTTPS server."]
    pub fn set_tls_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tls_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest {
    type O = terrars::BlockAssignable<ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustTunnelCloudflaredConfig_Config_OriginRequest {}

impl BuildZeroTrustTunnelCloudflaredConfig_Config_OriginRequest {
    pub fn build(self) -> ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest {
        ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest {
            access: core::default::Default::default(),
            ca_pool: core::default::Default::default(),
            connect_timeout: core::default::Default::default(),
            disable_chunked_encoding: core::default::Default::default(),
            http2_origin: core::default::Default::default(),
            http_host_header: core::default::Default::default(),
            keep_alive_connections: core::default::Default::default(),
            keep_alive_timeout: core::default::Default::default(),
            match_sn_ito_host: core::default::Default::default(),
            no_happy_eyeballs: core::default::Default::default(),
            no_tls_verify: core::default::Default::default(),
            origin_server_name: core::default::Default::default(),
            proxy_type: core::default::Default::default(),
            tcp_keep_alive: core::default::Default::default(),
            tls_timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef {
        ZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access` after provisioning.\nFor all L7 requests to this hostname, cloudflared will validate each request's Cf-Access-Jwt-Assertion request header."]
    pub fn access(&self) -> ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef {
        ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef::new(
            self.shared().clone(),
            format!("{}.access", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `ca_pool` after provisioning.\nPath to the certificate authority (CA) for the certificate of your origin. This option should be used only if your certificate is not signed by Cloudflare."]
    pub fn ca_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connect_timeout` after provisioning.\nTimeout for establishing a new TCP connection to your origin server. This excludes the time taken to establish TLS, which is controlled by tlsTimeout."]
    pub fn connect_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_chunked_encoding` after provisioning.\nDisables chunked transfer encoding. Useful if you are running a WSGI server."]
    pub fn disable_chunked_encoding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_chunked_encoding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http2_origin` after provisioning.\nAttempt to connect to origin using HTTP2. Origin must be configured as https."]
    pub fn http2_origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http2_origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_host_header` after provisioning.\nSets the HTTP Host header on requests sent to the local service."]
    pub fn http_host_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_host_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keep_alive_connections` after provisioning.\nMaximum number of idle keepalive connections between Tunnel and your origin. This does not restrict the total number of concurrent connections."]
    pub fn keep_alive_connections(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_alive_connections", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keep_alive_timeout` after provisioning.\nTimeout after which an idle keepalive connection can be discarded."]
    pub fn keep_alive_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_alive_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `match_sn_ito_host` after provisioning.\nAuto configure the Hostname on the origin server certificate."]
    pub fn match_sn_ito_host(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_sn_ito_host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_happy_eyeballs` after provisioning.\nDisable the “happy eyeballs” algorithm for IPv4/IPv6 fallback if your local network has misconfigured one of the protocols."]
    pub fn no_happy_eyeballs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_happy_eyeballs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_tls_verify` after provisioning.\nDisables TLS verification of the certificate presented by your origin. Will allow any certificate from the origin to be accepted."]
    pub fn no_tls_verify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_tls_verify", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_server_name` after provisioning.\nHostname that cloudflared should expect from your origin server certificate."]
    pub fn origin_server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_server_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_type` after provisioning.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures what type of proxy will be started. Valid options are: \"\" for the regular proxy and \"socks\" for a SOCKS5 proxy."]
    pub fn proxy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tcp_keep_alive` after provisioning.\nThe timeout after which a TCP keepalive packet is sent on a connection between Tunnel and the origin server."]
    pub fn tcp_keep_alive(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_keep_alive", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_timeout` after provisioning.\nTimeout for completing a TLS handshake to your origin server, if you have chosen to connect Tunnel to an HTTPS server."]
    pub fn tls_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustTunnelCloudflaredConfig_Config_Ingress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_request: std::option::Option<ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
}

impl ZeroTrustTunnelCloudflaredConfig_Config_Ingress {
    #[doc = "Set the field `hostname`.\nPublic hostname for this service."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_request`.\nConfiguration parameters for the public hostname specific connection settings between cloudflared and origin server."]
    pub fn set_origin_request(
        mut self,
        v: impl std::convert::Into<ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest>,
    ) -> Self {
        self.origin_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\nRequests with this path route to this public hostname."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustTunnelCloudflaredConfig_Config_Ingress {
    type O = terrars::BlockAssignable<ZeroTrustTunnelCloudflaredConfig_Config_Ingress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustTunnelCloudflaredConfig_Config_Ingress {
    #[doc =
        "Protocol and address of destination server. Supported protocols: http://, https://, unix://, tcp://, ssh://, rdp://, unix+tls://, smb://. Alternatively can return a HTTP status code http_status:[code] e.g. 'http_status:404'."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustTunnelCloudflaredConfig_Config_Ingress {
    pub fn build(self) -> ZeroTrustTunnelCloudflaredConfig_Config_Ingress {
        ZeroTrustTunnelCloudflaredConfig_Config_Ingress {
            hostname: core::default::Default::default(),
            origin_request: core::default::Default::default(),
            path: core::default::Default::default(),
            service: self.service,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustTunnelCloudflaredConfig_Config_IngressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustTunnelCloudflaredConfig_Config_IngressRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustTunnelCloudflaredConfig_Config_IngressRef {
        ZeroTrustTunnelCloudflaredConfig_Config_IngressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustTunnelCloudflaredConfig_Config_IngressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nPublic hostname for this service."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_request` after provisioning.\nConfiguration parameters for the public hostname specific connection settings between cloudflared and origin server."]
    pub fn origin_request(&self) -> ZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef {
        ZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef::new(
            self.shared().clone(),
            format!("{}.origin_request", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nRequests with this path route to this public hostname."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nProtocol and address of destination server. Supported protocols: http://, https://, unix://, tcp://, ssh://, rdp://, unix+tls://, smb://. Alternatively can return a HTTP status code http_status:[code] e.g. 'http_status:404'."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustTunnelCloudflaredConfig_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress: std::option::Option<std::vec::Vec<ZeroTrustTunnelCloudflaredConfig_Config_Ingress>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_request: std::option::Option<ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest>,
}

impl ZeroTrustTunnelCloudflaredConfig_Config {
    #[doc =
        "Set the field `ingress`.\nList of public hostname definitions. At least one ingress rule needs to be defined for the tunnel."]
    pub fn set_ingress(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustTunnelCloudflaredConfig_Config_Ingress>>,
    ) -> Self {
        self.ingress = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_request`.\nConfiguration parameters for the public hostname specific connection settings between cloudflared and origin server."]
    pub fn set_origin_request(
        mut self,
        v: impl std::convert::Into<ZeroTrustTunnelCloudflaredConfig_Config_OriginRequest>,
    ) -> Self {
        self.origin_request = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustTunnelCloudflaredConfig_Config {
    type O = terrars::BlockAssignable<ZeroTrustTunnelCloudflaredConfig_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustTunnelCloudflaredConfig_Config {}

impl BuildZeroTrustTunnelCloudflaredConfig_Config {
    pub fn build(self) -> ZeroTrustTunnelCloudflaredConfig_Config {
        ZeroTrustTunnelCloudflaredConfig_Config {
            ingress: core::default::Default::default(),
            origin_request: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustTunnelCloudflaredConfig_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustTunnelCloudflaredConfig_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustTunnelCloudflaredConfig_ConfigRef {
        ZeroTrustTunnelCloudflaredConfig_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustTunnelCloudflaredConfig_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ingress` after provisioning.\nList of public hostname definitions. At least one ingress rule needs to be defined for the tunnel."]
    pub fn ingress(&self) -> terrars::ListRef<ZeroTrustTunnelCloudflaredConfig_Config_IngressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_request` after provisioning.\nConfiguration parameters for the public hostname specific connection settings between cloudflared and origin server."]
    pub fn origin_request(&self) -> ZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef {
        ZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef::new(
            self.shared().clone(),
            format!("{}.origin_request", self.base),
        )
    }
}

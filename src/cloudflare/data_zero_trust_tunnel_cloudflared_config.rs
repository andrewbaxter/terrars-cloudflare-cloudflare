#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustTunnelCloudflaredConfigData_ {
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

struct DataZeroTrustTunnelCloudflaredConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustTunnelCloudflaredConfigData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustTunnelCloudflaredConfig(std::rc::Rc<DataZeroTrustTunnelCloudflaredConfig_>);

impl DataZeroTrustTunnelCloudflaredConfig {
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
        "Get a reference to the value of field `config` after provisioning.\nThe tunnel configuration and ingress rules."]
    pub fn config(&self) -> DataZeroTrustTunnelCloudflaredConfig_ConfigRef {
        DataZeroTrustTunnelCloudflaredConfig_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
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

impl terrars::Referable for DataZeroTrustTunnelCloudflaredConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustTunnelCloudflaredConfig { }

impl terrars::ToListMappable for DataZeroTrustTunnelCloudflaredConfig {
    type O = terrars::ListRef<DataZeroTrustTunnelCloudflaredConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustTunnelCloudflaredConfig_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_tunnel_cloudflared_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustTunnelCloudflaredConfig {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "UUID of the tunnel."]
    pub tunnel_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustTunnelCloudflaredConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustTunnelCloudflaredConfig {
        let out = DataZeroTrustTunnelCloudflaredConfig(std::rc::Rc::new(DataZeroTrustTunnelCloudflaredConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustTunnelCloudflaredConfigData_ {
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

pub struct DataZeroTrustTunnelCloudflaredConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelCloudflaredConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustTunnelCloudflaredConfigRef {
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
        "Get a reference to the value of field `config` after provisioning.\nThe tunnel configuration and ingress rules."]
    pub fn config(&self) -> DataZeroTrustTunnelCloudflaredConfig_ConfigRef {
        DataZeroTrustTunnelCloudflaredConfig_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
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
pub struct DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_Access {}

impl DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_Access { }

impl terrars::ToListMappable for DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_Access {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_Access>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_Access {}

impl BuildDataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_Access {
    pub fn build(self) -> DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_Access {
        DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_Access {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_AccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_AccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_AccessRef {
        DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_AccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_AccessRef {
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
pub struct DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest {}

impl DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest { }

impl terrars::ToListMappable for DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest {}

impl BuildDataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest {
    pub fn build(self) -> DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest {
        DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequestRef {
        DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access` after provisioning.\nFor all L7 requests to this hostname, cloudflared will validate each request's Cf-Access-Jwt-Assertion request header."]
    pub fn access(&self) -> DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_AccessRef {
        DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequest_AccessRef::new(
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
pub struct DataZeroTrustTunnelCloudflaredConfig_Config_Ingress {}

impl DataZeroTrustTunnelCloudflaredConfig_Config_Ingress { }

impl terrars::ToListMappable for DataZeroTrustTunnelCloudflaredConfig_Config_Ingress {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelCloudflaredConfig_Config_Ingress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelCloudflaredConfig_Config_Ingress {}

impl BuildDataZeroTrustTunnelCloudflaredConfig_Config_Ingress {
    pub fn build(self) -> DataZeroTrustTunnelCloudflaredConfig_Config_Ingress {
        DataZeroTrustTunnelCloudflaredConfig_Config_Ingress {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelCloudflaredConfig_Config_IngressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelCloudflaredConfig_Config_IngressRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustTunnelCloudflaredConfig_Config_IngressRef {
        DataZeroTrustTunnelCloudflaredConfig_Config_IngressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelCloudflaredConfig_Config_IngressRef {
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
    pub fn origin_request(&self) -> DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequestRef {
        DataZeroTrustTunnelCloudflaredConfig_Config_Ingress_OriginRequestRef::new(
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
pub struct DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access {}

impl DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access { }

impl terrars::ToListMappable for DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access {}

impl BuildDataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access {
    pub fn build(self) -> DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access {
        DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_Access {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef {
        DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef {
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
pub struct DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest {}

impl DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest { }

impl terrars::ToListMappable for DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest {}

impl BuildDataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest {
    pub fn build(self) -> DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest {
        DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef {
        DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access` after provisioning.\nFor all L7 requests to this hostname, cloudflared will validate each request's Cf-Access-Jwt-Assertion request header."]
    pub fn access(&self) -> DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef {
        DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequest_AccessRef::new(
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
pub struct DataZeroTrustTunnelCloudflaredConfig_Config {}

impl DataZeroTrustTunnelCloudflaredConfig_Config { }

impl terrars::ToListMappable for DataZeroTrustTunnelCloudflaredConfig_Config {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelCloudflaredConfig_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelCloudflaredConfig_Config {}

impl BuildDataZeroTrustTunnelCloudflaredConfig_Config {
    pub fn build(self) -> DataZeroTrustTunnelCloudflaredConfig_Config {
        DataZeroTrustTunnelCloudflaredConfig_Config {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelCloudflaredConfig_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelCloudflaredConfig_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustTunnelCloudflaredConfig_ConfigRef {
        DataZeroTrustTunnelCloudflaredConfig_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelCloudflaredConfig_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ingress` after provisioning.\nList of public hostname definitions. At least one ingress rule needs to be defined for the tunnel."]
    pub fn ingress(&self) -> terrars::ListRef<DataZeroTrustTunnelCloudflaredConfig_Config_IngressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_request` after provisioning.\nConfiguration parameters for the public hostname specific connection settings between cloudflared and origin server."]
    pub fn origin_request(&self) -> DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef {
        DataZeroTrustTunnelCloudflaredConfig_Config_OriginRequestRef::new(
            self.shared().clone(),
            format!("{}.origin_request", self.base),
        )
    }
}

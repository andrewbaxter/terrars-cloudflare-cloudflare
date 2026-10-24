use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct TunnelConfigData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    tunnel_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<TunnelConfigConfigEl>>,
    dynamic: TunnelConfigDynamic,
}

struct TunnelConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TunnelConfigData>,
}

#[derive(Clone)]
pub struct TunnelConfig(std::rc::Rc<TunnelConfig_>);

impl TunnelConfig {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl std::convert::Into<terrars::BlockAssignable<TunnelConfigConfigEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tunnel_id` after provisioning.\nIdentifier of the Tunnel to target for this configuration."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<TunnelConfigConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }
}

impl terrars::Referable for TunnelConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TunnelConfig { }

impl terrars::ToListMappable for TunnelConfig {
    type O = terrars::ListRef<TunnelConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TunnelConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_tunnel_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTunnelConfig {
    pub tf_id: std::string::String,
    #[doc = "The account identifier to target for the resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Identifier of the Tunnel to target for this configuration."]
    pub tunnel_id: terrars::PrimField<std::string::String>,
}

impl BuildTunnelConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> TunnelConfig {
        let out = TunnelConfig(std::rc::Rc::new(TunnelConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TunnelConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                id: core::default::Default::default(),
                tunnel_id: self.tunnel_id,
                config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TunnelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TunnelConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TunnelConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tunnel_id` after provisioning.\nIdentifier of the Tunnel to target for this configuration."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<TunnelConfigConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TunnelConfigConfigElIngressRuleElOriginRequestElAccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aud_tag: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    team_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TunnelConfigConfigElIngressRuleElOriginRequestElAccessEl {
    #[doc = "Set the field `aud_tag`.\nAudience tags of the access rule."]
    pub fn set_aud_tag(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.aud_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\nWhether the access rule is required."]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `team_name`.\nName of the team to which the access rule applies."]
    pub fn set_team_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.team_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TunnelConfigConfigElIngressRuleElOriginRequestElAccessEl {
    type O = terrars::BlockAssignable<TunnelConfigConfigElIngressRuleElOriginRequestElAccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTunnelConfigConfigElIngressRuleElOriginRequestElAccessEl {}

impl BuildTunnelConfigConfigElIngressRuleElOriginRequestElAccessEl {
    pub fn build(self) -> TunnelConfigConfigElIngressRuleElOriginRequestElAccessEl {
        TunnelConfigConfigElIngressRuleElOriginRequestElAccessEl {
            aud_tag: core::default::Default::default(),
            required: core::default::Default::default(),
            team_name: core::default::Default::default(),
        }
    }
}

pub struct TunnelConfigConfigElIngressRuleElOriginRequestElAccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TunnelConfigConfigElIngressRuleElOriginRequestElAccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TunnelConfigConfigElIngressRuleElOriginRequestElAccessElRef {
        TunnelConfigConfigElIngressRuleElOriginRequestElAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TunnelConfigConfigElIngressRuleElOriginRequestElAccessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aud_tag` after provisioning.\nAudience tags of the access rule."]
    pub fn aud_tag(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.aud_tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required` after provisioning.\nWhether the access rule is required."]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `team_name` after provisioning.\nName of the team to which the access rule applies."]
    pub fn team_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.team_name", self.base))
    }
}

#[derive(Serialize)]
pub struct TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesEl {
    #[doc = "Set the field `allow`.\nWhether to allow the IP prefix."]
    pub fn set_allow(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ports`.\nPorts to use within the IP rule."]
    pub fn set_ports(mut self, v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>) -> Self {
        self.ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\nIP rule prefix."]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesEl {
    type O = terrars::BlockAssignable<TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTunnelConfigConfigElIngressRuleElOriginRequestElIpRulesEl {}

impl BuildTunnelConfigConfigElIngressRuleElOriginRequestElIpRulesEl {
    pub fn build(self) -> TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesEl {
        TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesEl {
            allow: core::default::Default::default(),
            ports: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

pub struct TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesElRef {
        TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow` after provisioning.\nWhether to allow the IP prefix."]
    pub fn allow(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\nPorts to use within the IP rule."]
    pub fn ports(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nIP rule prefix."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct TunnelConfigConfigElIngressRuleElOriginRequestElDynamic {
    access: std::option::Option<terrars::DynamicBlock<TunnelConfigConfigElIngressRuleElOriginRequestElAccessEl>>,
    ip_rules: std::option::Option<terrars::DynamicBlock<TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesEl>>,
}

#[derive(Serialize)]
pub struct TunnelConfigConfigElIngressRuleElOriginRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bastion_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connect_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_chunked_encoding: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2_origin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_host_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_alive_connections: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_alive_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_happy_eyeballs: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_tls_verify: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_keep_alive: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access: std::option::Option<std::vec::Vec<TunnelConfigConfigElIngressRuleElOriginRequestElAccessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_rules: std::option::Option<std::vec::Vec<TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesEl>>,
    dynamic: TunnelConfigConfigElIngressRuleElOriginRequestElDynamic,
}

impl TunnelConfigConfigElIngressRuleElOriginRequestEl {
    #[doc = "Set the field `bastion_mode`.\nRuns as jump host."]
    pub fn set_bastion_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bastion_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ca_pool`.\nPath to the certificate authority (CA) for the certificate of your origin. This option should be used only if your certificate is not signed by Cloudflare. Defaults to `\"\"`."]
    pub fn set_ca_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connect_timeout`.\nTimeout for establishing a new TCP connection to your origin server. This excludes the time taken to establish TLS, which is controlled by `tlsTimeout`. Defaults to `30s`."]
    pub fn set_connect_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connect_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_chunked_encoding`.\nDisables chunked transfer encoding. Useful if you are running a Web Server Gateway Interface (WSGI) server. Defaults to `false`."]
    pub fn set_disable_chunked_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_chunked_encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http2_origin`.\nEnables HTTP/2 support for the origin connection. Defaults to `false`."]
    pub fn set_http2_origin(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.http2_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `http_host_header`.\nSets the HTTP Host header on requests sent to the local service. Defaults to `\"\"`."]
    pub fn set_http_host_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_host_header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `keep_alive_connections`.\nMaximum number of idle keepalive connections between Tunnel and your origin. This does not restrict the total number of concurrent connections. Defaults to `100`."]
    pub fn set_keep_alive_connections(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.keep_alive_connections = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `keep_alive_timeout`.\nTimeout after which an idle keepalive connection can be discarded. Defaults to `1m30s`."]
    pub fn set_keep_alive_timeout(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.keep_alive_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `no_happy_eyeballs`.\nDisable the “happy eyeballs” algorithm for IPv4/IPv6 fallback if your local network has misconfigured one of the protocols. Defaults to `false`."]
    pub fn set_no_happy_eyeballs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_happy_eyeballs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `no_tls_verify`.\nDisables TLS verification of the certificate presented by your origin. Will allow any certificate from the origin to be accepted. Defaults to `false`."]
    pub fn set_no_tls_verify(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_tls_verify = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_server_name`.\nHostname that cloudflared should expect from your origin server certificate. Defaults to `\"\"`."]
    pub fn set_origin_server_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.origin_server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_address`.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen address for that proxy. Defaults to `127.0.0.1`."]
    pub fn set_proxy_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_port`.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen port for that proxy. If set to zero, an unused port will randomly be chosen. Defaults to `0`."]
    pub fn set_proxy_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.proxy_port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_type`.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures what type of proxy will be started. Available values: `\"\"`, `socks`. Defaults to `\"\"`."]
    pub fn set_proxy_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tcp_keep_alive`.\nThe timeout after which a TCP keepalive packet is sent on a connection between Tunnel and the origin server. Defaults to `30s`."]
    pub fn set_tcp_keep_alive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tcp_keep_alive = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tls_timeout`.\nTimeout for completing a TLS handshake to your origin server, if you have chosen to connect Tunnel to an HTTPS server. Defaults to `10s`."]
    pub fn set_tls_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access`.\n"]
    pub fn set_access(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TunnelConfigConfigElIngressRuleElOriginRequestElAccessEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_rules`.\n"]
    pub fn set_ip_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TunnelConfigConfigElIngressRuleElOriginRequestElIpRulesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TunnelConfigConfigElIngressRuleElOriginRequestEl {
    type O = terrars::BlockAssignable<TunnelConfigConfigElIngressRuleElOriginRequestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTunnelConfigConfigElIngressRuleElOriginRequestEl {}

impl BuildTunnelConfigConfigElIngressRuleElOriginRequestEl {
    pub fn build(self) -> TunnelConfigConfigElIngressRuleElOriginRequestEl {
        TunnelConfigConfigElIngressRuleElOriginRequestEl {
            bastion_mode: core::default::Default::default(),
            ca_pool: core::default::Default::default(),
            connect_timeout: core::default::Default::default(),
            disable_chunked_encoding: core::default::Default::default(),
            http2_origin: core::default::Default::default(),
            http_host_header: core::default::Default::default(),
            keep_alive_connections: core::default::Default::default(),
            keep_alive_timeout: core::default::Default::default(),
            no_happy_eyeballs: core::default::Default::default(),
            no_tls_verify: core::default::Default::default(),
            origin_server_name: core::default::Default::default(),
            proxy_address: core::default::Default::default(),
            proxy_port: core::default::Default::default(),
            proxy_type: core::default::Default::default(),
            tcp_keep_alive: core::default::Default::default(),
            tls_timeout: core::default::Default::default(),
            access: core::default::Default::default(),
            ip_rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TunnelConfigConfigElIngressRuleElOriginRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TunnelConfigConfigElIngressRuleElOriginRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TunnelConfigConfigElIngressRuleElOriginRequestElRef {
        TunnelConfigConfigElIngressRuleElOriginRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TunnelConfigConfigElIngressRuleElOriginRequestElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bastion_mode` after provisioning.\nRuns as jump host."]
    pub fn bastion_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bastion_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ca_pool` after provisioning.\nPath to the certificate authority (CA) for the certificate of your origin. This option should be used only if your certificate is not signed by Cloudflare. Defaults to `\"\"`."]
    pub fn ca_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connect_timeout` after provisioning.\nTimeout for establishing a new TCP connection to your origin server. This excludes the time taken to establish TLS, which is controlled by `tlsTimeout`. Defaults to `30s`."]
    pub fn connect_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_chunked_encoding` after provisioning.\nDisables chunked transfer encoding. Useful if you are running a Web Server Gateway Interface (WSGI) server. Defaults to `false`."]
    pub fn disable_chunked_encoding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_chunked_encoding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http2_origin` after provisioning.\nEnables HTTP/2 support for the origin connection. Defaults to `false`."]
    pub fn http2_origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http2_origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_host_header` after provisioning.\nSets the HTTP Host header on requests sent to the local service. Defaults to `\"\"`."]
    pub fn http_host_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_host_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keep_alive_connections` after provisioning.\nMaximum number of idle keepalive connections between Tunnel and your origin. This does not restrict the total number of concurrent connections. Defaults to `100`."]
    pub fn keep_alive_connections(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_alive_connections", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keep_alive_timeout` after provisioning.\nTimeout after which an idle keepalive connection can be discarded. Defaults to `1m30s`."]
    pub fn keep_alive_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_alive_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_happy_eyeballs` after provisioning.\nDisable the “happy eyeballs” algorithm for IPv4/IPv6 fallback if your local network has misconfigured one of the protocols. Defaults to `false`."]
    pub fn no_happy_eyeballs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_happy_eyeballs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_tls_verify` after provisioning.\nDisables TLS verification of the certificate presented by your origin. Will allow any certificate from the origin to be accepted. Defaults to `false`."]
    pub fn no_tls_verify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_tls_verify", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_server_name` after provisioning.\nHostname that cloudflared should expect from your origin server certificate. Defaults to `\"\"`."]
    pub fn origin_server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_server_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_address` after provisioning.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen address for that proxy. Defaults to `127.0.0.1`."]
    pub fn proxy_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_port` after provisioning.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen port for that proxy. If set to zero, an unused port will randomly be chosen. Defaults to `0`."]
    pub fn proxy_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_type` after provisioning.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures what type of proxy will be started. Available values: `\"\"`, `socks`. Defaults to `\"\"`."]
    pub fn proxy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tcp_keep_alive` after provisioning.\nThe timeout after which a TCP keepalive packet is sent on a connection between Tunnel and the origin server. Defaults to `30s`."]
    pub fn tcp_keep_alive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_keep_alive", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_timeout` after provisioning.\nTimeout for completing a TLS handshake to your origin server, if you have chosen to connect Tunnel to an HTTPS server. Defaults to `10s`."]
    pub fn tls_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `access` after provisioning.\n"]
    pub fn access(&self) -> terrars::ListRef<TunnelConfigConfigElIngressRuleElOriginRequestElAccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access", self.base))
    }
}

#[derive(Serialize, Default)]
struct TunnelConfigConfigElIngressRuleElDynamic {
    origin_request: std::option::Option<terrars::DynamicBlock<TunnelConfigConfigElIngressRuleElOriginRequestEl>>,
}

#[derive(Serialize)]
pub struct TunnelConfigConfigElIngressRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_request: std::option::Option<std::vec::Vec<TunnelConfigConfigElIngressRuleElOriginRequestEl>>,
    dynamic: TunnelConfigConfigElIngressRuleElDynamic,
}

impl TunnelConfigConfigElIngressRuleEl {
    #[doc =
        "Set the field `hostname`.\nHostname to match the incoming request with. If the hostname matches, the request will be sent to the service."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path`.\nPath of the incoming request. If the path matches, the request will be sent to the local service."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_request`.\n"]
    pub fn set_origin_request(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TunnelConfigConfigElIngressRuleElOriginRequestEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.origin_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.origin_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TunnelConfigConfigElIngressRuleEl {
    type O = terrars::BlockAssignable<TunnelConfigConfigElIngressRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTunnelConfigConfigElIngressRuleEl {
    #[doc = "Name of the service to which the request will be sent."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildTunnelConfigConfigElIngressRuleEl {
    pub fn build(self) -> TunnelConfigConfigElIngressRuleEl {
        TunnelConfigConfigElIngressRuleEl {
            hostname: core::default::Default::default(),
            path: core::default::Default::default(),
            service: self.service,
            origin_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TunnelConfigConfigElIngressRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TunnelConfigConfigElIngressRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TunnelConfigConfigElIngressRuleElRef {
        TunnelConfigConfigElIngressRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TunnelConfigConfigElIngressRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nHostname to match the incoming request with. If the hostname matches, the request will be sent to the service."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath of the incoming request. If the path matches, the request will be sent to the local service."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nName of the service to which the request will be sent."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_request` after provisioning.\n"]
    pub fn origin_request(&self) -> terrars::ListRef<TunnelConfigConfigElIngressRuleElOriginRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_request", self.base))
    }
}

#[derive(Serialize)]
pub struct TunnelConfigConfigElOriginRequestElAccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aud_tag: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    team_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TunnelConfigConfigElOriginRequestElAccessEl {
    #[doc = "Set the field `aud_tag`.\nAudience tags of the access rule."]
    pub fn set_aud_tag(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.aud_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\nWhether the access rule is required."]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `team_name`.\nName of the team to which the access rule applies."]
    pub fn set_team_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.team_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TunnelConfigConfigElOriginRequestElAccessEl {
    type O = terrars::BlockAssignable<TunnelConfigConfigElOriginRequestElAccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTunnelConfigConfigElOriginRequestElAccessEl {}

impl BuildTunnelConfigConfigElOriginRequestElAccessEl {
    pub fn build(self) -> TunnelConfigConfigElOriginRequestElAccessEl {
        TunnelConfigConfigElOriginRequestElAccessEl {
            aud_tag: core::default::Default::default(),
            required: core::default::Default::default(),
            team_name: core::default::Default::default(),
        }
    }
}

pub struct TunnelConfigConfigElOriginRequestElAccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TunnelConfigConfigElOriginRequestElAccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TunnelConfigConfigElOriginRequestElAccessElRef {
        TunnelConfigConfigElOriginRequestElAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TunnelConfigConfigElOriginRequestElAccessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aud_tag` after provisioning.\nAudience tags of the access rule."]
    pub fn aud_tag(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.aud_tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required` after provisioning.\nWhether the access rule is required."]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `team_name` after provisioning.\nName of the team to which the access rule applies."]
    pub fn team_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.team_name", self.base))
    }
}

#[derive(Serialize)]
pub struct TunnelConfigConfigElOriginRequestElIpRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TunnelConfigConfigElOriginRequestElIpRulesEl {
    #[doc = "Set the field `allow`.\nWhether to allow the IP prefix."]
    pub fn set_allow(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ports`.\nPorts to use within the IP rule."]
    pub fn set_ports(mut self, v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>) -> Self {
        self.ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\nIP rule prefix."]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TunnelConfigConfigElOriginRequestElIpRulesEl {
    type O = terrars::BlockAssignable<TunnelConfigConfigElOriginRequestElIpRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTunnelConfigConfigElOriginRequestElIpRulesEl {}

impl BuildTunnelConfigConfigElOriginRequestElIpRulesEl {
    pub fn build(self) -> TunnelConfigConfigElOriginRequestElIpRulesEl {
        TunnelConfigConfigElOriginRequestElIpRulesEl {
            allow: core::default::Default::default(),
            ports: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

pub struct TunnelConfigConfigElOriginRequestElIpRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TunnelConfigConfigElOriginRequestElIpRulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TunnelConfigConfigElOriginRequestElIpRulesElRef {
        TunnelConfigConfigElOriginRequestElIpRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TunnelConfigConfigElOriginRequestElIpRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow` after provisioning.\nWhether to allow the IP prefix."]
    pub fn allow(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\nPorts to use within the IP rule."]
    pub fn ports(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nIP rule prefix."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct TunnelConfigConfigElOriginRequestElDynamic {
    access: std::option::Option<terrars::DynamicBlock<TunnelConfigConfigElOriginRequestElAccessEl>>,
    ip_rules: std::option::Option<terrars::DynamicBlock<TunnelConfigConfigElOriginRequestElIpRulesEl>>,
}

#[derive(Serialize)]
pub struct TunnelConfigConfigElOriginRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bastion_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connect_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_chunked_encoding: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2_origin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_host_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_alive_connections: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_alive_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_happy_eyeballs: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_tls_verify: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_keep_alive: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access: std::option::Option<std::vec::Vec<TunnelConfigConfigElOriginRequestElAccessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_rules: std::option::Option<std::vec::Vec<TunnelConfigConfigElOriginRequestElIpRulesEl>>,
    dynamic: TunnelConfigConfigElOriginRequestElDynamic,
}

impl TunnelConfigConfigElOriginRequestEl {
    #[doc = "Set the field `bastion_mode`.\nRuns as jump host."]
    pub fn set_bastion_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bastion_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ca_pool`.\nPath to the certificate authority (CA) for the certificate of your origin. This option should be used only if your certificate is not signed by Cloudflare. Defaults to `\"\"`."]
    pub fn set_ca_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connect_timeout`.\nTimeout for establishing a new TCP connection to your origin server. This excludes the time taken to establish TLS, which is controlled by `tlsTimeout`. Defaults to `30s`."]
    pub fn set_connect_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connect_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_chunked_encoding`.\nDisables chunked transfer encoding. Useful if you are running a Web Server Gateway Interface (WSGI) server. Defaults to `false`."]
    pub fn set_disable_chunked_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_chunked_encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http2_origin`.\nEnables HTTP/2 support for the origin connection. Defaults to `false`."]
    pub fn set_http2_origin(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.http2_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `http_host_header`.\nSets the HTTP Host header on requests sent to the local service. Defaults to `\"\"`."]
    pub fn set_http_host_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_host_header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `keep_alive_connections`.\nMaximum number of idle keepalive connections between Tunnel and your origin. This does not restrict the total number of concurrent connections. Defaults to `100`."]
    pub fn set_keep_alive_connections(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.keep_alive_connections = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `keep_alive_timeout`.\nTimeout after which an idle keepalive connection can be discarded. Defaults to `1m30s`."]
    pub fn set_keep_alive_timeout(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.keep_alive_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `no_happy_eyeballs`.\nDisable the “happy eyeballs” algorithm for IPv4/IPv6 fallback if your local network has misconfigured one of the protocols. Defaults to `false`."]
    pub fn set_no_happy_eyeballs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_happy_eyeballs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `no_tls_verify`.\nDisables TLS verification of the certificate presented by your origin. Will allow any certificate from the origin to be accepted. Defaults to `false`."]
    pub fn set_no_tls_verify(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_tls_verify = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_server_name`.\nHostname that cloudflared should expect from your origin server certificate. Defaults to `\"\"`."]
    pub fn set_origin_server_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.origin_server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_address`.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen address for that proxy. Defaults to `127.0.0.1`."]
    pub fn set_proxy_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_port`.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen port for that proxy. If set to zero, an unused port will randomly be chosen. Defaults to `0`."]
    pub fn set_proxy_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.proxy_port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_type`.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures what type of proxy will be started. Available values: `\"\"`, `socks`. Defaults to `\"\"`."]
    pub fn set_proxy_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tcp_keep_alive`.\nThe timeout after which a TCP keepalive packet is sent on a connection between Tunnel and the origin server. Defaults to `30s`."]
    pub fn set_tcp_keep_alive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tcp_keep_alive = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tls_timeout`.\nTimeout for completing a TLS handshake to your origin server, if you have chosen to connect Tunnel to an HTTPS server. Defaults to `10s`."]
    pub fn set_tls_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access`.\n"]
    pub fn set_access(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TunnelConfigConfigElOriginRequestElAccessEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_rules`.\n"]
    pub fn set_ip_rules(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TunnelConfigConfigElOriginRequestElIpRulesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TunnelConfigConfigElOriginRequestEl {
    type O = terrars::BlockAssignable<TunnelConfigConfigElOriginRequestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTunnelConfigConfigElOriginRequestEl {}

impl BuildTunnelConfigConfigElOriginRequestEl {
    pub fn build(self) -> TunnelConfigConfigElOriginRequestEl {
        TunnelConfigConfigElOriginRequestEl {
            bastion_mode: core::default::Default::default(),
            ca_pool: core::default::Default::default(),
            connect_timeout: core::default::Default::default(),
            disable_chunked_encoding: core::default::Default::default(),
            http2_origin: core::default::Default::default(),
            http_host_header: core::default::Default::default(),
            keep_alive_connections: core::default::Default::default(),
            keep_alive_timeout: core::default::Default::default(),
            no_happy_eyeballs: core::default::Default::default(),
            no_tls_verify: core::default::Default::default(),
            origin_server_name: core::default::Default::default(),
            proxy_address: core::default::Default::default(),
            proxy_port: core::default::Default::default(),
            proxy_type: core::default::Default::default(),
            tcp_keep_alive: core::default::Default::default(),
            tls_timeout: core::default::Default::default(),
            access: core::default::Default::default(),
            ip_rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TunnelConfigConfigElOriginRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TunnelConfigConfigElOriginRequestElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TunnelConfigConfigElOriginRequestElRef {
        TunnelConfigConfigElOriginRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TunnelConfigConfigElOriginRequestElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bastion_mode` after provisioning.\nRuns as jump host."]
    pub fn bastion_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bastion_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ca_pool` after provisioning.\nPath to the certificate authority (CA) for the certificate of your origin. This option should be used only if your certificate is not signed by Cloudflare. Defaults to `\"\"`."]
    pub fn ca_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connect_timeout` after provisioning.\nTimeout for establishing a new TCP connection to your origin server. This excludes the time taken to establish TLS, which is controlled by `tlsTimeout`. Defaults to `30s`."]
    pub fn connect_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_chunked_encoding` after provisioning.\nDisables chunked transfer encoding. Useful if you are running a Web Server Gateway Interface (WSGI) server. Defaults to `false`."]
    pub fn disable_chunked_encoding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_chunked_encoding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http2_origin` after provisioning.\nEnables HTTP/2 support for the origin connection. Defaults to `false`."]
    pub fn http2_origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http2_origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_host_header` after provisioning.\nSets the HTTP Host header on requests sent to the local service. Defaults to `\"\"`."]
    pub fn http_host_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_host_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keep_alive_connections` after provisioning.\nMaximum number of idle keepalive connections between Tunnel and your origin. This does not restrict the total number of concurrent connections. Defaults to `100`."]
    pub fn keep_alive_connections(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_alive_connections", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keep_alive_timeout` after provisioning.\nTimeout after which an idle keepalive connection can be discarded. Defaults to `1m30s`."]
    pub fn keep_alive_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_alive_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_happy_eyeballs` after provisioning.\nDisable the “happy eyeballs” algorithm for IPv4/IPv6 fallback if your local network has misconfigured one of the protocols. Defaults to `false`."]
    pub fn no_happy_eyeballs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_happy_eyeballs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_tls_verify` after provisioning.\nDisables TLS verification of the certificate presented by your origin. Will allow any certificate from the origin to be accepted. Defaults to `false`."]
    pub fn no_tls_verify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_tls_verify", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_server_name` after provisioning.\nHostname that cloudflared should expect from your origin server certificate. Defaults to `\"\"`."]
    pub fn origin_server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_server_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_address` after provisioning.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen address for that proxy. Defaults to `127.0.0.1`."]
    pub fn proxy_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_port` after provisioning.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen port for that proxy. If set to zero, an unused port will randomly be chosen. Defaults to `0`."]
    pub fn proxy_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_type` after provisioning.\ncloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures what type of proxy will be started. Available values: `\"\"`, `socks`. Defaults to `\"\"`."]
    pub fn proxy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tcp_keep_alive` after provisioning.\nThe timeout after which a TCP keepalive packet is sent on a connection between Tunnel and the origin server. Defaults to `30s`."]
    pub fn tcp_keep_alive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_keep_alive", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_timeout` after provisioning.\nTimeout for completing a TLS handshake to your origin server, if you have chosen to connect Tunnel to an HTTPS server. Defaults to `10s`."]
    pub fn tls_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `access` after provisioning.\n"]
    pub fn access(&self) -> terrars::ListRef<TunnelConfigConfigElOriginRequestElAccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access", self.base))
    }
}

#[derive(Serialize)]
pub struct TunnelConfigConfigElWarpRoutingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl TunnelConfigConfigElWarpRoutingEl {
    #[doc = "Set the field `enabled`.\nWhether WARP routing is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TunnelConfigConfigElWarpRoutingEl {
    type O = terrars::BlockAssignable<TunnelConfigConfigElWarpRoutingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTunnelConfigConfigElWarpRoutingEl {}

impl BuildTunnelConfigConfigElWarpRoutingEl {
    pub fn build(self) -> TunnelConfigConfigElWarpRoutingEl {
        TunnelConfigConfigElWarpRoutingEl { enabled: core::default::Default::default() }
    }
}

pub struct TunnelConfigConfigElWarpRoutingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TunnelConfigConfigElWarpRoutingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TunnelConfigConfigElWarpRoutingElRef {
        TunnelConfigConfigElWarpRoutingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TunnelConfigConfigElWarpRoutingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether WARP routing is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct TunnelConfigConfigElDynamic {
    ingress_rule: std::option::Option<terrars::DynamicBlock<TunnelConfigConfigElIngressRuleEl>>,
    origin_request: std::option::Option<terrars::DynamicBlock<TunnelConfigConfigElOriginRequestEl>>,
    warp_routing: std::option::Option<terrars::DynamicBlock<TunnelConfigConfigElWarpRoutingEl>>,
}

#[derive(Serialize)]
pub struct TunnelConfigConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_rule: std::option::Option<std::vec::Vec<TunnelConfigConfigElIngressRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_request: std::option::Option<std::vec::Vec<TunnelConfigConfigElOriginRequestEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warp_routing: std::option::Option<std::vec::Vec<TunnelConfigConfigElWarpRoutingEl>>,
    dynamic: TunnelConfigConfigElDynamic,
}

impl TunnelConfigConfigEl {
    #[doc = "Set the field `ingress_rule`.\n"]
    pub fn set_ingress_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TunnelConfigConfigElIngressRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `origin_request`.\n"]
    pub fn set_origin_request(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TunnelConfigConfigElOriginRequestEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.origin_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.origin_request = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `warp_routing`.\n"]
    pub fn set_warp_routing(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TunnelConfigConfigElWarpRoutingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.warp_routing = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.warp_routing = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TunnelConfigConfigEl {
    type O = terrars::BlockAssignable<TunnelConfigConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTunnelConfigConfigEl {}

impl BuildTunnelConfigConfigEl {
    pub fn build(self) -> TunnelConfigConfigEl {
        TunnelConfigConfigEl {
            ingress_rule: core::default::Default::default(),
            origin_request: core::default::Default::default(),
            warp_routing: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TunnelConfigConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TunnelConfigConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TunnelConfigConfigElRef {
        TunnelConfigConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TunnelConfigConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ingress_rule` after provisioning.\n"]
    pub fn ingress_rule(&self) -> terrars::ListRef<TunnelConfigConfigElIngressRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_request` after provisioning.\n"]
    pub fn origin_request(&self) -> terrars::ListRef<TunnelConfigConfigElOriginRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_request", self.base))
    }

    #[doc = "Get a reference to the value of field `warp_routing` after provisioning.\n"]
    pub fn warp_routing(&self) -> terrars::ListRef<TunnelConfigConfigElWarpRoutingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warp_routing", self.base))
    }
}

#[derive(Serialize, Default)]
struct TunnelConfigDynamic {
    config: std::option::Option<terrars::DynamicBlock<TunnelConfigConfigEl>>,
}

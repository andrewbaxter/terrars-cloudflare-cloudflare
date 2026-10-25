#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ConnectivityDirectoryServiceData_ {
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
    app_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    host: ConnectivityDirectoryService_Host,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_port: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_settings: std::option::Option<ConnectivityDirectoryService_TlsSettings>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

struct ConnectivityDirectoryService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConnectivityDirectoryServiceData_>,
}

#[derive(Clone)]
pub struct ConnectivityDirectoryService(std::rc::Rc<ConnectivityDirectoryService_>);

impl ConnectivityDirectoryService {
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

    #[doc = "Set the field `app_protocol`.\nAvailable values: \"postgresql\", \"mysql\"."]
    pub fn set_app_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().app_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_port`.\n"]
    pub fn set_http_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().http_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `https_port`.\n"]
    pub fn set_https_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().https_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_port`.\n"]
    pub fn set_tcp_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tcp_port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tls_settings`.\nTLS settings for a connectivity service.\n\nIf omitted, the default mode (`verify_full`) is used."]
    pub fn set_tls_settings(self, v: impl std::convert::Into<ConnectivityDirectoryService_TlsSettings>) -> Self {
        self.0.data.borrow_mut().tls_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_protocol` after provisioning.\nAvailable values: \"postgresql\", \"mysql\"."]
    pub fn app_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> ConnectivityDirectoryService_HostRef {
        ConnectivityDirectoryService_HostRef::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_port` after provisioning.\n"]
    pub fn http_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `https_port` after provisioning.\n"]
    pub fn https_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tcp_port` after provisioning.\n"]
    pub fn tcp_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls_settings` after provisioning.\nTLS settings for a connectivity service.\n\nIf omitted, the default mode (`verify_full`) is used."]
    pub fn tls_settings(&self) -> ConnectivityDirectoryService_TlsSettingsRef {
        ConnectivityDirectoryService_TlsSettingsRef::new(
            self.shared().clone(),
            format!("{}.tls_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"tcp\", \"http\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for ConnectivityDirectoryService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ConnectivityDirectoryService { }

impl terrars::ToListMappable for ConnectivityDirectoryService {
    type O = terrars::ListRef<ConnectivityDirectoryServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ConnectivityDirectoryService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_connectivity_directory_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectivityDirectoryService {
    pub tf_id: std::string::String,
    #[doc = "Account identifier"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: ConnectivityDirectoryService_Host,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Available values: \"tcp\", \"http\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildConnectivityDirectoryService {
    pub fn build(self, stack: &mut terrars::Stack) -> ConnectivityDirectoryService {
        let out = ConnectivityDirectoryService(std::rc::Rc::new(ConnectivityDirectoryService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ConnectivityDirectoryServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                app_protocol: core::default::Default::default(),
                host: self.host,
                http_port: core::default::Default::default(),
                https_port: core::default::Default::default(),
                name: self.name,
                tcp_port: core::default::Default::default(),
                tls_settings: core::default::Default::default(),
                type_: self.type_,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectivityDirectoryServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectivityDirectoryServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConnectivityDirectoryServiceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_protocol` after provisioning.\nAvailable values: \"postgresql\", \"mysql\"."]
    pub fn app_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> ConnectivityDirectoryService_HostRef {
        ConnectivityDirectoryService_HostRef::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_port` after provisioning.\n"]
    pub fn http_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `https_port` after provisioning.\n"]
    pub fn https_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tcp_port` after provisioning.\n"]
    pub fn tcp_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls_settings` after provisioning.\nTLS settings for a connectivity service.\n\nIf omitted, the default mode (`verify_full`) is used."]
    pub fn tls_settings(&self) -> ConnectivityDirectoryService_TlsSettingsRef {
        ConnectivityDirectoryService_TlsSettingsRef::new(
            self.shared().clone(),
            format!("{}.tls_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"tcp\", \"http\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConnectivityDirectoryService_Host_Network {
    tunnel_id: terrars::PrimField<std::string::String>,
}

impl ConnectivityDirectoryService_Host_Network { }

impl terrars::ToListMappable for ConnectivityDirectoryService_Host_Network {
    type O = terrars::BlockAssignable<ConnectivityDirectoryService_Host_Network>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectivityDirectoryService_Host_Network {
    #[doc = ""]
    pub tunnel_id: terrars::PrimField<std::string::String>,
}

impl BuildConnectivityDirectoryService_Host_Network {
    pub fn build(self) -> ConnectivityDirectoryService_Host_Network {
        ConnectivityDirectoryService_Host_Network { tunnel_id: self.tunnel_id }
    }
}

#[allow(dead_code)]
pub struct ConnectivityDirectoryService_Host_NetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectivityDirectoryService_Host_NetworkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConnectivityDirectoryService_Host_NetworkRef {
        ConnectivityDirectoryService_Host_NetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectivityDirectoryService_Host_NetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\n"]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ConnectivityDirectoryService_Host_ResolverNetwork {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolver_ips: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    tunnel_id: terrars::PrimField<std::string::String>,
}

impl ConnectivityDirectoryService_Host_ResolverNetwork {
    #[doc = "Set the field `resolver_ips`.\n"]
    pub fn set_resolver_ips(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resolver_ips = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConnectivityDirectoryService_Host_ResolverNetwork {
    type O = terrars::BlockAssignable<ConnectivityDirectoryService_Host_ResolverNetwork>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectivityDirectoryService_Host_ResolverNetwork {
    #[doc = ""]
    pub tunnel_id: terrars::PrimField<std::string::String>,
}

impl BuildConnectivityDirectoryService_Host_ResolverNetwork {
    pub fn build(self) -> ConnectivityDirectoryService_Host_ResolverNetwork {
        ConnectivityDirectoryService_Host_ResolverNetwork {
            resolver_ips: core::default::Default::default(),
            tunnel_id: self.tunnel_id,
        }
    }
}

#[allow(dead_code)]
pub struct ConnectivityDirectoryService_Host_ResolverNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectivityDirectoryService_Host_ResolverNetworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectivityDirectoryService_Host_ResolverNetworkRef {
        ConnectivityDirectoryService_Host_ResolverNetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectivityDirectoryService_Host_ResolverNetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resolver_ips` after provisioning.\n"]
    pub fn resolver_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resolver_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\n"]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ConnectivityDirectoryService_Host {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<ConnectivityDirectoryService_Host_Network>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolver_network: std::option::Option<ConnectivityDirectoryService_Host_ResolverNetwork>,
}

impl ConnectivityDirectoryService_Host {
    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4`.\n"]
    pub fn set_ipv4(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv4 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6`.\n"]
    pub fn set_ipv6(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<ConnectivityDirectoryService_Host_Network>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resolver_network`.\n"]
    pub fn set_resolver_network(
        mut self,
        v: impl std::convert::Into<ConnectivityDirectoryService_Host_ResolverNetwork>,
    ) -> Self {
        self.resolver_network = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConnectivityDirectoryService_Host {
    type O = terrars::BlockAssignable<ConnectivityDirectoryService_Host>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectivityDirectoryService_Host {}

impl BuildConnectivityDirectoryService_Host {
    pub fn build(self) -> ConnectivityDirectoryService_Host {
        ConnectivityDirectoryService_Host {
            hostname: core::default::Default::default(),
            ipv4: core::default::Default::default(),
            ipv6: core::default::Default::default(),
            network: core::default::Default::default(),
            resolver_network: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ConnectivityDirectoryService_HostRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectivityDirectoryService_HostRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConnectivityDirectoryService_HostRef {
        ConnectivityDirectoryService_HostRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectivityDirectoryService_HostRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4` after provisioning.\n"]
    pub fn ipv4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> ConnectivityDirectoryService_Host_NetworkRef {
        ConnectivityDirectoryService_Host_NetworkRef::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `resolver_network` after provisioning.\n"]
    pub fn resolver_network(&self) -> ConnectivityDirectoryService_Host_ResolverNetworkRef {
        ConnectivityDirectoryService_Host_ResolverNetworkRef::new(
            self.shared().clone(),
            format!("{}.resolver_network", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ConnectivityDirectoryService_TlsSettings {
    cert_verification_mode: terrars::PrimField<std::string::String>,
}

impl ConnectivityDirectoryService_TlsSettings { }

impl terrars::ToListMappable for ConnectivityDirectoryService_TlsSettings {
    type O = terrars::BlockAssignable<ConnectivityDirectoryService_TlsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectivityDirectoryService_TlsSettings {
    #[doc =
        "TLS certificate verification mode for the connection to the origin.\n\n- `\"verify_full\"` — verify certificate chain and hostname (default)\n- `\"verify_ca\"` — verify certificate chain only, skip hostname check\n- `\"disabled\"` — do not verify the server certificate at all"]
    pub cert_verification_mode: terrars::PrimField<std::string::String>,
}

impl BuildConnectivityDirectoryService_TlsSettings {
    pub fn build(self) -> ConnectivityDirectoryService_TlsSettings {
        ConnectivityDirectoryService_TlsSettings { cert_verification_mode: self.cert_verification_mode }
    }
}

#[allow(dead_code)]
pub struct ConnectivityDirectoryService_TlsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectivityDirectoryService_TlsSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConnectivityDirectoryService_TlsSettingsRef {
        ConnectivityDirectoryService_TlsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectivityDirectoryService_TlsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cert_verification_mode` after provisioning.\nTLS certificate verification mode for the connection to the origin.\n\n- `\"verify_full\"` — verify certificate chain and hostname (default)\n- `\"verify_ca\"` — verify certificate chain only, skip hostname check\n- `\"disabled\"` — do not verify the server certificate at all"]
    pub fn cert_verification_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert_verification_mode", self.base))
    }
}

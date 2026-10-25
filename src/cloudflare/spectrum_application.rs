#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct SpectrumApplicationData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    argo_smart_routing: std::option::Option<terrars::PrimField<bool>>,
    dns: SpectrumApplication_Dns,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_ips: std::option::Option<SpectrumApplication_EdgeIps>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_firewall: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_direct: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_dns: std::option::Option<SpectrumApplication_OriginDns>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_port: std::option::Option<terrars::PrimField<serde_json::Value>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_worker_id: std::option::Option<terrars::PrimField<std::string::String>>,
    protocol: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_network_id: std::option::Option<terrars::PrimField<std::string::String>>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct SpectrumApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SpectrumApplicationData_>,
}

#[derive(Clone)]
pub struct SpectrumApplication(std::rc::Rc<SpectrumApplication_>);

impl SpectrumApplication {
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
        "Set the field `argo_smart_routing`.\nEnables Argo Smart Routing for this application.\nNotes: Only available for TCP or UDP applications with traffic_type set to \"direct\"."]
    pub fn set_argo_smart_routing(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().argo_smart_routing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edge_ips`.\nThe anycast edge IP configuration for the hostname of this application."]
    pub fn set_edge_ips(self, v: impl std::convert::Into<SpectrumApplication_EdgeIps>) -> Self {
        self.0.data.borrow_mut().edge_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_firewall`.\nEnables IP Access Rules for this application.\nNotes: Only available for TCP applications."]
    pub fn set_ip_firewall(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ip_firewall = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_direct`.\nList of origin IP addresses. Array may contain multiple IP addresses for load balancing."]
    pub fn set_origin_direct(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().origin_direct = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_dns`.\nThe name and type of DNS record for the Spectrum application."]
    pub fn set_origin_dns(self, v: impl std::convert::Into<SpectrumApplication_OriginDns>) -> Self {
        self.0.data.borrow_mut().origin_dns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_port`.\nThe destination port at the origin. Only specified in conjunction with origin_dns. May use an integer to specify a single origin port, for example `1000`, or a string to specify a range of origin ports, for example `\"1000-2000\"`.\nNotes: If specifying a port range, the number of ports in the range must match the number of ports specified in the \"protocol\" field."]
    pub fn set_origin_port(self, v: impl std::convert::Into<terrars::PrimField<serde_json::Value>>) -> Self {
        self.0.data.borrow_mut().origin_port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_worker_id`.\nOptional Worker script tag (worker ID) to use as the application's origin. Only supported for TCP applications with traffic_type \"worker\"; mutually exclusive with origin_direct, origin_dns, origin_port, proxy_protocol, and argo_smart_routing. tls may only be \"off\" or \"flexible\"."]
    pub fn set_origin_worker_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().origin_worker_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_protocol`.\nEnables Proxy Protocol to the origin. Refer to [Enable Proxy protocol](https://developers.cloudflare.com/spectrum/getting-started/proxy-protocol/) for implementation details on PROXY Protocol V1, PROXY Protocol V2, and Simple Proxy Protocol.\nAvailable values: \"off\", \"v1\", \"v2\", \"simple\"."]
    pub fn set_proxy_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().proxy_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tls`.\nThe type of TLS termination associated with the application.\nAvailable values: \"off\", \"flexible\", \"full\", \"strict\"."]
    pub fn set_tls(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tls = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `traffic_type`.\nDetermines how data travels from the edge to your origin. When set to \"direct\", Spectrum will send traffic directly to your origin, and the application's type is derived from the `protocol`. When set to \"http\" or \"https\", Spectrum will apply Cloudflare's HTTP/HTTPS features as it sends traffic to your origin, and the application type matches this property exactly. When set to \"worker\", traffic is sent to the Worker specified by `origin_worker_id`.\nAvailable values: \"direct\", \"http\", \"https\", \"worker\"."]
    pub fn set_traffic_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().traffic_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `virtual_network_id`.\nOptional UUID of a virtual network for routing origin traffic through tunnel virtual networks."]
    pub fn set_virtual_network_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().virtual_network_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `argo_smart_routing` after provisioning.\nEnables Argo Smart Routing for this application.\nNotes: Only available for TCP or UDP applications with traffic_type set to \"direct\"."]
    pub fn argo_smart_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.argo_smart_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the Application was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns` after provisioning.\nThe name and type of DNS record for the Spectrum application."]
    pub fn dns(&self) -> SpectrumApplication_DnsRef {
        SpectrumApplication_DnsRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edge_ips` after provisioning.\nThe anycast edge IP configuration for the hostname of this application."]
    pub fn edge_ips(&self) -> SpectrumApplication_EdgeIpsRef {
        SpectrumApplication_EdgeIpsRef::new(self.shared().clone(), format!("{}.edge_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nApp identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_firewall` after provisioning.\nEnables IP Access Rules for this application.\nNotes: Only available for TCP applications."]
    pub fn ip_firewall(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_firewall", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the Application was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_direct` after provisioning.\nList of origin IP addresses. Array may contain multiple IP addresses for load balancing."]
    pub fn origin_direct(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_direct", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_dns` after provisioning.\nThe name and type of DNS record for the Spectrum application."]
    pub fn origin_dns(&self) -> SpectrumApplication_OriginDnsRef {
        SpectrumApplication_OriginDnsRef::new(self.shared().clone(), format!("{}.origin_dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_port` after provisioning.\nThe destination port at the origin. Only specified in conjunction with origin_dns. May use an integer to specify a single origin port, for example `1000`, or a string to specify a range of origin ports, for example `\"1000-2000\"`.\nNotes: If specifying a port range, the number of ports in the range must match the number of ports specified in the \"protocol\" field."]
    pub fn origin_port(&self) -> terrars::PrimExpr<serde_json::Value> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_worker_id` after provisioning.\nOptional Worker script tag (worker ID) to use as the application's origin. Only supported for TCP applications with traffic_type \"worker\"; mutually exclusive with origin_direct, origin_dns, origin_port, proxy_protocol, and argo_smart_routing. tls may only be \"off\" or \"flexible\"."]
    pub fn origin_worker_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_worker_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nThe port configuration at Cloudflare's edge. May specify a single port, for example `\"tcp/1000\"`, or a range of ports, for example `\"tcp/1000-2000\"`."]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxy_protocol` after provisioning.\nEnables Proxy Protocol to the origin. Refer to [Enable Proxy protocol](https://developers.cloudflare.com/spectrum/getting-started/proxy-protocol/) for implementation details on PROXY Protocol V1, PROXY Protocol V2, and Simple Proxy Protocol.\nAvailable values: \"off\", \"v1\", \"v2\", \"simple\"."]
    pub fn proxy_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls` after provisioning.\nThe type of TLS termination associated with the application.\nAvailable values: \"off\", \"flexible\", \"full\", \"strict\"."]
    pub fn tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic_type` after provisioning.\nDetermines how data travels from the edge to your origin. When set to \"direct\", Spectrum will send traffic directly to your origin, and the application's type is derived from the `protocol`. When set to \"http\" or \"https\", Spectrum will apply Cloudflare's HTTP/HTTPS features as it sends traffic to your origin, and the application type matches this property exactly. When set to \"worker\", traffic is sent to the Worker specified by `origin_worker_id`.\nAvailable values: \"direct\", \"http\", \"https\", \"worker\"."]
    pub fn traffic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `virtual_network_id` after provisioning.\nOptional UUID of a virtual network for routing origin traffic through tunnel virtual networks."]
    pub fn virtual_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nZone identifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for SpectrumApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SpectrumApplication { }

impl terrars::ToListMappable for SpectrumApplication {
    type O = terrars::ListRef<SpectrumApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SpectrumApplication_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_spectrum_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSpectrumApplication {
    pub tf_id: std::string::String,
    #[doc = "The name and type of DNS record for the Spectrum application."]
    pub dns: SpectrumApplication_Dns,
    #[doc =
        "The port configuration at Cloudflare's edge. May specify a single port, for example `\"tcp/1000\"`, or a range of ports, for example `\"tcp/1000-2000\"`."]
    pub protocol: terrars::PrimField<std::string::String>,
    #[doc = "Zone identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildSpectrumApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> SpectrumApplication {
        let out = SpectrumApplication(std::rc::Rc::new(SpectrumApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SpectrumApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                argo_smart_routing: core::default::Default::default(),
                dns: self.dns,
                edge_ips: core::default::Default::default(),
                ip_firewall: core::default::Default::default(),
                origin_direct: core::default::Default::default(),
                origin_dns: core::default::Default::default(),
                origin_port: core::default::Default::default(),
                origin_worker_id: core::default::Default::default(),
                protocol: self.protocol,
                proxy_protocol: core::default::Default::default(),
                tls: core::default::Default::default(),
                traffic_type: core::default::Default::default(),
                virtual_network_id: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SpectrumApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpectrumApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SpectrumApplicationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `argo_smart_routing` after provisioning.\nEnables Argo Smart Routing for this application.\nNotes: Only available for TCP or UDP applications with traffic_type set to \"direct\"."]
    pub fn argo_smart_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.argo_smart_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the Application was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns` after provisioning.\nThe name and type of DNS record for the Spectrum application."]
    pub fn dns(&self) -> SpectrumApplication_DnsRef {
        SpectrumApplication_DnsRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edge_ips` after provisioning.\nThe anycast edge IP configuration for the hostname of this application."]
    pub fn edge_ips(&self) -> SpectrumApplication_EdgeIpsRef {
        SpectrumApplication_EdgeIpsRef::new(self.shared().clone(), format!("{}.edge_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nApp identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_firewall` after provisioning.\nEnables IP Access Rules for this application.\nNotes: Only available for TCP applications."]
    pub fn ip_firewall(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_firewall", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the Application was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_direct` after provisioning.\nList of origin IP addresses. Array may contain multiple IP addresses for load balancing."]
    pub fn origin_direct(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_direct", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_dns` after provisioning.\nThe name and type of DNS record for the Spectrum application."]
    pub fn origin_dns(&self) -> SpectrumApplication_OriginDnsRef {
        SpectrumApplication_OriginDnsRef::new(self.shared().clone(), format!("{}.origin_dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_port` after provisioning.\nThe destination port at the origin. Only specified in conjunction with origin_dns. May use an integer to specify a single origin port, for example `1000`, or a string to specify a range of origin ports, for example `\"1000-2000\"`.\nNotes: If specifying a port range, the number of ports in the range must match the number of ports specified in the \"protocol\" field."]
    pub fn origin_port(&self) -> terrars::PrimExpr<serde_json::Value> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_worker_id` after provisioning.\nOptional Worker script tag (worker ID) to use as the application's origin. Only supported for TCP applications with traffic_type \"worker\"; mutually exclusive with origin_direct, origin_dns, origin_port, proxy_protocol, and argo_smart_routing. tls may only be \"off\" or \"flexible\"."]
    pub fn origin_worker_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_worker_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nThe port configuration at Cloudflare's edge. May specify a single port, for example `\"tcp/1000\"`, or a range of ports, for example `\"tcp/1000-2000\"`."]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxy_protocol` after provisioning.\nEnables Proxy Protocol to the origin. Refer to [Enable Proxy protocol](https://developers.cloudflare.com/spectrum/getting-started/proxy-protocol/) for implementation details on PROXY Protocol V1, PROXY Protocol V2, and Simple Proxy Protocol.\nAvailable values: \"off\", \"v1\", \"v2\", \"simple\"."]
    pub fn proxy_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls` after provisioning.\nThe type of TLS termination associated with the application.\nAvailable values: \"off\", \"flexible\", \"full\", \"strict\"."]
    pub fn tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic_type` after provisioning.\nDetermines how data travels from the edge to your origin. When set to \"direct\", Spectrum will send traffic directly to your origin, and the application's type is derived from the `protocol`. When set to \"http\" or \"https\", Spectrum will apply Cloudflare's HTTP/HTTPS features as it sends traffic to your origin, and the application type matches this property exactly. When set to \"worker\", traffic is sent to the Worker specified by `origin_worker_id`.\nAvailable values: \"direct\", \"http\", \"https\", \"worker\"."]
    pub fn traffic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `virtual_network_id` after provisioning.\nOptional UUID of a virtual network for routing origin traffic through tunnel virtual networks."]
    pub fn virtual_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nZone identifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SpectrumApplication_Dns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpectrumApplication_Dns {
    #[doc = "Set the field `name`.\nThe name of the DNS record associated with the application."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type of DNS record associated with the application.\nAvailable values: \"CNAME\", \"ADDRESS\"."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpectrumApplication_Dns {
    type O = terrars::BlockAssignable<SpectrumApplication_Dns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpectrumApplication_Dns {}

impl BuildSpectrumApplication_Dns {
    pub fn build(self) -> SpectrumApplication_Dns {
        SpectrumApplication_Dns {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpectrumApplication_DnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpectrumApplication_DnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpectrumApplication_DnsRef {
        SpectrumApplication_DnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpectrumApplication_DnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the DNS record associated with the application."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of DNS record associated with the application.\nAvailable values: \"CNAME\", \"ADDRESS\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct SpectrumApplication_EdgeIps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connectivity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ips: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpectrumApplication_EdgeIps {
    #[doc =
        "Set the field `connectivity`.\nThe IP versions supported for inbound connections on Spectrum anycast IPs.\nAvailable values: \"all\", \"ipv4\", \"ipv6\"."]
    pub fn set_connectivity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connectivity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ips`.\nThe array of customer owned IPs we broadcast via anycast for this hostname and application."]
    pub fn set_ips(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ips = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type of edge IP configuration specified. Dynamically allocated edge IPs use Spectrum anycast IPs in accordance with the connectivity you specify. Only valid with CNAME DNS names.\nAvailable values: \"dynamic\", \"static\"."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpectrumApplication_EdgeIps {
    type O = terrars::BlockAssignable<SpectrumApplication_EdgeIps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpectrumApplication_EdgeIps {}

impl BuildSpectrumApplication_EdgeIps {
    pub fn build(self) -> SpectrumApplication_EdgeIps {
        SpectrumApplication_EdgeIps {
            connectivity: core::default::Default::default(),
            ips: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpectrumApplication_EdgeIpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpectrumApplication_EdgeIpsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpectrumApplication_EdgeIpsRef {
        SpectrumApplication_EdgeIpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpectrumApplication_EdgeIpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connectivity` after provisioning.\nThe IP versions supported for inbound connections on Spectrum anycast IPs.\nAvailable values: \"all\", \"ipv4\", \"ipv6\"."]
    pub fn connectivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connectivity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ips` after provisioning.\nThe array of customer owned IPs we broadcast via anycast for this hostname and application."]
    pub fn ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of edge IP configuration specified. Dynamically allocated edge IPs use Spectrum anycast IPs in accordance with the connectivity you specify. Only valid with CNAME DNS names.\nAvailable values: \"dynamic\", \"static\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct SpectrumApplication_OriginDns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpectrumApplication_OriginDns {
    #[doc = "Set the field `name`.\nThe name of the DNS record associated with the origin."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\nThe TTL of our resolution of your DNS record in seconds."]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type of DNS record associated with the origin. \"\" is used to specify a combination of A/AAAA records.\nAvailable values: \"\", \"A\", \"AAAA\", \"SRV\"."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpectrumApplication_OriginDns {
    type O = terrars::BlockAssignable<SpectrumApplication_OriginDns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpectrumApplication_OriginDns {}

impl BuildSpectrumApplication_OriginDns {
    pub fn build(self) -> SpectrumApplication_OriginDns {
        SpectrumApplication_OriginDns {
            name: core::default::Default::default(),
            ttl: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpectrumApplication_OriginDnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpectrumApplication_OriginDnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpectrumApplication_OriginDnsRef {
        SpectrumApplication_OriginDnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpectrumApplication_OriginDnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the DNS record associated with the origin."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nThe TTL of our resolution of your DNS record in seconds."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of DNS record associated with the origin. \"\" is used to specify a combination of A/AAAA records.\nAvailable values: \"\", \"A\", \"AAAA\", \"SRV\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

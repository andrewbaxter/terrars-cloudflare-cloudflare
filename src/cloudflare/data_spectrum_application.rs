#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataSpectrumApplicationData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataSpectrumApplication_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataSpectrumApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSpectrumApplicationData_>,
}

#[derive(Clone)]
pub struct DataSpectrumApplication(std::rc::Rc<DataSpectrumApplication_>);

impl DataSpectrumApplication {
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

    #[doc = "Set the field `app_id`.\nApp identifier."]
    pub fn set_app_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().app_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataSpectrumApplication_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nZone identifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nApp identifier."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
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
    pub fn dns(&self) -> DataSpectrumApplication_DnsRef {
        DataSpectrumApplication_DnsRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edge_ips` after provisioning.\nThe anycast edge IP configuration for the hostname of this application."]
    pub fn edge_ips(&self) -> DataSpectrumApplication_EdgeIpsRef {
        DataSpectrumApplication_EdgeIpsRef::new(self.shared().clone(), format!("{}.edge_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataSpectrumApplication_FilterRef {
        DataSpectrumApplication_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
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
    pub fn origin_dns(&self) -> DataSpectrumApplication_OriginDnsRef {
        DataSpectrumApplication_OriginDnsRef::new(
            self.shared().clone(),
            format!("{}.origin_dns", self.extract_ref()),
        )
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

impl terrars::Referable for DataSpectrumApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSpectrumApplication { }

impl terrars::ToListMappable for DataSpectrumApplication {
    type O = terrars::ListRef<DataSpectrumApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSpectrumApplication_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_spectrum_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSpectrumApplication {
    pub tf_id: std::string::String,
}

impl BuildDataSpectrumApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSpectrumApplication {
        let out = DataSpectrumApplication(std::rc::Rc::new(DataSpectrumApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSpectrumApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                app_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSpectrumApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpectrumApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSpectrumApplicationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nApp identifier."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
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
    pub fn dns(&self) -> DataSpectrumApplication_DnsRef {
        DataSpectrumApplication_DnsRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edge_ips` after provisioning.\nThe anycast edge IP configuration for the hostname of this application."]
    pub fn edge_ips(&self) -> DataSpectrumApplication_EdgeIpsRef {
        DataSpectrumApplication_EdgeIpsRef::new(self.shared().clone(), format!("{}.edge_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataSpectrumApplication_FilterRef {
        DataSpectrumApplication_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
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
    pub fn origin_dns(&self) -> DataSpectrumApplication_OriginDnsRef {
        DataSpectrumApplication_OriginDnsRef::new(
            self.shared().clone(),
            format!("{}.origin_dns", self.extract_ref()),
        )
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
pub struct DataSpectrumApplication_Dns {}

impl DataSpectrumApplication_Dns { }

impl terrars::ToListMappable for DataSpectrumApplication_Dns {
    type O = terrars::BlockAssignable<DataSpectrumApplication_Dns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpectrumApplication_Dns {}

impl BuildDataSpectrumApplication_Dns {
    pub fn build(self) -> DataSpectrumApplication_Dns {
        DataSpectrumApplication_Dns {}
    }
}

#[allow(dead_code)]
pub struct DataSpectrumApplication_DnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpectrumApplication_DnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSpectrumApplication_DnsRef {
        DataSpectrumApplication_DnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpectrumApplication_DnsRef {
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
pub struct DataSpectrumApplication_EdgeIps {}

impl DataSpectrumApplication_EdgeIps { }

impl terrars::ToListMappable for DataSpectrumApplication_EdgeIps {
    type O = terrars::BlockAssignable<DataSpectrumApplication_EdgeIps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpectrumApplication_EdgeIps {}

impl BuildDataSpectrumApplication_EdgeIps {
    pub fn build(self) -> DataSpectrumApplication_EdgeIps {
        DataSpectrumApplication_EdgeIps {}
    }
}

#[allow(dead_code)]
pub struct DataSpectrumApplication_EdgeIpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpectrumApplication_EdgeIpsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSpectrumApplication_EdgeIpsRef {
        DataSpectrumApplication_EdgeIpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpectrumApplication_EdgeIpsRef {
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
pub struct DataSpectrumApplication_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSpectrumApplication_Filter {
    #[doc =
        "Set the field `direction`.\nSets the direction by which results are ordered.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nApplication field by which results are ordered.\nAvailable values: \"protocol\", \"app_id\", \"created_on\", \"modified_on\", \"dns\"."]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSpectrumApplication_Filter {
    type O = terrars::BlockAssignable<DataSpectrumApplication_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpectrumApplication_Filter {}

impl BuildDataSpectrumApplication_Filter {
    pub fn build(self) -> DataSpectrumApplication_Filter {
        DataSpectrumApplication_Filter {
            direction: core::default::Default::default(),
            order: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSpectrumApplication_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpectrumApplication_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSpectrumApplication_FilterRef {
        DataSpectrumApplication_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpectrumApplication_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nSets the direction by which results are ordered.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nApplication field by which results are ordered.\nAvailable values: \"protocol\", \"app_id\", \"created_on\", \"modified_on\", \"dns\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSpectrumApplication_OriginDns {}

impl DataSpectrumApplication_OriginDns { }

impl terrars::ToListMappable for DataSpectrumApplication_OriginDns {
    type O = terrars::BlockAssignable<DataSpectrumApplication_OriginDns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpectrumApplication_OriginDns {}

impl BuildDataSpectrumApplication_OriginDns {
    pub fn build(self) -> DataSpectrumApplication_OriginDns {
        DataSpectrumApplication_OriginDns {}
    }
}

#[allow(dead_code)]
pub struct DataSpectrumApplication_OriginDnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpectrumApplication_OriginDnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSpectrumApplication_OriginDnsRef {
        DataSpectrumApplication_OriginDnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpectrumApplication_OriginDnsRef {
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

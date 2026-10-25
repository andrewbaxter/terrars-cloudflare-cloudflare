#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataSpectrumApplicationsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataSpectrumApplications_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSpectrumApplicationsData_>,
}

#[derive(Clone)]
pub struct DataSpectrumApplications(std::rc::Rc<DataSpectrumApplications_>);

impl DataSpectrumApplications {
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

    #[doc =
        "Set the field `direction`.\nSets the direction by which results are ordered.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nApplication field by which results are ordered.\nAvailable values: \"protocol\", \"app_id\", \"created_on\", \"modified_on\", \"dns\"."]
    pub fn set_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nZone identifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nSets the direction by which results are ordered.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nApplication field by which results are ordered.\nAvailable values: \"protocol\", \"app_id\", \"created_on\", \"modified_on\", \"dns\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataSpectrumApplications_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nZone identifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataSpectrumApplications {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSpectrumApplications { }

impl terrars::ToListMappable for DataSpectrumApplications {
    type O = terrars::ListRef<DataSpectrumApplicationsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSpectrumApplications_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_spectrum_applications".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSpectrumApplications {
    pub tf_id: std::string::String,
}

impl BuildDataSpectrumApplications {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSpectrumApplications {
        let out = DataSpectrumApplications(std::rc::Rc::new(DataSpectrumApplications_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSpectrumApplicationsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                direction: core::default::Default::default(),
                max_items: core::default::Default::default(),
                order: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSpectrumApplicationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpectrumApplicationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSpectrumApplicationsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nSets the direction by which results are ordered.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nApplication field by which results are ordered.\nAvailable values: \"protocol\", \"app_id\", \"created_on\", \"modified_on\", \"dns\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataSpectrumApplications_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nZone identifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSpectrumApplications_Result_Dns {}

impl DataSpectrumApplications_Result_Dns { }

impl terrars::ToListMappable for DataSpectrumApplications_Result_Dns {
    type O = terrars::BlockAssignable<DataSpectrumApplications_Result_Dns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpectrumApplications_Result_Dns {}

impl BuildDataSpectrumApplications_Result_Dns {
    pub fn build(self) -> DataSpectrumApplications_Result_Dns {
        DataSpectrumApplications_Result_Dns {}
    }
}

#[allow(dead_code)]
pub struct DataSpectrumApplications_Result_DnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpectrumApplications_Result_DnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSpectrumApplications_Result_DnsRef {
        DataSpectrumApplications_Result_DnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpectrumApplications_Result_DnsRef {
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
pub struct DataSpectrumApplications_Result_EdgeIps {}

impl DataSpectrumApplications_Result_EdgeIps { }

impl terrars::ToListMappable for DataSpectrumApplications_Result_EdgeIps {
    type O = terrars::BlockAssignable<DataSpectrumApplications_Result_EdgeIps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpectrumApplications_Result_EdgeIps {}

impl BuildDataSpectrumApplications_Result_EdgeIps {
    pub fn build(self) -> DataSpectrumApplications_Result_EdgeIps {
        DataSpectrumApplications_Result_EdgeIps {}
    }
}

#[allow(dead_code)]
pub struct DataSpectrumApplications_Result_EdgeIpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpectrumApplications_Result_EdgeIpsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSpectrumApplications_Result_EdgeIpsRef {
        DataSpectrumApplications_Result_EdgeIpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpectrumApplications_Result_EdgeIpsRef {
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
pub struct DataSpectrumApplications_Result_OriginDns {}

impl DataSpectrumApplications_Result_OriginDns { }

impl terrars::ToListMappable for DataSpectrumApplications_Result_OriginDns {
    type O = terrars::BlockAssignable<DataSpectrumApplications_Result_OriginDns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpectrumApplications_Result_OriginDns {}

impl BuildDataSpectrumApplications_Result_OriginDns {
    pub fn build(self) -> DataSpectrumApplications_Result_OriginDns {
        DataSpectrumApplications_Result_OriginDns {}
    }
}

#[allow(dead_code)]
pub struct DataSpectrumApplications_Result_OriginDnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpectrumApplications_Result_OriginDnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSpectrumApplications_Result_OriginDnsRef {
        DataSpectrumApplications_Result_OriginDnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpectrumApplications_Result_OriginDnsRef {
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

#[derive(Serialize)]
pub struct DataSpectrumApplications_Result {}

impl DataSpectrumApplications_Result { }

impl terrars::ToListMappable for DataSpectrumApplications_Result {
    type O = terrars::BlockAssignable<DataSpectrumApplications_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpectrumApplications_Result {}

impl BuildDataSpectrumApplications_Result {
    pub fn build(self) -> DataSpectrumApplications_Result {
        DataSpectrumApplications_Result {}
    }
}

#[allow(dead_code)]
pub struct DataSpectrumApplications_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpectrumApplications_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSpectrumApplications_ResultRef {
        DataSpectrumApplications_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpectrumApplications_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `argo_smart_routing` after provisioning.\nEnables Argo Smart Routing for this application.\nNotes: Only available for TCP applications with traffic_type set to \"direct\"."]
    pub fn argo_smart_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.argo_smart_routing", self.base))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the Application was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dns` after provisioning.\nThe name and type of DNS record for the Spectrum application."]
    pub fn dns(&self) -> DataSpectrumApplications_Result_DnsRef {
        DataSpectrumApplications_Result_DnsRef::new(self.shared().clone(), format!("{}.dns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `edge_ips` after provisioning.\nThe anycast edge IP configuration for the hostname of this application."]
    pub fn edge_ips(&self) -> DataSpectrumApplications_Result_EdgeIpsRef {
        DataSpectrumApplications_Result_EdgeIpsRef::new(self.shared().clone(), format!("{}.edge_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nApp identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_firewall` after provisioning.\nEnables IP Access Rules for this application.\nNotes: Only available for TCP applications."]
    pub fn ip_firewall(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_firewall", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the Application was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_direct` after provisioning.\nList of origin IP addresses. Array may contain multiple IP addresses for load balancing."]
    pub fn origin_direct(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_direct", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_dns` after provisioning.\nThe name and type of DNS record for the Spectrum application."]
    pub fn origin_dns(&self) -> DataSpectrumApplications_Result_OriginDnsRef {
        DataSpectrumApplications_Result_OriginDnsRef::new(self.shared().clone(), format!("{}.origin_dns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_port` after provisioning.\nThe destination port at the origin. Only specified in conjunction with origin_dns. May use an integer to specify a single origin port, for example `1000`, or a string to specify a range of origin ports, for example `\"1000-2000\"`.\nNotes: If specifying a port range, the number of ports in the range must match the number of ports specified in the \"protocol\" field."]
    pub fn origin_port(&self) -> terrars::PrimExpr<serde_json::Value> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nThe port configuration at Cloudflare's edge. May specify a single port, for example `\"tcp/1000\"`, or a range of ports, for example `\"tcp/1000-2000\"`."]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_protocol` after provisioning.\nEnables Proxy Protocol to the origin. Refer to [Enable Proxy protocol](https://developers.cloudflare.com/spectrum/getting-started/proxy-protocol/) for implementation details on PROXY Protocol V1, PROXY Protocol V2, and Simple Proxy Protocol.\nAvailable values: \"off\", \"v1\", \"v2\", \"simple\"."]
    pub fn proxy_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls` after provisioning.\nThe type of TLS termination associated with the application.\nAvailable values: \"off\", \"flexible\", \"full\", \"strict\"."]
    pub fn tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls", self.base))
    }

    #[doc =
        "Get a reference to the value of field `traffic_type` after provisioning.\nDetermines how data travels from the edge to your origin. When set to \"direct\", Spectrum will send traffic directly to your origin, and the application's type is derived from the `protocol`. When set to \"http\" or \"https\", Spectrum will apply Cloudflare's HTTP/HTTPS features as it sends traffic to your origin, and the application type matches this property exactly.\nAvailable values: \"direct\", \"http\", \"https\"."]
    pub fn traffic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_type", self.base))
    }
}

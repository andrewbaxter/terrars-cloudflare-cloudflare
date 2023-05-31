use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct SpectrumApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    argo_smart_routing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_firewall: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_direct: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_port: Option<PrimField<f64>>,
    protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_type: Option<PrimField<String>>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns: Option<Vec<SpectrumApplicationDnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_ips: Option<Vec<SpectrumApplicationEdgeIpsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_dns: Option<Vec<SpectrumApplicationOriginDnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_port_range: Option<Vec<SpectrumApplicationOriginPortRangeEl>>,
    dynamic: SpectrumApplicationDynamic,
}

struct SpectrumApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SpectrumApplicationData>,
}

#[derive(Clone)]
pub struct SpectrumApplication(Rc<SpectrumApplication_>);

impl SpectrumApplication {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `argo_smart_routing`.\nEnables Argo Smart Routing. Defaults to `false`."]
    pub fn set_argo_smart_routing(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().argo_smart_routing = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_firewall`.\nEnables the IP Firewall for this application. Defaults to `true`."]
    pub fn set_ip_firewall(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ip_firewall = Some(v.into());
        self
    }

    #[doc= "Set the field `origin_direct`.\nA list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`."]
    pub fn set_origin_direct(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().origin_direct = Some(v.into());
        self
    }

    #[doc= "Set the field `origin_port`.\nOrigin port to proxy traffice to. Conflicts with `origin_port_range`."]
    pub fn set_origin_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().origin_port = Some(v.into());
        self
    }

    #[doc= "Set the field `proxy_protocol`.\nEnables a proxy protocol to the origin. Available values: `off`, `v1`, `v2`, `simple`. Defaults to `off`."]
    pub fn set_proxy_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().proxy_protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `tls`.\nTLS configuration option for Cloudflare to connect to your origin. Available values: `off`, `flexible`, `full`, `strict`. Defaults to `off`."]
    pub fn set_tls(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tls = Some(v.into());
        self
    }

    #[doc= "Set the field `traffic_type`.\nSets application type. Available values: `direct`, `http`, `https`. Defaults to `direct`."]
    pub fn set_traffic_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().traffic_type = Some(v.into());
        self
    }

    #[doc= "Set the field `dns`.\n"]
    pub fn set_dns(self, v: impl Into<BlockAssignable<SpectrumApplicationDnsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dns = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dns = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `edge_ips`.\n"]
    pub fn set_edge_ips(self, v: impl Into<BlockAssignable<SpectrumApplicationEdgeIpsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().edge_ips = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.edge_ips = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `origin_dns`.\n"]
    pub fn set_origin_dns(self, v: impl Into<BlockAssignable<SpectrumApplicationOriginDnsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin_dns = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin_dns = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `origin_port_range`.\n"]
    pub fn set_origin_port_range(self, v: impl Into<BlockAssignable<SpectrumApplicationOriginPortRangeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin_port_range = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin_port_range = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `argo_smart_routing` after provisioning.\nEnables Argo Smart Routing. Defaults to `false`."]
    pub fn argo_smart_routing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.argo_smart_routing", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_firewall` after provisioning.\nEnables the IP Firewall for this application. Defaults to `true`."]
    pub fn ip_firewall(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_firewall", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `origin_direct` after provisioning.\nA list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`."]
    pub fn origin_direct(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.origin_direct", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `origin_port` after provisioning.\nOrigin port to proxy traffice to. Conflicts with `origin_port_range`."]
    pub fn origin_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\nThe port configuration at Cloudflare's edge. e.g. `tcp/22`."]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxy_protocol` after provisioning.\nEnables a proxy protocol to the origin. Available values: `off`, `v1`, `v2`, `simple`. Defaults to `off`."]
    pub fn proxy_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tls` after provisioning.\nTLS configuration option for Cloudflare to connect to your origin. Available values: `off`, `flexible`, `full`, `strict`. Defaults to `off`."]
    pub fn tls(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `traffic_type` after provisioning.\nSets application type. Available values: `direct`, `http`, `https`. Defaults to `direct`."]
    pub fn traffic_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.traffic_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> ListRef<SpectrumApplicationDnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `edge_ips` after provisioning.\n"]
    pub fn edge_ips(&self) -> ListRef<SpectrumApplicationEdgeIpsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.edge_ips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `origin_dns` after provisioning.\n"]
    pub fn origin_dns(&self) -> ListRef<SpectrumApplicationOriginDnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.origin_dns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `origin_port_range` after provisioning.\n"]
    pub fn origin_port_range(&self) -> ListRef<SpectrumApplicationOriginPortRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.origin_port_range", self.extract_ref()))
    }
}

impl Referable for SpectrumApplication {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SpectrumApplication { }

impl ToListMappable for SpectrumApplication {
    type O = ListRef<SpectrumApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SpectrumApplication_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_spectrum_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSpectrumApplication {
    pub tf_id: String,
    #[doc= "The port configuration at Cloudflare's edge. e.g. `tcp/22`."]
    pub protocol: PrimField<String>,
    #[doc= "The zone identifier to target for the resource."]
    pub zone_id: PrimField<String>,
}

impl BuildSpectrumApplication {
    pub fn build(self, stack: &mut Stack) -> SpectrumApplication {
        let out = SpectrumApplication(Rc::new(SpectrumApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SpectrumApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                argo_smart_routing: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_firewall: core::default::Default::default(),
                origin_direct: core::default::Default::default(),
                origin_port: core::default::Default::default(),
                protocol: self.protocol,
                proxy_protocol: core::default::Default::default(),
                tls: core::default::Default::default(),
                traffic_type: core::default::Default::default(),
                zone_id: self.zone_id,
                dns: core::default::Default::default(),
                edge_ips: core::default::Default::default(),
                origin_dns: core::default::Default::default(),
                origin_port_range: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SpectrumApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpectrumApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SpectrumApplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `argo_smart_routing` after provisioning.\nEnables Argo Smart Routing. Defaults to `false`."]
    pub fn argo_smart_routing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.argo_smart_routing", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_firewall` after provisioning.\nEnables the IP Firewall for this application. Defaults to `true`."]
    pub fn ip_firewall(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_firewall", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `origin_direct` after provisioning.\nA list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`."]
    pub fn origin_direct(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.origin_direct", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `origin_port` after provisioning.\nOrigin port to proxy traffice to. Conflicts with `origin_port_range`."]
    pub fn origin_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\nThe port configuration at Cloudflare's edge. e.g. `tcp/22`."]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxy_protocol` after provisioning.\nEnables a proxy protocol to the origin. Available values: `off`, `v1`, `v2`, `simple`. Defaults to `off`."]
    pub fn proxy_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tls` after provisioning.\nTLS configuration option for Cloudflare to connect to your origin. Available values: `off`, `flexible`, `full`, `strict`. Defaults to `off`."]
    pub fn tls(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `traffic_type` after provisioning.\nSets application type. Available values: `direct`, `http`, `https`. Defaults to `direct`."]
    pub fn traffic_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.traffic_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> ListRef<SpectrumApplicationDnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `edge_ips` after provisioning.\n"]
    pub fn edge_ips(&self) -> ListRef<SpectrumApplicationEdgeIpsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.edge_ips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `origin_dns` after provisioning.\n"]
    pub fn origin_dns(&self) -> ListRef<SpectrumApplicationOriginDnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.origin_dns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `origin_port_range` after provisioning.\n"]
    pub fn origin_port_range(&self) -> ListRef<SpectrumApplicationOriginPortRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.origin_port_range", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SpectrumApplicationDnsEl {
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl SpectrumApplicationDnsEl { }

impl ToListMappable for SpectrumApplicationDnsEl {
    type O = BlockAssignable<SpectrumApplicationDnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpectrumApplicationDnsEl {
    #[doc= "The name of the DNS record associated with the application."]
    pub name: PrimField<String>,
    #[doc= "The type of DNS record associated with the application."]
    pub type_: PrimField<String>,
}

impl BuildSpectrumApplicationDnsEl {
    pub fn build(self) -> SpectrumApplicationDnsEl {
        SpectrumApplicationDnsEl {
            name: self.name,
            type_: self.type_,
        }
    }
}

pub struct SpectrumApplicationDnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpectrumApplicationDnsElRef {
    fn new(shared: StackShared, base: String) -> SpectrumApplicationDnsElRef {
        SpectrumApplicationDnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpectrumApplicationDnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the DNS record associated with the application."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of DNS record associated with the application."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct SpectrumApplicationEdgeIpsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connectivity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ips: Option<SetField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl SpectrumApplicationEdgeIpsEl {
    #[doc= "Set the field `connectivity`.\nThe IP versions supported for inbound connections on Spectrum anycast IPs. Required when `type` is not `static`. Available values: `all`, `ipv4`, `ipv6`."]
    pub fn set_connectivity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connectivity = Some(v.into());
        self
    }

    #[doc= "Set the field `ips`.\nThe collection of customer owned IPs to broadcast via anycast for this hostname and application. Requires [Bring Your Own IP](https://developers.cloudflare.com/spectrum/getting-started/byoip/) provisioned."]
    pub fn set_ips(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ips = Some(v.into());
        self
    }
}

impl ToListMappable for SpectrumApplicationEdgeIpsEl {
    type O = BlockAssignable<SpectrumApplicationEdgeIpsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpectrumApplicationEdgeIpsEl {
    #[doc= "The type of edge IP configuration specified. Available values: `dynamic`, `static`."]
    pub type_: PrimField<String>,
}

impl BuildSpectrumApplicationEdgeIpsEl {
    pub fn build(self) -> SpectrumApplicationEdgeIpsEl {
        SpectrumApplicationEdgeIpsEl {
            connectivity: core::default::Default::default(),
            ips: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct SpectrumApplicationEdgeIpsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpectrumApplicationEdgeIpsElRef {
    fn new(shared: StackShared, base: String) -> SpectrumApplicationEdgeIpsElRef {
        SpectrumApplicationEdgeIpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpectrumApplicationEdgeIpsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `connectivity` after provisioning.\nThe IP versions supported for inbound connections on Spectrum anycast IPs. Required when `type` is not `static`. Available values: `all`, `ipv4`, `ipv6`."]
    pub fn connectivity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connectivity", self.base))
    }

    #[doc= "Get a reference to the value of field `ips` after provisioning.\nThe collection of customer owned IPs to broadcast via anycast for this hostname and application. Requires [Bring Your Own IP](https://developers.cloudflare.com/spectrum/getting-started/byoip/) provisioned."]
    pub fn ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ips", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of edge IP configuration specified. Available values: `dynamic`, `static`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct SpectrumApplicationOriginDnsEl {
    name: PrimField<String>,
}

impl SpectrumApplicationOriginDnsEl { }

impl ToListMappable for SpectrumApplicationOriginDnsEl {
    type O = BlockAssignable<SpectrumApplicationOriginDnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpectrumApplicationOriginDnsEl {
    #[doc= "Fully qualified domain name of the origin."]
    pub name: PrimField<String>,
}

impl BuildSpectrumApplicationOriginDnsEl {
    pub fn build(self) -> SpectrumApplicationOriginDnsEl {
        SpectrumApplicationOriginDnsEl { name: self.name }
    }
}

pub struct SpectrumApplicationOriginDnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpectrumApplicationOriginDnsElRef {
    fn new(shared: StackShared, base: String) -> SpectrumApplicationOriginDnsElRef {
        SpectrumApplicationOriginDnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpectrumApplicationOriginDnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nFully qualified domain name of the origin."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct SpectrumApplicationOriginPortRangeEl {
    end: PrimField<f64>,
    start: PrimField<f64>,
}

impl SpectrumApplicationOriginPortRangeEl { }

impl ToListMappable for SpectrumApplicationOriginPortRangeEl {
    type O = BlockAssignable<SpectrumApplicationOriginPortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpectrumApplicationOriginPortRangeEl {
    #[doc= "Upper bound of the origin port range."]
    pub end: PrimField<f64>,
    #[doc= "Lower bound of the origin port range."]
    pub start: PrimField<f64>,
}

impl BuildSpectrumApplicationOriginPortRangeEl {
    pub fn build(self) -> SpectrumApplicationOriginPortRangeEl {
        SpectrumApplicationOriginPortRangeEl {
            end: self.end,
            start: self.start,
        }
    }
}

pub struct SpectrumApplicationOriginPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpectrumApplicationOriginPortRangeElRef {
    fn new(shared: StackShared, base: String) -> SpectrumApplicationOriginPortRangeElRef {
        SpectrumApplicationOriginPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpectrumApplicationOriginPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `end` after provisioning.\nUpper bound of the origin port range."]
    pub fn end(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc= "Get a reference to the value of field `start` after provisioning.\nLower bound of the origin port range."]
    pub fn start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpectrumApplicationDynamic {
    dns: Option<DynamicBlock<SpectrumApplicationDnsEl>>,
    edge_ips: Option<DynamicBlock<SpectrumApplicationEdgeIpsEl>>,
    origin_dns: Option<DynamicBlock<SpectrumApplicationOriginDnsEl>>,
    origin_port_range: Option<DynamicBlock<SpectrumApplicationOriginPortRangeEl>>,
}

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct SpectrumApplicationData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_firewall: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_direct: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_port: std::option::Option<terrars::PrimField<f64>>,
    protocol: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_type: std::option::Option<terrars::PrimField<std::string::String>>,
    zone_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns: std::option::Option<std::vec::Vec<SpectrumApplicationDnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_ips: std::option::Option<std::vec::Vec<SpectrumApplicationEdgeIpsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_dns: std::option::Option<std::vec::Vec<SpectrumApplicationOriginDnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_port_range: std::option::Option<std::vec::Vec<SpectrumApplicationOriginPortRangeEl>>,
    dynamic: SpectrumApplicationDynamic,
}

struct SpectrumApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SpectrumApplicationData>,
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

    #[doc = "Set the field `argo_smart_routing`.\nEnables Argo Smart Routing. Defaults to `false`."]
    pub fn set_argo_smart_routing(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().argo_smart_routing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_firewall`.\nEnables the IP Firewall for this application. Defaults to `true`."]
    pub fn set_ip_firewall(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ip_firewall = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_direct`.\nA list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`."]
    pub fn set_origin_direct(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().origin_direct = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_port`.\nOrigin port to proxy traffice to. Conflicts with `origin_port_range`."]
    pub fn set_origin_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().origin_port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_protocol`.\nEnables a proxy protocol to the origin. Available values: `off`, `v1`, `v2`, `simple`. Defaults to `off`."]
    pub fn set_proxy_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().proxy_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tls`.\nTLS configuration option for Cloudflare to connect to your origin. Available values: `off`, `flexible`, `full`, `strict`. Defaults to `off`."]
    pub fn set_tls(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tls = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `traffic_type`.\nSets application type. Available values: `direct`, `http`, `https`. Defaults to `direct`."]
    pub fn set_traffic_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().traffic_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns`.\n"]
    pub fn set_dns(self, v: impl std::convert::Into<terrars::BlockAssignable<SpectrumApplicationDnsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `edge_ips`.\n"]
    pub fn set_edge_ips(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpectrumApplicationEdgeIpsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().edge_ips = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.edge_ips = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `origin_dns`.\n"]
    pub fn set_origin_dns(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpectrumApplicationOriginDnsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin_dns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin_dns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `origin_port_range`.\n"]
    pub fn set_origin_port_range(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpectrumApplicationOriginPortRangeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin_port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin_port_range = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `argo_smart_routing` after provisioning.\nEnables Argo Smart Routing. Defaults to `false`."]
    pub fn argo_smart_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.argo_smart_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_firewall` after provisioning.\nEnables the IP Firewall for this application. Defaults to `true`."]
    pub fn ip_firewall(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_firewall", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_direct` after provisioning.\nA list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`."]
    pub fn origin_direct(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_direct", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_port` after provisioning.\nOrigin port to proxy traffice to. Conflicts with `origin_port_range`."]
    pub fn origin_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nThe port configuration at Cloudflare's edge. e.g. `tcp/22`."]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxy_protocol` after provisioning.\nEnables a proxy protocol to the origin. Available values: `off`, `v1`, `v2`, `simple`. Defaults to `off`."]
    pub fn proxy_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls` after provisioning.\nTLS configuration option for Cloudflare to connect to your origin. Available values: `off`, `flexible`, `full`, `strict`. Defaults to `off`."]
    pub fn tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic_type` after provisioning.\nSets application type. Available values: `direct`, `http`, `https`. Defaults to `direct`."]
    pub fn traffic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> terrars::ListRef<SpectrumApplicationDnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edge_ips` after provisioning.\n"]
    pub fn edge_ips(&self) -> terrars::ListRef<SpectrumApplicationEdgeIpsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edge_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_dns` after provisioning.\n"]
    pub fn origin_dns(&self) -> terrars::ListRef<SpectrumApplicationOriginDnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_port_range` after provisioning.\n"]
    pub fn origin_port_range(&self) -> terrars::ListRef<SpectrumApplicationOriginPortRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_port_range", self.extract_ref()))
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
    #[doc = "The port configuration at Cloudflare's edge. e.g. `tcp/22`."]
    pub protocol: terrars::PrimField<std::string::String>,
    #[doc = "The zone identifier to target for the resource."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildSpectrumApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> SpectrumApplication {
        let out = SpectrumApplication(std::rc::Rc::new(SpectrumApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SpectrumApplicationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
                dynamic: core::default::Default::default(),
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
        "Get a reference to the value of field `argo_smart_routing` after provisioning.\nEnables Argo Smart Routing. Defaults to `false`."]
    pub fn argo_smart_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.argo_smart_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_firewall` after provisioning.\nEnables the IP Firewall for this application. Defaults to `true`."]
    pub fn ip_firewall(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_firewall", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_direct` after provisioning.\nA list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`."]
    pub fn origin_direct(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_direct", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_port` after provisioning.\nOrigin port to proxy traffice to. Conflicts with `origin_port_range`."]
    pub fn origin_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nThe port configuration at Cloudflare's edge. e.g. `tcp/22`."]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxy_protocol` after provisioning.\nEnables a proxy protocol to the origin. Available values: `off`, `v1`, `v2`, `simple`. Defaults to `off`."]
    pub fn proxy_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls` after provisioning.\nTLS configuration option for Cloudflare to connect to your origin. Available values: `off`, `flexible`, `full`, `strict`. Defaults to `off`."]
    pub fn tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic_type` after provisioning.\nSets application type. Available values: `direct`, `http`, `https`. Defaults to `direct`."]
    pub fn traffic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> terrars::ListRef<SpectrumApplicationDnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edge_ips` after provisioning.\n"]
    pub fn edge_ips(&self) -> terrars::ListRef<SpectrumApplicationEdgeIpsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edge_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_dns` after provisioning.\n"]
    pub fn origin_dns(&self) -> terrars::ListRef<SpectrumApplicationOriginDnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_port_range` after provisioning.\n"]
    pub fn origin_port_range(&self) -> terrars::ListRef<SpectrumApplicationOriginPortRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_port_range", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SpectrumApplicationDnsEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl SpectrumApplicationDnsEl { }

impl terrars::ToListMappable for SpectrumApplicationDnsEl {
    type O = terrars::BlockAssignable<SpectrumApplicationDnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpectrumApplicationDnsEl {
    #[doc = "The name of the DNS record associated with the application."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The type of DNS record associated with the application."]
    pub type_: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpectrumApplicationDnsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpectrumApplicationDnsElRef {
        SpectrumApplicationDnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpectrumApplicationDnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the DNS record associated with the application."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of DNS record associated with the application."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct SpectrumApplicationEdgeIpsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connectivity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl SpectrumApplicationEdgeIpsEl {
    #[doc =
        "Set the field `connectivity`.\nThe IP versions supported for inbound connections on Spectrum anycast IPs. Required when `type` is not `static`. Available values: `all`, `ipv4`, `ipv6`."]
    pub fn set_connectivity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connectivity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ips`.\nThe collection of customer owned IPs to broadcast via anycast for this hostname and application. Requires [Bring Your Own IP](https://developers.cloudflare.com/spectrum/getting-started/byoip/) provisioned."]
    pub fn set_ips(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ips = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpectrumApplicationEdgeIpsEl {
    type O = terrars::BlockAssignable<SpectrumApplicationEdgeIpsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpectrumApplicationEdgeIpsEl {
    #[doc = "The type of edge IP configuration specified. Available values: `dynamic`, `static`."]
    pub type_: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpectrumApplicationEdgeIpsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpectrumApplicationEdgeIpsElRef {
        SpectrumApplicationEdgeIpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpectrumApplicationEdgeIpsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connectivity` after provisioning.\nThe IP versions supported for inbound connections on Spectrum anycast IPs. Required when `type` is not `static`. Available values: `all`, `ipv4`, `ipv6`."]
    pub fn connectivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connectivity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ips` after provisioning.\nThe collection of customer owned IPs to broadcast via anycast for this hostname and application. Requires [Bring Your Own IP](https://developers.cloudflare.com/spectrum/getting-started/byoip/) provisioned."]
    pub fn ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of edge IP configuration specified. Available values: `dynamic`, `static`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct SpectrumApplicationOriginDnsEl {
    name: terrars::PrimField<std::string::String>,
}

impl SpectrumApplicationOriginDnsEl { }

impl terrars::ToListMappable for SpectrumApplicationOriginDnsEl {
    type O = terrars::BlockAssignable<SpectrumApplicationOriginDnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpectrumApplicationOriginDnsEl {
    #[doc = "Fully qualified domain name of the origin."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSpectrumApplicationOriginDnsEl {
    pub fn build(self) -> SpectrumApplicationOriginDnsEl {
        SpectrumApplicationOriginDnsEl { name: self.name }
    }
}

pub struct SpectrumApplicationOriginDnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpectrumApplicationOriginDnsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpectrumApplicationOriginDnsElRef {
        SpectrumApplicationOriginDnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpectrumApplicationOriginDnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFully qualified domain name of the origin."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct SpectrumApplicationOriginPortRangeEl {
    end: terrars::PrimField<f64>,
    start: terrars::PrimField<f64>,
}

impl SpectrumApplicationOriginPortRangeEl { }

impl terrars::ToListMappable for SpectrumApplicationOriginPortRangeEl {
    type O = terrars::BlockAssignable<SpectrumApplicationOriginPortRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpectrumApplicationOriginPortRangeEl {
    #[doc = "Upper bound of the origin port range."]
    pub end: terrars::PrimField<f64>,
    #[doc = "Lower bound of the origin port range."]
    pub start: terrars::PrimField<f64>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpectrumApplicationOriginPortRangeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpectrumApplicationOriginPortRangeElRef {
        SpectrumApplicationOriginPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpectrumApplicationOriginPortRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\nUpper bound of the origin port range."]
    pub fn end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\nLower bound of the origin port range."]
    pub fn start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpectrumApplicationDynamic {
    dns: std::option::Option<terrars::DynamicBlock<SpectrumApplicationDnsEl>>,
    edge_ips: std::option::Option<terrars::DynamicBlock<SpectrumApplicationEdgeIpsEl>>,
    origin_dns: std::option::Option<terrars::DynamicBlock<SpectrumApplicationOriginDnsEl>>,
    origin_port_range: std::option::Option<terrars::DynamicBlock<SpectrumApplicationOriginPortRangeEl>>,
}

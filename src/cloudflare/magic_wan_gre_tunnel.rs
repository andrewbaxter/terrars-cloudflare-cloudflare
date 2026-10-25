#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct MagicWanGreTunnelData_ {
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
    automatic_return_routing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp: std::option::Option<MagicWanGreTunnel_Bgp>,
    cloudflare_gre_endpoint: terrars::PrimField<std::string::String>,
    customer_gre_endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<MagicWanGreTunnel_HealthCheck>,
    interface_address: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_address6: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mtu: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
}

struct MagicWanGreTunnel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MagicWanGreTunnelData_>,
}

#[derive(Clone)]
pub struct MagicWanGreTunnel(std::rc::Rc<MagicWanGreTunnel_>);

impl MagicWanGreTunnel {
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
        "Set the field `automatic_return_routing`.\nTrue if automatic stateful return routing should be enabled for a tunnel, false otherwise."]
    pub fn set_automatic_return_routing(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().automatic_return_routing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bgp`.\n"]
    pub fn set_bgp(self, v: impl std::convert::Into<MagicWanGreTunnel_Bgp>) -> Self {
        self.0.data.borrow_mut().bgp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn optional description of the GRE tunnel."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(self, v: impl std::convert::Into<MagicWanGreTunnel_HealthCheck>) -> Self {
        self.0.data.borrow_mut().health_check = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interface_address6`.\nA 127 bit IPV6 prefix from within the virtual_subnet6 prefix space with the address being the first IP of the subnet and not same as the address of virtual_subnet6. Eg if virtual_subnet6 is 2606:54c1:7:0:a9fe:12d2::/127 , interface_address6 could be 2606:54c1:7:0:a9fe:12d2:1:200/127"]
    pub fn set_interface_address6(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().interface_address6 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mtu`.\nMaximum Transmission Unit (MTU) in bytes for the GRE tunnel. The minimum value is 576."]
    pub fn set_mtu(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().mtu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\nTime To Live (TTL) in number of hops of the GRE tunnel."]
    pub fn set_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `automatic_return_routing` after provisioning.\nTrue if automatic stateful return routing should be enabled for a tunnel, false otherwise."]
    pub fn automatic_return_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_return_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp` after provisioning.\n"]
    pub fn bgp(&self) -> MagicWanGreTunnel_BgpRef {
        MagicWanGreTunnel_BgpRef::new(self.shared().clone(), format!("{}.bgp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_status` after provisioning.\n"]
    pub fn bgp_status(&self) -> MagicWanGreTunnel_BgpStatusRef {
        MagicWanGreTunnel_BgpStatusRef::new(self.shared().clone(), format!("{}.bgp_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_gre_endpoint` after provisioning.\nThe IP address assigned to the Cloudflare side of the GRE tunnel."]
    pub fn cloudflare_gre_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_gre_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe date and time the tunnel was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_gre_endpoint` after provisioning.\nThe IP address assigned to the customer side of the GRE tunnel."]
    pub fn customer_gre_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_gre_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of the GRE tunnel."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> MagicWanGreTunnel_HealthCheckRef {
        MagicWanGreTunnel_HealthCheckRef::new(self.shared().clone(), format!("{}.health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interface_address` after provisioning.\nA 31-bit prefix (/31 in CIDR notation) supporting two hosts, one for each side of the tunnel. Select the subnet from the following private IP space: 10.0.0.0–10.255.255.255, 172.16.0.0–172.31.255.255, 192.168.0.0–192.168.255.255."]
    pub fn interface_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interface_address6` after provisioning.\nA 127 bit IPV6 prefix from within the virtual_subnet6 prefix space with the address being the first IP of the subnet and not same as the address of virtual_subnet6. Eg if virtual_subnet6 is 2606:54c1:7:0:a9fe:12d2::/127 , interface_address6 could be 2606:54c1:7:0:a9fe:12d2:1:200/127"]
    pub fn interface_address6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_address6", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe date and time the tunnel was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mtu` after provisioning.\nMaximum Transmission Unit (MTU) in bytes for the GRE tunnel. The minimum value is 576."]
    pub fn mtu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the tunnel. The name cannot contain spaces or special characters, must be 15 characters or less, and cannot share a name with another GRE tunnel."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nTime To Live (TTL) in number of hops of the GRE tunnel."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}

impl terrars::Referable for MagicWanGreTunnel {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MagicWanGreTunnel { }

impl terrars::ToListMappable for MagicWanGreTunnel {
    type O = terrars::ListRef<MagicWanGreTunnelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MagicWanGreTunnel_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_magic_wan_gre_tunnel".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMagicWanGreTunnel {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "The IP address assigned to the Cloudflare side of the GRE tunnel."]
    pub cloudflare_gre_endpoint: terrars::PrimField<std::string::String>,
    #[doc = "The IP address assigned to the customer side of the GRE tunnel."]
    pub customer_gre_endpoint: terrars::PrimField<std::string::String>,
    #[doc =
        "A 31-bit prefix (/31 in CIDR notation) supporting two hosts, one for each side of the tunnel. Select the subnet from the following private IP space: 10.0.0.0–10.255.255.255, 172.16.0.0–172.31.255.255, 192.168.0.0–192.168.255.255."]
    pub interface_address: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the tunnel. The name cannot contain spaces or special characters, must be 15 characters or less, and cannot share a name with another GRE tunnel."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMagicWanGreTunnel {
    pub fn build(self, stack: &mut terrars::Stack) -> MagicWanGreTunnel {
        let out = MagicWanGreTunnel(std::rc::Rc::new(MagicWanGreTunnel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MagicWanGreTunnelData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                automatic_return_routing: core::default::Default::default(),
                bgp: core::default::Default::default(),
                cloudflare_gre_endpoint: self.cloudflare_gre_endpoint,
                customer_gre_endpoint: self.customer_gre_endpoint,
                description: core::default::Default::default(),
                health_check: core::default::Default::default(),
                interface_address: self.interface_address,
                interface_address6: core::default::Default::default(),
                mtu: core::default::Default::default(),
                name: self.name,
                ttl: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MagicWanGreTunnelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicWanGreTunnelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MagicWanGreTunnelRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `automatic_return_routing` after provisioning.\nTrue if automatic stateful return routing should be enabled for a tunnel, false otherwise."]
    pub fn automatic_return_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_return_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp` after provisioning.\n"]
    pub fn bgp(&self) -> MagicWanGreTunnel_BgpRef {
        MagicWanGreTunnel_BgpRef::new(self.shared().clone(), format!("{}.bgp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_status` after provisioning.\n"]
    pub fn bgp_status(&self) -> MagicWanGreTunnel_BgpStatusRef {
        MagicWanGreTunnel_BgpStatusRef::new(self.shared().clone(), format!("{}.bgp_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_gre_endpoint` after provisioning.\nThe IP address assigned to the Cloudflare side of the GRE tunnel."]
    pub fn cloudflare_gre_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_gre_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe date and time the tunnel was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_gre_endpoint` after provisioning.\nThe IP address assigned to the customer side of the GRE tunnel."]
    pub fn customer_gre_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_gre_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of the GRE tunnel."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> MagicWanGreTunnel_HealthCheckRef {
        MagicWanGreTunnel_HealthCheckRef::new(self.shared().clone(), format!("{}.health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interface_address` after provisioning.\nA 31-bit prefix (/31 in CIDR notation) supporting two hosts, one for each side of the tunnel. Select the subnet from the following private IP space: 10.0.0.0–10.255.255.255, 172.16.0.0–172.31.255.255, 192.168.0.0–192.168.255.255."]
    pub fn interface_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interface_address6` after provisioning.\nA 127 bit IPV6 prefix from within the virtual_subnet6 prefix space with the address being the first IP of the subnet and not same as the address of virtual_subnet6. Eg if virtual_subnet6 is 2606:54c1:7:0:a9fe:12d2::/127 , interface_address6 could be 2606:54c1:7:0:a9fe:12d2:1:200/127"]
    pub fn interface_address6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_address6", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe date and time the tunnel was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mtu` after provisioning.\nMaximum Transmission Unit (MTU) in bytes for the GRE tunnel. The minimum value is 576."]
    pub fn mtu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the tunnel. The name cannot contain spaces or special characters, must be 15 characters or less, and cannot share a name with another GRE tunnel."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nTime To Live (TTL) in number of hops of the GRE tunnel."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MagicWanGreTunnel_Bgp {
    customer_asn: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_filter_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extra_prefixes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    import_filter_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    md5_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MagicWanGreTunnel_Bgp {
    #[doc =
        "Set the field `export_filter_id`.\nUUID of the BGP filter profile to apply to routes advertised by Cloudflare."]
    pub fn set_export_filter_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.export_filter_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `extra_prefixes`.\nPrefixes in this list will be advertised to the customer device, in addition to the routes in the Magic routing table."]
    pub fn set_extra_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.extra_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `import_filter_id`.\nUUID of the BGP filter profile to apply to routes advertised to Cloudflare."]
    pub fn set_import_filter_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.import_filter_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `md5_key`.\nMD5 key to use for session authentication.\n\nNote that *this is not a security measure*. MD5 is not a valid security mechanism, and the\nkey is not treated as a secret value. This is *only* supported for preventing\nmisconfiguration, not for defending against malicious attacks.\n\nThe MD5 key, if set, must be of non-zero length and consist only of the following types of\ncharacter:\n\n* ASCII alphanumerics: `[a-zA-Z0-9]`\n* Special characters in the set `'!@#$%^&*()+[]{}<>/.,;:_-~`= \\|`\n\nIn other words, MD5 keys may contain any printable ASCII character aside from newline (0x0A),\nquotation mark (`\"`), vertical tab (0x0B), carriage return (0x0D), tab (0x09), form feed\n(0x0C), and the question mark (`?`). Requests specifying an MD5 key with one or more of\nthese disallowed characters will be rejected."]
    pub fn set_md5_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.md5_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicWanGreTunnel_Bgp {
    type O = terrars::BlockAssignable<MagicWanGreTunnel_Bgp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicWanGreTunnel_Bgp {
    #[doc = "ASN used on the customer end of the BGP session"]
    pub customer_asn: terrars::PrimField<f64>,
}

impl BuildMagicWanGreTunnel_Bgp {
    pub fn build(self) -> MagicWanGreTunnel_Bgp {
        MagicWanGreTunnel_Bgp {
            customer_asn: self.customer_asn,
            export_filter_id: core::default::Default::default(),
            extra_prefixes: core::default::Default::default(),
            import_filter_id: core::default::Default::default(),
            md5_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MagicWanGreTunnel_BgpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicWanGreTunnel_BgpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicWanGreTunnel_BgpRef {
        MagicWanGreTunnel_BgpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicWanGreTunnel_BgpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `customer_asn` after provisioning.\nASN used on the customer end of the BGP session"]
    pub fn customer_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_asn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `export_filter_id` after provisioning.\nUUID of the BGP filter profile to apply to routes advertised by Cloudflare."]
    pub fn export_filter_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_filter_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `extra_prefixes` after provisioning.\nPrefixes in this list will be advertised to the customer device, in addition to the routes in the Magic routing table."]
    pub fn extra_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extra_prefixes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `import_filter_id` after provisioning.\nUUID of the BGP filter profile to apply to routes advertised to Cloudflare."]
    pub fn import_filter_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_filter_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `md5_key` after provisioning.\nMD5 key to use for session authentication.\n\nNote that *this is not a security measure*. MD5 is not a valid security mechanism, and the\nkey is not treated as a secret value. This is *only* supported for preventing\nmisconfiguration, not for defending against malicious attacks.\n\nThe MD5 key, if set, must be of non-zero length and consist only of the following types of\ncharacter:\n\n* ASCII alphanumerics: `[a-zA-Z0-9]`\n* Special characters in the set `'!@#$%^&*()+[]{}<>/.,;:_-~`= \\|`\n\nIn other words, MD5 keys may contain any printable ASCII character aside from newline (0x0A),\nquotation mark (`\"`), vertical tab (0x0B), carriage return (0x0D), tab (0x09), form feed\n(0x0C), and the question mark (`?`). Requests specifying an MD5 key with one or more of\nthese disallowed characters will be rejected."]
    pub fn md5_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.md5_key", self.base))
    }
}

#[derive(Serialize)]
pub struct MagicWanGreTunnel_BgpStatus {}

impl MagicWanGreTunnel_BgpStatus { }

impl terrars::ToListMappable for MagicWanGreTunnel_BgpStatus {
    type O = terrars::BlockAssignable<MagicWanGreTunnel_BgpStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicWanGreTunnel_BgpStatus {}

impl BuildMagicWanGreTunnel_BgpStatus {
    pub fn build(self) -> MagicWanGreTunnel_BgpStatus {
        MagicWanGreTunnel_BgpStatus {}
    }
}

#[allow(dead_code)]
pub struct MagicWanGreTunnel_BgpStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicWanGreTunnel_BgpStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicWanGreTunnel_BgpStatusRef {
        MagicWanGreTunnel_BgpStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicWanGreTunnel_BgpStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bgp_state` after provisioning.\n"]
    pub fn bgp_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgp_state", self.base))
    }

    #[doc = "Get a reference to the value of field `cf_speaker_ip` after provisioning.\n"]
    pub fn cf_speaker_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cf_speaker_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `cf_speaker_port` after provisioning.\n"]
    pub fn cf_speaker_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cf_speaker_port", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_speaker_ip` after provisioning.\n"]
    pub fn customer_speaker_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_speaker_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_speaker_port` after provisioning.\n"]
    pub fn customer_speaker_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_speaker_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nAvailable values: \"BGP_DOWN\", \"BGP_UP\", \"BGP_ESTABLISHING\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_established` after provisioning.\n"]
    pub fn tcp_established(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_established", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct MagicWanGreTunnel_HealthCheck_Target {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saved: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MagicWanGreTunnel_HealthCheck_Target {
    #[doc =
        "Set the field `saved`.\nThe saved health check target. Setting the value to the empty string indicates that the calculated default value will be used."]
    pub fn set_saved(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.saved = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicWanGreTunnel_HealthCheck_Target {
    type O = terrars::BlockAssignable<MagicWanGreTunnel_HealthCheck_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicWanGreTunnel_HealthCheck_Target {}

impl BuildMagicWanGreTunnel_HealthCheck_Target {
    pub fn build(self) -> MagicWanGreTunnel_HealthCheck_Target {
        MagicWanGreTunnel_HealthCheck_Target { saved: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MagicWanGreTunnel_HealthCheck_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicWanGreTunnel_HealthCheck_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicWanGreTunnel_HealthCheck_TargetRef {
        MagicWanGreTunnel_HealthCheck_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicWanGreTunnel_HealthCheck_TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effective` after provisioning.\nThe effective health check target. If 'saved' is empty, then this field will be populated with the calculated default value on GET requests. Ignored in POST, PUT, and PATCH requests."]
    pub fn effective(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective", self.base))
    }

    #[doc =
        "Get a reference to the value of field `saved` after provisioning.\nThe saved health check target. Setting the value to the empty string indicates that the calculated default value will be used."]
    pub fn saved(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.saved", self.base))
    }
}

#[derive(Serialize)]
pub struct MagicWanGreTunnel_HealthCheck {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<MagicWanGreTunnel_HealthCheck_Target>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MagicWanGreTunnel_HealthCheck {
    #[doc =
        "Set the field `direction`.\nThe direction of the flow of the healthcheck. Either unidirectional, where the probe comes to you via the tunnel and the result comes back to Cloudflare via the open Internet, or bidirectional where both the probe and result come and go via the tunnel.\nAvailable values: \"unidirectional\", \"bidirectional\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nDetermines whether to run healthchecks for a tunnel."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rate`.\nHow frequent the health check is run. The default value is `mid`.\nAvailable values: \"low\", \"mid\", \"high\"."]
    pub fn set_rate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target`.\nThe destination address in a request type health check. After the healthcheck is decapsulated at the customer end of the tunnel, the ICMP echo will be forwarded to this address. This field defaults to `customer_gre_endpoint address`. This field is ignored for bidirectional healthchecks as the interface_address (not assigned to the Cloudflare side of the tunnel) is used as the target. Must be in object form if the x-magic-new-hc-target header is set to true and string form if x-magic-new-hc-target is absent or set to false."]
    pub fn set_target(mut self, v: impl std::convert::Into<MagicWanGreTunnel_HealthCheck_Target>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type of healthcheck to run, reply or request. The default value is `reply`.\nAvailable values: \"reply\", \"request\"."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicWanGreTunnel_HealthCheck {
    type O = terrars::BlockAssignable<MagicWanGreTunnel_HealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicWanGreTunnel_HealthCheck {}

impl BuildMagicWanGreTunnel_HealthCheck {
    pub fn build(self) -> MagicWanGreTunnel_HealthCheck {
        MagicWanGreTunnel_HealthCheck {
            direction: core::default::Default::default(),
            enabled: core::default::Default::default(),
            rate: core::default::Default::default(),
            target: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MagicWanGreTunnel_HealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicWanGreTunnel_HealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicWanGreTunnel_HealthCheckRef {
        MagicWanGreTunnel_HealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicWanGreTunnel_HealthCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe direction of the flow of the healthcheck. Either unidirectional, where the probe comes to you via the tunnel and the result comes back to Cloudflare via the open Internet, or bidirectional where both the probe and result come and go via the tunnel.\nAvailable values: \"unidirectional\", \"bidirectional\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nDetermines whether to run healthchecks for a tunnel."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rate` after provisioning.\nHow frequent the health check is run. The default value is `mid`.\nAvailable values: \"low\", \"mid\", \"high\"."]
    pub fn rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nThe destination address in a request type health check. After the healthcheck is decapsulated at the customer end of the tunnel, the ICMP echo will be forwarded to this address. This field defaults to `customer_gre_endpoint address`. This field is ignored for bidirectional healthchecks as the interface_address (not assigned to the Cloudflare side of the tunnel) is used as the target. Must be in object form if the x-magic-new-hc-target header is set to true and string form if x-magic-new-hc-target is absent or set to false."]
    pub fn target(&self) -> MagicWanGreTunnel_HealthCheck_TargetRef {
        MagicWanGreTunnel_HealthCheck_TargetRef::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of healthcheck to run, reply or request. The default value is `reply`.\nAvailable values: \"reply\", \"request\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

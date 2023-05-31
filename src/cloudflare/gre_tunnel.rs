use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct GreTunnelData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    cloudflare_gre_endpoint: PrimField<String>,
    customer_gre_endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    interface_address: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mtu: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
}

struct GreTunnel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GreTunnelData>,
}

#[derive(Clone)]
pub struct GreTunnel(Rc<GreTunnel_>);

impl GreTunnel {
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

    #[doc= "Set the field `account_id`.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nDescription of the GRE tunnel intent."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `health_check_enabled`.\nSpecifies if ICMP tunnel health checks are enabled."]
    pub fn set_health_check_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().health_check_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `health_check_target`.\nThe IP address of the customer endpoint that will receive tunnel health checks."]
    pub fn set_health_check_target(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().health_check_target = Some(v.into());
        self
    }

    #[doc= "Set the field `health_check_type`.\nSpecifies the ICMP echo type for the health check. Available values: `request`, `reply`."]
    pub fn set_health_check_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().health_check_type = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `mtu`.\nMaximum Transmission Unit (MTU) in bytes for the GRE tunnel."]
    pub fn set_mtu(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().mtu = Some(v.into());
        self
    }

    #[doc= "Set the field `ttl`.\nTime To Live (TTL) in number of hops of the GRE tunnel."]
    pub fn set_ttl(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ttl = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloudflare_gre_endpoint` after provisioning.\nThe IP address assigned to the Cloudflare side of the GRE tunnel."]
    pub fn cloudflare_gre_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_gre_endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_gre_endpoint` after provisioning.\nThe IP address assigned to the customer side of the GRE tunnel."]
    pub fn customer_gre_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_gre_endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the GRE tunnel intent."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_enabled` after provisioning.\nSpecifies if ICMP tunnel health checks are enabled."]
    pub fn health_check_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_target` after provisioning.\nThe IP address of the customer endpoint that will receive tunnel health checks."]
    pub fn health_check_target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_target", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_type` after provisioning.\nSpecifies the ICMP echo type for the health check. Available values: `request`, `reply`."]
    pub fn health_check_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interface_address` after provisioning.\n31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel."]
    pub fn interface_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mtu` after provisioning.\nMaximum Transmission Unit (MTU) in bytes for the GRE tunnel."]
    pub fn mtu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the GRE tunnel."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nTime To Live (TTL) in number of hops of the GRE tunnel."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}

impl Referable for GreTunnel {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GreTunnel { }

impl ToListMappable for GreTunnel {
    type O = ListRef<GreTunnelRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GreTunnel_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_gre_tunnel".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGreTunnel {
    pub tf_id: String,
    #[doc= "The IP address assigned to the Cloudflare side of the GRE tunnel."]
    pub cloudflare_gre_endpoint: PrimField<String>,
    #[doc= "The IP address assigned to the customer side of the GRE tunnel."]
    pub customer_gre_endpoint: PrimField<String>,
    #[doc= "31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel."]
    pub interface_address: PrimField<String>,
    #[doc= "Name of the GRE tunnel."]
    pub name: PrimField<String>,
}

impl BuildGreTunnel {
    pub fn build(self, stack: &mut Stack) -> GreTunnel {
        let out = GreTunnel(Rc::new(GreTunnel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GreTunnelData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                cloudflare_gre_endpoint: self.cloudflare_gre_endpoint,
                customer_gre_endpoint: self.customer_gre_endpoint,
                description: core::default::Default::default(),
                health_check_enabled: core::default::Default::default(),
                health_check_target: core::default::Default::default(),
                health_check_type: core::default::Default::default(),
                id: core::default::Default::default(),
                interface_address: self.interface_address,
                mtu: core::default::Default::default(),
                name: self.name,
                ttl: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GreTunnelRef {
    shared: StackShared,
    base: String,
}

impl Ref for GreTunnelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GreTunnelRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloudflare_gre_endpoint` after provisioning.\nThe IP address assigned to the Cloudflare side of the GRE tunnel."]
    pub fn cloudflare_gre_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_gre_endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_gre_endpoint` after provisioning.\nThe IP address assigned to the customer side of the GRE tunnel."]
    pub fn customer_gre_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_gre_endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the GRE tunnel intent."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_enabled` after provisioning.\nSpecifies if ICMP tunnel health checks are enabled."]
    pub fn health_check_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_target` after provisioning.\nThe IP address of the customer endpoint that will receive tunnel health checks."]
    pub fn health_check_target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_target", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_type` after provisioning.\nSpecifies the ICMP echo type for the health check. Available values: `request`, `reply`."]
    pub fn health_check_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interface_address` after provisioning.\n31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel."]
    pub fn interface_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mtu` after provisioning.\nMaximum Transmission Unit (MTU) in bytes for the GRE tunnel."]
    pub fn mtu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the GRE tunnel."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nTime To Live (TTL) in number of hops of the GRE tunnel."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}
